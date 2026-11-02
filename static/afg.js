/* The AFG room (docs/adr/0023): show the fields the chosen way of computing
   uses, the address it will call and the efforts its agent knows. Without
   scripting every field is shown, and the address said is the saved one. */
(function () {
  "use strict";
  var form = document.querySelector("[data-afg-form]");
  if (!form) return;
  var kind = form.querySelector("[data-afg-kind]");
  var url = form.querySelector("#afg-url");
  var model = form.querySelector("#afg-model");
  var called = form.querySelector("[data-afg-endpoint]");
  var efforts = document.getElementById("afg-efforts");
  function option() {
    return kind.options[kind.selectedIndex];
  }
  function endpoint() {
    if (!called) return;
    var opt = option();
    var path = opt.getAttribute("data-path") || "";
    var base = (url && url.value.trim()) || opt.getAttribute("data-url") || "";
    var name = ((model && model.value.trim()) || "").replace(/^models\//, "");
    called.textContent = path ? base.replace(/\/+$/, "") + path.replace("{model}", name) : "";
  }
  function apply() {
    var opt = option();
    var uses = (opt.getAttribute("data-uses") || "").split(" ");
    form.querySelectorAll("[data-afg-field]").forEach(function (f) {
      f.hidden = uses.indexOf(f.getAttribute("data-afg-field")) < 0;
    });
    if (efforts) {
      efforts.textContent = "";
      (opt.getAttribute("data-efforts") || "").split(" ").forEach(function (e) {
        if (!e) return;
        var o = document.createElement("option");
        o.value = e;
        efforts.appendChild(o);
      });
    }
    endpoint();
  }
  kind.addEventListener("change", apply);
  if (url) url.addEventListener("input", endpoint);
  if (model) model.addEventListener("input", endpoint);
  apply();
})();

/* A test's result (docs/adr/0024): the room opens with its window when the
   address says ?run=<id>. Here it becomes modal, and while the answer is
   awaited the window alone is asked for again; closed, the room is drawn
   afresh, its last test and its brain with it. */
(function () {
  "use strict";
  var dialog = document.querySelector("[data-afg-dialog]");
  if (!dialog) return;
  var body = dialog.querySelector("[data-afg-dialog-body]");
  var id = dialog.getAttribute("data-run");
  if (typeof dialog.showModal === "function") {
    if (dialog.open) dialog.close();
    dialog.showModal();
  }
  function waiting() {
    var r = body.querySelector("[data-afg-result]");
    return !r || r.getAttribute("data-state") === "running";
  }
  function poll() {
    fetch("/admin/afg/run/" + encodeURIComponent(id), {
      credentials: "same-origin",
      headers: { Accept: "application/json" }
    })
      .then(function (r) { return r.json(); })
      .then(function (d) {
        if (d.html) {
          body.innerHTML = d.html;
        } else if (d.text) {
          var p = document.createElement("p");
          p.textContent = d.text;
          body.textContent = "";
          body.appendChild(p);
          return;
        }
        if (waiting()) setTimeout(poll, 2000);
      })
      .catch(function () { setTimeout(poll, 5000); });
  }
  if (waiting()) setTimeout(poll, 2000);
  dialog.addEventListener("click", function (e) {
    if (e.target === dialog) dialog.close();
  });
  dialog.addEventListener("close", function () {
    // Reopening the window as modal closed it first, and that close arrives
    // late: with the window open again, it was not the reader's.
    if (dialog.open) return;
    window.location.replace("/admin/afg");
  });
})();
