/* The river's travel. Enhancement only: the server's SVG is the first paint,
 * every person in it is a link, and the page is complete without this file.
 *
 * What it adds: a click slides the river until the chosen person holds the
 * fixed point (one 560 ms tween, no bounce, no stagger), hover lights the
 * route from the centre, the keyboard moves ↓ to a parent, ↑ to the eldest
 * child and ← back along the trail. Between two layouts it draws frames
 * itself with `paint`, a port of `river::render_svg`; at rest the SVG is
 * always the server's own. prefers-reduced-motion, or "cut", is an instant
 * cut, and the trail alone records the direction. */
(function () {
  'use strict';
  var root = document.querySelector('.river');
  var canvas = document.getElementById('rv-canvas');
  var dataEl = document.getElementById('rv-data');
  if (!root || !canvas || !dataEl) return;

  var BG = '#0c1310', MONO = 'ui-monospace,Menlo,monospace', W = 900, H = 640;
  var WIDTHS = [0, 1.6, 3, 5, 8, 12];
  var base = root.dataset.base || '/river';
  var panel = document.getElementById('tree-panel');
  var back = document.getElementById('rv-back');
  var data = JSON.parse(dataEl.textContent);
  var cur = data.river;                 // the layout at rest
  var trail = [cur.layout.meta.centre];
  var names = {};                       // id -> short name, for trail chips
  var cut = false, raf = null, busy = false;
  var reduce = window.matchMedia && matchMedia('(prefers-reduced-motion: reduce)').matches;

  function f(n) { var r = Math.round(n * 10) / 10; return r === 0 ? '0' : String(r); }
  function cls(d) { return d <= 1 ? 1 : d <= 5 ? 2 : d <= 20 ? 3 : d <= 80 ? 4 : 5; }
  function vc(x1, y1, x2, y2) { var m = (y1 + y2) / 2; return 'M' + f(x1) + ' ' + f(y1) + 'C' + f(x1) + ' ' + f(m) + ' ' + f(x2) + ' ' + f(m) + ' ' + f(x2) + ' ' + f(y2); }
  function esc(s) { return String(s).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;'); }
  var RAMP = [[0, [43, 90, 82]], [0.4, [63, 128, 114]], [0.68, [134, 176, 143]], [0.88, [211, 207, 158]], [1, [230, 176, 98]]];
  function ramp(year) {
    var t = Math.max(0, Math.min(1, (year - 1690) / 295)) || 0;
    for (var i = 1; i < RAMP.length; i++) if (t <= RAMP[i][0]) {
      var a = RAMP[i - 1], b = RAMP[i], k = (t - a[0]) / (b[0] - a[0]);
      return '#' + a[1].map(function (v, j) { return Math.round(v + (b[1][j] - v) * k).toString(16).padStart(2, '0'); }).join('');
    }
    return '#e6b062';
  }
  function byId(list) { var m = {}; list.forEach(function (o) { m[o.id || o.key] = o; }); return m; }
  var privateWord = (panel && panel.dataset.private) || '·';
  function remember(r) { r.shown.forEach(function (s) { names[s.id] = s.redacted ? privateWord : s.names[1]; }); }
  remember(cur);

  /* ---- frames ---- */
  function still(r) {
    var sh = byId(r.shown);
    return {
      persons: r.layout.persons.map(function (p) { return Object.assign({ a: 1, s: sh[p.id] }, p); }),
      couples: r.layout.couples.map(function (c) { return Object.assign({ a: 1 }, c); }),
      tails: r.layout.tails.map(function (t) { return Object.assign({ a: 1 }, t); }),
      meta: r.layout.meta
    };
  }
  function tween(A, B, e) {
    var pa = byId(A.persons), pb = byId(B.persons), anchor = A.meta.centre;
    var dx = pb[anchor] ? pb[anchor].x - pa[anchor].x : 0, dy = pb[anchor] ? pb[anchor].y - pa[anchor].y : 0;
    function mix(a, b) { return a + (b - a) * e; }
    function merge(la, lb, key, fn) {
      var ma = {}, mb = {}, keys = [];
      la.forEach(function (o) { ma[o[key]] = o; keys.push(o[key]); });
      lb.forEach(function (o) { mb[o[key]] = o; if (!(o[key] in ma)) keys.push(o[key]); });
      return keys.map(function (k) { return fn(ma[k], mb[k]); });
    }
    function slide(o, s, al) { return Object.assign({}, o, { x: o.x + dx * s, y: o.y + dy * s, a: al }); }
    var persons = merge(A.persons, B.persons, 'id', function (a, b) {
      if (a && b) return Object.assign({}, e < 0.5 ? a : b, { x: mix(a.x, b.x), y: mix(a.y, b.y), a: 1 });
      return a ? slide(a, e, 1 - e) : slide(b, -(1 - e), e);
    });
    var couples = merge(A.couples, B.couples, 'key', function (a, b) {
      if (a && b) return Object.assign({}, b, { x: mix(a.x, b.x), y: mix(a.y, b.y), a: 1,
        parents: a.parents.concat(b.parents.filter(function (p) { return a.parents.indexOf(p) < 0; })),
        kids: a.kids.concat(b.kids.filter(function (k) { return a.kids.indexOf(k) < 0; })),
        pconf: Object.assign({}, a.pconf, b.pconf), kconf: Object.assign({}, a.kconf, b.kconf) });
      return a ? slide(a, e, 1 - e) : slide(b, -(1 - e), e);
    });
    var tails = merge(A.tails, B.tails, 'key', function (a, b) {
      if (a && b) return Object.assign({}, b, { x1: mix(a.x1, b.x1), y1: mix(a.y1, b.y1), x2: mix(a.x2, b.x2), y2: mix(a.y2, b.y2), a: 1 });
      var t = a || b, s = a ? e : -(1 - e);
      return Object.assign({}, t, { x1: t.x1 + dx * s, y1: t.y1 + dy * s, x2: t.x2 + dx * s, y2: t.y2 + dy * s, a: a ? 1 - e : e });
    });
    var ka = A.meta.scale.knots, kb = B.meta.scale.knots, knots = kb;
    if (ka.length === kb.length) knots = ka.map(function (k, i) { return [mix(k[0], kb[i][0]), mix(k[1], kb[i][1])]; });
    return { persons: persons, couples: couples, tails: tails,
      meta: Object.assign({}, B.meta, { scale: { knots: knots }, rail: e < 0.5 ? A.meta.rail : B.meta.rail }) };
  }
  function yOf(knots, year) {
    var k = knots, i = 1;
    while (i < k.length - 1 && year > k[i][0]) i++;
    var a = k[i - 1], b = k[i];
    return a[1] + (year - a[0]) * (b[1] - a[1]) / (b[0] - a[0]);
  }

  /* ---- paint: a port of river::render_svg, for frames in motion ---- */
  function paint(fr) {
    var out = '', defs = '', gi = 0, PM = {}, named = false;
    var colour = fr.meta.era ? ramp : function () { return '#86b08f'; };
    fr.persons.forEach(function (p) { PM[p.id] = p; });
    // river::Meta::bands — none without an era, none for a signed-out reader.
    for (var Y = fr.meta.bands ? 1600 : 2050; Y < 2050; Y += 50) {
      var ya = yOf(fr.meta.scale.knots, Y), yb = yOf(fr.meta.scale.knots, Y + 50);
      if (yb > H || ya < 0) continue;
      var live = Y >= 1950, t = Math.max(0, yb), b = Math.min(H, ya);
      out += '<rect x="0" y="' + f(t) + '" width="900" height="' + f(b - t) + '" fill="' + (live ? '#15150f' : (Y / 50) % 2 ? '#0e1714' : BG) + '"/>';
      if (ya <= H) out += '<line x1="0" y1="' + f(ya) + '" x2="900" y2="' + f(ya) + '" stroke="#18241f"/><text x="10" y="' + f(ya - 6) + '" font-family="' + MONO + '" font-size="10" fill="' + (live ? '#8a7a52' : '#4f6158') + '">' + Y + '</text>';
      if (live && !named && t + 16 < b - 18 && (named = true)) out += '<text x="10" y="' + f(t + 16) + '" font-family="' + MONO + '" font-size="10" letter-spacing="1" fill="#8a7a52">' + esc(cur.words[0]) + '</text>';
    }
    fr.meta.rail.forEach(function (r) {
      if (r[1] < 8 || r[1] > H - 4) return;
      out += '<text x="890" y="' + f(r[1] + 3.5) + '" text-anchor="end" font-family="' + MONO + '" font-size="10" fill="' + (r[0] === 0 ? '#dcae64' : '#4f6158') + '">' + (r[0] > 0 ? '+' + r[0] : r[0] === 0 ? '0 ◂' : '−' + (-r[0])) + '</text>';
    });
    fr.tails.forEach(function (t) {
      var c = colour(t.year), w = WIDTHS[cls(t.d)], d = vc(t.x1, t.y1, t.x2, t.y2);
      if (t.kind === 'cont') {
        out += '<g opacity="' + f(t.a) + '"><path d="' + d + '" fill="none" stroke="' + c + '" stroke-width="' + w + '"/><polygon points="' + f(t.x2 - 4) + ',' + f(t.y2) + ' ' + f(t.x2 + 4) + ',' + f(t.y2) + ' ' + f(t.x2) + ',' + f(t.y2 + t.dir * 6) + '" fill="' + c + '"/><text x="' + f(t.x2 + 8) + '" y="' + f(t.y2 + (t.dir > 0 ? 4 : 2)) + '" font-family="' + MONO + '" font-size="9.5" fill="#8d9c92">+' + t.count + '</text></g>';
      } else {
        var id = 'tg' + (gi++);
        defs += '<linearGradient id="' + id + '" gradientUnits="userSpaceOnUse" x1="' + f(t.x1) + '" y1="' + f(t.y1) + '" x2="' + f(t.x2) + '" y2="' + f(t.y2) + '"><stop offset="0" stop-color="' + c + '" stop-opacity="0.85"/><stop offset="1" stop-color="' + c + '" stop-opacity="0"/></linearGradient>';
        out += '<g opacity="' + f(t.a) + '"><path d="' + d + '" fill="none" stroke="url(#' + id + ')" stroke-width="' + w + '"/><circle cx="' + f(t.x2) + '" cy="' + f(t.y2) + '" r="3.4" fill="' + BG + '" stroke="' + c + '" stroke-width="1.4"/></g>';
      }
    });
    fr.couples.forEach(function (c) {
      if (!c.stub) return;
      var side = c.x < 150 ? 1 : -1, x2 = c.x + side * 36, y2 = c.y - 32;
      out += '<g opacity="' + f(c.a) + '"><path d="' + vc(c.x, c.y, x2, y2) + '" fill="none" stroke="' + colour(c.stub_year) + '" stroke-opacity="0.45" stroke-width="' + WIDTHS[cls(c.stub_d)] + '"/><text x="' + f(x2 + side * 5) + '" y="' + f(y2 - 3) + '" text-anchor="' + (side < 0 ? 'end' : 'start') + '" font-family="' + MONO + '" font-size="10" fill="#8d9c92">+' + c.stub + '</text></g>';
    });
    edges(fr, PM).sort(function (a, b) { return b.d - a.d; }).forEach(function (e) {
      var d = vc(e.x1, e.y1, e.x2, e.y2), w = WIDTHS[cls(e.d)], o = ' opacity="' + f(e.a) + '"';
      if (e.conf === 'a') out += '<path d="' + d + '" fill="none" stroke="' + e.c + '" stroke-width="' + w + '" stroke-linecap="round"' + o + '/>';
      else if (e.conf === 'd') out += '<path d="' + d + '" fill="none" stroke="' + e.c + '" stroke-width="' + w + '" stroke-dasharray="' + f(Math.max(4, w * 1.3)) + ' ' + f(Math.max(3, w * 0.6)) + '"' + o + '/>';
      else out += '<g' + o + '><path d="' + d + '" fill="none" stroke="' + e.c + '" stroke-width="' + w + '" stroke-opacity="0.14"/><path d="' + d + '" fill="none" stroke="' + e.c + '" stroke-width="2.6" stroke-linecap="round" stroke-dasharray="0.1 5.5"/></g>';
    });
    fr.persons.forEach(function (p) {
      var s = p.s || {}, isC = p.role === 'centre', r = isC ? 7.5 : 4.8, x = f(p.x), y = f(p.y);
      out += '<g opacity="' + f(p.a) + '">';
      if (s.living) out += '<circle cx="' + x + '" cy="' + y + '" r="' + (r + 4) + '" fill="#e6b062" fill-opacity="0.16"/>';
      out += isC ? '<circle cx="' + x + '" cy="' + y + '" r="14" fill="none" stroke="#dcae64" stroke-width="1.2"/><circle cx="' + x + '" cy="' + y + '" r="' + r + '" fill="#dcae64"/>'
        : '<circle cx="' + x + '" cy="' + y + '" r="' + r + '" fill="' + (s.sparse || s.redacted ? BG : s.colour) + '" stroke="' + (s.sparse || s.redacted ? s.colour : BG) + '" stroke-width="1.6"/>';
      var tier = s.redacted ? 0 : isC ? 3 : edge(fit(p.lab, s, p.room), s, p.x, r);
      if (tier > 0) {
        var lx = f(p.x + r + (isC ? 10 : 6)), halo = ' paint-order="stroke" stroke="' + BG + '" stroke-width="4" stroke-linejoin="round"';
        out += '<text x="' + lx + '" y="' + f(p.y - 1) + '" font-family="\'Iowan Old Style\',Palatino,Georgia,serif" font-size="' + (isC ? 16 : 12) + '" font-weight="' + (isC ? 600 : 400) + '" fill="' + (isC ? '#f6efdc' : '#e2dccb') + '"' + halo + '>' + esc(s.names[tier - 1]) + '</text>'
          + '<text x="' + lx + '" y="' + f(p.y + (isC ? 14 : 11.5)) + '" font-family="' + MONO + '" font-size="' + (isC ? 11 : 9.5) + '" fill="#8d9c92"' + halo + '>' + esc(s.years) + '</text>';
      }
      out += '</g>';
    });
    return '<svg class="rv-svg" viewBox="0 0 900 640" width="100%" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><defs>' + defs + '</defs><rect width="900" height="640" fill="' + BG + '"/>' + out + '</svg>';
  }
  // river::fit_edge: shorten a label rather than run it past the right rail.
  function edge(t, s, x, r) {
    while (t > 0 && x + r + 6 + Math.max(s.names[t - 1].length * 6.3, s.years.length * 5.8) > 866) t--;
    return t;
  }
  // river::fit_tier: step a label down while it would run into its neighbour.
  function fit(t, s, room) {
    while (t > 0 && s.names[t - 1].length * 6.3 > room - 12) t--;
    return t;
  }
  function edges(fr, PM) {
    var list = [];
    fr.couples.forEach(function (c) {
      c.parents.forEach(function (id) { var p = PM[id]; if (p) list.push({ x1: p.x, y1: p.y, x2: c.x, y2: c.y, d: c.d, conf: c.pconf[id] || 'a', c: (p.s || {}).colour || '#86b08f', a: Math.min(c.a, p.a), from: id, cp: c }); });
      c.kids.forEach(function (id) { var k = PM[id]; if (k) list.push({ x1: c.x, y1: c.y, x2: k.x, y2: k.y, d: k.d, conf: c.kconf[id] || 'a', c: (k.s || {}).colour || '#86b08f', a: Math.min(c.a, k.a), to: id, cp: c }); });
    });
    return list;
  }

  /* ---- hover: the route from the centre, three stacked strokes ---- */
  function route(fr, target) {
    var adj = {}, prev = {}, q = [fr.meta.centre];
    function link(a, b) { (adj[a] = adj[a] || []).push(b); (adj[b] = adj[b] || []).push(a); }
    fr.couples.forEach(function (c) { var all = c.parents.concat(c.kids); all.forEach(function (a) { all.forEach(function (b) { if (a !== b) link(a, b); }); }); });
    prev[fr.meta.centre] = null;
    while (q.length) { var x = q.shift(); if (x === target) break; (adj[x] || []).forEach(function (y) { if (!(y in prev)) { prev[y] = x; q.push(y); } }); }
    if (!(target in prev)) return null;
    var set = {}; for (var y = target; y; y = prev[y]) set[y] = 1; return set;
  }
  function light(id) {
    var svg = canvas.querySelector('svg'), lit = svg && svg.querySelector('.rv-lit');
    if (!lit) return;
    var hl = svg.querySelector('.rv-hl') || svg.appendChild(document.createElementNS('http://www.w3.org/2000/svg', 'g'));
    hl.setAttribute('class', 'rv-hl');
    var fr = still(cur), on = id && route(fr, id), PM = byId(fr.persons), html = '', labels = '';
    if (on) {
      edges(fr, PM).forEach(function (e) {
        var c = e.cp, hit = e.from ? on[e.from] && (c.kids.some(function (k) { return on[k]; }) || c.parents.some(function (q) { return q !== e.from && on[q]; })) : on[e.to] && c.parents.some(function (q) { return on[q]; });
        if (hit) html += '<path d="' + vc(e.x1, e.y1, e.x2, e.y2) + '" fill="none" stroke="#f3dfae" STW stroke-linecap="round"/>';
      });
      fr.persons.forEach(function (p) {
        var s = p.s || {};
        if (!on[p.id] || p.role === 'centre' || s.redacted) return;
        if (p.id === id) labels += '<circle cx="' + f(p.x) + '" cy="' + f(p.y) + '" r="11" fill="none" stroke="#f3dfae" stroke-width="1.3"/>';
        // On hover the full name, right-aligned to the dot where it would
        // otherwise run past the rail: a hover label is alone on its row.
        var hw = s.names[2].length * 6.3, right = p.x + 10.8 + hw > 866;
        labels += '<text x="' + f(right ? p.x - 10.8 : p.x + 10.8) + '" y="' + f(p.y - 1) + '"' + (right ? ' text-anchor="end"' : '') + ' font-family="\'Iowan Old Style\',Palatino,Georgia,serif" font-size="12" fill="#f3dfae" paint-order="stroke" stroke="' + BG + '" stroke-width="4" stroke-linejoin="round">' + esc(s.names[2]) + '</text>';
      });
    }
    lit.innerHTML = html.replace(/STW/g, 'stroke-width="11" stroke-opacity="0.07"') + html.replace(/STW/g, 'stroke-width="5" stroke-opacity="0.18"') + html.replace(/STW/g, 'stroke-width="2"');
    hl.innerHTML = labels;
  }

  /* ---- travel ---- */
  function ease(t) { return t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2; }
  function travel(id, n, fromHistory) {
    n = n || cur.layout.meta.n;
    if (busy || (id === cur.layout.meta.centre && n === cur.layout.meta.n)) return;
    busy = true;
    fetch('/river/data?p=' + encodeURIComponent(id) + '&n=' + n, { credentials: 'same-origin' })
      .then(function (r) { if (!r.ok) throw r; return r.json(); })
      .then(function (body) {
        var A = still(cur), B = still(body.river);
        cur = body.river; remember(cur);
        if (!fromHistory) {
          if (id !== trail[trail.length - 1]) trail = trail.filter(function (x) { return x !== id; }).concat([id]).slice(-7);
          history.pushState({ p: id, n: n }, '', base + '?p=' + encodeURIComponent(id) + '&n=' + n);
        }
        swapPanel(id); controls(n);
        var done = function () { canvas.innerHTML = body.svg; busy = false; raf = null; };
        if (cut || reduce) return done();
        var t0 = performance.now();
        (function tick(now) {
          var t = Math.min(1, (now - t0) / 560);
          canvas.innerHTML = paint(tween(A, B, ease(t)));
          if (t < 1) raf = requestAnimationFrame(tick); else done();
        })(t0);
      })
      .catch(function () { busy = false; location.href = base + '?p=' + encodeURIComponent(id) + '&n=' + n; });
  }
  function swapPanel(id) {
    if (!panel) return;
    fetch('/tree/panel/' + encodeURIComponent(id), { credentials: 'same-origin' })
      .then(function (r) { return r.ok ? r.text() : null; })
      .then(function (html) {
        // A record this reader may not read is refused, and the panel says
        // so rather than keeping the previous person's record beside the
        // new centre.
        panel.innerHTML = html !== null ? html
          : '<div class="panel-inner panel-empty"><p class="muted">' + esc(panel.dataset.privateTitle || '') + '</p></div>';
      });
  }
  function controls(n) {
    var c = cur.layout.meta.centre;
    root.querySelectorAll('[data-range]').forEach(function (a) {
      a.href = base + '?p=' + encodeURIComponent(c) + '&n=' + a.dataset.range;
      if (+a.dataset.range === n) a.setAttribute('aria-current', 'true'); else a.removeAttribute('aria-current');
    });
    var grid = root.querySelector('.rv-grid'); if (grid) grid.href = '/tree?root=' + encodeURIComponent(c);
    var ol = root.querySelector('.rv-trail');
    ol.innerHTML = trail.map(function (id) {
      return '<li><a class="rv-chip" data-go="' + esc(id) + '" href="' + base + '?p=' + encodeURIComponent(id) + '&amp;n=' + n + '"' + (id === c ? ' aria-current="page"' : '') + '>' + esc(names[id] || '·') + '</a></li>';
    }).join('');
    back.disabled = trail.length < 2;
  }
  function goBack() {
    if (trail.length < 2) return;
    trail = trail.slice(0, -1);
    travel(trail[trail.length - 1]);
  }

  /* ---- wiring ---- */
  // Hit circles are r 20 in the 900-unit viewBox: about 40 px on a desktop,
  // under the 44 px a finger needs. On touch, a tap that misses every circle
  // goes to the nearest person within 44 px instead.
  var touch = false;
  canvas.addEventListener('pointerdown', function (e) { touch = e.pointerType === 'touch'; });
  function nearest(e) {
    var svg = canvas.querySelector('svg'); if (!svg) return null;
    var box = svg.getBoundingClientRect(), k = 900 / box.width, best = null, bd = 44 * k;
    var x = (e.clientX - box.left) * k, y = (e.clientY - box.top) * k;
    cur.layout.persons.forEach(function (p) { var d = Math.hypot(p.x - x, p.y - y); if (d < bd) { bd = d; best = p.id; } });
    return best;
  }
  canvas.addEventListener('click', function (e) {
    var a = e.target.closest && e.target.closest('.rv-p');
    var id = a ? a.getAttribute('data-id') : touch ? nearest(e) : null;
    if (!id) return;
    e.preventDefault(); travel(id);
  });
  canvas.addEventListener('mouseover', function (e) {
    var a = e.target.closest && e.target.closest('.rv-p');
    if (!busy) light(a ? a.getAttribute('data-id') : null);
  });
  canvas.addEventListener('mouseleave', function () { if (!busy) light(null); });
  root.addEventListener('click', function (e) {
    var t = e.target.closest && e.target.closest('[data-range],[data-motion],[data-go],[data-centre]');
    if (!t) return;
    if (t.dataset.range) { e.preventDefault(); travel(cur.layout.meta.centre, +t.dataset.range); }
    else if (t.dataset.motion) {
      cut = t.dataset.motion === 'cut';
      root.querySelectorAll('[data-motion]').forEach(function (b) { b.setAttribute('aria-pressed', String(b === t)); });
    } else { e.preventDefault(); travel(t.dataset.go || t.dataset.centre); }
  });
  back.addEventListener('click', goBack);
  document.addEventListener('keydown', function (e) {
    var tag = document.activeElement && document.activeElement.tagName;
    if (tag === 'INPUT' || tag === 'SELECT' || tag === 'TEXTAREA' || e.altKey || e.ctrlKey || e.metaKey) return;
    var to = e.key === 'ArrowDown' ? cur.down : e.key === 'ArrowUp' ? cur.up : null;
    if (to) { e.preventDefault(); travel(to); }
    else if (e.key === 'ArrowLeft' && trail.length > 1) { e.preventDefault(); goBack(); }
  });
  window.addEventListener('popstate', function (e) {
    if (e.state && e.state.p) travel(e.state.p, e.state.n, true);
  });
  history.replaceState({ p: cur.layout.meta.centre, n: cur.layout.meta.n }, '');
  controls(cur.layout.meta.n);
})();
