/* The Impact timeline (docs/adr/0034), enhancement only.
 *
 * Pointing at a life, or reaching it with the keyboard, lights the band of
 * its years across the world's landmarks; the landmarks it crossed stand
 * out, the others step back, and the person's name and years show beside
 * the pointer. A person's own view is crossed from the start, by the page.
 * Countries are named in the reader's language when the browser knows how.
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
  if (!time || time.hasAttribute('data-from')) return;
  var band = time.querySelector('.ip-band'), tip = time.querySelector('.ip-tip');
  var marks = Array.prototype.slice.call(time.querySelectorAll('.ip-ev, .ip-bar'));
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
  time.addEventListener('focusout', off);
})();
