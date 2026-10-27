/* The one place vanilla JS is justified: filtering cards by name without a
 * round trip. Case-insensitive substring, no fuzzy matching.
 *
 * The filter dims rather than removes, so the layout and the SVG connectors
 * stay aligned — a person's position in the tree is information, and
 * collapsing rows on every keystroke would destroy it. */
function axgfTreeFilter() {
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
}

/* Hover highlight.
 *
 * The static layout removes every crossing it can and colours the ones it
 * cannot, but where two lines still meet the eye can lose the thread. Hovering
 * a card raises that person's own connectors to full and dims the rest, which
 * resolves the ambiguity instantly — the one interaction worth a few lines of
 * vanilla JS. It touches opacity only; hue stays put, because the two encode
 * different things (a raised line is the one you are looking at, a coloured one
 * is one that crosses another). */
function axgfTreeHover() {
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
}

/* Telling the server how wide the tree column actually is.
 *
 * The layout is computed in Rust and shipped as absolute coordinates — the SVG
 * connectors have to line up with the cards without a layout pass in the
 * browser — so the width a generation wraps to is chosen before the reader's
 * own width is knowable. The server's default covers the no-JavaScript case;
 * this measures the column that was actually rendered and stores it, so the
 * next navigation is folded to the real one.
 *
 * Deliberately not a reload. A page that re-fetched itself on load would flash
 * and would cost every reader a second request to save some of them a
 * scrollbar. Re-rooting the tree, changing the depth and following a card are
 * all full navigations already, so the measured width is used within one
 * click of arriving. */
function axgfTreeWidth() {
  var split = document.querySelector('.tree-split');
  var scroll = document.querySelector('.tree-scroll');
  if (!split || !scroll) return;
  try {
    var cs = getComputedStyle(split);
    var inner = split.clientWidth -
      parseFloat(cs.paddingInlineStart || 0) - parseFloat(cs.paddingInlineEnd || 0);
    var panel = document.getElementById('tree-panel');
    var gap = parseFloat(cs.columnGap) || 0;

    /* Is the record actually beside the tree? Below laptop width the split is
     * one column and the record sits underneath, where it takes none of the
     * tree's width. */
    var beside = false;
    if (panel && panel.getClientRects().length) {
      var a = scroll.getBoundingClientRect(), b = panel.getBoundingClientRect();
      beside = Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top) > 0;
    }

    /* The record column's width is itself derived from the tree's — it grows
     * into whatever the tree leaves — so measuring it now would answer a
     * question about the layout we already have rather than the one we want.
     * The fixed point is the narrowest the record may be: give the tree
     * everything except that. */
    var panelMin = parseFloat(cs.getPropertyValue('--panel-min')) || 320;
    /* The record column keeps its own width whatever the tree draws
     * (2026-10-23), so the tree may have everything else: tree | gap | record. */
    var panelW = beside ? Math.max(panelMin, panel.getBoundingClientRect().width) : 0;
    var avail = beside ? inner - panelW - gap : inner;
    avail = Math.round(avail);
    if (!(avail > 0)) return;
    // Round to a step so that a one-pixel scrollbar difference does not write
    // a new cookie — and a new layout — on every navigation.
    avail = Math.round(avail / 20) * 20;
    var current = (document.cookie.match(/(?:^|;\s*)axgf_tw=(\d+)/) || [])[1];
    if (current && Math.abs(parseInt(current, 10) - avail) < 20) return;
    document.cookie = 'axgf_tw=' + avail + ';path=/;max-age=31536000;samesite=lax';
  } catch (e) {
    // A preference, not a feature: if anything here fails the server default
    // is a working tree.
  }
}

/* Moving through the tree (the operator's review, 2026-10-23).
 *
 * A card is the way onto that person: the tree is drawn again around them,
 * in the same drawing, and their record opens in the panel beside it. With
 * scripting on it is a move, not a reload: the new page is fetched, its
 * <main> replaces this one's, and the tree slides from where the clicked
 * card was to where that person now stands, so the eye follows them. The
 * record panel keeps its place at the side whatever the new tree's width.
 * The URL follows with pushState, so the back button walks back through the
 * people visited. A modified click (new tab, middle button) is left to the
 * browser, and with scripting off every card is the same move as a plain
 * link. Inside the panel, a relative's name moves the tree onto them too;
 * "Open full page" stays a real navigation. */
(function () {
  var reduce = window.matchMedia && matchMedia('(prefers-reduced-motion: reduce)').matches;
  var busy = false;

  function plainClick(e) {
    return e.button === 0 && !e.metaKey && !e.ctrlKey && !e.shiftKey && !e.altKey;
  }
  function depthNow() {
    var p = document.getElementById('tree-panel');
    return (p && p.dataset.depth) || new URLSearchParams(window.location.search).get('depth') || '';
  }
  function urlFor(id) {
    var d = depthNow();
    return '/tree?root=' + encodeURIComponent(id) + (d ? '&depth=' + encodeURIComponent(d) : '') +
           '&sel=' + encodeURIComponent(id);
  }
  function cardOf(id) {
    var canvas = document.getElementById('tree-canvas');
    if (!canvas) return null;
    var cards = canvas.querySelectorAll('.tcard');
    for (var i = 0; i < cards.length; i++) if (cards[i].dataset.id === id) return cards[i];
    return null;
  }
  // The person the tree now stands on, in the middle of the tree's column
  // when the tree is wider than it.
  function centreOn(card) {
    var scroll = document.querySelector('.tree-scroll');
    if (!scroll || !card || scroll.scrollWidth <= scroll.clientWidth) return;
    var s = scroll.getBoundingClientRect(), c = card.getBoundingClientRect();
    scroll.scrollLeft += (c.left + c.width / 2) - (s.left + s.width / 2);
  }

  function go(id, push, from) {
    if (busy) return;
    busy = true;
    var url = urlFor(id);
    var was = from ? from.getBoundingClientRect() : null;
    var main = document.querySelector('main');
    if (main) main.setAttribute('aria-busy', 'true');
    fetch(url, { credentials: 'same-origin' })
      .then(function (r) { if (!r.ok) throw new Error('fetch'); return r.text(); })
      .then(function (html) {
        var doc = new DOMParser().parseFromString(html, 'text/html');
        var next = doc.querySelector('main');
        if (!main || !next) throw new Error('no main');
        document.title = doc.title;
        main.innerHTML = next.innerHTML;
        main.removeAttribute('aria-busy');
        if (push) history.pushState({ root: id }, '', url);
        wire();
        var card = cardOf(id);
        centreOn(card);
        var canvas = document.getElementById('tree-canvas');
        if (was && card && canvas && !reduce) {
          var now = card.getBoundingClientRect();
          canvas.style.transition = 'none';
          canvas.style.transform = 'translate(' + Math.round(was.left - now.left) + 'px,' + Math.round(was.top - now.top) + 'px)';
          canvas.style.opacity = '0.4';
          requestAnimationFrame(function () {
            requestAnimationFrame(function () {
              canvas.style.transition = 'transform 560ms cubic-bezier(0.22, 0.61, 0.36, 1), opacity 420ms ease-out';
              canvas.style.transform = '';
              canvas.style.opacity = '';
            });
          });
        }
        // Keep the reader where the tree is: bring its top back into view
        // when the new page starts above them.
        var split = document.querySelector('.tree-split');
        if (split && split.getBoundingClientRect().top < 0) split.scrollIntoView({ block: 'start' });
        busy = false;
      })
      .catch(function () {
        // Network or server error: the plain page, as without scripts.
        window.location.href = url;
      });
  }

  /* A card, folded and open (the operator's review, 2026-10-27). A click on
   * its face opens it and pins it open — its portrait in its laurels, its
   * dates, two ways on — and shows the person's record in the panel, the
   * tree staying where it is; a second click folds it. "Centre the tree
   * here" moves the tree onto them, as a click on a card used to. Near the
   * chart's edges the open card grows the other way; its portrait is
   * fetched the first time it opens. */
  // Folds every open card but the one `inside` is in (the card itself, or
  // a click's target): a click elsewhere, or Escape, folds them all.
  function close(inside) {
    var open = document.querySelectorAll('#tree-canvas .tcard.is-open');
    for (var i = 0; i < open.length; i++) {
      if (inside && open[i].contains(inside)) continue;
      open[i].classList.remove('is-open');
      var f = open[i].querySelector('.tc-face');
      if (f) f.setAttribute('aria-expanded', 'false');
    }
  }
  function prepare(card) {
    if (!card || card.dataset.ready) return;
    card.dataset.ready = '1';
    var img = card.querySelector('.tc-portrait img[data-src]');
    if (img) { img.src = img.getAttribute('data-src'); img.removeAttribute('data-src'); }
  }
  function place(card) {
    var scroll = document.querySelector('.tree-scroll');
    if (!card || !scroll) return;
    var s = scroll.getBoundingClientRect(), c = card.getBoundingClientRect();
    var rtl = document.documentElement.dir === 'rtl';
    var room = 132; // half the open card, and a margin
    var before = rtl ? s.right - (c.left + c.width / 2) : (c.left + c.width / 2) - s.left;
    var after = rtl ? (c.left + c.width / 2) - s.left : s.right - (c.left + c.width / 2);
    card.classList.toggle('tc-start', before < room);
    card.classList.toggle('tc-end', before >= room && after < room);
    card.classList.toggle('tc-up', s.bottom - c.top < 236 && c.top - s.top > 236);
  }
  function select(card) {
    var id = card && card.dataset.id;
    var panel = document.getElementById('tree-panel');
    if (!id || !panel) return;
    var cards = document.querySelectorAll('#tree-canvas .tcard');
    for (var i = 0; i < cards.length; i++) cards[i].classList.toggle('is-selected', cards[i].dataset.id === id);
    try {
      var u = new URL(window.location.href);
      u.searchParams.set('sel', id);
      history.replaceState(history.state, '', u.pathname + u.search);
    } catch (err) { /* the address bar stays as it was */ }
    fetch('/tree/panel/' + encodeURIComponent(id), { credentials: 'same-origin' })
      .then(function (r) { if (!r.ok) throw new Error('panel'); return r.text(); })
      .then(function (html) { panel.innerHTML = html; })
      .catch(function () { /* the record stays the one shown */ });
  }
  // A folded card is only its face, so the pointer or the keyboard reaches
  // the face first: the card learns then which way to grow, and fetches its
  // portrait. The card itself is never claimed — its links stay links.
  document.addEventListener('mouseover', function (e) {
    var face = e.target.closest && e.target.closest('#tree-canvas .tc-face');
    if (face && !face.contains(e.relatedTarget)) { place(face.parentNode); prepare(face.parentNode); }
  });
  document.addEventListener('focusin', function (e) {
    var face = e.target.closest && e.target.closest('#tree-canvas .tc-face');
    if (face) { place(face.parentNode); prepare(face.parentNode); }
  });
  document.addEventListener('keydown', function (e) {
    if (e.key === 'Escape') close(null);
  });

  // Cards, and a relative's name in the panel: the tree moves onto them.
  document.addEventListener('click', function (e) {
    var centre = e.target.closest && e.target.closest('#tree-canvas .tc-centre');
    if (centre && plainClick(e)) {
      var cc = centre.closest('.tcard[data-id]');
      if (cc) {
        e.preventDefault();
        close(null);
        go(cc.dataset.id, true, cc);
        return;
      }
    }
    var face = e.target.closest && e.target.closest('#tree-canvas .tc-face');
    if (face && plainClick(e)) {
      var card = face.parentNode;
      e.preventDefault();
      if (card.classList.contains('is-open')) {
        card.classList.remove('is-open');
        face.setAttribute('aria-expanded', 'false');
        return;
      }
      close(card);
      place(card);
      prepare(card);
      card.classList.add('is-open');
      face.setAttribute('aria-expanded', 'true');
      select(card);
      return;
    }
    close(e.target);
    var link = e.target.closest && e.target.closest('#tree-panel a[href^="/person/"]');
    if (link && plainClick(e) && !link.classList.contains('panel-open-full')) {
      e.preventDefault();
      var pid = decodeURIComponent(link.getAttribute('href').slice('/person/'.length).split(/[?#]/)[0]);
      if (pid) go(pid, true, cardOf(pid));
    }
  });

  // The page arrives on the person it stands on, in the middle of the
  // tree's column when the tree is wider than it — on a phone, always:
  // before, it arrived on the tree's left edge, rows of empty boxes.
  centreOn(document.querySelector('#tree-canvas .tcard.is-selected') || document.querySelector('#tree-canvas .tcard.is-root'));

  // The back and forward buttons walk the people visited.
  window.addEventListener('popstate', function () {
    var root = new URLSearchParams(window.location.search).get('root');
    if (root && document.getElementById('tree-canvas')) go(root, false, null);
  });
})();

/* The chart's size (2026-10-27): smaller to see a wide family whole, or
 * fitted to the column, remembered from one visit to the next. CSS `zoom`
 * scales the chart and its lines together, so the cards stay on them. */
function axgfTreeZoom() {
  var box = document.querySelector('[data-tree-zoom]');
  var canvas = document.getElementById('tree-canvas');
  var scroll = document.querySelector('.tree-scroll');
  if (!box || !canvas || !scroll) return;
  box.hidden = false;
  var z = 1;
  try { z = parseFloat(localStorage.getItem('axgf-tree-zoom')) || 1; } catch (e) { /* private window */ }
  function apply(v) {
    z = Math.max(0.5, Math.min(1.5, Math.round(v * 20) / 20));
    canvas.style.zoom = z === 1 ? '' : String(z);
    try { localStorage.setItem('axgf-tree-zoom', String(z)); } catch (e) { /* private window */ }
  }
  apply(z);
  box.addEventListener('click', function (e) {
    var b = e.target.closest && e.target.closest('button[data-z]');
    if (!b) return;
    e.preventDefault();
    if (b.dataset.z === 'fit') {
      var w = parseFloat(canvas.style.width) || canvas.scrollWidth;
      apply(Math.min(1, (scroll.clientWidth - 8) / w));
    } else {
      apply(z + (b.dataset.z === '+' ? 0.1 : -0.1));
    }
  });
}

/* Everything above that reads the page runs on the page as it arrives, and
 * again on every page the tree moves to. */
function wire() {
  // The depth's own button is for a reader without scripts (ui.js hides it
  // on the page as it arrives; a moved-to page brings it back).
  document.querySelectorAll('[data-autosubmit-go]').forEach(function (b) { b.hidden = true; });
  axgfTreeFilter();
  axgfTreeHover();
  axgfTreeWidth();
  axgfTreeZoom();
}
wire();
