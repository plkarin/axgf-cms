/* Clarity's funnel (v0.5.0, docs/adr/0013).
 *
 * The river's own layout — the same people, the same links, from the same
 * payload, through the same reader's lens — drawn as an inverted cone, the
 * progressive chart of the operator's reference: one glass slice per
 * generation, the oldest at the narrow end below, the newest at the wide
 * mouth above, the family meshed from one slice to the next by its links.
 * The person at the fixed point stands at the front of their own slice;
 * partners beside them. Each slice says, at its right, its generation and
 * how many people it holds, as a funnel's stages say theirs.
 *
 * Two things the page knows by name:
 * - `window.axgfView` (river.js): where its people stand (`shape`), how a
 *   frame is painted, the curve of a link (`edge`, which the lit route
 *   follows too), and the dashboard beside it at rest. Travel, the trail, the
 *   panel, the keyboard, the camera and every touch rule stay river.js's.
 * - `window.axgfRiverScene`: light rising along every attested link from the
 *   older generation to the newer, and a sheen crossing the glass. It holds
 *   still as the river's ground does: reduced motion, "Cut", the pause, a
 *   hidden tab, a river off screen, a machine too slow for it.
 *
 * No colour is named here: every one is a token of the theme (`--cl-*`,
 * tokens.css), read from the page or set by class in app.css.
 */
(function () {
  'use strict';
  var root = document.querySelector('.river');
  if (!root || root.dataset.view !== 'funnel') return;
  var stage = document.getElementById('rv-canvas');

  /* ---- the rings (redrawn 2026-10-27, after a kit made in Canva) ----
   * One ring of glass per generation, seen a little from above: the oldest
   * at the narrow end below, the newest at the wide mouth above, each apart
   * from the next so that it reads as its own. */
  // How far a ring is seen from above: its depth against its half-width,
  // and never more than this share of the room between two rings.
  var TILT = 0.15, DEEP = 0.44;
  // Between two families on one ring, in places (a place is one person).
  var FAMILY = 0.7;
  // People along a ring: never closer than MINSP, never further apart than
  // MAXSP, and never past this share of the ring's half-width.
  var MINSP = 40, MAXSP = 96, SPREAD = 0.86;
  // The rings as the last frame drew them, for the edges and the scene.
  var GEO = null;
  function clamp(v, a, b) { return Math.max(a, Math.min(b, v)); }
  // `lo`..`hi` in levels (up is newer), mapped onto the stage; a ring's
  // half-width at each level (`widths`), between them in proportion.
  function geometry(m) {
    var g = m.funnel, ws = g.widths, P = (g.bottom - g.top) / (g.hi - g.lo + 1);
    var dw0 = g.hi > g.lo ? ws[g.lo + 1] - ws[g.lo] : 12, dw1 = g.hi > g.lo ? ws[g.hi] - ws[g.hi - 1] : 12;
    function W(l) {
      if (l <= g.lo) return Math.max(ws[g.lo] * 0.6, ws[g.lo] - (g.lo - l) * dw0);
      if (l >= g.hi) return ws[g.hi] + (l - g.hi) * dw1;
      var a = Math.floor(l);
      return ws[a] + (ws[a + 1] - ws[a]) * (l - a);
    }
    function R(l) { return Math.min(W(l) * TILT, P * DEEP); }
    function Y(l) { return g.top + (g.hi + 0.5 - l) * P; }
    return {
      sx: g.sx, lo: g.lo, hi: g.hi, P: P, W: W, R: R, Y: Y, wmax: W(g.ghi === undefined ? g.hi : g.ghi),
      // The front of a ring, nearest the reader, at `x`.
      front: function (l, x) { var e = (x - g.sx) / (W(l) || 1); return Y(l) + R(l) * Math.sqrt(Math.max(0, 1 - e * e)); }
    };
  }

  /* ---- where everyone stands ---- */
  function shape(fr) {
    var m = fr.meta, hl = m.hview[0], hw = m.hview[1], vt = m.view[0], vh = m.view[1];
    var C = fr.persons.filter(function (p) { return p.role === 'centre'; })[0];
    var cy = C ? C.y : m.cy, step0 = m.step || 100;
    // A level: generations from the person at the fixed point, up is newer.
    function lev(y) { return -(y - cy) / step0; }
    var lo = 0, hi = 0;
    fr.persons.forEach(function (p) { var l = Math.round(lev(p.y)); lo = Math.min(lo, l); hi = Math.max(hi, l); });
    // Room for what goes beyond: a line the record continues runs past the
    // last ring, and its count is written past its end.
    var rlo = lo, rhi = hi;
    function reach(y) { var l = lev(y); rlo = Math.min(rlo, Math.floor(l + 0.4)); rhi = Math.max(rhi, Math.ceil(l - 0.4)); }
    fr.tails.forEach(function (t) { reach(t.y2); if (t.count_at) reach(t.count_at.y); });
    fr.couples.forEach(function (c) { if (c.stub) { reach(c.y + c.stub_dy); if (c.stub_at) reach(c.stub_at.y); } });

    // Each generation's people in the river's order, left to right, and the
    // place each takes along its ring: brothers and sisters, and partners,
    // side by side, and a little room between one family and the next. The
    // river's order is the one that crosses its lines least; spacing it
    // evenly is what keeps a family from bunching into a knot of beads.
    var asKid = {}, asParent = {};
    fr.couples.forEach(function (c) {
      c.kids.forEach(function (k) { (asKid[k] = asKid[k] || []).push(c.key); });
      c.parents.forEach(function (k) { (asParent[k] = asParent[k] || []).push(c.key); });
    });
    function share(a, b, M) { return (M[a] || []).some(function (k) { return (M[b] || []).indexOf(k) >= 0; }); }
    var rows = {}, slot = {}, mid = {}, half = {};
    fr.persons.forEach(function (p) { var l = Math.round(lev(p.y)); (rows[l] = rows[l] || []).push(p); });
    var gens = Object.keys(rows).map(Number).sort(function (a, b) { return a - b; });
    gens.forEach(function (l) {
      var row = rows[l].sort(function (a, b) { return a.x - b.x; }), s = 0;
      row.forEach(function (p, i) {
        if (i) s += 1 + (share(row[i - 1].key, p.key, asKid) || share(row[i - 1].key, p.key, asParent) ? 0 : FAMILY);
        slot[p.key] = s;
      });
      // The person at the fixed point stands at the front of their ring.
      mid[l] = l === 0 && C && (C.key in slot) ? slot[C.key] : s / 2;
      half[l] = Math.max(mid[l], s - mid[l]);
    });
    // The rings: each wide enough for its people, each wider than the one
    // below it, and the mouth no wider than the stage allows; the funnel
    // stands a little left of the middle, its stages' figures on the right.
    var WMAX = hw * 0.42, WMIN = hw * 0.15, widths = {};
    for (var l = rlo; l <= rhi; l++) {
      var need = (l in half) ? half[l] * MINSP / SPREAD + 18 : 0;
      widths[l] = Math.max(WMIN + (WMAX - WMIN) * (l - rlo + 0.5) / (rhi - rlo + 1), need, l > rlo ? widths[l - 1] + 10 : 0);
    }
    if (widths[rhi] > WMAX) {
      var shrink = WMAX / widths[rhi];
      for (l = rlo; l <= rhi; l++) widths[l] *= shrink;
    }
    var funnel = { sx: hl + hw * 0.44, top: vt + vh * 0.05, bottom: vt + vh * 0.9, lo: rlo, hi: rhi, glo: lo, ghi: hi, widths: widths };
    var G = geometry({ funnel: funnel });
    var at = {}, pos = {};
    gens.forEach(function (l) {
      var d = half[l] > 0 ? Math.min(MAXSP, G.W(l) * SPREAD / half[l]) : 0;
      rows[l].forEach(function (p) {
        var X = G.sx + (slot[p.key] - mid[l]) * d;
        at[p.key] = [X, G.front(l, X)];
        pos[Math.round(p.x * 2) + ',' + Math.round(p.y * 2)] = at[p.key];
      });
    });
    // Anything else on the drawing (where a couple's lines meet, a line the
    // record continues, a count) keeps its place among its generation's
    // people: between two of them, it stays between them; between two
    // generations, it is between the two.
    var knots = {};
    gens.forEach(function (l) {
      knots[l] = rows[l].map(function (p) { return [p.x, (at[p.key][0] - G.sx) / G.W(l)]; });
    });
    function across(l, x) {
      var ks = knots[l];
      if (ks.length === 1) return clamp(ks[0][1] + (x - ks[0][0]) / (hw * 0.6), -0.96, 0.96);
      var i = 1;
      while (i < ks.length - 1 && x > ks[i][0]) i++;
      var a = ks[i - 1], b = ks[i], k = b[0] === a[0] ? 0.5 : (x - a[0]) / (b[0] - a[0]);
      return clamp(a[1] + (b[1] - a[1]) * k, -0.96, 0.96);
    }
    function place(x, y) {
      var exact = pos[Math.round(x * 2) + ',' + Math.round(y * 2)];
      if (exact) return exact;
      var l = lev(y), e = 0;
      if (gens.length) {
        var a = gens[0], b = gens[gens.length - 1];
        if (l <= a) e = across(a, x);
        else if (l >= b) e = across(b, x);
        else {
          var i = 1;
          while (i < gens.length - 1 && l > gens[i]) i++;
          var g0 = gens[i - 1], g1 = gens[i], k = (l - g0) / (g1 - g0);
          e = across(g0, x) * (1 - k) + across(g1, x) * k;
        }
      }
      var X = G.sx + e * G.W(l);
      return [X, G.front(l, X)];
    }
    function moved(o) { var q = place(o.x, o.y); return Object.assign({}, o, { x: q[0], y: q[1] }); }
    function markAt(mk) {
      if (!mk) return mk;
      var q = place(mk.x, mk.y), out = Object.assign({}, mk, { x: q[0], y: q[1] });
      if (mk.leader) { var a = place(mk.leader[0], mk.leader[1]), b = place(mk.leader[2], mk.leader[3]); out.leader = [a[0], a[1], b[0], b[1]]; }
      return out;
    }
    return {
      persons: fr.persons.map(function (p) {
        return Object.assign({}, p, { x: at[p.key][0], y: at[p.key][1], lv: Math.round(lev(p.y)) });
      }),
      couples: fr.couples.map(function (c) {
        var o = moved(c);
        // A couple's lines meet over the middle of the two of them.
        var ps = c.parents.filter(function (k) { return at[k]; });
        if (ps.length) {
          o.x = ps.reduce(function (s, k) { return s + at[k][0]; }, 0) / ps.length;
          o.y = G.front(lev(c.y), o.x);
        }
        if (c.stub) {
          var e = place(c.x + c.stub_dx, c.y + c.stub_dy);
          o.stub_dx = e[0] - o.x; o.stub_dy = e[1] - o.y; o.stub_at = markAt(c.stub_at);
        }
        return o;
      }),
      tails: fr.tails.map(function (t) {
        var a = place(t.x1, t.y1), b = place(t.x2, t.y2);
        return Object.assign({}, t, { x1: a[0], y1: a[1], x2: b[0], y2: b[1], count_at: markAt(t.count_at) });
      }),
      meta: Object.assign({}, m, { funnel: funnel })
    };
  }
  // A link runs from one ring to the next: an S, as a thread drawn on the
  // funnel's wall.
  function edge(x1, y1, x2, y2) {
    var r = function (v) { return Math.round(v * 10) / 10; }, my = (y1 + y2) / 2;
    return 'M' + r(x1) + ' ' + r(y1) + 'C' + r(x1) + ' ' + r(my) + ' ' + r(x2) + ' ' + r(my) + ' ' + r(x2) + ' ' + r(y2);
  }

  /* ---- the drawing ---- */
  var named = {};
  // A ring's colour, from the mouth (1, blue) to the narrow end (5, pink).
  function tone(l, lo, hi) { return hi === lo ? 3 : 1 + Math.round(4 * (hi - l) / (hi - lo)); }
  // Whether a box would sit across the outline of a ring: some of it on
  // the glass, some of it off. A name is never written over an outline.
  function overRim(rings, b) {
    var pts = [];
    for (var i = 0; i <= 4; i++) { var x = b[0] + (b[2] - b[0]) * i / 4; pts.push([x, b[1]], [x, b[3]]); }
    pts.push([b[0], (b[1] + b[3]) / 2], [b[2], (b[1] + b[3]) / 2]);
    for (var r = 0; r < rings.length; r++) {
      var q = rings[r], inside = 0;
      for (var j = 0; j < pts.length; j++) {
        var dx = (pts[j][0] - q[0]) / q[2], dy = (pts[j][1] - q[1]) / q[3];
        if (dx * dx + dy * dy < 1) inside++;
      }
      if (inside && inside < pts.length) return true;
    }
    return false;
  }
  var RINGS = [];
  function paint(fr, api, rest) {
    var f = api.f, esc = api.esc, m = fr.meta, PM = api.byKey(fr.persons), out = '', links = '', nodes = '', bands = '', figs = '';
    // Above the glass, in this order: what the record continues past the
    // last ring, the threads, the knots where a couple's threads meet, then
    // the counts.
    var tails = '', unions = '', marks = '', base = '', beam = '';
    var hl = m.hview[0], hw = m.hview[1], vt = m.view[0], vh = m.view[1];
    var G = geometry(m);
    GEO = G;
    out += '<rect class="cl-bg" x="' + f(hl) + '" y="' + f(vt) + '" width="' + f(hw) + '" height="' + f(vh) + '"/>';
    // How many people each generation holds, each person once.
    var count = {}, seen = {};
    fr.persons.forEach(function (p) {
      var k = p.lv + ':' + p.id;
      if (seen[k] || p.lv === undefined) return;
      seen[k] = 1;
      count[p.lv] = (count[p.lv] || 0) + 1;
    });
    var glo = m.funnel.glo, ghi = m.funnel.ghi;
    // The pedestal of light the funnel stands on, under its narrow end and
    // the lines that run past it.
    var pr = G.W(glo) * 0.92, py = Math.min(vt + vh - 8 - pr * TILT, Math.max(G.Y(glo) + G.R(glo) + 16, G.Y(m.funnel.lo - 0.5) + 10));
    base += '<ellipse class="cl-base-glow" cx="' + f(G.sx) + '" cy="' + f(py) + '" rx="' + f(pr * 1.3) + '" ry="' + f(pr * 1.3 * TILT * 1.4) + '"/>'
      + '<ellipse class="cl-base-ring" cx="' + f(G.sx) + '" cy="' + f(py) + '" rx="' + f(pr) + '" ry="' + f(pr * TILT * 1.4) + '"/>'
      + '<ellipse class="cl-base" cx="' + f(G.sx) + '" cy="' + f(py) + '" rx="' + f(pr * 0.55) + '" ry="' + f(pr * 0.55 * TILT * 1.4) + '"/>';
    G.py = py; G.pr = pr;
    // The rings, the narrow end first, so that a wider ring above lies over
    // the one below as glass does: a thin disc, its face tinted, a light
    // along its far side, a fine rim; and the stage's figures in a column
    // on the right, each on a thread from its ring.
    var boxes = [], rings = [], fx = G.sx + G.W(ghi) + 34;
    for (var l = glo; l <= ghi; l++) {
      var t = tone(l, glo, ghi), Y = G.Y(l), W = G.W(l), R = G.R(l), sx = G.sx, th = 2.4 + W * 0.012;
      rings.push([sx, Y, W, R]);
      var front = 'M' + f(sx - W) + ' ' + f(Y) + 'A' + f(W) + ' ' + f(R) + ' 0 0 0 ' + f(sx + W) + ' ' + f(Y);
      bands += '<path class="cl-ring-edge cl-rim-' + t + '" d="' + front + 'L' + f(sx + W) + ' ' + f(Y + th) + 'A' + f(W) + ' ' + f(R) + ' 0 0 1 ' + f(sx - W) + ' ' + f(Y + th) + 'Z"/>'
        + '<ellipse class="cl-ring" cx="' + f(sx) + '" cy="' + f(Y) + '" rx="' + f(W) + '" ry="' + f(R) + '" fill="url(#cl-g' + t + ')"/>'
        + '<path class="cl-ring-light" d="M' + f(sx - W * 0.8) + ' ' + f(Y - R * 0.42) + 'Q' + f(sx) + ' ' + f(Y - R * 1.3) + ' ' + f(sx + W * 0.8) + ' ' + f(Y - R * 0.42) + '"/>'
        + '<path class="cl-rim-back" d="M' + f(sx - W) + ' ' + f(Y) + 'A' + f(W) + ' ' + f(R) + ' 0 0 1 ' + f(sx + W) + ' ' + f(Y) + '"/>'
        + '<path class="cl-rim-front" d="' + front + '"/>';
      var word = l > 0 ? '+' + l : l < 0 ? '−' + (-l) : '0', n = count[l] || 0, num;
      try { num = n.toLocaleString(document.documentElement.lang || undefined); } catch (e) { num = String(n); }
      figs += '<line class="cl-leader" x1="' + f(sx + W + 6) + '" y1="' + f(Y) + '" x2="' + f(fx - 9) + '" y2="' + f(Y) + '"/>'
        + '<circle class="cl-leader-tip" cx="' + f(fx - 9) + '" cy="' + f(Y) + '" r="1.8"/>'
        + '<text x="' + f(fx) + '" y="' + f(Y + 1) + '" font-size="17" font-weight="600" class="cl-figure">' + esc(num) + '</text>'
        + '<text x="' + f(fx) + '" y="' + f(Y + 13) + '" font-family="' + api.mono + '" font-size="9.5" class="k-band-text cl-gen">' + word + '</text>';
      boxes.push([fx - 2, Y - 15, fx + Math.max(api.width(num, 17, false), 22) + 2, Y + 16]);
    }
    RINGS = rings;
    // Lines the record continues past the last ring, and lines it ends.
    var counts = [];
    fr.tails.forEach(function (t) {
      tails += '<g opacity="' + f(t.a) + '"><path d="' + edge(t.x1, t.y1, t.x2, t.y2) + '" fill="none" class="cl-tail"/>';
      tails += t.kind === 'cont'
        ? '<circle cx="' + f(t.x2) + '" cy="' + f(t.y2) + '" r="2.4" class="cl-tail-tip"/>'
        : '<circle cx="' + f(t.x2) + '" cy="' + f(t.y2) + '" r="3.2" class="cl-end"/>';
      tails += '</g>';
      if (t.kind === 'cont' && t.count_at) counts.push({ mk: t.count_at, text: '+' + t.count, size: 9.5, dx: t.cref ? t.x2 - t.cref[0] : 0, dy: t.cref ? t.y2 - t.cref[1] : 0, a: t.a });
    });
    fr.couples.forEach(function (c) {
      if (!c.stub) return;
      tails += '<g opacity="' + f(c.a) + '"><path d="' + edge(c.x, c.y, c.x + c.stub_dx, c.y + c.stub_dy) + '" fill="none" class="cl-tail cl-stub"/></g>';
      if (c.stub_at) counts.push({ mk: c.stub_at, text: '+' + c.stub, size: 10, dx: c.sref ? c.x - c.sref[0] : 0, dy: c.sref ? c.y - c.sref[1] : 0, a: c.a });
    });
    // The threads: parent to the couple's knot, the knot to each child.
    api.edges(fr, PM).forEach(function (e) {
      var d = edge(e.x1, e.y1, e.x2, e.y2), w = f(0.7 + api.widths[api.cls(e.d)] * 0.24), o = e.a < 1 ? ' opacity="' + f(e.a) + '"' : '';
      // Edged in light, so that the ink reads on the deep and the bright
      // parts of the prism alike.
      links += '<path d="' + d + '" class="cl-link-halo" stroke-width="' + f(+w + 2.2) + '"' + o + '/>';
      if (e.conf === 'a') links += '<path d="' + d + '" fill="none" class="cl-link" stroke-width="' + w + '" stroke-linecap="round"' + o + '/>';
      else if (e.conf === 'd') links += '<path d="' + d + '" fill="none" class="cl-link cl-link-d" stroke-width="' + w + '" stroke-dasharray="5 3.5"' + o + '/>';
      else links += '<path d="' + d + '" fill="none" class="cl-link cl-link-s" stroke-width="1.8" stroke-linecap="round" stroke-dasharray="0.1 4.5"' + o + '/>';
    });
    fr.couples.forEach(function (c) {
      unions += '<circle cx="' + f(c.x) + '" cy="' + f(c.y) + '" r="2.7" class="cl-union" opacity="' + f(c.a) + '"/>';
    });
    // Every bead, every knot and every figure is a place no name may cover.
    fr.persons.forEach(function (p) {
      var rr = p.role === 'centre' ? 13 : 7;
      boxes.push([p.x - rr, p.y - rr, p.x + rr, p.y + rr]);
    });
    fr.couples.forEach(function (c) { boxes.push([c.x - 3.5, c.y - 3.5, c.x + 3.5, c.y + 3.5]); });
    function hit(box) {
      for (var i = 0; i < boxes.length; i++) {
        var b = boxes[i];
        if (box[0] < b[2] && box[2] > b[0] && box[1] < b[3] && box[3] > b[1]) return true;
      }
      return false;
    }
    function free(b) {
      return b[0] >= hl + 2 && b[2] <= hl + hw - 2 && b[1] >= vt + 2 && b[3] <= vt + vh - 2 && !hit(b) && !overRim(rings, b);
    }
    // Names (2026-10-27): each on a small frosted label, on its own ring's
    // glass above its bead when there is room — or beside it, beside its
    // ring, or below — and never across an outline, over a bead or over
    // another name. A name with no such place is left out, as the river
    // leaves out a word it has no room for; the pointer brings it back.
    var keep = {}, PH = 15;
    function spots(p, w, h) {
      var Wl = G.W(p.lv), e = (p.x - G.sx) / (Wl || 1), out = [];
      var o = e < 0 ? -1 : 1;
      out.push([p.x - w / 2, p.y - 7 - h]);
      out.push([o < 0 ? p.x - 4 - w : p.x + 4, p.y - 7 - h]);
      out.push([o < 0 ? p.x + 4 : p.x - 4 - w, p.y - 7 - h]);
      if (Math.abs(e) > 0.4) out.push([o < 0 ? G.sx - Wl - 8 - w : G.sx + Wl + 8, p.y - h / 2]);
      out.push([p.x - w / 2, p.y - 10 - 2 * h]);
      out.push([p.x - w / 2, p.y + 8]);
      out.push([o < 0 ? p.x - 10 - w : p.x + 10, p.y - h / 2]);
      return out.map(function (q) { return [q[0], q[1], q[0] + w, q[1] + h]; });
    }
    // The person at the fixed point: their full name and years on a larger
    // label, on their ring's glass above them, else below; on one line
    // where two do not fit, as on the flat rings of the whole family.
    var C0 = fr.persons.filter(function (p) { return p.role === 'centre'; })[0], cpill = null;
    if (C0 && C0.s && !C0.s.redacted) {
      var cn = api.clip(C0.s.names[2], 260), line = cn + (C0.s.years ? ' · ' + C0.s.years : '');
      var w2 = Math.max(api.width(cn, 15, true), api.width(C0.s.years, 10.5, true)) + 24, w1 = api.width(line, 13.5, true) + 22;
      var cs = [[w2, 30, -13, 2], [w2, 30, 14, 2], [w1, 20, -11, 1], [w1, 20, 13, 1]].map(function (q) {
        var y0 = q[2] < 0 ? C0.y + q[2] - q[1] : C0.y + q[2];
        return { box: [C0.x - q[0] / 2, y0, C0.x + q[0] / 2, y0 + q[1]], lines: q[3] };
      });
      cpill = cs.filter(function (c) { return free(c.box); })[0] || cs[2];
      cpill.text = line;
      boxes.push(cpill.box);
    }
    fr.persons.slice().sort(function (a, b) {
      return Math.abs(a.lv) - Math.abs(b.lv) || (b.role === 'spouse') - (a.role === 'spouse') || a.x - b.x;
    }).forEach(function (p) {
      var s = p.s || {}, gd = Math.abs(p.lv);
      if (p.role === 'centre' || s.redacted || !s.names) return;
      var word = gd <= 1 || p.role === 'spouse' ? s.names[1] : s.names[0], w = api.width(word, 11.5, false) + 14;
      var spot = spots(p, w, PH).filter(free)[0];
      if (!spot) return;
      boxes.push(spot);
      keep[p.key] = { box: spot, word: word };
    });
    // The counts: never dropped, the only sign of the people beyond, but
    // stepped aside from a name rather than written over it.
    counts.forEach(function (c) {
      var mk = c.mk, w = api.width(c.text, c.size, true), at = [0, 0], spot = null;
      [[0, 0], [0, 11], [0, -11], [14, 0], [-14, 0], [0, 22]].some(function (o) {
        var x = mk.x + c.dx + o[0], y = mk.y + c.dy + o[1];
        var x0 = mk.anchor < 0 ? x - w : mk.anchor === 0 ? x - w / 2 : x, box = [x0 - 1, y - c.size, x0 + w + 1, y + 2];
        if (!spot) spot = box;
        if (hit(box)) return false;
        at = o; spot = box;
        return true;
      });
      boxes.push(spot);
      // On the glass a count wears the page's ground around it, as names do.
      var mv = Object.assign({}, mk, { x: mk.x + at[0], y: mk.y + at[1], halo: true });
      if (mk.leader) mv.leader = [mk.leader[0], mk.leader[1], mk.leader[2] + at[0], mk.leader[3] + at[1]];
      marks += '<g class="cl-count" opacity="' + f(c.a) + '">' + api.mark(mv, c.text, c.size, c.dx, c.dy) + '</g>';
    });
    // The people: a pearl on the front of their ring, and the person at the
    // fixed point a larger one with the theme's ring.
    fr.persons.forEach(function (p) {
      var s = p.s || {}, isC = p.role === 'centre', x = f(p.x), y = f(p.y), r = 5.2;
      var k = '<g opacity="' + f(p.a) + '">';
      if (rest) k += '<a class="rv-p" data-id="' + esc(p.id) + '" data-key="' + esc(p.key) + '" href="' + api.base + '?p=' + encodeURIComponent(p.id) + '&amp;n=' + m.n + '" aria-label="' + esc(api.said(p)) + '"><circle cx="' + x + '" cy="' + y + '" r="16" fill="transparent"/>';
      if (isC) {
        k += '<circle cx="' + x + '" cy="' + y + '" r="19" class="cl-halo"/><circle cx="' + x + '" cy="' + y + '" r="10.5" class="cl-centre"/><circle cx="' + x + '" cy="' + y + '" r="4.2" class="cl-centre-core"/>';
      } else {
        if (p.repeat > 1) k += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 3) + '" fill="none" class="k-data-line" stroke-width="1"/>';
        if (p.again) k += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 3 + (p.repeat > 1 ? 2.2 : 0)) + '" fill="none" class="k-data-line" stroke-width="1" stroke-dasharray="2.4 1.8"/>';
        if (s.redacted || s.sparse) k += '<circle cx="' + x + '" cy="' + y + '" r="' + r + '" class="cl-bead cl-hidden" stroke-width="1.4" stroke-dasharray="1.6 1.6"/>';
        else if (s.living) k += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 3.6) + '" class="cl-bead-glow cl-living"/><circle cx="' + x + '" cy="' + y + '" r="' + r + '" class="cl-bead cl-living" stroke-width="1.8"/>';
        else k += '<circle cx="' + x + '" cy="' + y + '" r="' + r + '" class="cl-bead cl-gone" stroke-width="1.6"/>';
      }
      if (isC && cpill) {
        var cb = cpill.box, cx0 = f((cb[0] + cb[2]) / 2);
        k += '<g class="k-name"><rect class="cl-pill cl-pill-centre" x="' + f(cb[0]) + '" y="' + f(cb[1]) + '" width="' + f(cb[2] - cb[0]) + '" height="' + f(cb[3] - cb[1]) + '" rx="' + (cpill.lines === 2 ? 9 : 10) + '"/>';
        if (cpill.lines === 2) {
          k += '<text x="' + cx0 + '" y="' + f(cb[1] + 14) + '" text-anchor="middle" font-size="15" font-weight="600" class="cl-name cl-centre-name">' + esc(api.clip(s.names[2], 260)) + '</text>'
            + '<text x="' + cx0 + '" y="' + f(cb[1] + 26) + '" text-anchor="middle" font-family="' + api.mono + '" font-size="10.5" class="cl-data">' + esc(s.years) + '</text>';
        } else {
          k += '<text x="' + cx0 + '" y="' + f(cb[1] + 14.5) + '" text-anchor="middle" font-size="13.5" font-weight="600" class="cl-name cl-centre-name">' + esc(cpill.text) + '</text>';
        }
        k += '</g>';
      } else if (keep[p.key]) {
        var L = keep[p.key], b = L.box;
        // A name that stands off its bead (beside its ring) hangs on a thread.
        var tx = clamp(p.x, b[0], b[2]), ty = clamp(p.y, b[1], b[3]), dl = Math.hypot(tx - p.x, ty - p.y);
        k += '<g class="k-name">';
        if (dl > 12) k += '<line class="tv-leader cl-label-thread" x1="' + f(p.x + (tx - p.x) / dl * 7) + '" y1="' + f(p.y + (ty - p.y) / dl * 7) + '" x2="' + f(tx) + '" y2="' + f(ty) + '"/>';
        k += '<rect class="tv-panel cl-pill" x="' + f(b[0]) + '" y="' + f(b[1]) + '" width="' + f(b[2] - b[0]) + '" height="' + f(b[3] - b[1]) + '" rx="' + f((b[3] - b[1]) / 2) + '"/>'
          + '<text x="' + f((b[0] + b[2]) / 2) + '" y="' + f(b[1] + 11) + '" text-anchor="middle" font-size="11.5" class="cl-name">' + esc(L.word) + '</text></g>';
      }
      if (rest) k += '</a>';
      nodes += k + '</g>';
    });
    if (rest) named = keep;
    var defs = '';
    for (var i = 1; i <= 5; i++) {
      defs += '<linearGradient id="cl-g' + i + '" x1="0" x2="1" y1="0" y2="0">'
        + '<stop offset="0" class="cl-s' + i + ' cl-edge"/><stop offset="0.3" class="cl-s' + i + ' cl-mid"/>'
        + '<stop offset="0.47" class="cl-hi"/><stop offset="0.64" class="cl-s' + i + ' cl-mid"/><stop offset="1" class="cl-s' + i + ' cl-edge"/></linearGradient>';
    }
    // A pearl: lit from the upper left.
    defs += '<radialGradient id="cl-pearl" cx="0.36" cy="0.32" r="0.78"><stop offset="0" class="cl-pearl-0"/><stop offset="0.55" class="cl-pearl-1"/><stop offset="1" class="cl-pearl-2"/></radialGradient>';
    var C = fr.persons.filter(function (p) { return p.role === 'centre'; })[0], cname = C && C.s && !C.s.redacted ? C.s.names[2] : '';
    var lbl = (root.querySelector('.rv-stage') || root).getAttribute('aria-label') || '';
    return '<svg class="rv-svg cl-svg" data-view="funnel" viewBox="' + f(hl) + ' ' + f(vt) + ' ' + f(hw) + ' ' + f(vh) + '" width="100%" '
      + (rest ? 'role="group" aria-label="' + esc(lbl + (cname ? ' — ' + cname : '')) + '"' : 'aria-hidden="true"')
      + ' xmlns="http://www.w3.org/2000/svg"><defs>' + defs + '</defs>' + out + base + bands + beam + figs + tails + links + '<g class="rv-lit"></g>' + unions + marks + nodes + '</svg>';
  }

  /* ---- the dashboard: figures read from the cone on screen ---- */
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
    // The bars: the funnel's own profile, a bar per generation, the oldest
    // first as the cone reads from its narrow end.
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

  window.axgfView = { name: 'funnel', shape: shape, edge: edge, paint: paint, rest: rest, named: function () { return named; } };
  // A name the river adds (every name, or one the drawing had no room for)
  // keeps off the rings' outlines too.
  window.axgfView.keepOut = function (b) { return overRim(RINGS, b); };

  /* ---- the light ---- */
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
  var HOT = rgb('--cl-link-hot', [200, 80, 20]), SHEEN = rgb('--cl-gloss', [255, 255, 255]);
  var TONES = [1, 2, 3, 4, 5].map(function (i) { return rgb('--cl-slice-' + i, [99, 102, 241]); });
  function rgba(c, a) { return 'rgba(' + c[0] + ',' + c[1] + ',' + c[2] + ',' + a.toFixed(3) + ')'; }

  var over = document.createElement('canvas');
  over.className = 'rv-water';
  over.setAttribute('aria-hidden', 'true');
  stage.appendChild(over);
  // The laboratory under the cone (2026-10-23): a floor of light lines
  // running to the horizon behind the projector, rings rising from it, and
  // a slow network of data points drifting up the sheet. At rest, the same
  // laboratory, still.
  var ground = document.createElement('canvas');
  ground.className = 'rv-ground';
  ground.setAttribute('aria-hidden', 'true');
  stage.insertBefore(ground, stage.firstChild);
  stage.classList.add('rv-alive');
  var go = over.getContext('2d'), gg = ground.getContext('2d');
  if (!go || !gg) return;
  // The data points, the same on every visit: a position on the sheet
  // (0..1 each way), a speed, a sway and a tone of the cone.
  var DOTS = [];
  for (var di = 0; di < 64; di++) {
    var h1 = Math.abs(Math.sin(di * 12.9898) * 43758.5453) % 1, h2 = Math.abs(Math.sin(di * 78.233) * 12345.678) % 1;
    DOTS.push({ x: h1, y: h2, v: 0.012 + 0.02 * ((h1 * 7) % 1), sw: 0.004 + 0.008 * ((h2 * 5) % 1), ph: h1 * 6.283, tone: di % 5, r: 1.2 + 1.6 * ((h1 * 3) % 1) });
  }
  var flows = [], dpr = 1, cw = 0, ch = 0, clock = 0, travelling = false;
  function size() {
    var w = stage.clientWidth, h = stage.clientHeight, r = Math.min(window.devicePixelRatio || 1, 2);
    if (w === cw && h === ch && r === dpr) return false;
    cw = w; ch = h; dpr = r;
    over.width = ground.width = Math.max(1, Math.round(w * dpr));
    over.height = ground.height = Math.max(1, Math.round(h * dpr));
    return true;
  }
  function view() {
    var svg = stage.querySelector('svg.rv-svg'), vb = svg && svg.viewBox && svg.viewBox.baseVal;
    if (!vb || !vb.width || !cw) return null;
    var k = Math.min(cw / vb.width, ch / vb.height);
    return { x: vb.x - (cw / k - vb.width) / 2, y: vb.y - (ch / k - vb.height) / 2, k: k };
  }
  function cpoint(x1, y1, x2, y2, t) {
    var my = (y1 + y2) / 2, u = 1 - t;
    return [u * u * u * x1 + 3 * u * u * t * x1 + 3 * u * t * t * x2 + t * t * t * x2,
      u * u * u * y1 + 3 * u * u * t * my + 3 * u * t * t * my + t * t * t * y2];
  }
  // Light rises: from the older end of a link (lower on the cone) to the
  // newer, as the family reads from its narrow end to its mouth.
  function layout(L) {
    flows = [];
    if (!L || !L.edges) return;
    L.edges.forEach(function (e) {
      if (e.conf !== 'a') return;
      var a = e.y1 >= e.y2 ? [e.x1, e.y1, e.x2, e.y2] : [e.x2, e.y2, e.x1, e.y1];
      var n = 20, xs = [], ys = [], cum = [0];
      for (var i = 0; i <= n; i++) {
        var p = cpoint(a[0], a[1], a[2], a[3], i / n);
        xs.push(p[0]); ys.push(p[1]);
        if (i) cum.push(cum[i - 1] + Math.hypot(xs[i] - xs[i - 1], ys[i] - ys[i - 1]));
      }
      if (cum[n] < 8) return;
      flows.push({ x: xs, y: ys, cum: cum, n: n, len: cum[n], phase: Math.abs(Math.sin(e.x1 * 12.9898 + e.y2 * 78.233)) % 1 });
    });
    request();
  }
  var SPEED = 26, SWEEP = 7;
  function lab(moving) {
    var W = ground.width, H = ground.height, v = view(), t = moving ? clock : 2.2;
    gg.setTransform(1, 0, 0, 1, 0, 0);
    gg.clearRect(0, 0, W, H);
    if (!v || !GEO || GEO.py === undefined) return;
    var s = v.k * dpr, ox = -v.x * s, oy = -v.y * s;
    var bx = GEO.sx * s + ox, by = GEO.py * s + oy, br = GEO.pr * s, hz = Math.max(H * 0.42, by - H * 0.3);
    // The floor: lines from the sheet's lower edge to a point on the horizon
    // behind the projector, and rungs that close in toward it.
    gg.lineWidth = Math.max(1, dpr * 0.8);
    var fl = gg.createLinearGradient(0, hz, 0, H);
    fl.addColorStop(0, rgba(TONES[1], 0));
    fl.addColorStop(1, rgba(TONES[1], 0.2));
    gg.strokeStyle = fl;
    gg.beginPath();
    for (var i = -14; i <= 14; i++) { gg.moveTo(bx, hz); gg.lineTo(bx + i * W * 0.09, H); }
    var run = (t * 0.06) % 1;
    for (var j = 0; j < 12; j++) {
      var k = (j + run) / 12, y = hz + (H - hz) * k * k;
      gg.moveTo(0, y); gg.lineTo(W, y);
    }
    gg.stroke();
    // Rings rising from the projector, each fading as it widens.
    for (var q = 0; q < 3; q++) {
      var ph = ((moving ? t / 4.2 : 0.2) + q / 3) % 1, rx = br * (0.8 + 3.4 * ph);
      gg.strokeStyle = rgba(TONES[2 + q], 0.38 * (1 - ph));
      gg.lineWidth = Math.max(1, dpr * (1.6 - ph));
      gg.beginPath(); gg.ellipse(bx, by, rx, rx * TILT * 1.6, 0, 0, 6.283); gg.stroke();
    }
    // The network: each point drifts up the sheet and sways; points close
    // to one another are joined by a faint line.
    var pts = DOTS.map(function (d) {
      var yy = ((d.y - t * d.v) % 1 + 1) % 1, xx = d.x + Math.sin(d.ph + t * 0.6) * d.sw;
      return [xx * W, yy * H, d];
    });
    var near = 0.085 * Math.max(W, H);
    gg.lineWidth = Math.max(1, dpr * 0.7);
    for (var a = 0; a < pts.length; a++) {
      for (var b = a + 1; b < pts.length; b++) {
        var dx = pts[a][0] - pts[b][0], dy = pts[a][1] - pts[b][1], dd = Math.sqrt(dx * dx + dy * dy);
        if (dd > near) continue;
        gg.strokeStyle = rgba(TONES[pts[a][2].tone], 0.16 * (1 - dd / near));
        gg.beginPath(); gg.moveTo(pts[a][0], pts[a][1]); gg.lineTo(pts[b][0], pts[b][1]); gg.stroke();
      }
    }
    pts.forEach(function (p) {
      gg.fillStyle = rgba(TONES[p[2].tone], 0.5);
      gg.beginPath(); gg.arc(p[0], p[1], p[2].r * dpr, 0, 6.283); gg.fill();
    });
    // A scanner's band crossing the sheet upward now and then.
    if (moving) {
      var sc = (t % 11) / 11;
      if (sc < 0.6) {
        var sy = H * (1 - sc / 0.6), band = H * 0.06, sg = gg.createLinearGradient(0, sy - band, 0, sy + band);
        sg.addColorStop(0, rgba(TONES[0], 0)); sg.addColorStop(0.5, rgba(TONES[0], 0.07)); sg.addColorStop(1, rgba(TONES[0], 0));
        gg.fillStyle = sg; gg.fillRect(0, sy - band, W, band * 2);
      }
    }
  }
  function draw(moving) {
    size();
    var W = over.width, H = over.height;
    lab(moving);
    go.setTransform(1, 0, 0, 1, 0, 0);
    go.clearRect(0, 0, W, H);
    var v = view();
    if (!v || !moving || travelling || !GEO) return;
    var s = v.k * dpr, ox = -v.x * s, oy = -v.y * s;
    // The sheen: a band of light crossing the glass now and then, inside the
    // cone's outline only.
    var ph = (clock % SWEEP) / SWEEP;
    if (ph < 0.45) {
      var G = GEO, yT = G.Y(G.hi + 0.5), yB = G.Y(G.lo - 0.5), wT = G.W(G.hi + 0.5);
      go.save();
      go.beginPath();
      RINGS.forEach(function (q) {
        go.moveTo((q[0] + q[2]) * s + ox, q[1] * s + oy);
        go.ellipse(q[0] * s + ox, q[1] * s + oy, q[2] * s, q[3] * s, 0, 0, 6.283);
      });
      go.clip();
      var x = (G.sx - wT + (2 * wT + 120) * (ph / 0.45)) * s + ox, band = 60 * s;
      var g = go.createLinearGradient(x - band, 0, x + band, 0);
      g.addColorStop(0, rgba(SHEEN, 0)); g.addColorStop(0.5, rgba(SHEEN, 0.32)); g.addColorStop(1, rgba(SHEEN, 0));
      go.fillStyle = g;
      go.fillRect(x - band, yT * s + oy, band * 2, (yB - yT) * s);
      go.restore();
    }
    flows.forEach(function (fl) {
      var pos = (fl.phase * fl.len + clock * SPEED) % fl.len, i = 1;
      while (i < fl.n && fl.cum[i] < pos) i++;
      var t = (pos - fl.cum[i - 1]) / ((fl.cum[i] - fl.cum[i - 1]) || 1);
      var x = (fl.x[i - 1] + (fl.x[i] - fl.x[i - 1]) * t) * s + ox, y = (fl.y[i - 1] + (fl.y[i] - fl.y[i - 1]) * t) * s + oy;
      var fade = Math.min(1, pos / 14, (fl.len - pos) / 14), r = 5 * v.k * dpr;
      var pg = go.createRadialGradient(x, y, 0, x, y, r);
      pg.addColorStop(0, rgba(HOT, 0.9 * fade)); pg.addColorStop(0.45, rgba(HOT, 0.35 * fade)); pg.addColorStop(1, rgba(HOT, 0));
      go.fillStyle = pg; go.beginPath(); go.arc(x, y, r, 0, 6.283); go.fill();
    });
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
    var goOn = !still();
    if (goOn && !running) { running = true; lastNow = 0; last = 0; gaps = []; raf = requestAnimationFrame(loop); }
    else if (!goOn) { running = false; if (raf) cancelAnimationFrame(raf); raf = 0; draw(false); }
  }
  if ('IntersectionObserver' in window) {
    new IntersectionObserver(function (es) { onscreen = es[0].isIntersecting; request(); }).observe(stage);
  }
  document.addEventListener('visibilitychange', request);
  if ('ResizeObserver' in window) new ResizeObserver(function () { if (size()) request(); if (!running) draw(false); }).observe(stage);
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
  draw(false);
})();
