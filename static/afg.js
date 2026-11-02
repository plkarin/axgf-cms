/* The AFG room (docs/adr/0023, 0025): show the fields the chosen way of
   computing uses and the address it will call; suggest its models of the
   moment, and the efforts of the model chosen, with the one it takes by
   default. Suggestions only: any other name is kept and sent as it is, and
   the room says it is not one of them. Without scripting every field is
   shown, with the saved way's suggestions. */
(function () {
  "use strict";
  var form = document.querySelector("[data-afg-form]");
  if (!form) return;
  var kind = form.querySelector("[data-afg-kind]");
  var url = form.querySelector("#afg-url");
  var model = form.querySelector("#afg-model");
  var effort = form.querySelector("#afg-effort");
  var called = form.querySelector("[data-afg-endpoint]");
  var efforts = document.getElementById("afg-efforts");
  var modelAside = form.querySelector('[data-afg-unlisted="model"]');
  var effortAside = form.querySelector('[data-afg-unlisted="effort"]');
  var shown = null;
  function option() {
    return kind.options[kind.selectedIndex];
  }
  function words(s) {
    return (s || "").split(" ").filter(Boolean);
  }
  function value(field) {
    return field ? field.value.trim() : "";
  }
  // The models a way of computing suggests, as its list holds them.
  function models(opt) {
    var list = document.getElementById("afg-models-" + opt.value);
    return list ? Array.prototype.slice.call(list.options) : [];
  }
  // Ollama names a size or a build after a colon: the same model.
  function suggestion(opt, name) {
    var list = models(opt);
    var same = list.filter(function (o) { return o.value === name; })[0];
    if (same || opt.value !== "ollama" || !name.split(":")[0]) return same || null;
    return list.filter(function (o) {
      return o.value.split(":")[0] === name.split(":")[0];
    })[0] || null;
  }
  // The efforts offered: the chosen model's, or all its way's otherwise.
  function offered(opt) {
    var m = suggestion(opt, value(model));
    return m
      ? { list: words(m.getAttribute("data-efforts")), usual: m.getAttribute("data-default") || "" }
      : { list: words(opt.getAttribute("data-efforts")), usual: "" };
  }
  function endpoint() {
    if (!called) return;
    var opt = option();
    var path = opt.getAttribute("data-path") || "";
    var base = value(url) || opt.getAttribute("data-url") || "";
    var name = value(model).replace(/^models\//, "");
    called.textContent = path ? base.replace(/\/+$/, "") + path.replace("{model}", name) : "";
  }
  function suggest() {
    var opt = option();
    var o = offered(opt);
    if (efforts) {
      efforts.textContent = "";
      o.list.forEach(function (e) {
        var item = document.createElement("option");
        item.value = e;
        if (e === o.usual) item.label = efforts.getAttribute("data-default-label") || "";
        efforts.appendChild(item);
      });
    }
    var name = value(model);
    var e = value(effort);
    if (modelAside) modelAside.hidden = !(name && models(opt).length && !suggestion(opt, name));
    if (effortAside) effortAside.hidden = !(e && o.list.length && o.list.indexOf(e) < 0);
    endpoint();
  }
  function apply() {
    var opt = option();
    var uses = words(opt.getAttribute("data-uses"));
    form.querySelectorAll("[data-afg-field]").forEach(function (f) {
      f.hidden = uses.indexOf(f.getAttribute("data-afg-field")) < 0;
    });
    // What was picked from the last way's lists does not follow to the
    // next one; what was written by hand stays.
    if (shown && shown !== opt) {
      if (model && suggestion(shown, value(model)) && !suggestion(opt, value(model))) model.value = "";
      var e = value(effort);
      if (effort && e && words(shown.getAttribute("data-efforts")).indexOf(e) >= 0 &&
          words(opt.getAttribute("data-efforts")).indexOf(e) < 0) effort.value = "";
    }
    if (model) model.setAttribute("list", "afg-models-" + opt.value);
    shown = opt;
    suggest();
  }
  kind.addEventListener("change", apply);
  if (url) url.addEventListener("input", endpoint);
  if (model) model.addEventListener("input", suggest);
  if (effort) effort.addEventListener("input", suggest);
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
