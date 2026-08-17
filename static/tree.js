/* The one place vanilla JS is justified: filtering cards by name without a
 * round trip. Case-insensitive substring, no fuzzy matching.
 *
 * The filter dims rather than removes, so the layout and the SVG connectors
 * stay aligned — a person's position in the tree is information, and
 * collapsing rows on every keystroke would destroy it. */
(function () {
  var box = document.getElementById('tree-filter');
  var canvas = document.getElementById('tree-canvas');
  var count = document.getElementById('tree-filter-count');
  if (!box || !canvas) return;

  var cards = Array.prototype.slice.call(canvas.querySelectorAll('.tcard'));
  var timer = null;

  function apply() {
    var q = box.value.trim().toLowerCase();
    if (!q) {
      canvas.classList.remove('filtering');
      for (var i = 0; i < cards.length; i++) cards[i].classList.remove('match', 'nomatch');
      count.textContent = '';
      return;
    }
    canvas.classList.add('filtering');
    var hits = 0;
    for (var j = 0; j < cards.length; j++) {
      var c = cards[j];
      if ((c.dataset.search || '').indexOf(q) !== -1) {
        c.classList.add('match');
        c.classList.remove('nomatch');
        hits++;
      } else {
        c.classList.add('nomatch');
        c.classList.remove('match');
      }
    }
    count.textContent = hits + (hits === 1 ? ' match' : ' matches');
  }

  box.addEventListener('input', function () {
    clearTimeout(timer);
    timer = setTimeout(apply, 60);
  });

  box.addEventListener('keydown', function (e) {
    if (e.key === 'Escape') {
      box.value = '';
      apply();
    }
  });

  /* Jump to the first match. */
  box.form && box.form.addEventListener('submit', function (e) {
    e.preventDefault();
    apply();
    var first = canvas.querySelector('.tcard.match');
    if (first) first.scrollIntoView({ block: 'center', inline: 'center' });
  });
})();

/* Hover highlight.
 *
 * The static layout removes every crossing it can and colours the ones it
 * cannot, but where two lines still meet the eye can lose the thread. Hovering
 * a card raises that person's own connectors to full and dims the rest, which
 * resolves the ambiguity instantly — the one interaction worth a few lines of
 * vanilla JS. It touches opacity only; hue stays put, because the two encode
 * different things (a raised line is the one you are looking at, a coloured one
 * is one that crosses another). */
(function () {
  var canvas = document.getElementById('tree-canvas');
  if (!canvas) return;
  var wires = Array.prototype.slice.call(canvas.querySelectorAll('.wire'));
  if (!wires.length) return;

  function highlight(id) {
    for (var i = 0; i < wires.length; i++) {
      var w = wires[i];
      var mine = w.dataset.from === id || w.dataset.to === id;
      w.style.opacity = mine ? '1' : '0.06';
      if (mine) {
        w.classList.add('lit');
      } else {
        w.classList.remove('lit');
      }
    }
  }
  function reset() {
    for (var i = 0; i < wires.length; i++) {
      var w = wires[i];
      w.style.opacity = w.dataset.base || '';
      w.classList.remove('lit');
    }
  }

  var cards = canvas.querySelectorAll('.tcard');
  for (var i = 0; i < cards.length; i++) {
    (function (card) {
      var id = card.dataset.id;
      if (!id) return;
      card.addEventListener('mouseenter', function () { highlight(id); });
      card.addEventListener('mouseleave', reset);
    })(cards[i]);
  }
})();

/* The side panel.
 *
 * A card click is the primary action and it loads that person's record into
 * the panel over a small fragment fetch, so the tree itself never reloads. The
 * URL is updated with pushState so the back button walks the selection history
 * and the current selection is copy-pasteable; re-centring the tree is a
 * separate, explicit control inside the panel. With scripting off every card
 * is still a plain link to the standalone /person/:id page, so nothing here is
 * load-bearing for the content — only for keeping it on one page. */
(function () {
  var panel = document.getElementById('tree-panel');
  var canvas = document.getElementById('tree-canvas');
  if (!panel || !canvas) return; // the ?all=1 view has no panel

  var depth = panel.dataset.depth || '3';
  var root = panel.dataset.root || '';

  function markSelected(id) {
    var cards = canvas.querySelectorAll('.tcard');
    for (var i = 0; i < cards.length; i++) {
      cards[i].classList.toggle('is-selected', cards[i].dataset.id === id);
    }
  }

  /* A modified click (new tab, middle button) is left to the browser, so a
   * card and every person link stay openable as ordinary permalinks. */
  function plainClick(e) {
    return e.button === 0 && !e.metaKey && !e.ctrlKey && !e.shiftKey && !e.altKey;
  }

  function loadPerson(id, push) {
    fetch('/tree/panel/' + encodeURIComponent(id))
      .then(function (r) { if (!r.ok) throw new Error('fetch'); return r.text(); })
      .then(function (html) {
        panel.innerHTML = html;
        markSelected(id);
        panel.scrollTop = 0;
        if (push) {
          var url = '/tree?root=' + encodeURIComponent(root) +
                    '&depth=' + encodeURIComponent(depth) +
                    '&sel=' + encodeURIComponent(id);
          history.pushState({ sel: id }, '', url);
        }
      })
      .catch(function () {
        // Network or server error: fall back to the standalone page.
        window.location.href = '/person/' + encodeURIComponent(id);
      });
  }

  // Card click → load into the panel.
  canvas.addEventListener('click', function (e) {
    var card = e.target.closest && e.target.closest('.tcard');
    if (!card || !plainClick(e)) return;
    var id = card.dataset.id;
    if (!id) return;
    e.preventDefault();
    loadPerson(id, true);
  });

  // Inside the panel: person links walk the family without leaving the page;
  // "Centre the tree here" re-roots; "Open full page" is a real navigation.
  panel.addEventListener('click', function (e) {
    var centre = e.target.closest && e.target.closest('[data-centre]');
    if (centre) {
      e.preventDefault();
      var cid = centre.getAttribute('data-centre');
      window.location.href = '/tree?root=' + encodeURIComponent(cid) +
                             '&depth=' + encodeURIComponent(depth) +
                             '&sel=' + encodeURIComponent(cid);
      return;
    }
    var link = e.target.closest && e.target.closest('a[href^="/person/"]');
    if (link && plainClick(e) && !link.classList.contains('panel-open-full')) {
      e.preventDefault();
      var pid = decodeURIComponent(
        link.getAttribute('href').slice('/person/'.length).split(/[?#]/)[0]
      );
      if (pid) loadPerson(pid, true);
    }
  });

  // The back/forward buttons walk the selection history.
  window.addEventListener('popstate', function (ev) {
    var params = new URLSearchParams(window.location.search);
    var id = (ev.state && ev.state.sel) || params.get('sel') || root;
    if (id) loadPerson(id, false);
  });
})();
