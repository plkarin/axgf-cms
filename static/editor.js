// The record's editor: six tabs over one form (v0.4.2).
//
// Without this script the tab on screen is the URL's fragment, by CSS
// (:target), and everything still saves. With it: a tab is switched in place
// without the page jumping to it; the fragment is kept on the form's action,
// so a refused save comes back to the tab it was made on; a tab whose fields
// differ from what was drawn shows a mark in the strip, so one save across
// six tabs is not a leap of faith; and leaving with changes unsaved asks
// first.
(function () {
  "use strict";
  var form = document.querySelector("form.person-editor");
  if (!form) return;
  var panels = form.querySelector(".ed-panels");
  var tabs = Array.prototype.slice.call(form.querySelectorAll(".ed-tabs a[data-edit-tab]"));
  var keys = tabs.map(function (a) { return a.getAttribute("data-edit-tab"); });
  var tabInput = form.querySelector('input[name="tab"]');
  var base = form.getAttribute("action").split("#")[0];
  var submitting = false;
  panels.classList.add("js");

  function show(key, remember) {
    if (keys.indexOf(key) < 0) key = keys[0];
    tabs.forEach(function (a) {
      var on = a.getAttribute("data-edit-tab") === key;
      a.classList.toggle("is-current", on);
      if (on) a.setAttribute("aria-current", "true"); else a.removeAttribute("aria-current");
    });
    panels.querySelectorAll(".ed-panel").forEach(function (p) {
      p.classList.toggle("is-current", p.id === key);
    });
    tabInput.value = key;
    form.setAttribute("action", base + "#" + key);
    if (remember && location.hash !== "#" + key) history.replaceState(null, "", "#" + key);
    if (window.axgfShowCurrentTabs) window.axgfShowCurrentTabs(form);
  }

  // A refused save names its tab; otherwise the fragment does.
  var refused = form.hasAttribute("data-unsaved");
  var first = refused ? form.getAttribute("data-tab") : (location.hash.slice(1) || form.getAttribute("data-tab"));
  show(first, refused);

  document.addEventListener("click", function (e) {
    var a = e.target.closest && e.target.closest("a[data-edit-tab]");
    if (!a) return;
    e.preventDefault();
    show(a.getAttribute("data-edit-tab"), true);
    if (tabs.indexOf(a) < 0) form.scrollIntoView({ block: "start" });
  });
  window.addEventListener("hashchange", function () { show(location.hash.slice(1), false); });

  function changed(el) {
    if (el.type === "hidden" || el.type === "submit" || el.type === "button") return false;
    if (el.type === "checkbox" || el.type === "radio") return el.checked !== el.defaultChecked;
    if (el.type === "file") return el.files && el.files.length > 0;
    if (el.tagName === "SELECT") {
      var opts = Array.prototype.slice.call(el.options);
      if (el.multiple) return opts.some(function (o) { return o.selected !== o.defaultSelected; });
      // A select drawn with no option marked `selected` shows its first one,
      // which is then its drawn value and not a change. Comparing every
      // option's `selected` with its `defaultSelected` counted every such
      // select as changed, so leaving the editor always asked first — even
      // straight after a save.
      var drawn = -1;
      opts.forEach(function (o, i) { if (o.defaultSelected) drawn = i; });
      if (drawn < 0) opts.some(function (o, i) { if (!o.disabled) { drawn = i; return true; } return false; });
      return el.selectedIndex !== drawn;
    }
    return el.value !== el.defaultValue;
  }
  function dirty(panel) {
    return Array.prototype.some.call(panel.querySelectorAll("input, select, textarea"), changed);
  }
  // What the server marked (a refused save's unsaved tabs) stays marked.
  var server = {};
  tabs.forEach(function (a) { server[a.getAttribute("data-edit-tab")] = a.classList.contains("has-changes"); });
  function mark(panel) {
    var a = form.querySelector('.ed-tabs a[data-edit-tab="' + panel.id + '"]');
    if (!a) return;
    var on = server[panel.id] || dirty(panel);
    a.classList.toggle("has-changes", on);
    var said = a.querySelector(".ed-mark-text");
    if (said) said.hidden = !on;
  }
  var allPanels = Array.prototype.slice.call(panels.querySelectorAll(".ed-panel"));
  function onEdit(e) {
    allPanels.forEach(function (p) { if (p.contains(e.target)) mark(p); });
  }
  form.addEventListener("input", onEdit);
  form.addEventListener("change", onEdit);

  form.addEventListener("submit", function () { submitting = true; });
  window.addEventListener("beforeunload", function (e) {
    if (submitting) return;
    var any = refused || Array.prototype.some.call(panels.querySelectorAll(".ed-panel"), dirty);
    if (!any) return;
    e.preventDefault();
    e.returnValue = "";
  });
})();
