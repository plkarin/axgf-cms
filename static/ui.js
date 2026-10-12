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
      var cur = nav.querySelector(".is-current");
      if (!cur || nav.scrollWidth <= nav.clientWidth) return;
      var n = nav.getBoundingClientRect(), c = cur.getBoundingClientRect();
      nav.scrollLeft += (c.left + c.width / 2) - (n.left + n.width / 2);
    });
  }
  window.axgfShowCurrentTabs = showCurrentTabs;
  showCurrentTabs();

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
    if (out) out.textContent = input.value;
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
    }
  });
  // Back to a page from the history cache: the form is idle again.
  window.addEventListener("pageshow", function (ev) {
    if (!ev.persisted) return;
    document.querySelectorAll('form[aria-busy="true"]').forEach(function (f) {
      f.removeAttribute("aria-busy");
    });
    document.querySelectorAll("[data-idle]").forEach(function (b) {
      b.textContent = b.getAttribute("data-idle");
      b.removeAttribute("data-idle");
      b.removeAttribute("aria-busy");
      b.removeAttribute("aria-disabled");
    });
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
        fetch("/search.json?q=" + encodeURIComponent(q), { credentials: "same-origin" })
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
