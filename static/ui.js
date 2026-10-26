// Behaviours that used to be inline `onclick`, `oninput` and `onsubmit`
// attributes in the templates.
//
// They moved here for one reason: an inline handler is script inside the
// document, so a Content-Security-Policy that permits them has to say
// `script-src 'unsafe-inline'`, and that permits *every* inline script —
// including one an attacker managed to get into a page. Six small behaviours
// were not worth the whole protection, so they are declarative now: the
// template marks an element with a `data-` attribute, and this file wires it
// up on load.
//
// Everything is delegated from `document`, so markup that arrives after load
// works too, and everything degrades to a plain form post with scripting off.
(function () {
  "use strict";

  // `data-autosubmit` — a select that applies itself: the masthead's
  // language. The form's own button is for a reader without scripts, so it
  // goes when this runs.
  document.addEventListener("change", function (ev) {
    var sel = ev.target.closest("[data-autosubmit]");
    if (sel && sel.form) sel.form.submit();
  });
  document.querySelectorAll("[data-autosubmit-go]").forEach(function (b) {
    b.hidden = true;
  });

  // A tab strip scrolls sideways rather than wrap (B4), so the current tab
  // can start out of view. Bring it to the middle of its strip — the strip
  // only, never the page — on load, and whenever river.js swaps the record
  // beside the river (it calls this).
  function showCurrentTabs(root) {
    (root || document).querySelectorAll(".person-tabs").forEach(function (nav) {
      wireTabs(nav);
      var cur = nav.querySelector(".is-current");
      if (cur && nav.scrollWidth > nav.clientWidth) {
        var n = nav.getBoundingClientRect(), c = cur.getBoundingClientRect();
        nav.scrollLeft += (c.left + c.width / 2) - (n.left + n.width / 2);
      }
      markTabs(nav);
    });
  }
  // Under the kit the strip has no scrollbar (it would sit inside the pill's
  // round ends): `data-more` says on which side more tabs wait, and app.css
  // fades that side out. Read in the strip's own direction, so a
  // right-to-left page fades the side its hidden tabs are on.
  function markTabs(nav) {
    var room = nav.scrollWidth - nav.clientWidth;
    var at = Math.abs(nav.scrollLeft);
    var start = room > 1 && at > 1, end = room > 1 && at < room - 1;
    if (getComputedStyle(nav).direction === "rtl") { var s = start; start = end; end = s; }
    if (start && end) nav.setAttribute("data-more", "both");
    else if (end) nav.setAttribute("data-more", "right");
    else if (start) nav.setAttribute("data-more", "left");
    else nav.removeAttribute("data-more");
    var more = nav.getAttribute("data-more") || "", wrap = nav.parentNode;
    if (wrap && wrap.classList && wrap.classList.contains("tabs-wrap")) {
      wrap.querySelector(".tabs-arrow-left").hidden = !(more === "left" || more === "both");
      wrap.querySelector(".tabs-arrow-right").hidden = !(more === "right" || more === "both");
    }
  }
  // The strip slides three ways (the operator's review, 2026-10-23: "on ne
  // peut pas slider"): the wheel, as before; a pointer dragging it, as a
  // finger does on a phone; and an arrow at each end where more tabs wait.
  // The arrows are for a pointer: the keyboard reaches every tab, and a tab
  // reached brings itself into view, so they are kept out of the Tab order
  // and out of what a screen reader reads.
  function arrows(nav) {
    var wrap = document.createElement("div");
    wrap.className = "tabs-wrap";
    nav.parentNode.insertBefore(wrap, nav);
    wrap.appendChild(nav);
    ["left", "right"].forEach(function (side) {
      var b = document.createElement("button");
      b.type = "button";
      b.className = "tabs-arrow tabs-arrow-" + side;
      b.tabIndex = -1;
      b.hidden = true;
      b.setAttribute("aria-hidden", "true");
      b.textContent = side === "left" ? "‹" : "›";
      b.addEventListener("click", function () {
        nav.scrollBy({ left: (side === "left" ? -0.7 : 0.7) * nav.clientWidth, behavior: "smooth" });
      });
      wrap.appendChild(b);
    });
  }
  function drag(nav) {
    var x0 = 0, from = 0, id = null, moved = false;
    nav.addEventListener("pointerdown", function (ev) {
      if (ev.pointerType !== "mouse" || ev.button !== 0 || nav.scrollWidth <= nav.clientWidth + 1) return;
      x0 = ev.clientX; from = nav.scrollLeft; id = ev.pointerId; moved = false;
    });
    nav.addEventListener("pointermove", function (ev) {
      if (id !== ev.pointerId) return;
      var dx = ev.clientX - x0;
      if (!moved && Math.abs(dx) < 6) return;
      if (!moved) { moved = true; nav.classList.add("is-dragging"); try { nav.setPointerCapture(id); } catch (e) { /* gone */ } }
      nav.scrollLeft = from - dx;
    });
    function end(ev) {
      if (id !== ev.pointerId) return;
      id = null;
      nav.classList.remove("is-dragging");
    }
    nav.addEventListener("pointerup", end);
    nav.addEventListener("pointercancel", end);
    // A drag is not a click: the tab under the pointer at the end of one is
    // not followed.
    nav.addEventListener("click", function (ev) {
      if (moved) { ev.preventDefault(); ev.stopPropagation(); moved = false; }
    }, true);
  }
  // A mouse wheel turns the strip sideways while it has further to go that
  // way; at either end the page scrolls as usual, so the strip never traps it.
  function wireTabs(nav) {
    if (nav.axgfTabs) return;
    nav.axgfTabs = true;
    arrows(nav);
    drag(nav);
    nav.addEventListener("scroll", function () { markTabs(nav); }, { passive: true });
    // A tab reached with the keyboard is brought clear of the faded edge.
    nav.addEventListener("focusin", function (ev) {
      var n = nav.getBoundingClientRect(), c = ev.target.getBoundingClientRect(), pad = 44;
      if (c.left < n.left + pad) nav.scrollLeft -= n.left + pad - c.left;
      else if (c.right > n.right - pad) nav.scrollLeft += c.right - (n.right - pad);
    });
    nav.addEventListener("wheel", function (ev) {
      if (ev.ctrlKey || Math.abs(ev.deltaX) >= Math.abs(ev.deltaY)) return;
      var room = nav.scrollWidth - nav.clientWidth;
      if (room <= 1) return;
      // Distances along the strip's own direction: a right-to-left strip
      // starts at 0 and runs negative.
      var d = ev.deltaY * (ev.deltaMode === 1 ? 16 : ev.deltaMode === 2 ? nav.clientWidth : 1);
      var at = Math.abs(nav.scrollLeft);
      if ((d > 0 && at >= room - 1) || (d < 0 && at <= 1)) return;
      ev.preventDefault();
      var to = Math.max(0, Math.min(room, at + d));
      nav.scrollLeft = getComputedStyle(nav).direction === "rtl" ? -to : to;
    }, { passive: false });
  }
  window.axgfShowCurrentTabs = showCurrentTabs;
  showCurrentTabs();
  window.addEventListener("resize", function () {
    document.querySelectorAll(".person-tabs").forEach(markTabs);
  });

  // `data-bulk` — a listing's selection (B6). While any row is selected the
  // bulk bar takes over the header row and counts them, in the page's own
  // plural (the server rendered every form; `data-forms`); Escape clears.
  var bulk = document.querySelector("[data-bulk]");
  if (bulk) {
    var bar = bulk.querySelector("[data-bulk-bar]");
    var head = bulk.querySelector(".bulk-head");
    var count = bulk.querySelector("[data-bulk-count]");
    var all = bulk.querySelector("[data-bulk-all]");
    var forms = JSON.parse(count.getAttribute("data-forms"));
    var boxes = function () { return bulk.querySelectorAll('input[name="id"]'); };
    var update = function () {
      var list = boxes(), n = 0;
      list.forEach(function (b) { if (b.checked) n++; });
      count.textContent = forms.forms[forms.index[n < 200 ? n : 100 + (n % 100)]].replace("{n}", String(n));
      bar.hidden = n === 0;
      head.hidden = n > 0;
      // A merge takes exactly two records.
      bulk.querySelectorAll("[data-bulk-pair]").forEach(function (b) { b.disabled = n !== 2; });
      all.checked = n > 0 && n === list.length;
      all.indeterminate = n > 0 && n < list.length;
    };
    bulk.addEventListener("change", function (ev) {
      if (ev.target === all) boxes().forEach(function (b) { b.checked = all.checked; });
      update();
    });
    document.addEventListener("keydown", function (ev) {
      if (ev.key !== "Escape" || bar.hidden) return;
      boxes().forEach(function (b) { b.checked = false; });
      update();
    });
    update();
  }

  // `data-toggle="<id>"` — show or hide the element with that id. The delete
  // confirmation row under a listing.
  document.addEventListener("click", function (ev) {
    var btn = ev.target.closest("[data-toggle]");
    if (!btn) return;
    var target = document.getElementById(btn.getAttribute("data-toggle"));
    if (!target) return;
    target.hidden = !target.hidden;
  });

  // `data-output="<id>"` on a range input — mirror its value into that
  // element as it moves, so a slider has a number beside it.
  function mirror(input) {
    var out = document.getElementById(input.getAttribute("data-output"));
    if (!out) return;
    // A confidence slider says its value as the page's language writes a
    // percentage ("80 %" in French), from the catalogue's own pattern.
    var fmt = out.getAttribute("data-format");
    out.textContent = fmt
      ? fmt.replace("{n}", String(Math.round(parseFloat(input.value) * 100)))
      : input.value;
  }
  document.addEventListener("input", function (ev) {
    var input = ev.target.closest("[data-output]");
    if (input) mirror(input);
  });

  // `data-confirm="<question>"` on a form — ask before submitting. The text is
  // rendered server-side, so it is already in the reader's language.
  //
  // `data-no-submit` on a form — never submit it. The tree filter is typed
  // into and read by script; pressing Enter in it must not reload the page.
  document.addEventListener("submit", function (ev) {
    var form = ev.target;
    if (form.hasAttribute("data-no-submit")) {
      ev.preventDefault();
      return;
    }
    var question = form.getAttribute("data-confirm");
    if (question && !window.confirm(question)) {
      ev.preventDefault();
      return;
    }
    // `data-busy="<label>"` on a submit button — while the request is out,
    // the button says what is happening (B2: the label changes, no spinner)
    // and a second press does not send it twice. Disabling the button now
    // would drop its name and value from the submission, so it is marked
    // instead, and later submits of this form are refused.
    if (form.getAttribute("aria-busy") === "true") {
      ev.preventDefault();
      return;
    }
    var by = ev.submitter;
    if (by && by.hasAttribute("data-busy")) {
      form.setAttribute("aria-busy", "true");
      by.setAttribute("aria-busy", "true");
      by.setAttribute("aria-disabled", "true");
      by.setAttribute("data-idle", by.textContent);
      by.textContent = by.getAttribute("data-busy");
      // Beside the words, never in place of them: a loader that only the
      // reference theme draws (app.css, docs/adr/0004 §4) and that assistive
      // technology is spared — the words already say what is happening.
      by.appendChild(loader(by.getAttribute("data-loader")));
      // A file to download does not unload the page: the server answers
      // with a cookie naming this token once the file starts, and the
      // button is idle again (render::mark_download).
      if (form.hasAttribute("data-download")) awaitDownload(form);
    }
  });
  function idle(form) {
    form.removeAttribute("aria-busy");
    form.querySelectorAll("[data-idle]").forEach(function (b) {
      b.textContent = b.getAttribute("data-idle");
      b.removeAttribute("data-idle");
      b.removeAttribute("aria-busy");
      b.removeAttribute("aria-disabled");
    });
  }
  function awaitDownload(form) {
    var token = "";
    for (var i = 0; i < 24; i++) token += "abcdefghijklmnopqrstuvwxyz0123456789".charAt(Math.floor(Math.random() * 36));
    var field = form.querySelector('input[name="download_token"]');
    if (!field) {
      field = document.createElement("input");
      field.type = "hidden";
      field.name = "download_token";
      form.appendChild(field);
    }
    field.value = token;
    var started = Date.now();
    var timer = window.setInterval(function () {
      var arrived = document.cookie.split("; ").indexOf("axgf_download=" + token) !== -1;
      // Ten minutes at most: a download the server never answered for
      // (the connection dropped) leaves the button usable again.
      if (arrived || Date.now() - started > 600000) {
        window.clearInterval(timer);
        document.cookie = "axgf_download=; Path=/admin; Max-Age=0; SameSite=Strict";
        idle(form);
      }
    }, 400);
  }
  // A loader's markup, as templates/_icons.html writes it: three dots by
  // default, or the river or the hourglass where a page asks for one.
  function loader(kind) {
    var el = document.createElement("span");
    el.setAttribute("aria-hidden", "true");
    if (kind === "river" || kind === "hourglass") {
      el.className = "ld ld-" + kind;
      el.innerHTML = '<svg class="ico" viewBox="0 0 24 24" focusable="false">' + (kind === "river"
        ? '<path d="M2 12c2.4-3.2 4.8-3.2 7.2 0s4.8 3.2 7.2 0 4.2-3.2 5.6-1"/>'
        : '<path d="M7 3h10M7 21h10"/><path d="M8 3c0 5 8 5 8 9s-8 4-8 9"/><path d="M16 3c0 5-8 5-8 9s8 4 8 9"/>') + "</svg>";
    } else {
      el.className = "ld ld-dots";
      el.innerHTML = "<i></i><i></i><i></i>";
    }
    return el;
  }
  window.axgfLoader = loader;
  // `data-describe="<id>"` on a select: the block with that id holds one
  // paragraph per choice (`data-value`), and the one for the choice made is
  // the one shown — the licence's meaning beside the licence (2026-10-23).
  document.querySelectorAll("select[data-describe]").forEach(function (sel) {
    var box = document.getElementById(sel.getAttribute("data-describe"));
    if (!box) return;
    function show() {
      box.querySelectorAll("[data-value]").forEach(function (p) {
        p.hidden = p.getAttribute("data-value") !== sel.value;
      });
    }
    sel.addEventListener("change", show);
    show();
  });
  // Back to a page from the history cache: the form is idle again.
  window.addEventListener("pageshow", function (ev) {
    if (!ev.persisted) return;
    document.querySelectorAll('form[aria-busy="true"]').forEach(idle);
  });

  // Motion that answers the reader (v0.5.0, docs/adr/0004 §6). Only where
  // `prefers-reduced-motion` allows it, and only drawn by the reference
  // theme: `js-motion` on the root is what app.css waits for, so a page
  // without this script, or a reader who asked for stillness, has every
  // section in place and every number whole from the first paint.
  var still = window.matchMedia && matchMedia("(prefers-reduced-motion: reduce)").matches;
  if (!still && "IntersectionObserver" in window) {
    var root = document.documentElement;
    root.classList.add("js-motion");
    // The kit: Canopée's furniture, shared by Fusion and Galaxy (docs/adr/0007).
    var canopee = /^(canopee|fusion|galaxy|summit|clarity|genealogy)$/.test(root.getAttribute("data-theme") || "");
    // A section rises into place the first time it scrolls into view.
    var seen = new IntersectionObserver(function (entries) {
      entries.forEach(function (e) {
        if (!e.isIntersecting) return;
        e.target.classList.add("is-in");
        seen.unobserve(e.target);
      });
    }, { rootMargin: "0px 0px -8% 0px" });
    document.querySelectorAll("[data-reveal]").forEach(function (el) { seen.observe(el); });
    // A count runs up to its number once, the first time it is seen. The
    // server wrote the number; this only replays the arrival of it.
    if (canopee) {
      var counted = new IntersectionObserver(function (entries) {
        entries.forEach(function (e) {
          if (!e.isIntersecting) return;
          counted.unobserve(e.target);
          var el = e.target, to = parseInt(el.getAttribute("data-count"), 10);
          if (!(to > 0)) return;
          var t0 = performance.now(), ms = Math.min(1400, 500 + to / 3);
          (function tick(now) {
            var k = Math.min(1, (now - t0) / ms), eased = 1 - Math.pow(1 - k, 3);
            el.textContent = String(Math.round(to * eased));
            if (k < 1) requestAnimationFrame(tick); else el.textContent = String(to);
          })(t0);
        });
      }, { threshold: 0.6 });
      document.querySelectorAll("[data-count]").forEach(function (el) { counted.observe(el); });
    }
  }

  // The ink of a press (v0.5.0, docs/adr/0004 §6): under the reference
  // theme, a button, a tab or a chip shows where it was pressed — a ripple
  // from the pointer, gone in 0.6 s. Never where the reader asked for
  // stillness.
  document.addEventListener("pointerdown", function (ev) {
    if (still || !/^(canopee|fusion|galaxy|summit|clarity|genealogy)$/.test(document.documentElement.getAttribute("data-theme") || "")) return;
    var b = ev.target.closest && ev.target.closest(".btn, .person-tab");
    if (!b || b.disabled || b.getAttribute("aria-disabled") === "true") return;
    var r = b.getBoundingClientRect(), d = Math.max(r.width, r.height) * 2.4;
    var ink = document.createElement("span");
    ink.className = "ripple-ink";
    ink.setAttribute("aria-hidden", "true");
    ink.style.width = ink.style.height = d + "px";
    ink.style.left = (ev.clientX - r.left - d / 2) + "px";
    ink.style.top = (ev.clientY - r.top - d / 2) + "px";
    b.appendChild(ink);
    setTimeout(function () { if (ink.parentNode) ink.parentNode.removeChild(ink); }, 700);
  });

  // `data-search` — the masthead search. Suggestions from /search.json as
  // you type; ↓ ↑ choose one; Enter travels the river to them (on the river
  // page without leaving it, through river.js; elsewhere by opening the river
  // there), Shift+Enter opens their record. With nothing chosen, Enter submits
  // the form to /search like a plain form. Escape closes the list.
  var input = document.querySelector("[data-search]");
  if (input) {
    var list = document.getElementById(input.getAttribute("aria-controls"));
    var hits = [], at = -1, timer = null, asked = "";
    var close = function () {
      list.hidden = true; list.innerHTML = ""; hits = []; at = -1;
      input.setAttribute("aria-expanded", "false");
      input.removeAttribute("aria-activedescendant");
    };
    var go = function (hit, record) {
      close();
      if (record) { location.href = "/person/" + encodeURIComponent(hit.id); return; }
      if (window.axgfRiver && window.axgfRiver.travel) { window.axgfRiver.travel(hit.id); input.blur(); return; }
      location.href = "/?p=" + encodeURIComponent(hit.id);
    };
    var mark = function () {
      Array.prototype.forEach.call(list.querySelectorAll('[role="option"]'), function (li, i) {
        li.setAttribute("aria-selected", String(i === at));
      });
      if (at >= 0) input.setAttribute("aria-activedescendant", "masthead-q-" + at);
      else input.removeAttribute("aria-activedescendant");
    };
    var show = function (reply) {
      var found = reply.hits || [];
      hits = found; at = -1; list.innerHTML = "";
      found.forEach(function (h, i) {
        var li = document.createElement("li");
        li.id = "masthead-q-" + i;
        li.setAttribute("role", "option");
        li.setAttribute("aria-selected", "false");
        // The river's line for this person, at its discharge class: a
        // founder reads differently from a childless cousin (B3).
        var bar = document.createElement("i");
        bar.className = "search-class";
        bar.setAttribute("data-class", String(h["class"] || 1));
        bar.setAttribute("aria-hidden", "true");
        li.appendChild(bar);
        li.appendChild(document.createTextNode(h.name));
        if (h.years) {
          var y = document.createElement("span");
          y.className = "search-years";
          y.textContent = " " + h.years;
          li.appendChild(y);
        }
        li.addEventListener("mousedown", function (e) { e.preventDefault(); go(h, e.shiftKey); });
        list.appendChild(li);
      });
      if (found.length && reply.summary) {
        var foot = document.createElement("li");
        foot.className = "search-foot";
        foot.setAttribute("role", "presentation");
        foot.textContent = reply.summary;
        list.appendChild(foot);
      }
      list.hidden = found.length === 0;
      input.setAttribute("aria-expanded", String(found.length > 0));
    };
    input.addEventListener("input", function () {
      clearTimeout(timer);
      var q = input.value.trim();
      if (!q) { close(); return; }
      timer = setTimeout(function () {
        asked = q;
        // While the suggestions are out, the field says it is looking (the
        // reference theme draws the dots; every theme gets aria-busy).
        input.setAttribute("aria-busy", "true");
        fetch("/search.json?q=" + encodeURIComponent(q), { credentials: "same-origin" })
          .finally(function () { if (asked === q) input.removeAttribute("aria-busy"); })
          .then(function (r) { return r.ok ? r.json() : { hits: [] }; })
          .then(function (found) { if (asked === q) show(found); })
          .catch(function () {});
      }, 120);
    });
    input.addEventListener("keydown", function (e) {
      if (e.key === "ArrowDown" && hits.length) { e.preventDefault(); at = (at + 1) % hits.length; mark(); }
      else if (e.key === "ArrowUp" && hits.length) { e.preventDefault(); at = (at - 1 + hits.length) % hits.length; mark(); }
      else if (e.key === "Escape") { close(); }
      else if (e.key === "Enter" && hits.length) {
        // The highlighted person, or the first when none is highlighted.
        e.preventDefault();
        go(hits[at >= 0 ? at : 0], e.shiftKey);
      }
    });
    input.addEventListener("blur", function () { setTimeout(close, 150); });
  }
})();
