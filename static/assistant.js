/* My personal assistant (docs/adr/0026, docs/adr/0050): the composer, the
   thread and the project's knowledge. Without scripting every form still
   works: a message is sent by the form itself, the page asks again every
   three seconds while the AFG writes, and the answer is stopped from a
   button inside it. With it, a message is sent while the page stays, files
   are dropped, pasted, chosen or photographed, words are dictated, the
   answer is shown as it is written and stopped from the composer. */
(function () {
  "use strict";

  var reduce = window.matchMedia && window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  var coarse = window.matchMedia && window.matchMedia("(pointer: coarse)").matches;
  // How long an answer may take before the composer says it can be stopped.
  var SLOW_MS = 30000;
  var composer = document.querySelector("[data-composer]");
  var sendButton = composer ? composer.querySelector(".as-send") : null;
  var stopButton = composer ? composer.querySelector("[data-composer-stop]") : null;
  var stopUrl = composer ? composer.getAttribute("data-stop") : null;
  var note = composer ? composer.querySelector("[data-composer-note]") : null;

  function say(msg) {
    if (note) note.textContent = msg || "";
  }
  // ui.js says "sending" in the button's hidden words while a form is out
  // (`data-busy-words`); the page having stayed, they are put back here.
  function idleForm() {
    if (!composer) return;
    composer.removeAttribute("aria-busy");
    if (!sendButton) return;
    var words = sendButton.querySelector("[data-busy-words]");
    var idle = sendButton.getAttribute("data-idle");
    if (words && idle !== null) words.textContent = idle;
    ["aria-busy", "aria-disabled", "data-idle"].forEach(function (a) { sendButton.removeAttribute(a); });
  }

  // While the AFG writes, nothing more is sent in this conversation, and in
  // the Send button's place is the button that stops the answer. After a
  // while the composer says so, and the stop button asks to be seen.
  var slowTimer = null;
  function busy(on) {
    if (!composer) return;
    composer.classList.toggle("as-busy", !!on);
    if (sendButton) sendButton.disabled = !!on;
    clearTimeout(slowTimer);
    if (stopButton && stopUrl) {
      stopButton.hidden = !on;
      stopButton.disabled = false;
      stopButton.classList.remove("as-stop-urge");
      if (sendButton) sendButton.hidden = !!on;
      if (on) {
        slowTimer = setTimeout(function () {
          say(composer.getAttribute("data-slow"));
          stopButton.classList.add("as-stop-urge");
        }, SLOW_MS);
      }
    }
    var said = note ? note.textContent : "";
    if (!on && (said === composer.getAttribute("data-slow") || said === composer.getAttribute("data-stopping"))) say("");
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
  var jump = document.querySelector("[data-jump]");
  var liveTemplate = document.querySelector("[data-live-template]");
  var polling = false;
  var timer = null;
  // Each loop of polling has its number: a reply that comes back after a
  // new loop started (a stop) is let go, so that no message comes twice.
  var round = 0;

  function count() {
    return thread ? parseInt(thread.getAttribute("data-messages") || "0", 10) : 0;
  }
  function showJump(show) {
    if (jump) jump.hidden = !show;
  }
  if (jump) jump.addEventListener("click", function () { toBottom(true); showJump(false); });
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
  // The answer being written before its first word: the page's own
  // fragment, drawn by the server like every other.
  function startLive() {
    if (!thread || !liveTemplate) return;
    setLive(liveTemplate.innerHTML);
  }
  function poll() {
    if (!thread) return;
    var mine = round;
    var url = thread.getAttribute("data-state") + "?after=" + count();
    fetch(url, { credentials: "same-origin", headers: { Accept: "application/json" } })
      .then(function (r) { return r.json(); })
      .then(function (d) {
        if (mine !== round) return;
        var follow = nearBottom();
        if (d.messages && d.messages.length) {
          var live = thread.querySelector("[data-live]");
          d.messages.forEach(function (m) {
            var node = fragment(m.html).firstElementChild;
            var id = node && node.getAttribute("data-msg");
            if (id && thread.querySelector('[data-msg="' + id + '"]')) return;
            // The finished answer takes the place of the one being written,
            // without coming in again.
            if (live) { live.replaceWith(node); live = null; }
            else { thread.appendChild(node); settle(node); }
          });
        }
        if (typeof d.count === "number") thread.setAttribute("data-messages", String(d.count));
        setLive(d.answering ? d.live : "");
        if (follow) toBottom(true); else if (d.messages && d.messages.length) showJump(true);
        if (d.answering) { timer = setTimeout(poll, 650); }
        else { polling = false; busy(false); }
      })
      .catch(function () { if (mine === round) timer = setTimeout(poll, 3000); });
  }
  function startPolling() {
    if (polling) return;
    polling = true;
    round += 1;
    clearTimeout(timer);
    timer = setTimeout(poll, 400);
  }
  if (thread) {
    // The conversation grows down the page: it opens on its last message.
    toBottom(false);
    if (thread.querySelector("[data-live]")) { busy(true); startPolling(); }
    window.addEventListener("scroll", function () { if (nearBottom()) showJump(false); }, { passive: true });
  }

  /* ----- an answer stopped by the member ----- */
  if (stopButton && stopUrl && window.fetch) {
    stopButton.addEventListener("click", function () {
      stopButton.disabled = true;
      clearTimeout(slowTimer);
      say(composer.getAttribute("data-stopping"));
      fetch(stopUrl, { method: "POST", credentials: "same-origin", headers: { Accept: "application/json" } })
        .then(function (r) { return r.json(); })
        .then(function () {
          // The answer ends as any other does: the thread hears of it.
          clearTimeout(timer);
          polling = false;
          startPolling();
        })
        .catch(function () {
          stopButton.disabled = false;
          say("");
        });
    });
  }

  /* ----- an answer that failed or was stopped, asked for again ----- */
  document.addEventListener("submit", function (e) {
    // The submitted form itself, never an element around something else.
    var form = e.target;
    if (!form.hasAttribute || !form.hasAttribute("data-retry") || !thread || !window.fetch) return;
    e.preventDefault();
    var button = form.querySelector("button");
    if (button) button.disabled = true;
    fetch(form.action, { method: "POST", credentials: "same-origin", headers: { Accept: "application/json" } })
      .then(function (r) { return r.json(); })
      .then(function (d) {
        if (d.answering) {
          form.remove();
          startLive();
          busy(true);
          startPolling();
          toBottom(true);
        } else {
          // Nothing was asked: the page says why, as it would without this
          // script (the AFG closed, or already answering).
          window.location.reload();
        }
      })
      .catch(function () { form.submit(); });
  });

  /* ----- the project's knowledge: chosen or dropped, then sent ----- */
  var knowledge = document.querySelector("[data-knowledge]");
  var knowledgeInput = knowledge ? knowledge.querySelector("[data-knowledge-files]") : null;
  function sendKnowledge(files) {
    if (!knowledge || !knowledgeInput) return;
    var most = parseInt(knowledgeInput.getAttribute("data-max-bytes") || "0", 10);
    var heavy = most && Array.prototype.some.call(files, function (f) { return f.size > most; });
    if (heavy) {
      knowledgeInput.setCustomValidity(knowledgeInput.getAttribute("data-too-large") || "");
      knowledgeInput.reportValidity();
      knowledgeInput.value = "";
      return;
    }
    knowledgeInput.setCustomValidity("");
    if (files !== knowledgeInput.files) {
      try {
        var dt = new DataTransfer();
        Array.prototype.forEach.call(files, function (f) { dt.items.add(f); });
        knowledgeInput.files = dt.files;
      } catch (err) { return; }
    }
    if (!knowledgeInput.files || !knowledgeInput.files.length) return;
    var button = knowledge.querySelector("button[type=submit]");
    if (knowledge.requestSubmit) knowledge.requestSubmit(button || undefined);
    else knowledge.submit();
  }
  if (knowledgeInput) {
    knowledgeInput.addEventListener("change", function () { sendKnowledge(knowledgeInput.files); });
  }
  if (location.hash === "#knowledge") {
    var panel = document.getElementById("knowledge");
    if (panel) panel.open = true;
  }

  /* ----- dropped files: to the knowledge when dropped on it, else to the
     composer ----- */
  function hasFiles(e) {
    return e.dataTransfer && Array.prototype.indexOf.call(e.dataTransfer.types || [], "Files") >= 0;
  }
  function overKnowledge(e) {
    return knowledge && e.target instanceof Node && knowledge.contains(e.target);
  }
  var dropZone = knowledge ? knowledge.querySelector(".as-know-drop") : null;

  /* ----- the composer ----- */
  var text = composer ? composer.querySelector("[data-composer-text]") : null;
  var input = composer ? composer.querySelector("[data-composer-files]") : null;
  var camera = composer ? composer.querySelector("[data-composer-camera]") : null;
  var mic = composer ? composer.querySelector("[data-composer-mic]") : null;
  var chips = composer ? composer.querySelector("[data-chips]") : null;
  var box = composer ? composer.querySelector("[data-composer-box]") : null;
  var maxFiles = composer ? parseInt(composer.getAttribute("data-max-files") || "6", 10) : 0;
  var maxBytes = composer ? parseInt(composer.getAttribute("data-max-mb") || "20", 10) * 1048576 : 0;
  var picked = [];
  var sending = false;

  ["dragenter", "dragover"].forEach(function (type) {
    document.addEventListener(type, function (e) {
      if (!hasFiles(e)) return;
      if (overKnowledge(e)) {
        e.preventDefault();
        if (dropZone) dropZone.classList.add("as-drop");
        if (box) box.classList.remove("as-drop");
        return;
      }
      if (dropZone) dropZone.classList.remove("as-drop");
      // A composer that takes no file (a person's page) takes no drop.
      if (!input) return;
      e.preventDefault();
      (box || composer).classList.add("as-drop");
    });
  });
  ["dragleave", "drop"].forEach(function (type) {
    document.addEventListener(type, function (e) {
      if (type === "dragleave" && e.relatedTarget) return;
      if (dropZone) dropZone.classList.remove("as-drop");
      if (box) box.classList.remove("as-drop");
      if (type !== "drop" || !e.dataTransfer || !e.dataTransfer.files || !e.dataTransfer.files.length) return;
      if (overKnowledge(e)) {
        e.preventDefault();
        sendKnowledge(e.dataTransfer.files);
        return;
      }
      if (!input) return;
      e.preventDefault();
      add(e.dataTransfer.files);
      if (text) text.focus();
    });
  });

  if (!composer) return;

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
        var url = URL.createObjectURL(f);
        img.alt = "";
        img.addEventListener("load", function () { URL.revokeObjectURL(url); });
        img.addEventListener("error", function () { URL.revokeObjectURL(url); img.remove(); });
        img.src = url;
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
  // Files join the ones already chosen: choosing, pasting, dropping or
  // photographing again adds, it never replaces.
  function add(files) {
    var refused = "";
    Array.prototype.forEach.call(files || [], function (f) {
      var same = picked.some(function (p) { return p.name === f.name && p.size === f.size && p.lastModified === f.lastModified; });
      if (same) return;
      if (picked.length >= maxFiles) { refused = composer.getAttribute("data-too-many"); return; }
      if (f.size > maxBytes) { refused = composer.getAttribute("data-too-big"); return; }
      picked.push(f);
    });
    say(refused);
    syncInput();
    drawChips();
  }
  // A phone names every photograph it takes "image.jpg": each gets the time
  // it was taken, so that two of them are told apart in the thread.
  function photoName(f) {
    if (!/^image\.(jpe?g|png|heic|heif|webp)$/i.test(f.name || "")) return f;
    var d = new Date();
    var pad = function (n) { return (n < 10 ? "0" : "") + n; };
    var stamp = d.getFullYear() + "-" + pad(d.getMonth() + 1) + "-" + pad(d.getDate()) + "-" + pad(d.getHours()) + pad(d.getMinutes()) + pad(d.getSeconds());
    var ext = f.name.split(".").pop().toLowerCase();
    try { return new File([f], "photo-" + stamp + "." + ext, { type: f.type, lastModified: f.lastModified }); }
    catch (err) { return f; }
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
      var cd = e.clipboardData;
      var files = cd && cd.files;
      if (!input || !files || !files.length) return;
      // Text copied with a picture (from a page, a document) is pasted as
      // text too: only a paste of files alone is taken over.
      var words = cd.getData && cd.getData("text/plain");
      if (!words) e.preventDefault();
      add(files);
    });
    grow();
  }
  if (input) {
    input.addEventListener("change", function () {
      var chosen = Array.prototype.slice.call(input.files || []);
      // The field is rebuilt from the whole list, earlier choices included.
      add(chosen);
    });
  }
  if (camera) {
    camera.addEventListener("change", function () {
      var shot = Array.prototype.slice.call(camera.files || []).map(photoName);
      add(shot);
      // The photograph is now in the list, which the attach field carries.
      camera.value = "";
    });
  }

  /* ----- dictation: the browser writes out what is said ----- */
  var Recognition = window.SpeechRecognition || window.webkitSpeechRecognition;
  var policy = document.permissionsPolicy || document.featurePolicy;
  var mayListen = !policy || !policy.allowsFeature || policy.allowsFeature("microphone");
  var rec = null;
  var listening = false;
  var starting = false;
  var failed = false;
  function stopDictation() {
    if (rec && listening) { try { rec.stop(); } catch (err) { /* already ended */ } }
  }
  function startDictation() {
    if (!text || listening || starting) return;
    failed = false;
    starting = true;
    var r = new Recognition();
    rec = r;
    r.lang = document.documentElement.lang || "";
    r.interimResults = true;
    r.continuous = true;
    var caret = typeof text.selectionStart === "number" ? text.selectionStart : text.value.length;
    var before = text.value.slice(0, caret);
    var after = text.value.slice(caret);
    if (before && !/\s$/.test(before)) before += " ";
    r.onresult = function (e) {
      var said = "";
      for (var i = 0; i < e.results.length; i++) said += e.results[i][0].transcript;
      said = said.replace(/^\s+/, "");
      text.value = before + said + after;
      var at = (before + said).length;
      try { text.setSelectionRange(at, at); } catch (err) { /* not focused */ }
      grow();
    };
    r.onerror = function (e) {
      if (e.error === "aborted" || e.error === "no-speech") return;
      failed = true;
      var refused = e.error === "not-allowed" || e.error === "service-not-allowed";
      say(composer.getAttribute(refused ? "data-dictate-refused" : "data-dictate-failed"));
    };
    r.onend = function () {
      listening = false;
      mic.setAttribute("aria-pressed", "false");
      if (!failed) say("");
    };
    var begin = function () {
      starting = false;
      try {
        r.start();
        listening = true;
        mic.setAttribute("aria-pressed", "true");
        say(composer.getAttribute("data-dictating"));
        text.focus();
      } catch (err) {
        say(composer.getAttribute("data-dictate-failed"));
      }
    };
    // Recognised on the device where the browser offers it: the voice then
    // stays on it. Elsewhere the browser's own service, as the button says.
    if (typeof Recognition.available === "function") {
      Promise.resolve(Recognition.available({ langs: [r.lang], processLocally: true }))
        .then(function (state) { if (state === "available") r.processLocally = true; })
        .catch(function () { /* the browser's service */ })
        .then(begin);
    } else {
      begin();
    }
  }
  if (mic && Recognition && mayListen && text && !text.disabled) {
    mic.hidden = false;
    mic.addEventListener("click", function () {
      if (listening) stopDictation(); else startDictation();
    });
    document.addEventListener("visibilitychange", function () { if (document.hidden) stopDictation(); });
    window.addEventListener("blur", stopDictation);
  }

  composer.addEventListener("submit", function (e) {
    stopDictation();
    if (!window.fetch || !window.FormData) return;
    e.preventDefault();
    if (sending) return;
    var words = text ? text.value.trim() : "";
    if (!words && !picked.length) { idleForm(); if (text) text.focus(); return; }
    var data = new FormData();
    data.append("text", words);
    picked.forEach(function (f) { data.append("files", f, f.name); });
    sending = true;
    if (sendButton) sendButton.disabled = true;
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
            startLive();
            busy(true);
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
    stopDictation();
    var open = document.querySelector(".as-menu[open]");
    if (open) open.removeAttribute("open");
  });
})();
