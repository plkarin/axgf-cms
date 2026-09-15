// The profile editor's "add another entry".
//
// Enhancement only. Every series already renders one blank row, so with
// scripting off an editor fills that row, saves, and gets a new blank one; this
// adds a button that copies the blank row in place instead, numbered one past
// the last row so the server reads the rows back in order. The button's label
// comes from the page, which took it from the catalogue.
(function () {
  "use strict";
  document.querySelectorAll("fieldset.pf-attr[data-series]").forEach(function (set) {
    var spare = set.querySelector(".pf-row.is-spare");
    if (!spare) return;
    var template = spare.cloneNode(true);
    var button = document.createElement("button");
    button.type = "button";
    button.className = "btn small-btn pf-add";
    button.textContent = set.getAttribute("data-add-label") || "+";
    button.addEventListener("click", function () {
      var rows = set.querySelectorAll(".pf-row");
      var next = 0;
      rows.forEach(function (r) {
        next = Math.max(next, Number(r.getAttribute("data-index")) + 1);
      });
      var row = template.cloneNode(true);
      var from = template.getAttribute("data-index");
      row.setAttribute("data-index", String(next));
      row.querySelectorAll("[name]").forEach(function (el) {
        // Only the row number after the attribute's own path changes:
        // "morphology.height.3.v" becomes "morphology.height.4.v".
        var name = el.getAttribute("name");
        var marker = "." + from + ".";
        var at = name.lastIndexOf(marker);
        if (at >= 0) {
          el.setAttribute("name", name.slice(0, at) + "." + next + "." + name.slice(at + marker.length));
        }
        if (el.tagName === "SELECT") el.selectedIndex = 0;
        else if (el.type === "checkbox") el.checked = false;
        else el.value = "";
      });
      set.insertBefore(row, button);
      var first = row.querySelector("select, input, textarea");
      if (first) first.focus();
    });
    set.appendChild(button);
  });
})();
