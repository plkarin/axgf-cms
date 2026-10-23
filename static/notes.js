/* Notes and contributions (docs/adr/0014). Enhancement only: without this
   script every card is a link to the note's page, the search is a form and
   the letters are links; the writing form is a text box that posts.

   Reading: a note opens in a window (/notes/:id?frame=1), the list filters
   as the reader types (title, category, people, author and the summary on
   each card), Enter asks the server, which searches the whole text, and the
   letters filter it by the title's first letter.
   Writing: a toolbar for the Markdown, pictures pasted, dropped or chosen
   (held by the server until the note is saved), a preview, and the people a
   note is about as chips. */
(function () {
  'use strict';

  // ---- reading -----------------------------------------------------------
  function fold(s) {
    return (s || '').normalize('NFD').replace(/[̀-ͯ]/g, '').toLowerCase();
  }

  document.querySelectorAll('[data-notes]').forEach(function (root) {
    var dialog = root.querySelector('[data-note-dialog]');
    var body = root.querySelector('[data-note-dialog-body]');
    var q = root.querySelector('[data-notes-q]');
    var count = root.querySelector('[data-notes-count]');
    var empty = root.querySelector('[data-notes-empty]');
    var cards = Array.prototype.slice.call(root.querySelectorAll('[data-note]'));
    var letter = '';
    var current = root.querySelector('[data-notes-letter][aria-current]');
    if (current) letter = current.getAttribute('data-notes-letter') || '';

    function filter() {
      var needle = fold(q ? q.value.trim() : '');
      var shown = 0;
      cards.forEach(function (c) {
        var ok = (!letter || c.getAttribute('data-letter') === letter) &&
          (!needle || (c.getAttribute('data-haystack') || '').indexOf(needle) !== -1);
        c.hidden = !ok;
        if (ok) shown++;
      });
      if (count && count.dataset.template) {
        count.textContent = count.dataset.template.replace('%S', shown).replace('%T', count.dataset.total);
      }
      if (empty) empty.hidden = shown > 0;
    }
    if (q) q.addEventListener('input', filter);
    root.querySelectorAll('[data-notes-letter]').forEach(function (a) {
      a.addEventListener('click', function (e) {
        e.preventDefault();
        letter = a.getAttribute('data-notes-letter') || '';
        root.querySelectorAll('[data-notes-letter]').forEach(function (b) { b.removeAttribute('aria-current'); });
        a.setAttribute('aria-current', 'true');
        filter();
      });
    });

    if (!dialog || !body || typeof dialog.showModal !== 'function') return;
    function close() { dialog.close(); }
    dialog.addEventListener('click', function (e) {
      if (e.target === dialog || e.target.closest('[data-note-close]')) close();
    });
    root.addEventListener('click', function (e) {
      var open = e.target.closest('[data-note-open]');
      if (!open || e.metaKey || e.ctrlKey || e.shiftKey || e.button !== 0) return;
      e.preventDefault();
      var id = open.getAttribute('data-note-open');
      fetch('/notes/' + encodeURIComponent(id) + '?frame=1', { credentials: 'same-origin' })
        .then(function (r) { if (!r.ok) throw new Error(r.status); return r.text(); })
        .then(function (html) {
          body.innerHTML = html;
          dialog.showModal();
          var h = body.querySelector('h2');
          if (h) { h.setAttribute('tabindex', '-1'); h.focus(); }
        })
        .catch(function () { window.location.href = open.href; });
    });
  });

  // ---- writing -----------------------------------------------------------
  var form = document.querySelector('[data-note-form]');
  if (!form) return;
  var text = form.querySelector('[data-note-text]');
  var toolbar = form.querySelector('[data-note-toolbar]');
  var preview = form.querySelector('[data-note-preview]');
  var mode = form.querySelector('[data-note-mode]');
  var file = form.querySelector('[data-note-file]');
  if (toolbar) toolbar.hidden = false;

  function insert(before, after, placeholder) {
    var s = text.selectionStart, e = text.selectionEnd, v = text.value;
    var inner = v.slice(s, e) || placeholder || '';
    text.value = v.slice(0, s) + before + inner + after + v.slice(e);
    text.focus();
    text.selectionStart = s + before.length;
    text.selectionEnd = s + before.length + inner.length;
    text.dispatchEvent(new Event('input', { bubbles: true }));
  }
  function linePrefix(prefix) {
    var s = text.selectionStart, v = text.value;
    var start = v.lastIndexOf('\n', s - 1) + 1;
    text.value = v.slice(0, start) + prefix + v.slice(start);
    text.focus();
    text.selectionStart = text.selectionEnd = s + prefix.length;
  }
  if (toolbar) toolbar.addEventListener('click', function (e) {
    var b = e.target.closest('[data-md]');
    if (!b) return;
    switch (b.getAttribute('data-md')) {
      case 'bold': insert('**', '**'); break;
      case 'italic': insert('_', '_'); break;
      case 'heading': linePrefix('## '); break;
      case 'list': linePrefix('- '); break;
      case 'quote': linePrefix('> '); break;
      case 'link': insert('[', '](https://)'); break;
      case 'image': if (file) file.click(); break;
    }
  });

  // Pictures: pasted, dropped or chosen, sent to the server, which holds them
  // until the note is saved; the text names each by what it answers.
  function upload(files) {
    var images = Array.prototype.filter.call(files, function (f) { return /^image\//.test(f.type); });
    if (!images.length) return false;
    var data = new FormData();
    images.forEach(function (f, i) { data.append('image', f, f.name || ('image-' + (i + 1) + '.png')); });
    var mark = '\n' + (text.dataset.uploading || '…') + '\n';
    var at = text.selectionStart;
    text.value = text.value.slice(0, at) + mark + text.value.slice(at);
    fetch('/admin/notes/images', { method: 'POST', body: data, credentials: 'same-origin' })
      .then(function (r) { return r.json().then(function (j) { if (!r.ok) throw new Error(j.error || r.status); return j; }); })
      .then(function (list) {
        var md = list.map(function (img) {
          var alt = (img.name || '').replace(/\.[a-z0-9]+$/i, '').replace(/[\[\]]/g, '');
          return '![' + alt + '](' + img.ref + ')';
        }).join('\n\n');
        text.value = text.value.replace(mark, '\n' + md + '\n');
      })
      .catch(function (err) {
        text.value = text.value.replace(mark, '\n');
        window.alert(err && err.message ? err.message : (text.dataset.uploadFailed || ''));
      });
    return true;
  }
  text.addEventListener('paste', function (e) {
    var files = e.clipboardData && e.clipboardData.files;
    if (files && files.length && upload(files)) e.preventDefault();
  });
  text.addEventListener('dragover', function (e) { e.preventDefault(); });
  text.addEventListener('drop', function (e) {
    var files = e.dataTransfer && e.dataTransfer.files;
    if (files && files.length && upload(files)) e.preventDefault();
  });
  if (file) file.addEventListener('change', function () { upload(file.files); file.value = ''; });

  // The preview: the text as the family will read it, rendered by the server.
  if (mode && preview) mode.addEventListener('click', function () {
    var showing = mode.getAttribute('aria-pressed') === 'true';
    if (showing) {
      preview.hidden = true; text.hidden = false;
      mode.setAttribute('aria-pressed', 'false');
      mode.textContent = mode.dataset.preview;
      text.focus();
      return;
    }
    var body = new URLSearchParams();
    body.set('markdown', text.value);
    fetch('/admin/notes/preview', { method: 'POST', body: body, credentials: 'same-origin' })
      .then(function (r) { return r.text(); })
      .then(function (html) {
        preview.innerHTML = html;
        preview.hidden = false; text.hidden = true;
        mode.setAttribute('aria-pressed', 'true');
        mode.textContent = mode.dataset.write;
      });
  });

  // The people a note is about, as chips.
  var people = form.querySelector('[data-note-people]');
  if (people) {
    var input = people.querySelector('[data-person-input]');
    var add = people.querySelector('[data-person-add]');
    var chosen = people.querySelector('[data-note-chosen]');
    var options = Array.prototype.map.call(document.querySelectorAll('#pick-persons option'), function (o) { return o.value; });
    if (add) add.hidden = false;
    function addPerson() {
      var v = input.value.trim();
      if (!v || options.indexOf(v) === -1) { input.focus(); return; }
      var exists = Array.prototype.some.call(chosen.querySelectorAll('input[name="person"]'), function (i) { return i.value === v; });
      if (!exists) {
        var li = document.createElement('li');
        li.className = 'chip note-person';
        var hidden = document.createElement('input');
        hidden.type = 'hidden'; hidden.name = 'person'; hidden.value = v;
        var label = document.createElement('bdi');
        label.textContent = v;
        var rm = chosen.querySelector('[data-person-remove]');
        var btn = document.createElement('button');
        btn.type = 'button'; btn.className = 'note-person-remove';
        btn.setAttribute('data-person-remove', '');
        btn.setAttribute('aria-label', rm ? rm.getAttribute('aria-label') : (add.textContent || ''));
        btn.textContent = '×';
        li.appendChild(hidden); li.appendChild(label); li.appendChild(btn);
        chosen.appendChild(li);
      }
      input.value = '';
      input.focus();
    }
    if (add) add.addEventListener('click', addPerson);
    if (input) input.addEventListener('keydown', function (e) {
      if (e.key === 'Enter') { e.preventDefault(); addPerson(); }
    });
    if (input) input.addEventListener('change', function () {
      if (options.indexOf(input.value.trim()) !== -1) addPerson();
    });
    chosen.addEventListener('click', function (e) {
      var b = e.target.closest('[data-person-remove]');
      if (b) b.closest('li').remove();
    });
  }
})();
