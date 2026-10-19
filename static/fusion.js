/* Fusion's living tree (v0.5.0, docs/adr/0007).
 *
 * The river's own people — from the same payload, through the same reader's
 * lens, in the same order from left to right — grown as a tree. The person at
 * the fixed point stands where the trunk leaves the soil. The trunk rises to
 * the fork where their own family begins, and their descendants are its
 * crown: a dome of boughs, one generation further out at every fork, with the
 * living at the tips as green leaves and those gone as embers. Their
 * ancestors are its roots, spreading wide and shallow under the soil line, a
 * generation further down at every fork. A bough thins with every generation
 * it is from the trunk, and carries what its parentage is worth: an attested
 * link a vein of light, an inferred one a broken vein, a speculative one a
 * trace on faded bark.
 *
 * Two things the page knows by name:
 * - `window.axgfView` (river.js): where everyone stands in the tree (`shape`),
 *   how a bough bends (`edge`, which the lit route follows too), how a frame
 *   is painted at rest and in travel, which people the drawing named, and the
 *   dashboard beside it at rest. Travel, the trail, the panel, the keyboard,
 *   the camera and every touch rule stay river.js's.
 * - `window.axgfRiverScene`: the life over the drawing — sap drawn up from the
 *   root tips through the trunk to the leaves on every attested bough, leaves
 *   breathing, spores rising. It holds still as the river's ground does:
 *   reduced motion, "Cut", the pause, a hidden tab, a tree off screen, a
 *   machine too slow for it.
 *
 * No colour is named here: every one is a token of the theme (`--tv-*`,
 * tokens.css), read from the page or set by class in app.css.
 */
(function () {
  'use strict';
  var root = document.querySelector('.river');
  if (!root || root.dataset.view !== 'tree') return;
  var stage = document.getElementById('rv-canvas');

  /* ---- where everyone stands ---- */
  // The river places a union 38% of the way from the parents' row to the
  // children's: the centre's own family forks from the top of the trunk, and
  // their parents' from the foot of the taproot.
  var UP = 0.38, DOWN = 0.62;
  // The crown's dome runs a little past level on either side, as boughs
  // droop; the roots spread almost flat.
  var DROOP = 0.07 * Math.PI, FLAT = 0.04 * Math.PI;
  // The geometry of the frame painted last: what `edge` bends boughs by.
  var GEO = null;
  function shape(fr) {
    var m = fr.meta, hl = m.hview[0], hw = m.hview[1], vt = m.view[0], vh = m.view[1];
    var C = fr.persons.filter(function (p) { return p.role === 'centre'; })[0];
    var cx = C ? C.x : hl + hw / 2, cy = C ? C.y : m.cy, step = m.step || 86;
    // How far the frame reaches each way, and how wide each side is drawn in
    // the river: the crown takes the descendants' order from left to right
    // over its own breadth, the roots the ancestors' over theirs.
    var nd = 0, na = 0, lo = [Infinity, Infinity], hi = [-Infinity, -Infinity];
    fr.persons.forEach(function (p) {
      var g = (p.y - cy) / step, h;
      if (g < -0.25) { nd = Math.max(nd, Math.round(-g)); h = 0; } else if (g > 0.25) { na = Math.max(na, Math.round(g)); h = 1; } else return;
      lo[h] = Math.min(lo[h], p.x); hi[h] = Math.max(hi[h], p.x);
    });
    // The soil line: the crown and its trunk above, the roots below, each
    // given room for the generations it has.
    var share = Math.min(0.8, Math.max(0.4, (nd + 0.9) / (nd + 0.9 + na * 0.8 + 0.35)));
    var xc = hl + hw / 2, ys = vt + vh * share;
    var H = nd ? Math.min(0.17 * vh, (ys - vt) * 0.28) : 0, fy = ys - H, h0 = na ? 22 : 0, fry = ys + h0;
    var Rx = hw / 2 - 92, Ry = Math.max(40, fy - vt - 34), Rxr = Rx * 0.86, Ryr = Math.max(30, vt + vh - 34 - fry);
    var Pw = Math.min(96, hw * 0.12);
    // A generation's ring, as a share of the crown's or the roots' reach:
    // the first at 2/(n+1), the last at the rim.
    function ring(k, n, first) { return k < 1 ? Math.max(0, (k - first) / (1 - first)) * 2 / (n + 1) : (k + 1) / (n + 1); }
    function along(h, x) { var span = hi[h] - lo[h]; return span > 1 ? Math.min(1, Math.max(0, (x - lo[h]) / span)) : 0.5; }
    function place(x, y) {
      var g = (y - cy) / step, dx = x - cx;
      if (g < -(UP + 0.02) && nd) {
        var rc = ring(-g, nd, UP), th = Math.PI - DROOP + along(0, x) * (Math.PI + 2 * DROOP);
        return [xc + Rx * rc * Math.cos(th), fy + Ry * rc * Math.sin(th)];
      }
      if (g > DOWN + 0.02 && na) {
        var rr = ring(g, na, DOWN), tr = Math.PI - FLAT - along(1, x) * (Math.PI - 2 * FLAT);
        return [xc + Rxr * rr * Math.cos(tr), fry + Ryr * rr * Math.sin(tr)];
      }
      // On the trunk or the taproot: a partner stands on the soil line beside
      // the trunk, and every fork is on its axis.
      var side = (dx < 0 ? -1 : 1) * Pw * Math.min(1, Math.abs(dx) / 300);
      if (Math.abs(dx) < 1) side = 0;
      if (g <= 0) { var t = Math.min(1, -g / UP); return [xc + side * (1 - t), ys - t * H]; }
      var t2 = Math.min(1, g / DOWN); return [xc + side * (1 - t2), ys + t2 * h0];
    }
    var geo = { xc: xc, ys: ys, fy: fy, fry: fry, h: H, h0: h0, col: 8, rx: Rx, ry: Ry, rxr: Rxr, ryr: Ryr, nd: nd, na: na, n: m.n,
      rings: { up: [], down: [] } };
    for (var k = 1; k <= nd; k++) geo.rings.up.push(ring(k, nd, UP));
    for (k = 1; k <= na; k++) geo.rings.down.push(ring(k, na, DOWN));
    GEO = geo;
    var persons = fr.persons.map(function (p) { var q = place(p.x, p.y); return Object.assign({}, p, { x: q[0], y: q[1] }); });
    var PK = {};
    persons.forEach(function (p) { PK[p.key] = p; });
    var couples = fr.couples.map(function (c) {
      var q = place(c.x, c.y), o = Object.assign({}, c, { x: q[0], y: q[1] });
      if (c.stub) {
        // Brothers and sisters not drawn: a bud beside the bough that leads
        // to the one who is, leaning away from the trunk.
        var kid = c.kids.map(function (key) { return PK[key]; }).filter(Boolean)[0];
        var dx = kid ? kid.x - q[0] : 0, dy = kid ? kid.y - q[1] : -1, l = Math.hypot(dx, dy) || 1, s = q[0] < xc ? -1 : 1;
        var a = Math.atan2(dy / l, dx / l) + s * 0.7, ux = Math.cos(a), uy = Math.sin(a);
        o.stub_dx = ux * 20; o.stub_dy = uy * 20;
        o.stub_at = markAt(q[0] + ux * 20, q[1] + uy * 20, ux, uy);
      }
      return o;
    });
    // A line the record continues off the frame, or one it ends: a twig that
    // grows on the way its bough was growing.
    var tails = fr.tails.map(function (t) {
      var q = place(t.x1, t.y1), u = grow(q[0], q[1]) || [0, t.dir < 0 ? -1 : 1], len = t.kind === 'cont' ? 30 : 22;
      var x2 = q[0] + u[0] * len, y2 = q[1] + u[1] * len;
      return Object.assign({}, t, { x1: q[0], y1: q[1], x2: x2, y2: y2, count_at: t.kind === 'cont' ? markAt(x2, y2, u[0], u[1]) : null });
    });
    return { persons: persons, couples: couples, tails: tails, meta: Object.assign({}, m, { tree: geo }) };
  }
  // Where a count stands at the end of a twig, on the side it points to.
  function markAt(x, y, ux, uy) {
    var anchor = ux > 0.35 ? 1 : ux < -0.35 ? -1 : 0;
    return { x: x + ux * 5, y: y + (anchor ? uy * 5 + 3.5 : uy > 0 ? 12 : -5), anchor: anchor, halo: true };
  }
  // Which way the tree grows at a point: up the trunk, out from the fork
  // through the crown, down the taproot, out from its foot through the roots.
  // On the soil line itself it grows both ways, and the caller decides.
  function grow(x, y) {
    var G = GEO, dx, dy, l;
    if (!G) return null;
    if (y < G.ys - 0.5) {
      if (y >= G.fy - 0.5 && Math.abs(x - G.xc) <= G.col) return [0, -1];
      dx = x - G.xc; dy = y - G.fy; l = Math.hypot(dx, dy);
      return l > 0.5 ? [dx / l, dy / l] : [0, -1];
    }
    if (y > G.ys + 0.5) {
      if (y <= G.fry + 0.5 && Math.abs(x - G.xc) <= G.col) return [0, 1];
      dx = x - G.xc; dy = y - G.fry; l = Math.hypot(dx, dy);
      return l > 0.5 ? [dx / l, dy / l] : [0, 1];
    }
    return null;
  }
  // How far a point is from the foot of the trunk, along the tree.
  function depth(x, y) {
    var G = GEO;
    if (!G || Math.abs(y - G.ys) <= 0.5) return 0;
    if (y < G.ys) return y >= G.fy - 0.5 && Math.abs(x - G.xc) <= G.col ? G.ys - y : G.h + Math.hypot(x - G.xc, y - G.fy);
    return y <= G.fry + 0.5 && Math.abs(x - G.xc) <= G.col ? y - G.ys : G.h0 + Math.hypot(x - G.xc, y - G.fry);
  }
  // A bough leaves its inner end the way the tree grows there and reaches
  // its outer end the way the tree grows there: the crown's boughs rise from
  // the fork and droop at the rim, the roots dive and spread.
  function ctrl(x1, y1, x2, y2) {
    var k = 0.42 * Math.hypot(x2 - x1, y2 - y1);
    var a = grow(x1, y1) || [0, y2 > y1 ? 1 : -1], b = grow(x2, y2) || [0, y1 > y2 ? 1 : -1];
    if (depth(x1, y1) <= depth(x2, y2)) return [x1, y1, x1 + a[0] * k, y1 + a[1] * k, x2 - b[0] * k, y2 - b[1] * k, x2, y2];
    return [x1, y1, x1 - a[0] * k, y1 - a[1] * k, x2 + b[0] * k, y2 + b[1] * k, x2, y2];
  }
  function bez(c, t) {
    var u = 1 - t;
    return [u * u * u * c[0] + 3 * u * u * t * c[2] + 3 * u * t * t * c[4] + t * t * t * c[6],
            u * u * u * c[1] + 3 * u * u * t * c[3] + 3 * u * t * t * c[5] + t * t * t * c[7]];
  }
  function bezd(c, t) {
    var u = 1 - t;
    return [3 * u * u * (c[2] - c[0]) + 6 * u * t * (c[4] - c[2]) + 3 * t * t * (c[6] - c[4]),
            3 * u * u * (c[3] - c[1]) + 6 * u * t * (c[5] - c[3]) + 3 * t * t * (c[7] - c[5])];
  }
  function edge(x1, y1, x2, y2) {
    var c = ctrl(x1, y1, x2, y2), r = function (v) { return Math.round(v * 10) / 10; };
    return 'M' + r(c[0]) + ' ' + r(c[1]) + 'C' + r(c[2]) + ' ' + r(c[3]) + ' ' + r(c[4]) + ' ' + r(c[5]) + ' ' + r(c[6]) + ' ' + r(c[7]);
  }

  /* ---- the drawing ---- */
  // A bough's width where it leaves a person: the trunk's at the fixed
  // point, under two thirds of it a generation further, never under a
  // hairline; roots finer than boughs, a partner's graft finer still.
  var T0 = 22, TAPER = 0.64, MIN_W = 1.3;
  function thick(gen) { return Math.max(MIN_W, T0 * Math.pow(TAPER, Math.abs(gen))); }
  // Bark has a thickness, and it is not the same at both ends.
  function ribbon(f, c, w1, w2) {
    var N = 14, L = [], R = [];
    for (var i = 0; i <= N; i++) {
      var t = i / N, p = bez(c, t), d = bezd(c, t), l = Math.hypot(d[0], d[1]) || 1, nx = -d[1] / l, ny = d[0] / l, w = (w1 + (w2 - w1) * t) / 2;
      L.push(f(p[0] + nx * w) + ' ' + f(p[1] + ny * w));
      R.push(f(p[0] - nx * w) + ' ' + f(p[1] - ny * w));
    }
    return 'M' + L.join('L') + 'L' + R.reverse().join('L') + 'Z';
  }
  function generationOf(c, PM) {
    var g = [];
    c.parents.forEach(function (k) { if (PM[k]) g.push(PM[k].gen + 0.5); });
    c.kids.forEach(function (k) { if (PM[k]) g.push(PM[k].gen - 0.5); });
    return g.length ? g.reduce(function (a, b) { return a + b; }) / g.length : 0;
  }
  // A crown has leaves around every tip and along its outer boughs, and a
  // root has hairs: small lights, the same ones in every frame (seeded by who
  // they are), coloured by what the river already says of them.
  function seed(s) { var h = 2166136261; for (var i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 16777619); } return h >>> 0; }
  function next(h) { return Math.imul(h ^ (h >>> 15), 2246822507) >>> 0; }
  function foliage(f, p, kind) {
    var h = seed(p.key), out = '', n = kind === 'root' ? 5 : 16, flat = kind === 'root' ? 0.6 : 1;
    for (var i = 0; i < n; i++) {
      h = next(h);
      var a = (h % 6283) / 1000, d = 7 + (h >>> 9) % (kind === 'root' ? 12 : 19), r = 0.7 + ((h >>> 5) % 9) / 7;
      out += '<circle cx="' + f(p.x + Math.cos(a) * d) + '" cy="' + f(p.y + Math.sin(a) * d * flat) + '" r="' + f(r) + '" class="tv-dot tv-' + kind + '"/>';
    }
    return out;
  }
  function canopy(f, c, key, kind) {
    var h = seed(key), out = '';
    for (var i = 0; i < 8; i++) {
      h = next(h);
      var t = 0.35 + ((h >>> 3) % 1000) / 1600, p = bez(c, t), d = bezd(c, t), l = Math.hypot(d[0], d[1]) || 1;
      var off = (((h >>> 11) % 1000) / 500 - 1) * (5 + (h >>> 21) % 13), r = 0.8 + ((h >>> 7) % 9) / 7;
      out += '<circle cx="' + f(p[0] - d[1] / l * off) + '" cy="' + f(p[1] + d[0] / l * off) + '" r="' + f(r) + '" class="tv-dot tv-' + kind + '"/>';
    }
    return out;
  }
  function centreOf(fr) { return fr.persons.filter(function (p) { return p.role === 'centre'; })[0]; }
  function centreName(fr) { var c = centreOf(fr), s = c && c.s; return s && !s.redacted ? s.names[2] : ''; }
  var named = {};

  function paint(fr, api, rest) {
    var f = api.f, esc = api.esc, PM = api.byKey(fr.persons), m = fr.meta, out = '', bark = '', veins = '', nodes = '', tw = '', leaves = '';
    var vt = m.view[0], vh = m.view[1], hl = m.hview[0], hw = m.hview[1];
    var G = m.tree || GEO || { xc: hl + hw / 2, ys: vt + vh / 2, fy: vt + vh / 2, fry: vt + vh / 2, h: 0, h0: 0, col: 8, rx: hw / 3, ry: vh / 3, rxr: hw / 3, ryr: vh / 4, rings: { up: [], down: [] } };
    GEO = G;
    // The ground: the dashboard's dark tile, the crown's glow, and the soil
    // the trunk stands in, as wide as the window (the camera stretches all
    // three with it).
    out += '<rect class="tv-bg" x="' + f(hl) + '" y="' + f(vt) + '" width="' + f(hw) + '" height="' + f(vh) + '"/>';
    if (G.rings.up.length) {
      out += '<ellipse class="tv-crown" cx="' + f(G.xc) + '" cy="' + f(G.fy - G.ry * 0.38) + '" rx="' + f(G.rx * 1.12) + '" ry="' + f(G.ry * 0.86) + '" fill="url(#tv-crown)"/>';
    }
    out += '<rect class="tv-soil-glow" x="' + f(hl) + '" y="' + f(G.ys) + '" width="' + f(hw) + '" height="' + f(Math.max(0, vt + vh - G.ys)) + '" fill="url(#tv-soil)"/>';
    // Growth rings: where each generation stands, faint enough to read
    // through — the crown's over its dome, the roots' under their spread.
    G.rings.up.forEach(function (rc) {
      var rx = G.rx * rc, ry = G.ry * rc, c = Math.cos(DROOP), sn = Math.sin(DROOP);
      out += '<path d="M' + f(G.xc - rx * c) + ' ' + f(G.fy + ry * sn) + 'A' + f(rx) + ' ' + f(ry) + ' 0 1 1 ' + f(G.xc + rx * c) + ' ' + f(G.fy + ry * sn) + '" fill="none" class="tv-ring-arc"/>';
    });
    G.rings.down.forEach(function (rr) {
      var rx = G.rxr * rr, ry = G.ryr * rr, c = Math.cos(FLAT), sn = Math.sin(FLAT);
      out += '<path d="M' + f(G.xc + rx * c) + ' ' + f(G.fry + ry * sn) + 'A' + f(rx) + ' ' + f(ry) + ' 0 0 1 ' + f(G.xc - rx * c) + ' ' + f(G.fry + ry * sn) + '" fill="none" class="tv-ring-arc"/>';
    });
    out += '<line class="k-band-rule tv-soil" x1="' + f(hl) + '" y1="' + f(G.ys) + '" x2="' + f(hl + hw) + '" y2="' + f(G.ys) + '"/>';
    // Twigs: lines the record continues off the frame — with an arrowhead
    // and the count beyond it — and lines it ends, with the ring the legend
    // names. Their counts are written once the names are placed.
    var counts = [];
    fr.tails.forEach(function (t) {
      var w = thick((m.n || 3) + 1) * 0.8, dx = t.x2 - t.x1, dy = t.y2 - t.y1, l = Math.hypot(dx, dy) || 1, ux = dx / l, uy = dy / l;
      tw += '<g opacity="' + f(t.a) + '"><path d="M' + f(t.x1) + ' ' + f(t.y1) + 'L' + f(t.x2) + ' ' + f(t.y2) + '" fill="none" class="tv-twig" stroke-width="' + f(w) + '"/>';
      if (t.kind === 'cont') {
        tw += '<polygon class="tv-twig-tip" points="' + f(t.x2 - uy * 4) + ',' + f(t.y2 + ux * 4) + ' ' + f(t.x2 + uy * 4) + ',' + f(t.y2 - ux * 4) + ' ' + f(t.x2 + ux * 6) + ',' + f(t.y2 + uy * 6) + '"/>';
        if (t.count_at) counts.push({ mk: t.count_at, text: '+' + t.count, size: 9.5, dx: t.cref ? t.x2 - t.cref[0] : 0, dy: t.cref ? t.y2 - t.cref[1] : 0, a: t.a });
      } else {
        tw += '<circle cx="' + f(t.x2) + '" cy="' + f(t.y2) + '" r="3.4" class="tv-end"/>';
      }
      tw += '</g>';
    });
    fr.couples.forEach(function (c) {
      if (!c.stub) return;
      var x2 = c.x + c.stub_dx, y2 = c.y + c.stub_dy;
      tw += '<g opacity="' + f(c.a) + '"><path d="M' + f(c.x) + ' ' + f(c.y) + 'L' + f(x2) + ' ' + f(y2) + '" fill="none" class="tv-twig tv-bud" stroke-width="1.4"/></g>';
      if (c.stub_at) counts.push({ mk: c.stub_at, text: '+' + c.stub, size: 10, dx: c.sref ? c.x - c.sref[0] : 0, dy: c.sref ? c.y - c.sref[1] : 0, a: c.a });
    });
    // Boughs and roots, thickest first so the thin ones lie on top.
    function onAxis(x, y) { return Math.abs(x - G.xc) <= G.col; }
    var B = api.edges(fr, PM).map(function (e) {
      var c = e.cp, gc = generationOf(c, PM), a = e.from ? PM[e.from] : null, k = e.to ? PM[e.to] : null;
      var w1 = a ? thick(a.gen) : thick(gc), w2 = k ? thick(k.gen) : thick(gc), s = 0.85 + 0.15 * api.cls(e.d) / 5;
      if (gc < 0) s *= 0.72;
      if (a && (a.role === 'spouse' || a.role === 'dspouse')) s *= 0.5;
      if (onAxis(e.x1, e.y1) && onAxis(e.x2, e.y2)) s *= 1.3;
      var curve = ctrl(e.x1, e.y1, e.x2, e.y2), outer = depth(e.x1, e.y1) > depth(e.x2, e.y2) ? (a || k) : (k || a);
      return { e: e, c: curve, w1: w1 * s, w2: w2 * s, outer: outer, crown: gc > 0 };
    }).sort(function (a, b) { return Math.max(b.w1, b.w2) - Math.max(a.w1, a.w2); });
    B.forEach(function (b) {
      var e = b.e, o = e.a < 1 ? ' opacity="' + f(e.a) + '"' : '', faint = e.conf === 's' ? ' tv-faint' : '', d = edge(e.x1, e.y1, e.x2, e.y2);
      if (Math.max(b.w1, b.w2) < 2.2) {
        bark += '<path d="' + d + '" fill="none" class="tv-bark-line' + faint + '" stroke-width="' + f(Math.max(b.w1, b.w2)) + '" stroke-linecap="round"' + o + '/>';
      } else {
        bark += '<path d="' + ribbon(f, b.c, b.w1, b.w2) + '" class="tv-bark' + faint + '"' + o + '/>'
          + '<path d="' + d + '" fill="none" class="tv-ridge" stroke-width="' + f(Math.min(b.w1, b.w2) * 0.32) + '"' + o + '/>';
      }
      if (e.conf === 'a') veins += '<path d="' + d + '" fill="none" class="tv-vein" stroke-width="1.1" stroke-linecap="round"' + o + '/>';
      else if (e.conf === 'd') veins += '<path d="' + d + '" fill="none" class="tv-vein tv-vein-d" stroke-width="1.1" stroke-dasharray="4.5 3"' + o + '/>';
      else veins += '<path d="' + d + '" fill="none" class="tv-vein tv-vein-s" stroke-width="2" stroke-linecap="round" stroke-dasharray="0.1 5"' + o + '/>';
      // Leaves along the crown's outer boughs, the colour of the life at
      // their tip.
      var s = b.outer && b.outer.s || {};
      if (b.crown && b.outer) leaves += '<g opacity="' + f(e.a) + '">' + canopy(f, b.c, (e.from || '') + '>' + (e.to || '') + (e.cp.key || ''), s.redacted || s.sparse ? 'hidden' : s.living ? 'living' : 'gone') + '</g>';
    });
    fr.persons.forEach(function (p) {
      var s = p.s || {};
      if (p.role === 'centre') return;
      var kind = p.gen < 0 ? 'root' : s.redacted || s.sparse ? 'hidden' : s.living ? 'living' : 'gone';
      leaves += '<g opacity="' + f(p.a) + '">' + foliage(f, p, kind) + '</g>';
    });

    // Names stand outward, the way the tree grows where they are: the centre
    // beside the foot of the trunk, the nearest generations next, the next
    // ones while there is room. A name that would cover one already placed
    // is left out, as the river leaves out a word it has no room for; the
    // pointer brings it back (river.js `light`, through `named`).
    var keep = {}, boxes = [];
    function hit(box) {
      for (var i = 0; i < boxes.length; i++) {
        var q = boxes[i];
        if (box[0] < q[2] && box[2] > q[0] && box[1] < q[3] && box[3] > q[1]) return true;
      }
      return false;
    }
    var C = centreOf(fr), cs = C && C.s || {}, cleft = true;
    if (C) {
      // The centre's name goes to the side its partners leave free.
      var right = fr.persons.filter(function (p) { return p.role === 'spouse' && p.x > C.x + 1; }).length;
      var left = fr.persons.filter(function (p) { return p.role === 'spouse' && p.x < C.x - 1; }).length;
      cleft = !(left && !right);
      if (!cs.redacted) {
        var cw = Math.max(api.width(api.clip(cs.names[2], 230), 16, true), api.width(cs.years, 11, true)), cx0 = cleft ? C.x - 24 - cw : C.x + 24;
        boxes.push([cx0 - 2, C.y - 19, cx0 + cw + 2, C.y + 14]);
      }
      boxes.push([C.x - 16, C.y - 16, C.x + 16, C.y + 16]);
    }
    fr.persons.slice().sort(function (a, b) { return Math.abs(a.gen) - Math.abs(b.gen) || a.x - b.x; }).forEach(function (p) {
      var s = p.s || {}, gd = Math.abs(p.gen);
      if (p.role === 'centre' || s.redacted) return;
      var u = grow(p.x, p.y), anchor, lx, ly;
      if (!u) { anchor = p.x < G.xc ? 'end' : 'start'; lx = p.x + (p.x < G.xc ? -11 : 11); ly = p.y - 3; }
      else {
        anchor = u[0] < -0.3 ? 'end' : u[0] > 0.3 ? 'start' : 'middle';
        lx = p.x + u[0] * 11; ly = p.y + u[1] * 11 + (u[1] > 0.3 ? 9 : u[1] < -0.3 ? -2 : 4);
      }
      var word = gd <= 1 ? s.names[1] : gd === 2 ? s.names[1] : s.names[0], both = gd <= 1;
      var w = Math.max(api.width(word, 12, false), both ? api.width(s.years, 9.5, true) : 0);
      var x0 = anchor === 'end' ? lx - w : anchor === 'middle' ? lx - w / 2 : lx, box = [x0 - 2, ly - 11, x0 + w + 2, ly + (both ? 14 : 3)];
      if (hit(box)) {
        if (gd !== 2) return;
        // A grandchild's or grandparent's short name may give way to the
        // given name alone before it gives way altogether.
        word = s.names[0]; w = api.width(word, 12, false);
        x0 = anchor === 'end' ? lx - w : anchor === 'middle' ? lx - w / 2 : lx; box = [x0 - 2, ly - 11, x0 + w + 2, ly + 3];
        if (hit(box)) return;
      }
      boxes.push(box);
      keep[p.key] = { x: lx, y: ly, anchor: anchor, word: word, both: both };
    });
    // The counts: never left out, since a count is the only sign of the
    // people beyond, but stepped aside from a name rather than written over
    // it — a line lower, higher, or to one side.
    counts.forEach(function (c) {
      var mk = c.mk, w = api.width(c.text, c.size, true), at = [0, 0], spot = null;
      [[0, 0], [0, 11], [0, -11], [14, 0], [-14, 0], [0, 22]].some(function (o) {
        var x = mk.x + c.dx + o[0], y = mk.y + c.dy + o[1];
        var x0 = mk.anchor < 0 ? x - w : mk.anchor === 0 ? x - w / 2 : x, box = [x0 - 1, y - c.size, x0 + w + 1, y + 2];
        if (!spot) spot = box; // where it stays if no place is free
        if (hit(box)) return false;
        at = o; spot = box;
        return true;
      });
      boxes.push(spot);
      tw += '<g opacity="' + f(c.a) + '">' + api.mark(Object.assign({}, mk, { x: mk.x + at[0], y: mk.y + at[1] }), c.text, c.size, c.dx, c.dy) + '</g>';
    });
    // Generations, counted as the river's rail counts them: each of the
    // crown's rings at the end of its dome, each of the roots' at the end of
    // its spread, on the right where there is room, else on the left, else
    // not at all; and the soil line, the fixed point's, at the window's edge.
    function tick(word, x, y, side) {
      var w = api.width(word, 10, true), x0 = side > 0 ? x : x - w, box = [x0 - 1, y - 9, x0 + w + 1, y + 2];
      if (hit(box)) return false;
      boxes.push(box);
      out += '<text x="' + f(x) + '" y="' + f(y) + '"' + (side < 0 ? ' text-anchor="end"' : '') + ' font-family="' + api.mono + '" font-size="10" class="k-band-text tv-gen">' + word + '</text>';
      return true;
    }
    G.rings.up.forEach(function (rc, i) {
      var a = DROOP + 0.045 * Math.PI, y = G.fy + G.ry * rc * Math.sin(a) + 3.5, dx = G.rx * rc * Math.cos(a) + 6;
      if (!tick('+' + (i + 1), G.xc + dx, y, 1)) tick('+' + (i + 1), G.xc - dx, y, -1);
    });
    G.rings.down.forEach(function (rr, i) {
      var dx = G.rxr * rr + 8, y = G.fry + 3.5;
      if (!tick('−' + (i + 1), G.xc + dx, y, 1)) tick('−' + (i + 1), G.xc - dx, y, -1);
    });
    out += '<text x="' + f(hl + hw - 10) + '" y="' + f(G.ys - 4) + '" text-anchor="end" font-family="' + api.mono + '" font-size="10" class="k-accent">0 ◂</text>';

    // People: the heart at the foot of the trunk, leaves alive and leaves
    // gone, and the person this reader may not see as an outline only.
    fr.persons.forEach(function (p) {
      var s = p.s || {}, isC = p.role === 'centre', r = isC ? 7.5 : 4.8, x = f(p.x), y = f(p.y);
      var g = '<g opacity="' + f(p.a) + '">';
      if (rest) g += '<a class="rv-p" data-id="' + esc(p.id) + '" data-key="' + esc(p.key) + '" href="' + api.base + '?p=' + encodeURIComponent(p.id) + '&amp;n=' + m.n + '"><circle cx="' + x + '" cy="' + y + '" r="20" fill="transparent"/>';
      if (p.repeat > 1) g += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 2.6) + '" fill="none" class="k-data-line" stroke-width="1"/>';
      if (p.again) g += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 2.6 + (p.repeat > 1 ? 2.2 : 0)) + '" fill="none" class="k-data-line" stroke-width="1" stroke-dasharray="2.4 1.8"/>';
      if (isC) {
        g += '<circle cx="' + x + '" cy="' + y + '" r="19" class="tv-halo"/><circle cx="' + x + '" cy="' + y + '" r="14" fill="none" class="tv-ring" stroke-width="1.4"/>'
          + '<circle cx="' + x + '" cy="' + y + '" r="' + r + '" class="tv-heart"/>';
      } else if (s.redacted || s.sparse) {
        g += '<circle cx="' + x + '" cy="' + y + '" r="' + r + '" class="tv-leaf tv-hidden" stroke-width="1.5" stroke-dasharray="1.6 1.6"/>';
      } else if (s.living) {
        g += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 4.5) + '" class="tv-leaf-glow tv-living"/><circle cx="' + x + '" cy="' + y + '" r="' + r + '" class="tv-leaf tv-living"/>';
      } else {
        g += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 3) + '" class="tv-leaf-glow tv-gone"/><circle cx="' + x + '" cy="' + y + '" r="' + f(r - 0.4) + '" class="tv-leaf tv-gone"/>';
      }
      var halo = ' paint-order="stroke" stroke-width="4" stroke-linejoin="round"';
      if (isC && !s.redacted) {
        var an = cleft ? ' text-anchor="end"' : '', lx = f(p.x + (cleft ? -24 : 24));
        g += '<text x="' + lx + '" y="' + f(p.y - 4) + '"' + an + ' font-size="16" font-weight="600" class="tv-name tv-centre k-halo"' + halo + '>' + esc(api.clip(s.names[2], 230)) + '</text>'
          + '<text x="' + lx + '" y="' + f(p.y + 11) + '"' + an + ' font-family="' + api.mono + '" font-size="11" class="tv-data k-halo"' + halo + '>' + esc(s.years) + '</text>';
      } else if (keep[p.key]) {
        var L = keep[p.key], at = L.anchor !== 'start' ? ' text-anchor="' + L.anchor + '"' : '';
        g += '<text x="' + f(L.x) + '" y="' + f(L.y) + '"' + at + ' font-size="12" class="tv-name k-halo"' + halo + '>' + esc(L.word) + '</text>';
        if (L.both) g += '<text x="' + f(L.x) + '" y="' + f(L.y + 11.5) + '"' + at + ' font-family="' + api.mono + '" font-size="9.5" class="tv-data k-halo"' + halo + '>' + esc(s.years) + '</text>';
      }
      if (rest) g += '</a>';
      nodes += g + '</g>';
    });
    if (rest) named = keep;
    var defs = '<linearGradient id="tv-soil" x1="0" y1="0" x2="0" y2="1"><stop offset="0" class="tv-soil-top"/><stop offset="1" class="tv-soil-end"/></linearGradient>'
      + '<radialGradient id="tv-crown"><stop offset="0" class="tv-crown-in"/><stop offset="1" class="tv-crown-out"/></radialGradient>';
    var head = '<svg class="rv-svg tv-svg" data-view="tree" viewBox="' + f(hl) + ' ' + f(vt) + ' ' + f(hw) + ' ' + f(vh) + '" width="100%" '
      + (rest ? 'role="img" aria-label="' + esc(label() + (centreName(fr) ? ' — ' + centreName(fr) : '')) + '"' : 'aria-hidden="true"')
      + ' xmlns="http://www.w3.org/2000/svg"><defs>' + defs + '</defs>';
    // The lit route lies over the bark and under the people (river.js).
    return head + out + bark + tw + veins + leaves + '<g class="rv-lit"></g>' + nodes + '</svg>';
  }
  function label() {
    var s = root.querySelector('.rv-stage');
    return (s && s.getAttribute('aria-label')) || '';
  }

  /* ---- the dashboard: figures read from the tree on screen ---- */
  function rest(svg, fr, api) {
    var hud = root.querySelector('[data-rv-hud]');
    if (!hud) return;
    var lang = document.documentElement.lang || undefined, seen = {}, anc = 0, desc = 0, living = 0, lo = 0, hi = 0, per = {};
    fr.persons.forEach(function (p) {
      if (seen[p.id]) return;
      seen[p.id] = 1;
      if (p.role === 'anc') anc++;
      else if (p.role === 'desc') desc++;
      if (p.s && p.s.living) living++;
      lo = Math.min(lo, p.gen); hi = Math.max(hi, p.gen);
      per[p.gen] = (per[p.gen] || 0) + 1;
    });
    var E = api.edges(fr, api.byKey(fr.persons)), att = E.filter(function (e) { return e.conf === 'a'; }).length;
    var n = function (v) { try { return v.toLocaleString(lang); } catch (e) { return String(v); } };
    var set = function (k, v) { var dd = hud.querySelector('[data-hud="' + k + '"]'); if (dd) dd.textContent = v; };
    set('anc', n(anc)); set('desc', n(desc)); set('gens', n(hi - lo + 1)); set('living', n(living));
    var share = E.length ? att / E.length : 0, gauge = hud.querySelector('[data-hud="attested"]');
    if (gauge) {
      var pct;
      try { pct = new Intl.NumberFormat(lang, { style: 'percent' }).format(share); } catch (e) { pct = Math.round(share * 100) + ' %'; }
      gauge.innerHTML = '<svg class="rv-hud-ring" viewBox="0 0 36 36" aria-hidden="true"><circle cx="18" cy="18" r="15.9" class="rv-hud-track"/>'
        + '<circle cx="18" cy="18" r="15.9" class="rv-hud-arc" stroke-dasharray="' + (share * 100).toFixed(1) + ' 100" transform="rotate(-90 18 18)"/></svg>';
      gauge.appendChild(document.createTextNode(E.length ? pct : '—'));
    }
    // People per generation, oldest at the left: the roots' bars and the
    // crown's, as the reference's histogram under its tree.
    var bars = hud.querySelector('.rv-hud-bars') || hud.appendChild(document.createElement('div'));
    bars.className = 'rv-hud-bars';
    bars.setAttribute('aria-hidden', 'true');
    var most = 1, html = '';
    for (var g = lo; g <= hi; g++) most = Math.max(most, per[g] || 0);
    for (g = lo; g <= hi; g++) {
      html += '<i class="' + (g < 0 ? 'rv-bar-root' : g > 0 ? 'rv-bar-crown' : 'rv-bar-trunk') + '" style="block-size:' + Math.max(8, Math.round(100 * (per[g] || 0) / most)) + '%"></i>';
    }
    bars.innerHTML = html;
    hud.hidden = false;
  }

  window.axgfView = { name: 'tree', shape: shape, edge: edge, paint: paint, rest: rest, named: function () { return named; } };

  /* ---- the life over the drawing ---- */
  if (!stage) return;
  var reduce = window.matchMedia && matchMedia('(prefers-reduced-motion: reduce)').matches;
  var cut = root.dataset.riverMotion === 'cut';
  var STORE = 'axgf-river-still', paused = false;
  try { paused = localStorage.getItem(STORE) === '1'; } catch (e) { /* private window */ }
  var css = getComputedStyle(root);
  function rgb(name, fallback) {
    var v = css.getPropertyValue(name).trim();
    if (v.charAt(0) !== '#') return fallback;
    var h = v.slice(1);
    if (h.length === 3) h = h.charAt(0) + h.charAt(0) + h.charAt(1) + h.charAt(1) + h.charAt(2) + h.charAt(2);
    return [0, 2, 4].map(function (i) { return parseInt(h.slice(i, i + 2), 16); });
  }
  var SAP = rgb('--tv-vein', [255, 255, 255]), SAP2 = rgb('--tv-vein-soft', SAP), LEAF = rgb('--tv-leaf-living', [255, 255, 255]);
  var EMBER = rgb('--tv-leaf-gone', SAP), SPORE = rgb('--tv-teal', LEAF);
  function rgba(c, a) { return 'rgba(' + c[0] + ',' + c[1] + ',' + c[2] + ',' + a.toFixed(3) + ')'; }

  var layer = document.createElement('canvas');
  layer.className = 'rv-water';
  layer.setAttribute('aria-hidden', 'true');
  stage.appendChild(layer);
  stage.classList.add('rv-alive');
  var ctx = layer.getContext('2d');
  if (!ctx) return;

  var flows = [], glows = [], spores = [], dpr = 1, cw = 0, ch = 0, clock = 0, travelling = false;
  function size() {
    var w = stage.clientWidth, h = stage.clientHeight, r = Math.min(window.devicePixelRatio || 1, 2);
    if (w === cw && h === ch && r === dpr) return false;
    cw = w; ch = h; dpr = r;
    layer.width = Math.max(1, Math.round(w * dpr));
    layer.height = Math.max(1, Math.round(h * dpr));
    return true;
  }
  function view() {
    var svg = stage.querySelector('svg.rv-svg'), vb = svg && svg.viewBox && svg.viewBox.baseVal;
    if (!vb || !vb.width || !cw) return null;
    var k = Math.min(cw / vb.width, ch / vb.height);
    return { x: vb.x - (cw / k - vb.width) / 2, y: vb.y - (ch / k - vb.height) / 2, k: k };
  }
  // Sap is drawn up from the root tips to the foot of the trunk, then up
  // the trunk and out to the leaves: along every attested bough, from the
  // end that comes first on that way to the end that comes after.
  function order(x, y) { return (GEO && y > GEO.ys + 0.5 ? -1 : 1) * depth(x, y); }
  function layout(L) {
    flows = []; glows = [];
    if (!L || !L.edges) return;
    L.edges.forEach(function (e) {
      if (e.conf !== 'a') return;
      var c = order(e.x1, e.y1) <= order(e.x2, e.y2) ? ctrl(e.x1, e.y1, e.x2, e.y2) : ctrl(e.x2, e.y2, e.x1, e.y1);
      var n = 24, xs = [], ys = [], cum = [0];
      for (var i = 0; i <= n; i++) {
        var p = bez(c, i / n);
        xs.push(p[0]); ys.push(p[1]);
        if (i) cum.push(cum[i - 1] + Math.hypot(xs[i] - xs[i - 1], ys[i] - ys[i - 1]));
      }
      var len = cum[n];
      if (len < 10) return;
      flows.push({ x: xs, y: ys, cum: cum, n: n, len: len, k: Math.max(1, Math.round(len / 85)),
        phase: Math.abs(Math.sin(e.x1 * 12.9898 + e.y2 * 78.233)) % 1 });
    });
    (L.persons || []).forEach(function (p, i) {
      if (p.role === 'centre') return;
      glows.push({ x: p.x, y: p.y, living: p.living, phase: (i * 0.618) % 1 });
    });
    if (!spores.length) for (var s = 0; s < 34; s++) spores.push({ u: (s * 0.381966) % 1, v: (s * 0.7548) % 1, speed: 0.012 + (s % 5) * 0.004, r: 0.6 + (s % 3) * 0.35 });
    request();
  }
  var SPEED = 24;
  function draw(moving) {
    size();
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, layer.width, layer.height);
    var v = view();
    if (!v || !moving || travelling) return;
    var s = v.k * dpr, ox = -v.x * s, oy = -v.y * s;
    ctx.globalCompositeOperation = 'lighter';
    // Spores: the faintest thing on the page, drifting up through the crown.
    spores.forEach(function (p) {
      var y = (p.v - clock * p.speed) % 1; if (y < 0) y += 1;
      var x = p.u + Math.sin(clock * 0.4 + p.u * 9) * 0.01;
      ctx.fillStyle = rgba(SPORE, 0.16 + 0.1 * Math.sin(clock * 1.3 + p.u * 20));
      ctx.beginPath(); ctx.arc(x * layer.width, y * layer.height, p.r * dpr, 0, 6.283); ctx.fill();
    });
    // Leaves breathe: the living green, the others an ember.
    glows.forEach(function (l) {
      var a = 0.5 + 0.5 * Math.sin(clock * (l.living ? 1.6 : 0.9) + l.phase * 6.283);
      var r = (l.living ? 9 : 6) * v.k * dpr;
      var g = ctx.createRadialGradient(l.x * s + ox, l.y * s + oy, 0, l.x * s + ox, l.y * s + oy, r);
      g.addColorStop(0, rgba(l.living ? LEAF : EMBER, (l.living ? 0.34 : 0.16) * a));
      g.addColorStop(1, rgba(l.living ? LEAF : EMBER, 0));
      ctx.fillStyle = g;
      ctx.beginPath(); ctx.arc(l.x * s + ox, l.y * s + oy, r, 0, 6.283); ctx.fill();
    });
    // Sap: a bead of light and its trail on every attested bough.
    flows.forEach(function (fl) {
      for (var j = 0; j < fl.k; j++) {
        var pos = ((fl.phase + j / fl.k) * fl.len + clock * SPEED) % fl.len, i = 1;
        while (i < fl.n && fl.cum[i] < pos) i++;
        var t = (pos - fl.cum[i - 1]) / ((fl.cum[i] - fl.cum[i - 1]) || 1);
        var x = (fl.x[i - 1] + (fl.x[i] - fl.x[i - 1]) * t) * s + ox, y = (fl.y[i - 1] + (fl.y[i] - fl.y[i - 1]) * t) * s + oy;
        var fade = Math.min(1, pos / 18, (fl.len - pos) / 18), r = 3.2 * v.k * dpr;
        var g = ctx.createRadialGradient(x, y, 0, x, y, r * 2.4);
        g.addColorStop(0, rgba(SAP2, 0.9 * fade));
        g.addColorStop(0.35, rgba(SAP, 0.45 * fade));
        g.addColorStop(1, rgba(SAP, 0));
        ctx.fillStyle = g;
        ctx.beginPath(); ctx.arc(x, y, r * 2.4, 0, 6.283); ctx.fill();
      }
    });
    ctx.globalCompositeOperation = 'source-over';
  }

  /* ---- when it moves: as the river's ground (river-scene.js) ---- */
  var raf = 0, running = false, onscreen = true, FRAME = 1000 / 30, last = 0, lastNow = 0, gaps = [], tired = false;
  function still() { return reduce || cut || paused || tired || document.hidden || !onscreen; }
  function loop(now) {
    raf = 0;
    if (!running) return;
    if (now - last >= FRAME - 2) {
      if (last) { gaps.push(now - last); if (gaps.length > 12) gaps.shift(); }
      if (gaps.length === 12 && gaps.reduce(function (a, b) { return a + b; }) / 12 > 2.5 * FRAME) {
        gaps = [];
        if (FRAME < 60) FRAME = 1000 / 15; else { tired = true; request(); return; }
      }
      clock += Math.min(0.1, (now - (lastNow || now)) / 1000);
      lastNow = now; last = now;
      draw(true);
    } else lastNow = lastNow || now;
    raf = requestAnimationFrame(loop);
  }
  function request() {
    var go = !still();
    if (go && !running) { running = true; lastNow = 0; last = 0; gaps = []; raf = requestAnimationFrame(loop); }
    else if (!go) { running = false; if (raf) cancelAnimationFrame(raf); raf = 0; draw(false); }
  }
  if ('IntersectionObserver' in window) {
    new IntersectionObserver(function (es) { onscreen = es[0].isIntersecting; request(); }).observe(stage);
  }
  document.addEventListener('visibilitychange', request);
  if ('ResizeObserver' in window) new ResizeObserver(function () { if (size()) request(); }).observe(stage);
  // The pause (WCAG 2.2.2): the river's own button, shown only where
  // something moves, and remembered as the river remembers it.
  var btn = root.querySelector('[data-rv-still]');
  if (btn && !reduce && !cut) {
    btn.hidden = false;
    btn.setAttribute('aria-pressed', paused ? 'true' : 'false');
    btn.addEventListener('click', function () {
      paused = !paused;
      if (!paused) tired = false;
      btn.setAttribute('aria-pressed', paused ? 'true' : 'false');
      try { localStorage.setItem(STORE, paused ? '1' : '0'); } catch (e) { /* private window */ }
      request();
    });
  }
  window.axgfRiverScene = {
    layout: function (L) { travelling = false; layout(L); },
    travel: function () { travelling = true; draw(false); },
    view: function () { if (!running) draw(false); }
  };
  size();
})();
