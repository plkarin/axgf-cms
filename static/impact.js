/* The Impact timeline (docs/adr/0034, 0035), enhancement only.
 *
 * Pointing at a landmark, or reaching it with the keyboard, lights its
 * years across the timeline and on the axis, and its bar; choosing it opens
 * its window: the context, what was at stake, the victims. Pointing at a
 * life (the family's view) lights the band of its years: the landmarks it
 * crossed stand out, the others step back, and the person's name and years
 * show beside the pointer. Countries are named in the reader's language
 * when the browser knows how.
 */
(function () {
  'use strict';
  var lang = document.documentElement.lang || 'en';
  try {
    var names = new Intl.DisplayNames([lang], { type: 'region' });
    document.querySelectorAll('[data-country]').forEach(function (el) {
      var n = names.of(el.getAttribute('data-country'));
      if (n) el.textContent = n;
    });
  } catch (e) { /* the page's own names stay */ }

  var time = document.querySelector('[data-impact]');
  if (!time) return;
  var marks = Array.prototype.slice.call(time.querySelectorAll('.ip-ev, .ip-bar'));
  var evBand = time.querySelector('.ip-band-ev');
  var span = time.querySelector('.ip-axis-span');
  var dialog = document.querySelector('.ip-dialog');

  /* ---- a landmark: its years lit, its window ---- */
  var shown = null;
  function show(ev) {
    if (shown === ev) return;
    hide();
    shown = ev;
    var top = parseFloat(ev.getAttribute('data-top')) || 0;
    var h = Math.max(3, parseFloat(ev.getAttribute('data-height')) || 0);
    if (evBand) { evBand.style.top = top + 'px'; evBand.style.height = h + 'px'; evBand.hidden = false; }
    if (span) {
      span.style.top = top + 'px';
      span.style.height = h + 'px';
      span.querySelector('.ip-axis-from').textContent = ev.getAttribute('data-span') || '';
      span.hidden = false;
    }
    var key = ev.getAttribute('data-key');
    time.querySelectorAll('.ip-bar').forEach(function (b) { b.classList.toggle('ip-lit', b.getAttribute('data-key') === key); });
    ev.classList.add('ip-lit');
  }
  function hide() {
    if (!shown) return;
    shown.classList.remove('ip-lit');
    shown = null;
    if (evBand) evBand.hidden = true;
    if (span) span.hidden = true;
    time.querySelectorAll('.ip-bar.ip-lit').forEach(function (b) { b.classList.remove('ip-lit'); });
  }
  function open(ev) {
    var more = ev.querySelector('.ip-more');
    if (!dialog || !more || typeof dialog.showModal !== 'function') return;
    // The label's own nodes, cloned: what the page drew, as it drew it.
    var body = dialog.querySelector('.ip-dialog-body');
    body.replaceChildren.apply(body, Array.prototype.map.call(more.childNodes, function (n) { return n.cloneNode(true); }));
    dialog.showModal();
  }
  time.addEventListener('pointerover', function (e) {
    var ev = e.target.closest && e.target.closest('.ip-ev');
    if (ev) show(ev);
  });
  time.addEventListener('pointerout', function (e) {
    var ev = e.target.closest && e.target.closest('.ip-ev');
    if (ev && !(e.relatedTarget && ev.contains(e.relatedTarget))) hide();
  });
  time.addEventListener('focusin', function (e) {
    var ev = e.target.closest && e.target.closest('.ip-ev');
    if (ev) show(ev);
  });
  time.addEventListener('focusout', function (e) {
    if (e.target.closest && e.target.closest('.ip-ev')) hide();
  });
  time.addEventListener('click', function (e) {
    var ev = e.target.closest && e.target.closest('.ip-ev');
    if (ev) open(ev);
  });
  time.addEventListener('keydown', function (e) {
    var ev = e.target.closest && e.target.closest('.ip-ev');
    if (ev && (e.key === 'Enter' || e.key === ' ')) { e.preventDefault(); open(ev); }
  });
  if (dialog) {
    // A click on the backdrop closes it, as the button and Escape do.
    dialog.addEventListener('click', function (e) { if (e.target === dialog) dialog.close(); });
  }

  /* ---- a life (the family's view): the band of its years ---- */
  if (time.hasAttribute('data-from')) return;
  var band = time.querySelector('.ip-band:not(.ip-band-ev)'), tip = time.querySelector('.ip-tip');
  if (!band) return;
  var lit = null;
  function cross(from, to) {
    marks.forEach(function (m) {
      var a = parseFloat(m.getAttribute('data-from')), b = parseFloat(m.getAttribute('data-to'));
      m.classList.toggle('ip-hit', b >= from && a <= to);
    });
  }
  function on(life, x) {
    if (lit === life) return;
    lit = life;
    band.style.top = life.style.top;
    band.style.height = life.offsetHeight + 'px';
    band.hidden = false;
    cross(parseFloat(life.getAttribute('data-from')), parseFloat(life.getAttribute('data-to')));
    time.classList.add('ip-crossing');
    if (tip) {
      tip.textContent = life.getAttribute('title') || '';
      tip.style.top = (life.offsetTop - 30) + 'px';
      tip.style.left = (x === undefined ? life.offsetLeft + life.offsetParent.offsetLeft : x) + 'px';
      tip.hidden = false;
    }
  }
  function off() {
    lit = null;
    band.hidden = true;
    if (tip) tip.hidden = true;
    time.classList.remove('ip-crossing');
    marks.forEach(function (m) { m.classList.remove('ip-hit'); });
  }
  function xOf(e) { return e.clientX - time.getBoundingClientRect().left + 12; }
  time.addEventListener('pointerover', function (e) {
    var life = e.target.closest && e.target.closest('.ip-life');
    if (life) on(life, xOf(e));
  });
  time.addEventListener('pointerout', function (e) {
    var life = e.target.closest && e.target.closest('.ip-life');
    if (life && !(e.relatedTarget && life.contains(e.relatedTarget))) off();
  });
  time.addEventListener('focusin', function (e) {
    var life = e.target.closest && e.target.closest('.ip-life');
    if (life) on(life);
  });
  time.addEventListener('focusout', function (e) {
    if (e.target.closest && e.target.closest('.ip-life')) off();
  });
})();
