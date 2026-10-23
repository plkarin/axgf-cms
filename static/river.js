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

  // Every non-water colour is a class (k-*), painted by app.css from the
  // river's palette in tokens.css, exactly as river::render_svg does.
  var MONO = 'ui-monospace,Menlo,monospace';
  var WIDTHS = [0, 1.6, 3, 5, 8, 12];
  var base = root.dataset.base || '/';
  var panel = document.getElementById('tree-panel');
  var back = document.getElementById('rv-back');
  var data = JSON.parse(dataEl.textContent);
  var cur = data.river;                 // the layout at rest
  var trail = [cur.layout.meta.centre];
  var names = {};                       // id -> short name, for trail chips
  // The reader's motion choice, as the server rendered it from their cookie
  // (RIVER_MOTION_COOKIE): it survives travel, range changes, other pages and
  // reloads. prefers-reduced-motion forces a cut whatever was chosen, and
  // says so by disabling "travel".
  var cut = root.dataset.riverMotion === 'cut', raf = null, busy = false;
  var reduce = window.matchMedia && matchMedia('(prefers-reduced-motion: reduce)').matches;
  // Another drawing of the same river (v0.5.0, docs/adr/0007): Fusion's
  // living tree, Galaxy's solar system. A theme's script registers it before
  // this file runs; without one, the river. The layout, the travel, the
  // trail, the panel, the keyboard and the camera stay this file's under
  // every view: a view decides how a frame looks (`paint`), and may move
  // where its people stand (`shape`), from the same payload.
  var view = window.axgfView || null;
  // The frame at rest, as the view draws it: where the people stand on
  // screen, for the camera, the lit route and a finger's nearest person.
  var here = null;
  function shaped(r) { var fr = still(r); return view && view.shape ? view.shape(fr) : fr; }

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
  // A person drawn under two lines is two occurrences; edges name the key.
  function byKey(list) { var m = {}; list.forEach(function (o) { m[o.key] = o; }); return m; }
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
    var pa = byKey(A.persons), pb = byKey(B.persons), anchor = A.meta.centre;
    var dx = pb[anchor] ? pb[anchor].x - pa[anchor].x : 0, dy = pb[anchor] ? pb[anchor].y - pa[anchor].y : 0;
    function mix(a, b) { return a + (b - a) * e; }
    function merge(la, lb, key, fn) {
      var ma = {}, mb = {}, keys = [];
      la.forEach(function (o) { ma[o[key]] = o; keys.push(o[key]); });
      lb.forEach(function (o) { mb[o[key]] = o; if (!(o[key] in ma)) keys.push(o[key]); });
      return keys.map(function (k) { return fn(ma[k], mb[k]); });
    }
    function slide(o, s, al) { return Object.assign({}, o, { x: o.x + dx * s, y: o.y + dy * s, a: al, sref: [o.x, o.y] }); }
    var persons = merge(A.persons, B.persons, 'key', function (a, b) {
      if (a && b) return Object.assign({}, e < 0.5 ? a : b, { x: mix(a.x, b.x), y: mix(a.y, b.y), a: 1 });
      return a ? slide(a, e, 1 - e) : slide(b, -(1 - e), e);
    });
    var couples = merge(A.couples, B.couples, 'key', function (a, b) {
      if (a && b) return Object.assign({}, b, { x: mix(a.x, b.x), y: mix(a.y, b.y), a: 1, sref: [b.x, b.y],
        parents: a.parents.concat(b.parents.filter(function (p) { return a.parents.indexOf(p) < 0; })),
        kids: a.kids.concat(b.kids.filter(function (k) { return a.kids.indexOf(k) < 0; })),
        pconf: Object.assign({}, a.pconf, b.pconf), kconf: Object.assign({}, a.kconf, b.kconf) });
      return a ? slide(a, e, 1 - e) : slide(b, -(1 - e), e);
    });
    var tails = merge(A.tails, B.tails, 'key', function (a, b) {
      // cref: where the line ended when its count was placed, so the count
      // travels with the arrowhead.
      if (a && b) return Object.assign({}, b, { x1: mix(a.x1, b.x1), y1: mix(a.y1, b.y1), x2: mix(a.x2, b.x2), y2: mix(a.y2, b.y2), a: 1, cref: [b.x2, b.y2] });
      var t = a || b, s = a ? e : -(1 - e);
      return Object.assign({}, t, { x1: t.x1 + dx * s, y1: t.y1 + dy * s, x2: t.x2 + dx * s, y2: t.y2 + dy * s, a: a ? 1 - e : e, cref: [t.x2, t.y2] });
    });
    var ka = A.meta.scale.knots, kb = B.meta.scale.knots, knots = kb;
    if (ka.length === kb.length) knots = ka.map(function (k, i) { return [mix(k[0], kb[i][0]), mix(k[1], kb[i][1])]; });
    return { persons: persons, couples: couples, tails: tails,
      meta: Object.assign({}, B.meta, { scale: { knots: knots }, rail: e < 0.5 ? A.meta.rail : B.meta.rail,
        view: [mix(A.meta.view[0], B.meta.view[0]), mix(A.meta.view[1], B.meta.view[1])],
        hview: [mix(A.meta.hview[0], B.meta.hview[0]), mix(A.meta.hview[1], B.meta.hview[1])] }) };
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
    fr.persons.forEach(function (p) { PM[p.key] = p; });
    // river::Meta::bands — none without an era, none for a signed-out reader.
    var vt = fr.meta.view[0], vb = vt + fr.meta.view[1], hl0 = fr.meta.hview[0], hw = fr.meta.hview[1];
    for (var Y = fr.meta.bands ? 1600 : 2050; Y < 2050; Y += 50) {
      var ya = yOf(fr.meta.scale.knots, Y), yb = yOf(fr.meta.scale.knots, Y + 50);
      if (yb > vb || ya < vt) continue;
      var live = Y >= 1950, t = Math.max(vt, yb), b = Math.min(vb, ya);
      out += '<rect x="' + f(hl0) + '" y="' + f(t) + '" width="' + f(hw) + '" height="' + f(b - t) + '" class="' + (live ? 'k-band-live' : (Y / 50) % 2 ? 'k-band-odd' : 'k-bg') + '"/>';
      if (ya <= vb) out += '<line x1="' + f(hl0) + '" y1="' + f(ya) + '" x2="' + f(hl0 + hw) + '" y2="' + f(ya) + '" class="k-band-rule"/>'
        + (ya - 16 >= vt ? '<text x="' + f(hl0 + 10) + '" y="' + f(ya - 6) + '" font-family="' + MONO + '" font-size="10" class="' + (live ? 'k-band-text-live' : 'k-band-text') + '">' + Y + '</text>' : '');
      if (live && !named && t + 16 < b - 18 && (named = true)) out += '<text x="' + f(hl0 + 10) + '" y="' + f(t + 16) + '" font-family="' + MONO + '" font-size="10" letter-spacing="1" class="k-band-text-live">' + esc(cur.words[0]) + '</text>';
    }
    fr.meta.rail.forEach(function (r) {
      if (r[1] < vt + 8 || r[1] > vb - 4) return;
      out += '<text x="' + f(hl0 + hw - 10) + '" y="' + f(r[1] + 3.5) + '" text-anchor="end" font-family="' + MONO + '" font-size="10" class="' + (r[0] === 0 ? 'k-accent' : 'k-band-text') + '">' + (r[0] > 0 ? '+' + r[0] : r[0] === 0 ? '0 ◂' : '−' + (-r[0])) + '</text>';
    });
    // The Selva Verde ground (docs/adr/0004 §5): terrain under every line,
    // drawn from the same edges as the water, so it travels with them.
    var E = edges(fr, PM);
    out += banks(E);
    fr.tails.forEach(function (t) {
      var c = colour(t.year), w = WIDTHS[cls(t.d)], d = vc(t.x1, t.y1, t.x2, t.y2);
      if (t.kind === 'cont') {
        out += '<g opacity="' + f(t.a) + '"><path d="' + d + '" fill="none" stroke="' + c + '" stroke-width="' + w + '"/><polygon points="' + f(t.x2 - 4) + ',' + f(t.y2) + ' ' + f(t.x2 + 4) + ',' + f(t.y2) + ' ' + f(t.x2) + ',' + f(t.y2 + t.dir * 6) + '" fill="' + c + '"/>'
          + mark(t.count_at, '+' + t.count, 9.5, t.cref ? t.x2 - t.cref[0] : 0, t.cref ? t.y2 - t.cref[1] : 0) + '</g>';
      } else {
        var id = 'tg' + (gi++);
        defs += '<linearGradient id="' + id + '" gradientUnits="userSpaceOnUse" x1="' + f(t.x1) + '" y1="' + f(t.y1) + '" x2="' + f(t.x2) + '" y2="' + f(t.y2) + '"><stop offset="0" stop-color="' + c + '" stop-opacity="0.85"/><stop offset="1" stop-color="' + c + '" stop-opacity="0"/></linearGradient>';
        out += '<g opacity="' + f(t.a) + '"><path d="' + d + '" fill="none" stroke="url(#' + id + ')" stroke-width="' + w + '"/><circle cx="' + f(t.x2) + '" cy="' + f(t.y2) + '" r="3.4" class="k-bg" stroke="' + c + '" stroke-width="1.4"/></g>';
      }
    });
    fr.couples.forEach(function (c) {
      if (!c.stub) return;
      var x2 = c.x + c.stub_dx, y2 = c.y + c.stub_dy;
      out += '<g opacity="' + f(c.a) + '"><path d="' + vc(c.x, c.y, x2, y2) + '" fill="none" stroke="' + colour(c.stub_year) + '" stroke-opacity="0.45" stroke-width="' + WIDTHS[cls(c.stub_d)] + '"/>'
        + mark(c.stub_at, '+' + c.stub, 10, c.sref ? c.x - c.sref[0] : 0, c.sref ? c.y - c.sref[1] : 0) + '</g>';
    });
    E.slice().sort(function (a, b) { return b.d - a.d; }).forEach(function (e) {
      var d = vc(e.x1, e.y1, e.x2, e.y2), w = WIDTHS[cls(e.d)], o = ' opacity="' + f(e.a) + '"';
      if (e.conf === 'a') out += '<path d="' + d + '" fill="none" stroke="' + e.c + '" stroke-width="' + w + '" stroke-linecap="round"' + o + '/>';
      else if (e.conf === 'd') out += '<path d="' + d + '" fill="none" stroke="' + e.c + '" stroke-width="' + w + '" stroke-dasharray="' + f(Math.max(4, w * 1.3)) + ' ' + f(Math.max(3, w * 0.6)) + '"' + o + '/>';
      else out += '<g' + o + '><path d="' + d + '" fill="none" stroke="' + e.c + '" stroke-width="' + w + '" stroke-opacity="0.14"/><path d="' + d + '" fill="none" stroke="' + e.c + '" stroke-width="2.6" stroke-linecap="round" stroke-dasharray="0.1 5.5"/></g>';
    });
    out += glow(E) + cores(E);
    var T = tiers(fr);
    fr.persons.forEach(function (p) {
      var s = p.s || {}, isC = p.role === 'centre', r = isC ? 7.5 : 4.8, x = f(p.x), y = f(p.y);
      out += '<g opacity="' + f(p.a) + '">';
      // river::render_svg: a solid ring for a person reached by several
      // lines, a dashed one for a copy beside a later partner.
      if (p.repeat > 1) out += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 2.6) + '" fill="none" class="k-data-line" stroke-width="1"/>';
      if (p.again) out += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 2.6 + (p.repeat > 1 ? 2.2 : 0)) + '" fill="none" class="k-data-line" stroke-width="1" stroke-dasharray="2.4 1.8"/>';
      if (s.living) out += '<circle cx="' + x + '" cy="' + y + '" r="' + (r + 4) + '" class="k-glow" fill-opacity="0.16"/>';
      out += isC ? '<circle cx="' + x + '" cy="' + y + '" r="14" fill="none" class="k-accent-line" stroke-width="1.2"/><circle cx="' + x + '" cy="' + y + '" r="' + r + '" class="k-accent"/>'
        : '<circle cx="' + x + '" cy="' + y + '" r="' + r + (s.sparse || s.redacted ? '" class="k-bg" stroke="' : '" class="k-halo" fill="') + s.colour + '" stroke-width="1.6"/>';
      var tier = T[p.key] || 0;
      if (tier > 0) {
        var lx = f(p.x + r + (isC ? 10 : 6)), halo = ' paint-order="stroke" stroke-width="4" stroke-linejoin="round"';
        out += '<text x="' + lx + '" y="' + f(p.y - 1) + '" font-family="\'Iowan Old Style\',Palatino,Georgia,serif" font-size="' + (isC ? 16 : 12) + '" font-weight="' + (isC ? 600 : 400) + '" class="' + (isC ? 'k-centre' : 'k-name') + ' k-halo"' + halo + '>' + esc(isC ? clip(s.names[tier - 1], Math.min(p.right - 17.5 - 6.8, hl0 + hw - 34 - (p.x + r + 10))) : s.names[tier - 1]) + '</text>'
          + '<text x="' + lx + '" y="' + f(p.y + (isC ? 14 : 11.5)) + '" font-family="' + MONO + '" font-size="' + (isC ? 11 : 9.5) + '" class="k-data k-halo"' + halo + '>' + esc(s.years) + '</text>';
      }
      out += '</g>';
    });
    return '<svg class="rv-svg" viewBox="' + f(hl0) + ' ' + f(vt) + ' ' + f(hw) + ' ' + f(fr.meta.view[1]) + '" width="100%" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><defs>' + defs + '</defs><rect x="' + f(hl0) + '" y="' + f(vt) + '" width="' + f(hw) + '" height="' + f(fr.meta.view[1]) + '" class="k-bg"/>' + out + '</svg>';
  }
  /* ---- the ground's lights (v0.5.0, docs/adr/0004 §5) ----
   * Decoration only, and the client's: the server's SVG is unchanged, so the
   * byte-identical invariants of ADR 0001 hold of it exactly as before, and
   * everything drawn here is computed from the payload this page already
   * holds — the same edges, at the same places.
   *
   * Zones, after Selva Verde: the ground the lines run through, drawn as a
   * cartographer draws terrain — a translucent area with crisp contour lines
   * nested inside it, teal where the ancestors are (below the fixed point),
   * ochre where the descendants are (above it). They are the isolines of a
   * field made by the lines themselves (marching squares), so they follow
   * every branch and merge where branches gather, and they say nothing the
   * lines do not.
   * Banks: a thin dark edge under every line, so the water stays sharp on
   * the forest. Cores: attested water carries a thread of light down its
   * middle, so confidence also reads as brightness — solid and lit, dashed
   * and dull, dotted and faint. Plain strokes and fills: no filter, no
   * clip-path (the river's rendering contract). */
  var ZONE_CELL = 3, ZONE_PAD = 60, ZONE_LEVELS = [0.14, 0.34, 0.58, 0.8];
  function banks(E) {
    var out = '';
    E.forEach(function (e) {
      var w = WIDTHS[cls(e.d)], o = e.a < 1 ? ' opacity="' + f(e.a) + '"' : '';
      out += '<path d="' + vc(e.x1, e.y1, e.x2, e.y2) + '" stroke-width="' + f(w + 2.6) + '"' + o + '/>';
    });
    return out ? '<g class="rv-banks" fill="none" stroke-linecap="round" aria-hidden="true">' + out + '</g>' : '';
  }
  function cores(E) {
    var out = '';
    E.forEach(function (e) {
      if (e.conf !== 'a') return;
      var w = WIDTHS[cls(e.d)], o = e.a < 1 ? ' opacity="' + f(e.a) + '"' : '';
      out += '<path d="' + vc(e.x1, e.y1, e.x2, e.y2) + '" stroke-width="' + f(Math.max(0.6, w * 0.32)) + '"' + o + '/>';
    });
    return out ? '<g class="rv-cores k-core" fill="none" stroke-linecap="round" stroke-opacity="0.92" aria-hidden="true">' + out + '</g>' : '';
  }
  // The glow: the same threads, wider and faint, in the warm light of the
  // lit route — two passes, so the light falls off as Selva Verde's path
  // does. Attested water only, as the cores: what is surest shines.
  function glow(E) {
    var out = '';
    [[3.2, 0.1], [1.7, 0.22]].forEach(function (g) {
      var paths = '';
      E.forEach(function (e) {
        if (e.conf !== 'a') return;
        var w = WIDTHS[cls(e.d)], o = e.a < 1 ? ' opacity="' + f(e.a) + '"' : '';
        paths += '<path d="' + vc(e.x1, e.y1, e.x2, e.y2) + '" stroke-width="' + f(Math.max(0.6, w * 0.32) * g[0]) + '"' + o + '/>';
      });
      if (paths) out += '<g class="rv-glow" fill="none" stroke-linecap="round" stroke-opacity="' + g[1] + '" aria-hidden="true">' + paths + '</g>';
    });
    return out;
  }
  // A box blur, run twice, is close enough to a Gaussian for a contour.
  function blurField(fl, W, H, r) {
    var tmp = new Float32Array(W * H), n = 2 * r + 1, x, y, s, row;
    for (y = 0; y < H; y++) {
      row = y * W; s = 0;
      for (x = -r; x <= r; x++) s += fl[row + Math.min(W - 1, Math.max(0, x))];
      for (x = 0; x < W; x++) {
        tmp[row + x] = s / n;
        s += fl[row + Math.min(W - 1, x + r + 1)] - fl[row + Math.max(0, x - r)];
      }
    }
    for (x = 0; x < W; x++) {
      s = 0;
      for (y = -r; y <= r; y++) s += tmp[Math.min(H - 1, Math.max(0, y)) * W + x];
      for (y = 0; y < H; y++) {
        fl[y * W + x] = s / n;
        s += tmp[Math.min(H - 1, y + r + 1) * W + x] - tmp[Math.max(0, y - r) * W + x];
      }
    }
  }
  // Marching squares: the segments where the field crosses `lv`, joined into
  // loops. The field is padded with a cell of zero on every side, so every
  // contour closes.
  function isolines(fl, W, H, lv) {
    var segs = [], x, y;
    for (y = 0; y < H - 1; y++) for (x = 0; x < W - 1; x++) {
      var a = fl[y * W + x], b = fl[y * W + x + 1], c = fl[(y + 1) * W + x + 1], d = fl[(y + 1) * W + x];
      var k = (a > lv ? 8 : 0) | (b > lv ? 4 : 0) | (c > lv ? 2 : 0) | (d > lv ? 1 : 0);
      if (k === 0 || k === 15) continue;
      var T = [x + (lv - a) / (b - a), y], R = [x + 1, y + (lv - b) / (c - b)],
          B = [x + (lv - d) / (c - d), y + 1], L = [x, y + (lv - a) / (d - a)];
      switch (k) {
        case 1: case 14: segs.push([L, B]); break;
        case 2: case 13: segs.push([B, R]); break;
        case 3: case 12: segs.push([L, R]); break;
        case 4: case 11: segs.push([T, R]); break;
        case 6: case 9: segs.push([T, B]); break;
        case 7: case 8: segs.push([L, T]); break;
        case 5: segs.push([L, T], [B, R]); break;
        case 10: segs.push([L, B], [T, R]); break;
      }
    }
    var key = function (p) { return Math.round(p[0] * 64) + ',' + Math.round(p[1] * 64); };
    var at = {}, used = new Uint8Array(segs.length), loops = [];
    segs.forEach(function (sg, i) { [0, 1].forEach(function (e) { var kk = key(sg[e]); (at[kk] = at[kk] || []).push(i); }); });
    function grow(line, front) {
      for (;;) {
        var tip = front ? line[0] : line[line.length - 1], near = at[key(tip)] || [], next = -1;
        for (var i = 0; i < near.length; i++) if (!used[near[i]]) { next = near[i]; break; }
        if (next < 0) return;
        used[next] = 1;
        var sg = segs[next], other = key(sg[0]) === key(tip) ? sg[1] : sg[0];
        if (front) line.unshift(other); else line.push(other);
      }
    }
    for (var i = 0; i < segs.length; i++) {
      if (used[i]) continue;
      used[i] = 1;
      var line = [segs[i][0], segs[i][1]];
      grow(line, false); grow(line, true);
      if (line.length > 5) loops.push(line);
    }
    return loops;
  }
  function zoneField(list, meta) {
    // Beyond the server's window by ZONE_PAD on every side: the window
    // widens to the stage (below), and a zone stopped at its old edge would
    // show a straight cut.
    var hv = [meta.hview[0] - ZONE_PAD, meta.hview[1] + 2 * ZONE_PAD], v = [meta.view[0] - ZONE_PAD, meta.view[1] + 2 * ZONE_PAD], S = ZONE_CELL;
    var W = Math.ceil(hv[1] / S) + 3, H = Math.ceil(v[1] / S) + 3;
    var cv = document.createElement('canvas');
    cv.width = W; cv.height = H;
    var ctx = cv.getContext('2d', { willReadFrequently: true });
    if (!ctx) return null;
    ctx.setTransform(1 / S, 0, 0, 1 / S, 1.5 - hv[0] / S, 1.5 - v[0] / S);
    ctx.lineCap = 'round'; ctx.lineJoin = 'round'; ctx.strokeStyle = 'white';
    list.forEach(function (e) {
      var m = (e.y1 + e.y2) / 2;
      ctx.lineWidth = WIDTHS[cls(e.d)] + 46;
      ctx.beginPath(); ctx.moveTo(e.x1, e.y1); ctx.bezierCurveTo(e.x1, m, e.x2, m, e.x2, e.y2); ctx.stroke();
    });
    var px = ctx.getImageData(0, 0, W, H).data, fl = new Float32Array(W * H);
    for (var i = 0; i < W * H; i++) fl[i] = px[i * 4 + 3] / 255;
    blurField(fl, W, H, 5); blurField(fl, W, H, 5);
    // The padding cell back to zero, so the loops close inside the grid.
    for (var x = 0; x < W; x++) { fl[x] = 0; fl[(H - 1) * W + x] = 0; }
    for (var y = 0; y < H; y++) { fl[y * W] = 0; fl[y * W + W - 1] = 0; }
    return { fl: fl, W: W, H: H, x0: hv[0] - 1.5 * S, y0: v[0] - 1.5 * S };
  }
  function zonePath(F, lv) {
    var S = ZONE_CELL, d = '';
    isolines(F.fl, F.W, F.H, lv).forEach(function (loop) {
      var px = null, py = null, part = '';
      loop.forEach(function (p) {
        var x = F.x0 + p[0] * S, y = F.y0 + p[1] * S;
        if (px !== null && Math.abs(x - px) + Math.abs(y - py) < 1.6) return;
        part += (part ? 'L' : 'M') + f(x) + ' ' + f(y);
        px = x; py = y;
      });
      if (part) d += part + 'Z';
    });
    return d;
  }
  function zones(fr, E) {
    var up = [], down = [], cy = 318, out = '';
    E.forEach(function (e) { ((e.y1 + e.y2) / 2 < cy ? up : down).push(e); });
    fr.tails.forEach(function (t) { ((t.y1 + t.y2) / 2 < cy ? up : down).push({ x1: t.x1, y1: t.y1, x2: t.x2, y2: t.y2, d: t.d }); });
    [['up', up], ['down', down]].forEach(function (side) {
      if (!side[1].length) return;
      var F = zoneField(side[1], fr.meta);
      if (!F) return;
      ZONE_LEVELS.forEach(function (lv, k) {
        var d = zonePath(F, lv);
        if (d) out += '<path class="rv-zone rv-zone-' + side[0] + ' z' + k + '" d="' + d + '"/>';
      });
    });
    return out ? '<g class="rv-zones" fill-rule="evenodd" aria-hidden="true">' + out + '</g>' : '';
  }
  // Lay them on the server's still frame: zones and banks under the first
  // line, the cores over the last, under the lit route and the people.
  function decorate() {
    var svg = canvas.querySelector('svg.rv-svg');
    if (view) {
      if (!svg) return;
      if (view.rest) view.rest(svg, here, api);
      place(svg, here.meta);
      sizeHits();
      placeNames();
      if (window.axgfRiverScene) window.axgfRiverScene.layout(sceneLayout());
      return;
    }
    if (!svg || svg.querySelector('.rv-zones, .rv-banks, .rv-cores')) return;
    var fr = still(cur), E = edges(fr, byKey(fr.persons));
    var tmp = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    tmp.innerHTML = zones(fr, E) + banks(E) + glow(E) + cores(E);
    var first = null;
    for (var n = svg.firstElementChild; n; n = n.nextElementSibling) {
      var tag = n.tagName.toLowerCase();
      if (tag === 'g' || tag === 'path' || tag === 'a') { first = n; break; }
    }
    var z = tmp.querySelector('.rv-zones'), b = tmp.querySelector('.rv-banks'), c = tmp.querySelector('.rv-cores');
    if (z) svg.insertBefore(z, first);
    if (b) svg.insertBefore(b, first);
    var lit = svg.querySelector('.rv-lit') || first || null;
    tmp.querySelectorAll('.rv-glow').forEach(function (g) { svg.insertBefore(g, lit); });
    if (c) svg.insertBefore(c, lit);
    // The scale of a generation is a statement about time: drawn only for a
    // river that has an era (ADR 0001 §3), as the bands are.
    var sc = root.querySelector('[data-rv-scale]');
    if (sc) sc.hidden = !cur.layout.meta.bands;
    place(svg, cur.layout.meta);
    sizeHits();
    placeNames();
    if (window.axgfRiverScene) window.axgfRiverScene.layout(sceneLayout());
  }

  /* ---- names: the near ones, or all of them (2026-10-23) ----
   * A drawing names whom it has room for, and the reader could not tell why
   * one person had a name and the next none. Two ways now, the reader's
   * choice, under every view and the river alike, remembered:
   * - near (the default): the generations either side of the fixed point
   *   (−1, 0, +1) are all named, and the others are not;
   * - all: everyone is named, each name the drawing left out set on a
   *   ground of its own beside its person — to whichever side is free of
   *   names, people and other grounds, further out if it must, with a
   *   thread back to its person.
   * The drawing's own names stay where it put them; this only hides the far
   * ones (app.css, `data-far`) and adds the missing ones. */
  var NAMES = 'axgf-river-names', namesAll = false;
  try { namesAll = localStorage.getItem(NAMES) === 'all'; } catch (e) { /* private window */ }
  function placeNames() {
    var svg = canvas.querySelector('svg.rv-svg');
    root.classList.toggle('rv-names-all', namesAll);
    root.classList.toggle('rv-names-near', !namesAll);
    if (!svg || !here) return;
    var old = svg.querySelector('.rv-names');
    if (old) old.remove();
    var PK = byKey(here.persons), named = {};
    svg.querySelectorAll('a.rv-p').forEach(function (a) {
      var p = PK[a.getAttribute('data-key')], far = p && Math.abs(p.gen) > 1;
      if (far) a.setAttribute('data-far', ''); else a.removeAttribute('data-far');
      if (a.querySelector('text:not(.gn-initials)') && !(far && !namesAll)) named[a.getAttribute('data-key')] = 1;
    });
    // What is already on the page: every word and panel, every person.
    var boxes = [], vb = svg.viewBox.baseVal;
    svg.querySelectorAll('text, rect.tv-panel, rect.gn-card').forEach(function (t) {
      var a = t.closest('a.rv-p');
      if (t.closest('.rv-hl') || (!namesAll && a && a.hasAttribute('data-far'))) return;
      try { var b = t.getBBox(); if (b.width) boxes.push([b.x - 2, b.y - 1, b.x + b.width + 2, b.y + b.height + 1]); } catch (e) { /* not rendered */ }
    });
    here.persons.forEach(function (p) { boxes.push([p.x - 7, p.y - 7, p.x + 7, p.y + 7]); });
    function overlap(b) {
      var n = 0;
      for (var i = 0; i < boxes.length; i++) {
        var q = boxes[i];
        if (b[0] < q[2] && b[2] > q[0] && b[1] < q[3] && b[3] > q[1]) n++;
      }
      return n;
    }
    var out = '', lo = vb.x + 2, hi = vb.x + vb.width - 2, top = vb.y + 2, bot = vb.y + vb.height - 2;
    here.persons.slice().sort(function (a, b) { return Math.abs(a.gen) - Math.abs(b.gen) || a.x - b.x; }).forEach(function (p) {
      var s = p.s || {};
      if (named[p.key] || s.redacted || !s.names || (!namesAll && Math.abs(p.gen) > 1)) return;
      var word = Math.abs(p.gen) <= 1 ? s.names[1] : s.names[0], w = width(word, 11, false) + 8, h = 15;
      var best = null, least = Infinity;
      [10, 20, 32, 46, 62, 80].some(function (d) {
        return [[d, -h / 2], [-d - w, -h / 2], [-w / 2, -d - h], [-w / 2, d], [d * 0.7, -d * 0.7 - h], [-d * 0.7 - w, -d * 0.7 - h], [d * 0.7, d * 0.7], [-d * 0.7 - w, d * 0.7]].some(function (o) {
          var b = [p.x + o[0], p.y + o[1], p.x + o[0] + w, p.y + o[1] + h];
          if (b[0] < lo || b[2] > hi || b[1] < top || b[3] > bot) return false;
          var n = overlap(b);
          if (n < least) { least = n; best = b; }
          return n === 0;
        });
      });
      if (!best) return;
      boxes.push(best);
      // The thread from the person's edge to the nearest point of the ground.
      var tx = Math.max(best[0], Math.min(p.x, best[2])), ty = Math.max(best[1], Math.min(p.y, best[3]));
      var dx = tx - p.x, dy = ty - p.y, dl = Math.hypot(dx, dy);
      out += '<g class="rv-cs rv-name" style="transform-origin:' + f(p.x) + 'px ' + f(p.y) + 'px">';
      if (dl > 9) out += '<line class="rv-name-thread" x1="' + f(p.x + dx / dl * 6) + '" y1="' + f(p.y + dy / dl * 6) + '" x2="' + f(tx) + '" y2="' + f(ty) + '"/>';
      out += '<rect class="rv-name-box" x="' + f(best[0]) + '" y="' + f(best[1]) + '" width="' + f(w) + '" height="' + f(h) + '" rx="3"/>'
        + '<text class="rv-name-text" x="' + f(best[0] + 4) + '" y="' + f(best[1] + 11) + '" font-size="11">' + esc(word) + '</text></g>';
    });
    if (!out) return;
    var g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
    g.setAttribute('class', 'rv-names');
    g.setAttribute('aria-hidden', 'true');
    g.innerHTML = out;
    svg.insertBefore(g, svg.querySelector('.rv-hl'));
  }

  /* ---- the window and the camera (v0.5.0, docs/adr/0004 §11) ----
   * The drawing is never shown larger than KMAX pixels to the unit. On a
   * 1920-pixel screen it used to fill the stage, and its 12-unit names grew
   * to 22 pixels. Wider than that, the window widens instead: the ground and
   * the half-century bands run to the stage's edges, the years and the rail
   * go with them, and the drawing stays in the middle at the size of the
   * rest of the site. The window depends on the range and the stage alone,
   * so within a range the fixed point still does not move (ADR 0001 §5).
   *
   * The camera is a zoom, 1 to ZMAX, and the point it looks at. Zoomed, the
   * people and every word keep their size and only the distances between
   * them grow, as a map's symbols do (`rv-cs`, scaled back by app.css). */
  var KMAX = 1.15, ZMAX = 3, cam = { z: 1, x: null, y: null, iz: 1 };
  // The drawing is replaced, never the stage: the living ground and the
  // water's light (river-scene.js) sit beside it and keep their state.
  function setSvg(markup) {
    var tpl = document.createElement('template');
    tpl.innerHTML = markup;
    var svg = tpl.content.querySelector('svg'), old = canvas.querySelector('svg.rv-svg');
    if (old) old.replaceWith(svg); else canvas.appendChild(svg);
    return svg;
  }
  function frame(meta) {
    var hv = meta.hview, v = meta.view, sw = canvas.clientWidth || hv[1];
    var k = Math.min(sw / hv[1], KMAX), w = Math.max(hv[1], sw / k);
    return { x: hv[0] - (w - hv[1]) / 2, y: v[0], w: w, h: v[1], k: k, hv: hv };
  }
  // Once per drawing: which rects and rules are the ground and the bands
  // (as wide as the server's window), which words are the years and the
  // rail (at its edges), and the point every other word and person keeps
  // when scaled back.
  function prepare(svg, hv) {
    var near = function (a, b) { return Math.abs(a - b) < 0.6; }, P = { bands: [], rules: [], rails: [] };
    svg.querySelectorAll('rect').forEach(function (r) {
      if (near(+r.getAttribute('x'), hv[0]) && near(+r.getAttribute('width'), hv[1])) P.bands.push(r);
    });
    svg.querySelectorAll('line.k-band-rule').forEach(function (l) { P.rules.push(l); });
    svg.querySelectorAll('text').forEach(function (t) {
      if (t.closest('a.rv-p')) return;
      var x = +t.getAttribute('x');
      t.classList.add('rv-cs');
      if (near(x, hv[0] + 10)) P.rails.push([t, -1]);
      else if (t.getAttribute('text-anchor') === 'end' && near(x, hv[0] + hv[1] - 10)) P.rails.push([t, 1]);
      else t.style.transformOrigin = t.getAttribute('x') + 'px ' + t.getAttribute('y') + 'px';
    });
    svg.querySelectorAll('a.rv-p').forEach(function (a) {
      var c = a.querySelector('circle');
      if (c) a.style.transformOrigin = c.getAttribute('cx') + 'px ' + c.getAttribute('cy') + 'px';
    });
    svg._rv = P;
    return P;
  }
  function apply(svg, F) {
    var P = svg._rv || prepare(svg, F.hv);
    P.bands.forEach(function (r) { r.setAttribute('x', F.x.toFixed(2)); r.setAttribute('width', F.w.toFixed(2)); });
    P.rules.forEach(function (l) { l.setAttribute('x1', F.x.toFixed(2)); l.setAttribute('x2', (F.x + F.w).toFixed(2)); });
    var z = cam.z, vw = F.w / z, vh = F.h / z;
    var cx = cam.x === null ? F.x + F.w / 2 : cam.x, cy = cam.y === null ? F.y + F.h / 2 : cam.y;
    cx = Math.min(Math.max(cx, F.x + vw / 2), F.x + F.w - vw / 2);
    cy = Math.min(Math.max(cy, F.y + vh / 2), F.y + F.h - vh / 2);
    if (cam.x !== null) { cam.x = cx; cam.y = cy; }
    var x = cx - vw / 2, y = cy - vh / 2;
    svg.setAttribute('viewBox', x.toFixed(2) + ' ' + y.toFixed(2) + ' ' + vw.toFixed(2) + ' ' + vh.toFixed(2));
    // A word grows with the zoom until it reaches its own size (KMAX: on a
    // phone the drawing starts smaller), and not past it.
    var iz = cam.iz = Math.min(1, KMAX / (F.k * z));
    if (iz < 0.999) { svg.setAttribute('data-zoom', ''); svg.style.setProperty('--rv-iz', iz.toFixed(4)); }
    else { svg.removeAttribute('data-zoom'); svg.style.removeProperty('--rv-iz'); }
    P.rails.forEach(function (r) {
      var tx = (r[1] < 0 ? x + 10 / z : x + vw - 10 / z).toFixed(2);
      r[0].setAttribute('x', tx);
      r[0].style.transformOrigin = tx + 'px ' + r[0].getAttribute('y') + 'px';
    });
    canvas.classList.toggle('rv-zoomed', z > 1);
    if (zoomInput && Math.abs(+zoomInput.value - z) > 0.01) zoomInput.value = String(Math.round(z * 10) / 10);
    if (window.axgfRiverScene) window.axgfRiverScene.view();
  }
  function place(svg, meta) { if (svg) apply(svg, frame(meta)); return svg; }
  function centreOf(layout) {
    var c = layout.persons.filter(function (p) { return p.role === 'centre'; })[0];
    return c ? { x: c.x, y: c.y } : null;
  }
  // The layout the living ground draws its banks and its light from.
  function sceneLayout() {
    var fr = here;
    return {
      edges: edges(fr, byKey(fr.persons)).map(function (e) { return { x1: e.x1, y1: e.y1, x2: e.x2, y2: e.y2, d: e.d, conf: e.conf }; }),
      persons: fr.persons.map(function (p) { return { x: p.x, y: p.y, role: p.role, gen: p.gen, living: !!(p.s && p.s.living) }; })
    };
  }
  // Zoom to `z` about a point on screen, which stays where it is under the
  // pointer or the fingers; about the middle when there is none.
  function zoomTo(z, clientX, clientY) {
    var svg = canvas.querySelector('svg.rv-svg');
    if (!svg || busy) return;
    var F = frame(here.meta), vb = svg.viewBox.baseVal, box = svg.getBoundingClientRect();
    if (!box.width) return;
    z = Math.min(ZMAX, Math.max(1, z));
    var fx = clientX === undefined ? 0.5 : (clientX - box.left) / box.width;
    var fy = clientY === undefined ? 0.5 : (clientY - box.top) / box.height;
    var ux = vb.x + fx * vb.width, uy = vb.y + fy * vb.height;
    cam.z = z;
    if (z === 1) { cam.x = null; cam.y = null; }
    else { cam.x = ux + (0.5 - fx) * F.w / z; cam.y = uy + (0.5 - fy) * F.h / z; }
    apply(svg, F);
    sizeHits();
  }
  function panBy(dx, dy) {
    var svg = canvas.querySelector('svg.rv-svg');
    if (!svg || cam.z <= 1) return;
    var vb = svg.viewBox.baseVal, u = vb.width / svg.getBoundingClientRect().width;
    cam.x = vb.x + vb.width / 2 - dx * u; cam.y = vb.y + vb.height / 2 - dy * u;
    apply(svg, frame(here.meta));
  }
  var zoomInput = root.querySelector('[data-rv-zoom]');

  // river::write_mark: a count where river::place_counts put it, moved with
  // its mark while the river is in motion (dx, dy: how far the mark has
  // travelled from where the count was placed).
  function mark(m, text, size, dx, dy) {
    if (!m) return '';
    dx = dx || 0; dy = dy || 0;
    var out = '';
    if (m.leader) out += '<line x1="' + f(m.leader[0] + dx) + '" y1="' + f(m.leader[1] + dy) + '" x2="' + f(m.leader[2] + dx) + '" y2="' + f(m.leader[3] + dy) + '" class="k-data-line" stroke-width="0.6" stroke-opacity="0.7"/>';
    return out + '<text x="' + f(m.x + dx) + '" y="' + f(m.y + dy) + '"' + (m.anchor < 0 ? ' text-anchor="end"' : m.anchor === 0 ? ' text-anchor="middle"' : '')
      + (m.halo ? ' paint-order="stroke" stroke-width="3" stroke-linejoin="round"' : '')
      + ' font-family="' + MONO + '" font-size="' + size + '" class="k-data' + (m.halo ? ' k-halo' : '') + '">' + esc(text) + '</text>';
  }
  // river::text_width: 0.55 em serif, 0.6 em bold or mono, 1 em wide scripts.
  function width(text, size, bold) {
    var w = 0;
    for (var i = 0; i < text.length; i++) {
      var c = text.charCodeAt(i);
      if (c >= 0xD800 && c <= 0xDBFF) { i++; }
      var wide = (c >= 0x1100 && c <= 0x115F) || (c >= 0x2E80 && c <= 0xA4CF) || (c >= 0xAC00 && c <= 0xD7A3) || (c >= 0xF900 && c <= 0xFAFF) || (c >= 0xFE30 && c <= 0xFE4F) || (c >= 0xFF00 && c <= 0xFF60) || (c >= 0xFFE0 && c <= 0xFFE6);
      w += size * (wide ? 1 : bold ? 0.6 : 0.55);
    }
    return w;
  }
  // river::fit_edge: shorten a label rather than run it past the right rail
  // (river::Meta::label_edge); the centre keeps at least its given name.
  function edge(t, s, x, r, isC, lim) {
    var floor = isC ? 1 : 0, start = x + r + (isC ? 10 : 6), yw = width(s.years, isC ? 11 : 9.5, true);
    while (t > floor && start + Math.max(width(s.names[t - 1], isC ? 16 : 12, isC), yw) > lim) t--;
    return t;
  }
  // river::fit_tier: step a label down while it, or its years line, would run
  // into its right-hand neighbour's dot; the centre keeps its given name.
  function fit(t, s, gap, isC) {
    var yw = width(s.years, isC ? 11 : 9.5, true), room = gap - (isC ? 17.5 : 10.8) - 6.8;
    while (t > (isC ? 1 : 0) && Math.max(width(s.names[t - 1], isC ? 16 : 12, isC), yw) > room) t--;
    return t;
  }
  // river::label_tiers: row by row, left to right; a label that would start
  // inside the previous one is dropped, unless it is the centre's.
  function tiers(fr) {
    var rows = {}, out = {};
    fr.persons.forEach(function (p) { (rows[Math.round(p.y)] = rows[Math.round(p.y)] || []).push(p); });
    Object.keys(rows).forEach(function (k) {
      var last = -Infinity;
      // river::outer_radius: the centre's ring, a repeat ring, or the dot.
      function outer(q) { return q.role === 'centre' ? 14 : 4.8 + (q.again && q.repeat > 1 ? 4.8 : q.again || q.repeat > 1 ? 2.6 : 0); }
      var row = rows[k].sort(function (a, b) { return a.x - b.x; });
      row.forEach(function (p, j) {
        var wider = row[j + 1] ? outer(row[j + 1]) - 4.8 : 0;
        var s = p.s || {}, isC = p.role === 'centre', r = isC ? 7.5 : 4.8, start = p.x + r + (isC ? 10 : 6);
        var t = s.redacted ? 0 : isC ? 3 : p.lab;
        if (t > 0) t = edge(fit(t, s, p.right - wider, isC), s, p.x, r, isC, fr.meta.hview[0] + fr.meta.hview[1] - 34);
        if (t > 0 && !isC && start < last + 4) t = 0;
        if (t > 0) last = start + Math.max(width(s.names[t - 1], isC ? 16 : 12, isC), width(s.years, isC ? 11 : 9.5, true));
        out[p.key] = t;
      });
    });
    return out;
  }
  // river::clip_label: the centre always labels, cut to fit if it must.
  function clip(name, room) {
    if (width(name, 16, true) <= room) return name;
    var out = '';
    for (var i = 0; i < name.length && width(out + name[i] + '…', 16, true) <= room; i++) out += name[i];
    return out.replace(/\s+$/, '') + '…';
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
    var fr = here, on = id && route(fr, id), PM = byKey(fr.persons), html = '', labels = '';
    if (view && view.light) { view.light(svg, fr, on, id, api); return; }
    var curve = view && view.edge ? view.edge : vc;
    if (on) {
      edges(fr, PM).forEach(function (e) {
        var c = e.cp, hit = e.from ? on[e.from] && (c.kids.some(function (k) { return on[k]; }) || c.parents.some(function (q) { return q !== e.from && on[q]; })) : on[e.to] && c.parents.some(function (q) { return on[q]; });
        if (hit) html += '<path d="' + curve(e.x1, e.y1, e.x2, e.y2) + '" fill="none" class="KL" STW stroke-linecap="round"/>';
      });
      // Where a word may run before it meets the rail: the window as it is
      // on screen now (widened, zoomed), less the rail's room. A word is
      // scaled back by the zoom (cam.iz), its distance from the dot with it.
      var vb = svg.viewBox.baseVal, F = frame(fr.meta), u = 1 / (F.k * cam.z), T = view && view.named ? view.named() : tiers(fr);
      fr.persons.forEach(function (p) {
        var s = p.s || {};
        if (!on[p.key] || p.role === 'centre' || s.redacted) return;
        // Every person on the route is a station of the path, ringed in its
        // light; the one under the pointer, a wider ring. Each station is a
        // map symbol: it keeps its size when the river is zoomed.
        var target = p.key === id;
        labels += '<g class="rv-cs" style="transform-origin:' + f(p.x) + 'px ' + f(p.y) + 'px">';
        labels += '<circle cx="' + f(p.x) + '" cy="' + f(p.y) + '" r="' + (target ? 11 : 8.5) + '" fill="none" class="k-lit" stroke-width="' + (target ? 1.3 : 0.9) + '"' + (target ? '' : ' stroke-opacity="0.7"') + '/>';
        // The person under the pointer is named in full; a station the
        // drawing left unnamed, by its shortest name. A theme's view says
        // itself whom it named (docs/adr/0007). A row is rarely theirs
        // alone, so the word sits on a ground of its own (sized in place
        // below) and goes to whichever side the window has room on: it may
        // cover a neighbour's word for as long as it is lit, and is never
        // read through one.
        var word = target ? s.names[2] : T[p.key] ? '' : s.names[0];
        if (word) {
          var right = p.x + (10.8 + width(word, 12, false)) * cam.iz > vb.x + vb.width - 34 * u;
          labels += '<rect class="rv-hl-box" rx="3"/><text x="' + f(right ? p.x - 10.8 : p.x + 10.8) + '" y="' + f(p.y - 1) + '"' + (right ? ' text-anchor="end"' : '') + ' font-family="\'Iowan Old Style\',Palatino,Georgia,serif" font-size="12" class="k-lit-fill k-halo" paint-order="stroke" stroke-width="4" stroke-linejoin="round">' + esc(word) + '</text>';
        }
        labels += '</g>';
      });
    }
    // Three stacked strokes as before — a halo, a glow and the light — now
    // with a warm core and a thread of light running along it from the
    // sources toward the living, three times, then still (app.css). Strokes
    // only: no filter (the river's rendering contract).
    function layer(cls, stw) { return html.replace(/KL/g, cls).replace(/STW/g, stw); }
    lit.innerHTML = layer('k-lit-halo', 'stroke-width="11" stroke-opacity="0.1"') + layer('k-lit', 'stroke-width="5.5" stroke-opacity="0.24"')
      + layer('k-lit', 'stroke-width="2.8" stroke-opacity="0.92"') + layer('k-core', 'stroke-width="1.2"') + layer('k-flow', 'stroke-width="1.7"');
    hl.innerHTML = labels;
    // Each word's ground, the size the browser actually set it at.
    hl.querySelectorAll('rect.rv-hl-box').forEach(function (r) {
      var b = r.nextElementSibling && r.nextElementSibling.getBBox();
      if (!b || !b.width) return;
      r.setAttribute('x', f(b.x - 3)); r.setAttribute('y', f(b.y - 1.5));
      r.setAttribute('width', f(b.width + 6)); r.setAttribute('height', f(b.height + 3));
    });
    // The lit person's own word steps aside for their full name.
    svg.querySelectorAll('a.rv-p.rv-looked').forEach(function (a) { a.classList.remove('rv-looked'); });
    if (on) svg.querySelectorAll('a.rv-p').forEach(function (a) { if (a.getAttribute('data-key') === id) a.classList.add('rv-looked'); });
  }

  /* ---- travel ---- */
  function ease(t) { return t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2; }
  function travel(id, n, fromHistory) {
    n = n || cur.layout.meta.n;
    if (busy || (id === cur.layout.meta.centre && n === cur.layout.meta.n)) return;
    busy = true;
    // Where the river is going, ripples (the reference's loader), if the
    // answer is slow enough to be waited for. The first frame of the travel
    // takes them away.
    var waitTimer = setTimeout(function () { waitAt(id); }, 140);
    fetch('/river/data?p=' + encodeURIComponent(id) + '&n=' + n, { credentials: 'same-origin' })
      .then(function (r) { if (!r.ok) throw r; return r.json(); })
      .then(function (body) {
        var A = here, B = shaped(body.river);
        cur = body.river; here = B; remember(cur);
        if (!fromHistory) {
          if (id !== trail[trail.length - 1]) trail = trail.filter(function (x) { return x !== id; }).concat([id]).slice(-7);
          history.pushState({ p: id, n: n, tab: tab }, '', url(id, n));
        }
        swapPanel(id); controls(n);
        clearTimeout(waitTimer);
        if (window.axgfRiverScene) window.axgfRiverScene.travel();
        // Zoomed in, the camera glides to the fixed point with the river, so
        // the person travelled to arrives in view.
        var c0 = { x: cam.x, y: cam.y }, c1 = centreOf(B);
        var done = function () {
          canvas.querySelectorAll('.rv-wait').forEach(function (w) { w.remove(); });
          if (cam.z > 1 && c1) { cam.x = c1.x; cam.y = c1.y; }
          setSvg(view ? view.paint(here, api, true) : body.svg); busy = false; raf = null; decorate(); arrive();
        };
        if (cut || reduce) return done();
        var t0 = performance.now();
        (function tick(now) {
          // A view may blend two frames in its own space (Fusion's glass
          // turns round its axis), from the river's own blend.
          var t = Math.min(1, (now - t0) / (view && view.tween ? 900 : 560));
          var fr = view && view.tween ? view.tween(A, B, ease(t), tween) : tween(A, B, ease(t));
          if (cam.z > 1 && c0.x !== null && c1) { cam.x = c0.x + (c1.x - c0.x) * ease(t); cam.y = c0.y + (c1.y - c0.y) * ease(t); }
          place(setSvg(view ? view.paint(fr, api, false) : paint(fr)), fr.meta);
          if (t < 1) raf = requestAnimationFrame(tick); else done();
        })(t0);
      })
      .catch(function () { clearTimeout(waitTimer); busy = false; location.href = url(id, n); });
  }
  function waitAt(id) {
    var svg = canvas.querySelector('svg'), p = here.persons.filter(function (q) { return q.id === id; })[0];
    if (!svg || !p || cut || reduce) return;
    var box = svg.getBoundingClientRect(), vb = svg.viewBox.baseVal;
    var w = document.createElement('span');
    w.className = 'rv-wait';
    w.setAttribute('aria-hidden', 'true');
    w.innerHTML = '<i></i><i></i>';
    w.style.left = ((p.x - vb.x) / vb.width * box.width) + 'px';
    w.style.top = ((p.y - vb.y) / vb.height * box.height) + 'px';
    canvas.appendChild(w);
  }
  // The fixed point's ring breathes twice when a reader arrives (app.css).
  var arriveTimer = null;
  function arrive() {
    if (cut || reduce) return;
    root.classList.remove('rv-arrived');
    void root.offsetWidth;
    root.classList.add('rv-arrived');
    clearTimeout(arriveTimer);
    arriveTimer = setTimeout(function () { root.classList.remove('rv-arrived'); }, 4200);
  }
  // The record tab beside the river; kept as the reader travels. A person
  // without that tab is drawn on their first one by the server.
  var tab = new URLSearchParams(location.search).get('tab') || 'record';
  function tabQuery() { return tab !== 'record' ? '&tab=' + encodeURIComponent(tab) : ''; }
  function url(id, n) {
    return base + '?p=' + encodeURIComponent(id) + '&n=' + n + tabQuery();
  }
  function swapPanel(id) {
    if (!panel) return;
    fetch('/tree/panel/' + encodeURIComponent(id) + '?river=1&n=' + cur.layout.meta.n + tabQuery(), { credentials: 'same-origin' })
      .then(function (r) { return r.ok ? r.text() : null; })
      .then(function (html) {
        // A record this reader may not read is refused, and the panel says
        // so rather than keeping the previous person's record beside the
        // new centre.
        panel.innerHTML = html !== null ? html
          : '<div class="panel-inner panel-empty"><p class="muted">' + esc(panel.dataset.privateTitle || '') + '</p></div>';
        if (window.axgfShowCurrentTabs) window.axgfShowCurrentTabs(panel);
      });
  }
  function controls(n) {
    var c = cur.layout.meta.centre;
    // The range is one select (the minimalism pass); its form's `p` follows
    // the centre so that, without this script, submitting it still works.
    var sel = root.querySelector('select[data-range-select]');
    if (sel) sel.value = String(n);
    var pin = root.querySelector('input[data-range-p]');
    if (pin) pin.value = c;
    var ol = root.querySelector('.rv-trail');
    ol.innerHTML = trail.map(function (id) {
      return '<li><a class="btn small rv-chip" data-go="' + esc(id) + '" href="' + base + '?p=' + encodeURIComponent(id) + '&amp;n=' + n + '"' + (id === c ? ' aria-current="page"' : '') + '>' + esc(names[id] || '·') + '</a></li>';
    }).join('');
    // The itinerary keeps to one line and scrolls sideways (app.css): the
    // stop the reader is at, the last, is brought into view. Negative in
    // a right-to-left page, where the line starts at the right.
    ol.scrollLeft = getComputedStyle(ol).direction === 'rtl' ? -ol.scrollWidth : ol.scrollWidth;
    // Nothing to go back to is nothing drawn: hidden, not disabled.
    back.hidden = trail.length < 2;
    if (whole) whole.setAttribute('aria-pressed', n === 5 ? 'true' : 'false');
    sizeHits();
  }
  function goBack() {
    if (trail.length < 2) return;
    trail = trail.slice(0, -1);
    travel(trail[trail.length - 1]);
  }

  /* ---- wiring ---- */
  // Hit circles are r 20 in viewBox units: about 40 px on a desktop at ±5,
  // under the 44 px a finger needs. On touch, a tap that misses every circle
  // goes to the nearest person within 44 px instead.
  var touch = false;
  canvas.addEventListener('pointerdown', function (e) { touch = e.pointerType === 'touch'; });
  function nearest(e) {
    var svg = canvas.querySelector('svg'); if (!svg) return null;
    var vb = svg.viewBox.baseVal, box = svg.getBoundingClientRect(), k = vb.width / box.width, best = null, bd = 44 * k;
    var x = (e.clientX - box.left) * k + vb.x, y = (e.clientY - box.top) * k + vb.y;
    here.persons.forEach(function (p) { var d = Math.hypot(p.x - x, p.y - y); if (d < bd) { bd = d; best = p.id; } });
    return best;
  }
  // A target a finger can hit (44 CSS px): the hit circle is sized for the
  // drawing's current scale, never below its drawn 20 units. Zoomed, the
  // person is scaled back (cam.iz), and its circle with it.
  function sizeHits() {
    var svg = canvas.querySelector('svg'); if (!svg) return;
    var box = svg.getBoundingClientRect(); if (!box.width) return;
    var r = Math.max(20, 22 * svg.viewBox.baseVal.width / box.width / cam.iz);
    canvas.querySelectorAll('a.rv-p > circle:first-child').forEach(function (c) { c.setAttribute('r', r.toFixed(1)); });
  }
  // A new width is a new window: the drawing keeps its size, the ground
  // its edges.
  window.addEventListener('resize', function () { place(canvas.querySelector('svg.rv-svg'), here.meta); sizeHits(); });
  // Hover does not exist on a tablet. "Tap to look, tap again to go": the
  // first tap on a person does what hovering does with a mouse — lights the
  // route from the centre and shows their dropped label — and a second tap
  // on the same person travels there. A tap on open water clears it.
  var looked = null;
  canvas.addEventListener('click', function (e) {
    // The click a drag or a pinch ends with is not a choice of person.
    if (performance.now() - dragged < 400) { e.preventDefault(); return; }
    var a = e.target.closest && e.target.closest('.rv-p');
    var id = a ? a.getAttribute('data-id') : touch ? nearest(e) : null;
    if (touch && !busy) {
      e.preventDefault();
      if (!id) { looked = null; light(null); return; }
      if (id !== looked) {
        looked = id;
        var p = here.persons.filter(function (q) { return q.id === id; })[0];
        light(a ? a.getAttribute('data-key') : p && p.key);
        return;
      }
    }
    if (!id) return;
    looked = null;
    e.preventDefault(); travel(id);
  });
  // Hover is a mouse's (or a pen's): a finger's taps are handled above, and
  // the mouse events a browser fakes after a tap would otherwise put out the
  // route the tap has just lit.
  canvas.addEventListener('pointerover', function (e) {
    if (e.pointerType === 'touch' || busy) return;
    var a = e.target.closest && e.target.closest('.rv-p');
    light(a ? a.getAttribute('data-key') : null);
  });
  canvas.addEventListener('pointerleave', function (e) { if (e.pointerType !== 'touch' && !busy) light(null); });
  // Zoom: Ctrl (⌘) with the wheel — which is also what a trackpad's pinch
  // sends — two fingers, the slider, or + and − on the keyboard. A wheel
  // alone still scrolls the page. Zoomed, a drag moves the camera; at rest
  // a finger still scrolls the page (touch-action, app.css).
  canvas.addEventListener('wheel', function (e) {
    if (!(e.ctrlKey || e.metaKey)) return;
    e.preventDefault();
    zoomTo(cam.z * Math.exp(-e.deltaY * 0.0025), e.clientX, e.clientY);
  }, { passive: false });
  var pts = {}, drag = null, pinch = null, dragged = 0;
  canvas.addEventListener('pointerdown', function (e) {
    if (e.pointerType === 'mouse' && e.button !== 0) return;
    pts[e.pointerId] = [e.clientX, e.clientY];
    var ids = Object.keys(pts);
    if (ids.length === 2) {
      var a = pts[ids[0]], b = pts[ids[1]];
      pinch = { d: Math.hypot(a[0] - b[0], a[1] - b[1]) || 1, z: cam.z };
      drag = null;
    } else if (ids.length === 1) drag = { x: e.clientX, y: e.clientY, moved: false };
  });
  canvas.addEventListener('pointermove', function (e) {
    var prev = pts[e.pointerId];
    if (!prev) return;
    pts[e.pointerId] = [e.clientX, e.clientY];
    var ids = Object.keys(pts);
    if (pinch && ids.length === 2) {
      var a = pts[ids[0]], b = pts[ids[1]];
      zoomTo(pinch.z * Math.hypot(a[0] - b[0], a[1] - b[1]) / pinch.d, (a[0] + b[0]) / 2, (a[1] + b[1]) / 2);
      dragged = performance.now();
      return;
    }
    if (!drag || cam.z <= 1) return;
    if (!drag.moved) {
      if (Math.hypot(e.clientX - drag.x, e.clientY - drag.y) < 5) return;
      drag.moved = true;
      canvas.classList.add('rv-grabbing');
      if (canvas.setPointerCapture) canvas.setPointerCapture(e.pointerId);
    }
    dragged = performance.now();
    panBy(e.clientX - prev[0], e.clientY - prev[1]);
  });
  function lift(e) {
    delete pts[e.pointerId];
    if (Object.keys(pts).length < 2) pinch = null;
    if (!Object.keys(pts).length) { drag = null; canvas.classList.remove('rv-grabbing'); }
  }
  canvas.addEventListener('pointerup', lift);
  canvas.addEventListener('pointercancel', lift);
  if (zoomInput) {
    zoomInput.max = String(ZMAX);
    zoomInput.addEventListener('input', function () { zoomTo(+zoomInput.value); });
    root.querySelectorAll('[data-rv-zoom-wrap]').forEach(function (w) { w.hidden = false; });
  }
  root.addEventListener('click', function (e) {
    // The chips of the itinerary, and the grid panel's re-root: the controls
    // themselves, not any element that happens to carry the attribute —
    // `data-centre` on this root once swallowed every link inside it.
    var t = e.target.closest && e.target.closest('a[data-go],button[data-centre],a[data-tab]');
    if (!t || !root.contains(t)) return;
    if (t.dataset.tab) {
      // A record tab: the panel changes, the river does not.
      e.preventDefault();
      tab = t.dataset.tab;
      var c = cur.layout.meta.centre;
      history.pushState({ p: c, n: cur.layout.meta.n, tab: tab }, '', url(c, cur.layout.meta.n));
      swapPanel(c);
      return;
    }
    e.preventDefault(); travel(t.dataset.go || t.dataset.centre);
  });
  back.addEventListener('click', goBack);
  root.addEventListener('change', function (e) {
    var sel = e.target.closest && e.target.closest('select[data-range-select]');
    if (sel) travel(cur.layout.meta.centre, +sel.value);
  });
  document.addEventListener('keydown', function (e) {
    var tag = document.activeElement && document.activeElement.tagName;
    if (tag === 'INPUT' || tag === 'SELECT' || tag === 'TEXTAREA' || e.altKey || e.ctrlKey || e.metaKey) return;
    var to = e.key === 'ArrowDown' ? cur.down : e.key === 'ArrowUp' ? cur.up : null;
    if (to) { e.preventDefault(); travel(to); }
    else if (e.key === 'ArrowLeft' && trail.length > 1) { e.preventDefault(); goBack(); }
    else if (e.key === '+' || e.key === '=') { e.preventDefault(); zoomTo(cam.z * 1.25); }
    else if (e.key === '-') { e.preventDefault(); zoomTo(cam.z / 1.25); }
    else if (e.key === '0' && cam.z > 1) { e.preventDefault(); zoomTo(1); }
  });
  window.addEventListener('popstate', function (e) {
    if (!e.state || !e.state.p) return;
    var tabChanged = (e.state.tab || 'record') !== tab;
    tab = e.state.tab || 'record';
    if (e.state.p !== cur.layout.meta.centre || e.state.n !== cur.layout.meta.n) {
      travel(e.state.p, e.state.n, true);
    } else if (tabChanged) {
      swapPanel(e.state.p);
    }
  });
  // The whole family at once (v0.5.0, docs/adr/0007): the widest range and
  // the camera back to the whole window. Offered beside a view; under the
  // river, the range select says the same thing.
  var whole = root.querySelector('[data-rv-whole]'), before = null;
  if (whole && view) {
    whole.hidden = false;
    whole.addEventListener('click', function () {
      var n = cur.layout.meta.n, on = n !== 5;
      if (on) before = n;
      zoomTo(1);
      travel(cur.layout.meta.centre, on ? 5 : (before || 3));
    });
  }
  // Near names or all of them: the reader's choice, under every view.
  var namesBtn = root.querySelector('[data-rv-names]');
  if (namesBtn) {
    namesBtn.hidden = false;
    namesBtn.setAttribute('aria-pressed', namesAll ? 'true' : 'false');
    namesBtn.addEventListener('click', function () {
      namesAll = !namesAll;
      namesBtn.setAttribute('aria-pressed', namesAll ? 'true' : 'false');
      try { localStorage.setItem(NAMES, namesAll ? 'all' : 'near'); } catch (e) { /* private window */ }
      placeNames();
    });
  }
  // What a view may use of this file: the river's own helpers, so that a
  // view's words, curves and counts are measured and placed as the river's.
  var api = { f: f, vc: vc, esc: esc, cls: cls, widths: WIDTHS, mono: MONO, byKey: byKey, edges: edges,
    tiers: tiers, width: width, clip: clip, mark: mark, privateWord: privateWord, base: base,
    words: function () { return cur.words; } };
  // The masthead search (ui.js) travels here rather than reloading the page.
  window.axgfRiver = { travel: function (id, n) { travel(id, n); }, layout: sceneLayout };
  here = shaped(cur);
  // Under a view, its own drawing replaces the server's river at once; the
  // server's stays the page without scripts (app.css hides it meanwhile).
  if (view) setSvg(view.paint(here, api, true));
  history.replaceState({ p: cur.layout.meta.centre, n: cur.layout.meta.n, tab: tab }, '');
  controls(cur.layout.meta.n);
  decorate();
  // The river's entrance is CSS from the first paint (`rv-intro`, written by
  // the server): it plays once per page, so the class goes once it has, and
  // the stills a travel lays down later arrive without it.
  setTimeout(function () { root.classList.remove('rv-intro'); }, 2400);
})();
