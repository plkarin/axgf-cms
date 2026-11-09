/* A person created with what surrounds them (docs/adr/0039). Enhancement
   only: without this script the page is a form whose empty rows are the rows
   there are, and the save says what it reused and what it created.

   With it: a row is added or removed at need (nicknames, brothers and
   sisters, children, unions), the name is shown as it will read, and every
   name typed says before the save what the save will do with it — the
   person the archive holds, a new record, or several records to choose
   from. The rules are the server's (src/person_new.rs): a label picked from
   the list names its record; any other text names the one record whose name
   it is, letter for letter, case and spaces aside; no record, a new one;
   several, a choice the writer has to make. */
(function () {
  'use strict';
  var form = document.querySelector('[data-person-new]');
  if (!form) return;

  function folded(s) {
    return (s || '').trim().split(/\s+/).join(' ').toLowerCase();
  }
  function labels(id) {
    return Array.prototype.map.call(document.querySelectorAll('#' + id + ' option'), function (o) { return o.value; });
  }
  // What each list offers, and how many records answer to each name.
  function index(id, cut) {
    var all = labels(id), names = {};
    all.forEach(function (l) {
      var at = l.indexOf(cut);
      var n = folded(at === -1 ? l : l.slice(0, at));
      names[n] = (names[n] || 0) + 1;
    });
    return { labels: all, names: names };
  }
  var people = index('pick-persons', ' · ');
  var places = index('pick-places', ' · #');

  function said(input) {
    var v = input.value.trim();
    if (!v) return '';
    var place = input.getAttribute('data-who') === 'place';
    var list = place ? places : people;
    var n = list.labels.indexOf(v) !== -1 ? 1 : (list.names[folded(v)] || 0);
    if (n === 1) return form.dataset[place ? 'hintPlaceExisting' : 'hintExisting'] || '';
    if (n === 0) return form.dataset[place ? 'hintPlaceNew' : 'hintNew'] || '';
    return form.dataset.hintAmbiguous || '';
  }
  function tell(input) {
    var out = input.parentNode.querySelector('[data-said]');
    if (!out) return;
    var text = said(input);
    out.textContent = text;
    var ambiguous = text && text === form.dataset.hintAmbiguous;
    out.classList.toggle('pn-said-warn', !!ambiguous);
  }
  form.addEventListener('input', function (e) {
    if (e.target.matches('[data-who]')) tell(e.target);
    if (e.target.matches('[data-name-part], [name="nick"]')) preview();
    if (e.target.matches('[data-death-date], [name="death_place"]') && e.target.value.trim()) {
      var living = form.querySelector('[data-living]');
      if (living) living.checked = false;
    }
  });
  form.addEventListener('change', function (e) {
    if (e.target.matches('[data-who]')) tell(e.target);
  });
  form.querySelectorAll('[data-who]').forEach(tell);

  // The name as it will read: the first name, the nicknames quoted, the
  // family name.
  var shown = form.querySelector('[data-name-preview]');
  var out = form.querySelector('[data-name-out]');
  function preview() {
    if (!shown || !out) return;
    var quote = form.dataset.quote || '%S';
    var parts = [];
    var given = form.querySelector('[name="given"]');
    var family = form.querySelector('[name="family"]');
    if (given && given.value.trim()) parts.push(given.value.trim());
    form.querySelectorAll('[name="nick"]').forEach(function (n) {
      if (n.value.trim()) parts.push(quote.replace('%S', n.value.trim()));
    });
    if (family && family.value.trim()) parts.push(family.value.trim());
    out.textContent = parts.join(' ');
    shown.hidden = parts.length === 0;
  }
  preview();

  // Rows added and removed at need.
  function bindRows(set) {
    var list = set.querySelector('[data-row-list]');
    var add = set.querySelector('[data-row-add]');
    if (!list) return;
    if (add) add.hidden = false;
    list.querySelectorAll('[data-row-remove]').forEach(function (b) { b.hidden = false; });
    if (add) add.addEventListener('click', function () {
      var first = list.querySelector('[data-row]');
      if (!first) return;
      var row = first.cloneNode(true);
      var input = row.querySelector('input');
      input.value = '';
      input.removeAttribute('aria-invalid');
      var note = row.querySelector('[data-said]');
      if (note) note.textContent = '';
      list.appendChild(row);
      input.focus();
    });
    list.addEventListener('click', function (e) {
      var b = e.target.closest('[data-row-remove]');
      if (!b) return;
      var row = b.closest('[data-row]');
      if (list.querySelectorAll('[data-row]').length > 1) {
        row.remove();
      } else {
        var input = row.querySelector('input');
        input.value = '';
        tell(input);
      }
      preview();
    });
  }
  form.querySelectorAll('[data-rows]').forEach(bindRows);

  // A union more: the last one's fields, emptied and numbered after it.
  var unions = form.querySelector('[data-unions]');
  var addUnion = form.querySelector('[data-union-add]');
  if (unions && addUnion) {
    addUnion.hidden = false;
    addUnion.addEventListener('click', function () {
      var all = unions.querySelectorAll('[data-union]');
      var last = all[all.length - 1];
      var n = all.length;
      var copy = last.cloneNode(true);
      copy.querySelectorAll('[name]').forEach(function (f) {
        f.name = f.name.replace(/^u\.\d+\./, 'u.' + n + '.');
        if (f.tagName === 'SELECT') f.selectedIndex = 0;
        else f.value = '';
        f.removeAttribute('aria-invalid');
      });
      // Its children's list is named by its own legend.
      copy.querySelectorAll('[id]').forEach(function (el) {
        el.id = el.id.replace(/^pn-u\d+-/, 'pn-u' + n + '-');
      });
      copy.querySelectorAll('[aria-labelledby]').forEach(function (el) {
        el.setAttribute('aria-labelledby', el.getAttribute('aria-labelledby').replace(/^pn-u\d+-/, 'pn-u' + n + '-'));
      });
      copy.querySelectorAll('[data-rows]').forEach(function (s) {
        var rows = s.querySelectorAll('[data-row]');
        for (var i = 1; i < rows.length; i++) rows[i].remove();
      });
      copy.querySelectorAll('[data-said]').forEach(function (o) { o.textContent = ''; });
      var legend = copy.querySelector('[data-union-legend]');
      if (legend && unions.dataset.legend) legend.textContent = unions.dataset.legend.replace('%N', n + 1);
      unions.appendChild(copy);
      copy.querySelectorAll('[data-rows]').forEach(bindRowsFresh);
      var first = copy.querySelector('input');
      if (first) first.focus();
    });
  }
  // A copied set already shows its buttons: only its listeners are new.
  function bindRowsFresh(set) {
    var add = set.querySelector('[data-row-add]');
    if (add) {
      var clean = add.cloneNode(true);
      add.parentNode.replaceChild(clean, add);
    }
    var list = set.querySelector('[data-row-list]');
    if (list) {
      var fresh = list.cloneNode(true);
      list.parentNode.replaceChild(fresh, list);
    }
    bindRows(set);
  }
})();
