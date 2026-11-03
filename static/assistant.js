/* My personal assistant (docs/adr/0026): the composer and the thread.
   Without scripting every form still works: a message is sent by the form
   itself and the page asks again every three seconds while the AFG writes.
   With it, a message is sent while the page stays, files are dropped,
   pasted or chosen, and the answer is shown as it is written. */
(function () {
  "use strict";

  var reduce = window.matchMedia && window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  var coarse = window.matchMedia && window.matchMedia("(pointer: coarse)").matches;
  var composer = document.querySelector("[data-composer]");
  var sendButton = composer ? composer.querySelector(".as-send") : null;
  // ui.js says "sending" on the button while a form is out, and puts back
  // only its words: its icon is put back here, the page having stayed.
  var sendHtml = sendButton ? sendButton.innerHTML : "";
  function idleForm() {
    if (!composer) return;
    composer.removeAttribute("aria-busy");
    if (!sendButton) return;
    ["aria-busy", "aria-disabled", "data-idle"].forEach(function (a) { sendButton.removeAttribute(a); });
    sendButton.innerHTML = sendHtml;
  }

  // While the AFG writes, nothing more is sent in this conversation.
  function busy(on) {
    if (!composer) return;
    composer.classList.toggle("as-busy", !!on);
    if (sendButton) sendButton.disabled = !!on;
  }

  function nearBottom() {
    var doc = document.documentElement;
    return window.innerHeight + window.scrollY >= doc.scrollHeight - 160;
  }
  function toBottom(smooth) {
    window.scrollTo({ top: document.documentElement.scrollHeight, behavior: smooth && !reduce ? "smooth" : "auto" });
  }
  function fragment(html) {
    var t = document.createElement("template");
    t.innerHTML = html.trim();
    return t.content;
  }
  function size(bytes) {
    if (bytes < 1024) return bytes + " B";
    if (bytes < 1024 * 1024) return Math.ceil(bytes / 1024) + " KB";
    return (bytes / 1048576).toFixed(1) + " MB";
  }

  /* ----- the thread: what is new, and the answer being written ----- */
  var thread = document.querySelector("[data-thread]");
  var jump = null;
  var polling = false;
  var timer = null;

  function count() {
    return thread ? parseInt(thread.getAttribute("data-messages") || "0", 10) : 0;
  }
  function showJump(show) {
    if (!thread) return;
    if (!jump) {
      jump = document.createElement("button");
      jump.type = "button";
      jump.className = "btn as-jump";
      jump.textContent = "↓";
      jump.setAttribute("aria-label", thread.getAttribute("aria-label") || "");
      jump.addEventListener("click", function () { toBottom(true); showJump(false); });
      document.body.appendChild(jump);
    }
    jump.hidden = !show;
  }
  function settle(el) {
    if (reduce || !el || !el.classList) return;
    el.classList.add("as-enter");
    requestAnimationFrame(function () {
      requestAnimationFrame(function () { el.classList.remove("as-enter"); });
    });
  }
  function setLive(html) {
    var old = thread.querySelector("[data-live]");
    if (!html) {
      if (old) old.remove();
      return;
    }
    var node = fragment(html).firstElementChild;
    if (old) old.replaceWith(node);
    else { thread.appendChild(node); settle(node); }
  }
  function poll() {
    if (!thread) return;
    var url = thread.getAttribute("data-state") + "?after=" + count();
    fetch(url, { credentials: "same-origin", headers: { Accept: "application/json" } })
      .then(function (r) { return r.json(); })
      .then(function (d) {
        var follow = nearBottom();
        if (d.messages && d.messages.length) {
          var live = thread.querySelector("[data-live]");
          if (live) live.remove();
          d.messages.forEach(function (m) {
            var node = fragment(m.html).firstElementChild;
            thread.appendChild(node);
            settle(node);
          });
        }
        if (typeof d.count === "number") thread.setAttribute("data-messages", String(d.count));
        setLive(d.answering ? d.live : "");
        if (follow) toBottom(true); else if (d.messages && d.messages.length) showJump(true);
        if (d.answering) { timer = setTimeout(poll, 650); }
        else { polling = false; busy(false); }
      })
      .catch(function () { timer = setTimeout(poll, 3000); });
  }
  function startPolling() {
    if (polling) return;
    polling = true;
    clearTimeout(timer);
    timer = setTimeout(poll, 400);
  }
  if (thread) {
    toBottom(false);
    if (thread.querySelector("[data-live]")) { busy(true); startPolling(); }
    window.addEventListener("scroll", function () { if (nearBottom()) showJump(false); }, { passive: true });
  }

  /* ----- an answer that failed, asked for again ----- */
  document.addEventListener("submit", function (e) {
    // The submitted form itself, never an element around something else.
    var form = e.target;
    if (!form.hasAttribute || !form.hasAttribute("data-retry") || !thread || !window.fetch) return;
    e.preventDefault();
    fetch(form.action, { method: "POST", credentials: "same-origin", headers: { Accept: "application/json" } })
      .then(function (r) { return r.json(); })
      .then(function (d) {
        form.remove();
        if (d.answering) {
          setLive('<article class="as-msg as-msg-assistant as-live" data-live aria-busy="true"><div class="as-answer"><div class="as-typing" aria-hidden="true"><i></i><i></i><i></i></div></div></article>');
          busy(true);
          startPolling();
          toBottom(true);
        }
      });
  });

  /* ----- the composer ----- */
  if (!composer) return;
  var text = composer.querySelector("[data-composer-text]");
  var input = composer.querySelector("[data-composer-files]");
  var chips = composer.querySelector("[data-chips]");
  var note = composer.querySelector("[data-composer-note]");
  var box = composer.querySelector("[data-composer-box]");
  var maxFiles = parseInt(composer.getAttribute("data-max-files") || "6", 10);
  var maxBytes = parseInt(composer.getAttribute("data-max-mb") || "20", 10) * 1048576;
  var picked = [];
  var sending = false;

  function say(msg) {
    if (note) note.textContent = msg || "";
  }
  function grow() {
    if (!text) return;
    text.style.height = "auto";
    var most = Math.round(window.innerHeight * 0.4);
    text.style.height = Math.min(text.scrollHeight, most) + "px";
  }
  function syncInput() {
    // The form's own field holds the same files, for a browser that sends
    // it without this script.
    if (!input || typeof DataTransfer === "undefined") return;
    try {
      var dt = new DataTransfer();
      picked.forEach(function (f) { dt.items.add(f); });
      input.files = dt.files;
    } catch (err) { /* an older browser: the list stays this script's */ }
  }
  function drawChips() {
    if (!chips) return;
    chips.textContent = "";
    chips.hidden = picked.length === 0;
    picked.forEach(function (f, i) {
      var li = document.createElement("li");
      li.className = "as-chip";
      if (f.type && f.type.indexOf("image/") === 0 && window.URL) {
        var img = document.createElement("img");
        img.alt = "";
        img.src = URL.createObjectURL(f);
        img.onload = function () { URL.revokeObjectURL(img.src); };
        li.appendChild(img);
      }
      var name = document.createElement("span");
      name.className = "as-chip-name";
      name.textContent = f.name;
      li.appendChild(name);
      var weight = document.createElement("span");
      weight.className = "as-chip-size";
      weight.textContent = size(f.size);
      li.appendChild(weight);
      var x = document.createElement("button");
      x.type = "button";
      x.className = "as-chip-x";
      x.textContent = "×";
      x.setAttribute("aria-label", f.name);
      x.addEventListener("click", function () {
        picked.splice(i, 1);
        syncInput();
        drawChips();
      });
      li.appendChild(x);
      chips.appendChild(li);
    });
  }
  function add(files) {
    var refused = "";
    Array.prototype.forEach.call(files || [], function (f) {
      if (picked.length >= maxFiles) { refused = composer.getAttribute("data-too-many"); return; }
      if (f.size > maxBytes) { refused = composer.getAttribute("data-too-big"); return; }
      picked.push(f);
    });
    say(refused);
    syncInput();
    drawChips();
  }

  if (text) {
    text.addEventListener("input", grow);
    text.addEventListener("keydown", function (e) {
      if (e.key !== "Enter" || e.isComposing) return;
      var force = e.ctrlKey || e.metaKey;
      if (force || (!e.shiftKey && !coarse)) {
        e.preventDefault();
        if (sending || composer.classList.contains("as-busy")) return;
        if (composer.requestSubmit) composer.requestSubmit(sendButton || undefined);
        else composer.dispatchEvent(new Event("submit", { cancelable: true }));
      }
    });
    text.addEventListener("paste", function (e) {
      var items = e.clipboardData && e.clipboardData.files;
      if (items && items.length) { e.preventDefault(); add(items); }
    });
    grow();
  }
  if (input) {
    input.addEventListener("change", function () {
      var chosen = Array.prototype.slice.call(input.files || []);
      picked = [];
      add(chosen);
    });
  }
  var dropTarget = box || composer;
  ["dragenter", "dragover"].forEach(function (type) {
    document.addEventListener(type, function (e) {
      if (!e.dataTransfer || Array.prototype.indexOf.call(e.dataTransfer.types || [], "Files") < 0) return;
      e.preventDefault();
      dropTarget.classList.add("as-drop");
    });
  });
  ["dragleave", "drop"].forEach(function (type) {
    document.addEventListener(type, function (e) {
      if (type === "dragleave" && e.relatedTarget) return;
      dropTarget.classList.remove("as-drop");
      if (type === "drop" && e.dataTransfer && e.dataTransfer.files && e.dataTransfer.files.length) {
        e.preventDefault();
        add(e.dataTransfer.files);
        if (text) text.focus();
      }
    });
  });

  composer.addEventListener("submit", function (e) {
    if (!window.fetch || !window.FormData) return;
    e.preventDefault();
    if (sending) return;
    var words = text ? text.value.trim() : "";
    if (!words && !picked.length) { if (text) text.focus(); return; }
    var data = new FormData();
    data.append("text", words);
    picked.forEach(function (f) { data.append("files", f, f.name); });
    sending = true;
    busy(true);
    say(composer.getAttribute("data-sending"));
    fetch(composer.action, {
      method: "POST",
      body: data,
      credentials: "same-origin",
      headers: { Accept: "application/json" }
    })
      .then(function (r) {
        return r.json().then(function (d) { return { ok: r.ok, d: d }; });
      })
      .then(function (res) {
        sending = false;
        idleForm();
        var d = res.d || {};
        if (!res.ok || d.error) {
          busy(false);
          say(d.error || "");
          return;
        }
        if (d.redirect) { window.location.assign(d.redirect); return; }
        say("");
        if (text) { text.value = ""; grow(); }
        picked = [];
        syncInput();
        drawChips();
        if (thread && d.message) {
          var node = fragment(d.message.html).firstElementChild;
          var live = thread.querySelector("[data-live]");
          if (live) thread.insertBefore(node, live); else thread.appendChild(node);
          settle(node);
          if (typeof d.count === "number") thread.setAttribute("data-messages", String(d.count));
          if (d.answering) {
            setLive('<article class="as-msg as-msg-assistant as-live" data-live aria-busy="true"><div class="as-answer"><div class="as-typing" aria-hidden="true"><i></i><i></i><i></i></div></div></article>');
            startPolling();
          } else {
            busy(false);
          }
          toBottom(true);
        } else {
          busy(false);
        }
      })
      .catch(function () {
        sending = false;
        idleForm();
        busy(false);
        composer.submit();
      });
  });

  document.addEventListener("keydown", function (e) {
    if (e.key !== "Escape") return;
    var open = document.querySelector(".as-menu[open]");
    if (open) open.removeAttribute("open");
  });
})();
