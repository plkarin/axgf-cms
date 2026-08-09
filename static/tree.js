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
