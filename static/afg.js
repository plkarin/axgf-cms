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
