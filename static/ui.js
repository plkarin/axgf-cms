// Six behaviours that used to be inline `onclick`, `oninput` and `onsubmit`
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
    if (question && !window.confirm(question)) ev.preventDefault();
  });
})();
