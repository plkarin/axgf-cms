/* Fusion's tree in its glass (v0.5.0, docs/adr/0007; redrawn 2026-10-23).
 *
 * The river's own people — from the same payload, through the same reader's
 * lens, in the same order from left to right — grown as a tree inside a
 * glass cylinder, seen from the front and a little above, as a hologram in a
 * laboratory (static/img/fusion-lab.jpg, made in the operator's Canva; the
 * glass stands on its projector). The person at the fixed point is on the
 * axis, in a column of light. The tree is a hierarchy in tiers, a floor of
 * the glass per generation, straight across: descendants above, each
 * family's children side by side over it with the parents centred under
 * them; ancestors below, each person's father and mother under them. The
 * river's order from left to right decides who comes first; the spacing is
 * the tree's own. As the reader travels, the glass turns toward the side
 * the person travelled to came from, in perspective, and back (`tween3`,
 * through river.js's travel).
 *
 * People are named on small glass panels, as the reference's floating
 * screens; whom the drawing has no room for, the pointer names, or river.js
 * when every name is asked for. A branch carries what its parentage is
 * worth: an attested link a vein of light, an inferred one a broken vein, a
 * speculative one a trace.
 *
 * Two things the page knows by name:
 * - `window.axgfView` (river.js): where everyone stands (`shape`), how a
 *   branch bends (`edge`, which the lit route follows too), how a frame is
 *   painted at rest and in travel, which people the drawing named, the
 *   dashboard at rest, and how two frames blend (`tween`).
 * - `window.axgfRiverScene`: the life of the laboratory — motes turning
 *   round the glass, embers rising from the projector, a scanner's arc on
 *   each floor, and light drawn up every attested branch. It holds still as
 *   the river's ground does: reduced motion, "Cut", the pause, a hidden tab,
 *   a tree off screen, a machine too slow for it.
 *
 * No colour is named here: every one is a token of the theme (`--tv-*`,
 * tokens.css), read from the page or set by class in app.css.
 */
(function () {
  'use strict';
  var root = document.querySelector('.river');
  if (!root || root.dataset.view !== 'tree') return;
  var stage = document.getElementById('rv-canvas');

  /* ---- the glass ---- */
  // How much of a ring's depth shows (its height against its width), and
  // how much larger the front of the glass looks than its back.
  var TILT = 0.26, NEAR = 0.32;
  // The laboratory's picture, and where its projector is in it.
  var LAB = { src: '/static/img/fusion-lab.jpg?v=2', aspect: 1680 / 944, su: 0.5, sv: 0.8 };
  var GEO = null;
  function clamp(v, a, b) { return Math.max(a, Math.min(b, v)); }
  // A point on the glass's wall, on the page (the room's motes): its angle
  // round the axis (0 faces the reader), its floor (up is newer), its
  // distance from the axis. `z` is how near it is, -1 at the back to 1 at
  // the front.
  function project(G, th, lv, rho) {
    var z = rho * Math.cos(th);
    return { x: G.x0 + G.R * rho * Math.sin(th), y: G.y0 - lv * G.H + G.R * TILT * z, z: z, k: 1 + NEAR * z };
  }
  function centreOf(fr) { return fr.persons.filter(function (p) { return p.role === 'centre'; })[0]; }

  /* ---- where everyone stands: a tree in tiers ---- */
  // The family as a hierarchical tree (2026-10-23): a floor of the glass per
  // generation, straight across; descendants above the person at the fixed
  // point, each family's children side by side over it with the parents
  // centred under them; ancestors below, each person's father and mother
  // under them. The river's order from left to right decides who comes
  // first; the spacing is the tree's own, the same between every two people
  // of a side.
  function byKey(list) { var o = {}; list.forEach(function (p) { o[p.key] = p; }); return o; }
  function shape(fr) {
    var m = fr.meta, hl = m.hview[0], hw = m.hview[1], vt = m.view[0], vh = m.view[1];
    var C = centreOf(fr);
    var cx = C ? C.x : hl + hw / 2, cy = C ? C.y : m.cy, step = m.step || 86;
    function lev(y) { return -(y - cy) / step; }
    var PK = byKey(fr.persons), up = {}, under = {}, nd = 0, na = 0;
    fr.persons.forEach(function (p) { var g = Math.round(lev(p.y)); if (g > nd) nd = g; if (-g > na) na = -g; });
    // Which families lead up from a person (they are a parent in them, the
    // children above), and which leads down (they are a child in it).
    fr.couples.forEach(function (c) {
      c.parents.forEach(function (k) { if (PK[k] && lev(c.y) > lev(PK[k].y)) (up[k] = up[k] || []).push(c); });
      c.kids.forEach(function (k) { if (PK[k] && lev(c.y) < lev(PK[k].y)) under[k] = c; });
    });
    function rx(k) { return PK[k] ? PK[k].x : 0; }
    function shift(keys, X, d) { keys.forEach(function (k) { X[k] += d; }); }
    // The crown: a person, their partners beside them, and their children's
    // blocks side by side over them. Returns the block's width (in places)
    // and who is in it.
    var XD = {}, seenD = {};
    function crown(key, left) {
      seenD[key] = 1;
      var fams = (up[key] || []).slice().sort(function (a, b) { return a.x - b.x; }), partners = [], kids = [];
      fams.forEach(function (c) {
        c.parents.forEach(function (q) { if (q !== key && PK[q] && !seenD[q] && partners.indexOf(q) < 0) { partners.push(q); seenD[q] = 1; } });
        c.kids.slice().sort(function (a, b) { return rx(a) - rx(b); }).forEach(function (k) { if (PK[k] && !seenD[k] && kids.indexOf(k) < 0) kids.push(k); });
      });
      var members = [key].concat(partners), cursor = left;
      kids.forEach(function (k) { var b = crown(k, cursor); cursor += b.w; members = members.concat(b.members); });
      var kidsW = cursor - left, own = 1 + partners.length, w = Math.max(kidsW, own, 1);
      if (kidsW && kidsW < w) shift(members.slice(1 + partners.length), XD, (w - kidsW) / 2);
      var mid = kids.length ? (XD[kids[0]] + XD[kids[kids.length - 1]]) / 2 : left + w / 2;
      var row = [key].concat(partners).sort(function (a, b) { return rx(a) - rx(b); });
      row.forEach(function (k, i) { XD[k] = mid + (i - (row.length - 1) / 2); });
      return { w: w, members: members };
    }
    // The roots: a person, and their father's and mother's blocks side by
    // side under them.
    var XA = {}, seenA = {};
    function roots(key, left) {
      seenA[key] = 1;
      var c = under[key], parents = c ? c.parents.filter(function (q) { return PK[q] && !seenA[q]; }).sort(function (a, b) { return rx(a) - rx(b); }) : [];
      var members = [key], cursor = left;
      parents.forEach(function (q) { var b = roots(q, cursor); cursor += b.w; members = members.concat(b.members); });
      var pw = cursor - left, w = Math.max(pw, 1);
      if (pw && pw < w) shift(members.slice(1), XA, (w - pw) / 2);
      XA[key] = parents.length ? (XA[parents[0]] + XA[parents[parents.length - 1]]) / 2 : left + w / 2;
      return { w: w, members: members };
    }
    if (C) { crown(C.key, 0); roots(C.key, 0); }
    // Each side spread over the glass's width from the axis, its places as
    // wide as the side allows and never wider than a name needs.
    var R = clamp(Math.min(hw * 0.44, vh * 0.62), 120, 560);
    var span = nd + na + 0.9, H = Math.min(150, Math.max(24, (vh - 96 - 2 * R * TILT) / Math.max(1, span)));
    var y0 = vt + 48 + R * TILT + (nd + 0.45) * H;
    var G = { x0: hl + hw / 2, y0: y0, R: R, H: H, nd: nd, na: na };
    function unit(X) {
      if (!C || !(C.key in X)) return 0;
      var reach = 0;
      Object.keys(X).forEach(function (k) { reach = Math.max(reach, Math.abs(X[k] - X[C.key])); });
      return reach ? Math.min(120, R * 0.94 / reach) : 120;
    }
    var ud = unit(XD), ua = unit(XA), oD = C ? XD[C.key] : 0, oA = C ? XA[C.key] : 0;
    // Whoever neither tree reaches keeps the river's place, scaled to the glass.
    var reachR = 1;
    fr.persons.forEach(function (p) { reachR = Math.max(reachR, Math.abs(p.x - cx)); });
    var sr = R * 0.94 / reachR;
    var at = {};
    var persons = fr.persons.map(function (p) {
      var l = Math.round(lev(p.y)), x;
      if (p.key in XD && l >= 0) x = G.x0 + (XD[p.key] - oD) * ud;
      else if (p.key in XA) x = G.x0 + (XA[p.key] - oA) * ua;
      else x = G.x0 + (p.x - cx) * sr;
      var y = G.y0 - lev(p.y) * G.H;
      at[p.x + ',' + p.y] = [x, y];
      return Object.assign({}, p, { x: x, y: y, z: 0, k: 1, w: [x - G.x0, lev(p.y)] });
    });
    var NK = byKey(persons);
    var couples = fr.couples.map(function (c) {
      var ps = c.parents.map(function (k) { return NK[k]; }).filter(Boolean), ks = c.kids.map(function (k) { return NK[k]; }).filter(Boolean);
      var from = ps.length ? ps : ks, x = from.length ? from.reduce(function (s, q) { return s + q.x; }, 0) / from.length : G.x0 + (c.x - cx) * sr;
      var y = G.y0 - lev(c.y) * G.H, o = Object.assign({}, c, { x: x, y: y, z: 0, k: 1, w: [x - G.x0, lev(c.y)] });
      at[c.x + ',' + c.y] = [x, y];
      if (c.stub) {
        // Brothers and sisters not drawn: a bud leaning off the family
        // toward the children's floor, away from the axis.
        var side = x < G.x0 ? -1 : 1, vert = ks.length && ks[0].y < y ? -1 : 1;
        o.stub_dx = side * 18; o.stub_dy = vert * 14;
        o.stub_at = markAt(x + side * 18, y + vert * 14, side, vert);
      }
      return o;
    });
    // A line the record continues past the last floor runs on up (or down),
    // with its count beyond; a line it ends, a short one.
    var tails = fr.tails.map(function (t) {
      var s = at[t.x1 + ',' + t.y1] || [G.x0 + (t.x1 - cx) * sr, G.y0 - lev(t.y1) * G.H];
      var dir = t.y2 < t.y1 ? -1 : 1, len = (t.kind === 'cont' ? 0.42 : 0.24) * G.H;
      var e = [s[0], s[1] + dir * len];
      return Object.assign({}, t, { x1: s[0], y1: s[1], x2: e[0], y2: e[1], count_at: t.kind === 'cont' ? markAt(e[0], e[1], 0, dir) : null });
    });
    GEO = G;
    return { persons: persons, couples: couples, tails: tails, meta: Object.assign({}, m, { tv: G }) };
  }
  // Where a count stands at the end of a line: beside it, on its side.
  function markAt(x, y, side, vertical) {
    return { x: x + side * 6, y: y + (side ? 3.5 : vertical > 0 ? 13 : -6), anchor: side, halo: true };
  }
  // A branch climbs from one floor to the next: it leaves and reaches each
  // end upright, as a stem does.
  function edge(x1, y1, x2, y2) {
    var r = function (v) { return Math.round(v * 10) / 10; }, my = (y1 + y2) / 2;
    return 'M' + r(x1) + ' ' + r(y1) + 'C' + r(x1) + ' ' + r(my) + ' ' + r(x2) + ' ' + r(my) + ' ' + r(x2) + ' ' + r(y2);
  }
  // Two frames blended as the glass turns: the river's own blend, then the
  // whole tree turned round the axis toward the side the person travelled
  // to came from, and back — in perspective, so that the side coming round
  // grows nearer and the other recedes.
  function tween3(A, B, e, base) {
    var fr = base(A, B, e), G = A.meta.tv || B.meta.tv;
    if (!G) return fr;
    var to = byKey(A.persons)[B.meta.centre], dir = to ? (to.x < G.x0 - 2 ? -1 : to.x > G.x0 + 2 ? 1 : 0) : 0;
    if (!dir) return fr;
    var s = dir * 0.5 * Math.sin(Math.PI * e), D = 3 * G.R, cs = Math.cos(s), sn = Math.sin(s);
    function turn(x, y) {
      var r = x - G.x0, z = -r * sn, k = D / (D - z);
      return [G.x0 + r * cs * k, G.y0 + (y - G.y0) * (1 + (k - 1) * 0.35), z / G.R, k];
    }
    fr.persons.concat(fr.couples).forEach(function (o) { var q = turn(o.x, o.y); o.x = q[0]; o.y = q[1]; o.z = q[2]; o.k = q[3]; });
    fr.tails.forEach(function (t) { var a = turn(t.x1, t.y1), b = turn(t.x2, t.y2); t.x1 = a[0]; t.y1 = a[1]; t.x2 = b[0]; t.y2 = b[1]; });
    return fr;
  }

  /* ---- the drawing ---- */
  // A branch's width: the column's at the fixed point, narrowing a floor
  // further out each time, never under a hairline.
  function thick(gen) { return Math.max(1.3, 6.5 * Math.pow(0.68, Math.abs(gen))); }
  var named = {};
  function paint(fr, api, rest) {
    var f = api.f, esc = api.esc, PM = api.byKey(fr.persons), m = fr.meta;
    var vt = m.view[0], vh = m.view[1], hl = m.hview[0], hw = m.hview[1];
    var G = m.tv || GEO || { x0: hl + hw / 2, y0: vt + vh / 2, R: hw / 3, H: 90, nd: 0, na: 0 };
    GEO = G;
    var out = '', far = '', near = '', ringsBack = '', ringsFront = '', tw = '', marks = '';
    var top = G.y0 - (G.nd + 0.5) * G.H, bottom = G.y0 + (G.na + 0.5) * G.H;
    // The laboratory: its picture, the projector under the glass, and a
    // veil that keeps the drawing readable over it.
    var plat = bottom + G.R * TILT + 10;
    var Hi = Math.max(vh * 1.02, (plat - vt) / LAB.sv + 2, (vt + vh - plat) / (1 - LAB.sv) + 2, hw / LAB.aspect);
    var Wi = Hi * LAB.aspect;
    out += '<image class="tv-lab" href="' + LAB.src + '" x="' + f(G.x0 - LAB.su * Wi) + '" y="' + f(plat - LAB.sv * Hi) + '" width="' + f(Wi) + '" height="' + f(Hi) + '" preserveAspectRatio="none"/>';
    out += '<rect class="tv-veil" x="' + f(hl) + '" y="' + f(vt) + '" width="' + f(hw) + '" height="' + f(vh) + '"/>';
    // The glass: its walls, a ring per floor — the back half faint and
    // broken, the front half lit — and the column of light on the axis.
    var wallTop = top - 0.25 * G.H, wallBot = bottom + 0.15 * G.H;
    out += '<rect class="tv-glass" x="' + f(G.x0 - G.R) + '" y="' + f(wallTop) + '" width="' + f(2 * G.R) + '" height="' + f(wallBot - wallTop) + '" fill="url(#tv-glass)"/>';
    out += '<line class="tv-wall" x1="' + f(G.x0 - G.R) + '" y1="' + f(wallTop) + '" x2="' + f(G.x0 - G.R) + '" y2="' + f(wallBot) + '"/>'
      + '<line class="tv-wall" x1="' + f(G.x0 + G.R) + '" y1="' + f(wallTop) + '" x2="' + f(G.x0 + G.R) + '" y2="' + f(wallBot) + '"/>';
    function ring(cy, rx, cls) {
      var ry = rx * TILT;
      ringsBack += '<path class="' + cls + ' tv-ring-back" d="M' + f(G.x0 - rx) + ' ' + f(cy) + 'A' + f(rx) + ' ' + f(ry) + ' 0 0 1 ' + f(G.x0 + rx) + ' ' + f(cy) + '"/>';
      ringsFront += '<path class="' + cls + ' tv-ring-front" d="M' + f(G.x0 - rx) + ' ' + f(cy) + 'A' + f(rx) + ' ' + f(ry) + ' 0 0 0 ' + f(G.x0 + rx) + ' ' + f(cy) + '"/>';
    }
    ring(wallTop, G.R, 'tv-rim');
    ring(wallBot, G.R, 'tv-rim');
    var boxes = [];
    function hit(box) {
      for (var i = 0; i < boxes.length; i++) {
        var q = boxes[i];
        if (box[0] < q[2] && box[2] > q[0] && box[1] < q[3] && box[3] > q[1]) return true;
      }
      return false;
    }
    for (var l = -G.na; l <= G.nd; l++) {
      var ly = G.y0 - l * G.H;
      ring(ly, G.R * 0.98, l === 0 ? 'tv-floor tv-floor-0' : 'tv-floor');
      // Each floor's generation, at the glass's right, as the river's rail.
      var word = l > 0 ? '+' + l : l < 0 ? '−' + (-l) : '0 ◂', gx = G.x0 + G.R + 12;
      out += '<text x="' + f(gx) + '" y="' + f(ly + 3.5) + '" font-family="' + api.mono + '" font-size="10" class="' + (l === 0 ? 'k-accent' : 'k-band-text tv-gen') + '">' + word + '</text>';
      boxes.push([gx - 2, ly - 8, gx + 30, ly + 5]);
    }
    out += '<line class="tv-column-glow" x1="' + f(G.x0) + '" y1="' + f(wallTop) + '" x2="' + f(G.x0) + '" y2="' + f(plat) + '"/>'
      + '<line class="tv-column" x1="' + f(G.x0) + '" y1="' + f(wallTop) + '" x2="' + f(G.x0) + '" y2="' + f(plat) + '"/>';

    // Lines the record continues past the last floor, and lines it ends.
    var counts = [];
    fr.tails.forEach(function (t) {
      var dx = t.x2 - t.x1, dy = t.y2 - t.y1, len = Math.hypot(dx, dy) || 1, ux = dx / len, uy = dy / len;
      tw += '<g opacity="' + f(t.a) + '"><path d="M' + f(t.x1) + ' ' + f(t.y1) + 'L' + f(t.x2) + ' ' + f(t.y2) + '" class="tv-twig" stroke-width="1.4"/>';
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
      tw += '<g opacity="' + f(c.a) + '"><path d="M' + f(c.x) + ' ' + f(c.y) + 'L' + f(c.x + c.stub_dx) + ' ' + f(c.y + c.stub_dy) + '" class="tv-twig tv-bud" stroke-width="1.3"/></g>';
      if (c.stub_at) counts.push({ mk: c.stub_at, text: '+' + c.stub, size: 10, dx: c.sref ? c.x - c.sref[0] : 0, dy: c.sref ? c.y - c.sref[1] : 0, a: c.a });
    });

    // Branches, the far ones first and dimmed: a glow, the branch, and the
    // vein that says what the parentage is worth.
    var B = api.edges(fr, PM).map(function (e) {
      var a = e.from ? PM[e.from] : null, k = e.to ? PM[e.to] : null, c = e.cp || {};
      var z = ((a ? a.z : c.z || 0) + (k ? k.z : c.z || 0)) / 2, gen = Math.max(a ? Math.abs(a.gen) : 0, k ? Math.abs(k.gen) : 0);
      var w = thick(gen) * (0.75 + 0.25 * (1 + z) / 2);
      if (a && (a.role === 'spouse' || a.role === 'dspouse')) w *= 0.6;
      return { e: e, z: z, w: w };
    }).sort(function (a, b) { return a.z - b.z; });
    B.forEach(function (b) {
      var e = b.e, d = edge(e.x1, e.y1, e.x2, e.y2), back = b.z < -0.15;
      var o = (e.a < 1 || back) ? ' opacity="' + f((e.a < 1 ? e.a : 1) * (back ? 0.42 : 1)) + '"' : '';
      var faint = e.conf === 's' ? ' tv-faint' : '', s = '';
      s += '<path d="' + d + '" class="tv-branch-glow' + faint + '" stroke-width="' + f(b.w * 3.2) + '"' + o + '/>';
      s += '<path d="' + d + '" class="tv-branch' + faint + '" stroke-width="' + f(b.w) + '" stroke-linecap="round"' + o + '/>';
      if (e.conf === 'a') s += '<path d="' + d + '" class="tv-vein" stroke-width="1.1" stroke-linecap="round"' + o + '/>';
      else if (e.conf === 'd') s += '<path d="' + d + '" class="tv-vein tv-vein-d" stroke-width="1.1" stroke-dasharray="4.5 3"' + o + '/>';
      else s += '<path d="' + d + '" class="tv-vein tv-vein-s" stroke-width="2" stroke-linecap="round" stroke-dasharray="0.1 5"' + o + '/>';
      if (back) far += s; else near += s;
    });

    // Names on glass panels beside the near people, outward from the axis:
    // the centre first, then the nearest floors, the front before the back.
    // A panel that would cover another is left out, as the river leaves out
    // a word it has no room for; the pointer brings it back.
    var keep = {}, C = centreOf(fr), cs = C && C.s || {}, cside = 1;
    fr.persons.forEach(function (p) {
      var r = (p.role === 'centre' ? 13 : 6) * (p.k || 1);
      boxes.push([p.x - r, p.y - r, p.x + r, p.y + r]);
    });
    if (C && !cs.redacted) {
      var lefts = fr.persons.filter(function (p) { return p.role === 'spouse' && p.x < C.x - 1; }).length;
      var rights = fr.persons.filter(function (p) { return p.role === 'spouse' && p.x > C.x + 1; }).length;
      cside = rights && !lefts ? -1 : 1;
      var cn = api.clip(cs.names[2], 230), cw = Math.max(api.width(cn, 15, true), api.width(cs.years, 10.5, true)) + 20;
      // Beside the heart on the side its partners leave free, else on the
      // other, a little higher or lower if a person stands there.
      // The place that covers the fewest things already placed, the first
      // of them on a tie.
      // First as a caption under the heart, its leader straight down, which
      // leaves the sides to the partners; then beside it, then over it.
      var at = null, least = Infinity, tries = [[C.x - cw / 2, 42, 0], [C.x - cw / 2, 64, 0]];
      [26, 64, 110].forEach(function (gap) {
        [cside, -cside].forEach(function (sd) {
          [0, -26, 26, -52, 52].forEach(function (dy) { tries.push([sd > 0 ? C.x + gap : C.x - gap - cw, dy, sd]); });
        });
        tries.push([C.x - cw / 2, -40 - gap / 2, 0]);
      });
      tries.forEach(function (o) {
        var x0 = o[0], dy = o[1], box = [x0 - 2, C.y - 19 + dy, x0 + cw + 2, C.y + 19 + dy], n = 0;
        if (box[0] < hl + 2 || box[2] > hl + hw - 2 || box[1] < vt + 2 || box[3] > vt + vh - 2) return;
        boxes.forEach(function (q) { if (box[0] < q[2] && box[2] > q[0] && box[1] < q[3] && box[3] > q[1]) n++; });
        if (n < least) { least = n; at = { x: x0, dy: dy, side: o[2], box: box }; }
      });
      if (!at) at = { x: C.x + 26, dy: 0, side: 1, box: [C.x + 24, C.y - 19, C.x + 28 + cw, C.y + 19] };
      keep[C.key] = { x: at.x, y: C.y - 17 + at.dy, w: cw, h: 34, side: at.side, centre: true, word: cn, both: true };
      boxes.push(at.box);
    }
    fr.persons.slice().sort(function (a, b) { return Math.abs(a.gen) - Math.abs(b.gen) || b.z - a.z; }).forEach(function (p) {
      var s = p.s || {}, gd = Math.abs(p.gen);
      if (p.role === 'centre' || s.redacted || p.z < -0.2 || gd > 3) return;
      var side = p.x < G.x0 - 3 ? -1 : p.x > G.x0 + 3 ? 1 : (p.w && p.w[0] < 0 ? -1 : 1);
      var word = gd >= 3 ? s.names[0] : s.names[1], both = gd <= 1 && !!s.years;
      var w = Math.max(api.width(word, 11.5, false), both ? api.width(s.years, 9, true) : 0) + 14, h = both ? 28 : 17;
      var gap = 10 + 5 * (p.k || 1);
      for (var tries = 0; tries < 3; tries++) {
        var dy = tries === 0 ? 0 : tries === 1 ? -(h + 4) : h + 4;
        var x0 = side > 0 ? p.x + gap : p.x - gap - w, y0 = p.y - h / 2 + dy, box = [x0 - 2, y0 - 2, x0 + w + 2, y0 + h + 2];
        if (box[0] < hl + 2 || box[2] > hl + hw - 2 || hit(box)) continue;
        boxes.push(box);
        keep[p.key] = { x: x0, y: y0, w: w, h: h, side: side, word: word, both: both };
        return;
      }
    });
    // The counts: never left out — the only sign of the people beyond —
    // but stepped aside from a panel rather than written over it.
    counts.forEach(function (c) {
      var mk = c.mk, w = api.width(c.text, c.size, true), at = [0, 0], spot = null;
      [[0, 0], [0, 11], [0, -11], [14, 0], [-14, 0], [0, 22], [0, -22], [28, 0], [-28, 0], [0, 34], [0, -34], [42, 0], [-42, 0]].some(function (o) {
        var x = mk.x + c.dx + o[0], y = mk.y + c.dy + o[1];
        var x0 = mk.anchor < 0 ? x - w : mk.anchor === 0 ? x - w / 2 : x, box = [x0 - 1, y - c.size, x0 + w + 1, y + 2];
        if (!spot) spot = box;
        if (hit(box)) return false;
        at = o; spot = box;
        return true;
      });
      boxes.push(spot);
      marks += '<g class="tv-count" opacity="' + f(c.a) + '">' + api.mark(Object.assign({}, mk, { x: mk.x + at[0], y: mk.y + at[1] }), c.text, c.size, c.dx, c.dy) + '</g>';
    });

    // People, far to near: the heart on the axis, leaves alive and leaves
    // gone, the person this reader may not see as an outline only; each
    // with its panel, if it has one.
    var people = '';
    fr.persons.slice().sort(function (a, b) { return a.z - b.z; }).forEach(function (p) {
      var s = p.s || {}, isC = p.role === 'centre', k = p.k || 1, r = (isC ? 8 : 4.6) * k, x = f(p.x), y = f(p.y);
      var fade = p.z < -0.15 ? 0.5 : 1, g = '<g opacity="' + f(p.a * fade) + '">';
      if (rest) g += '<a class="rv-p" data-id="' + esc(p.id) + '" data-key="' + esc(p.key) + '" href="' + api.base + '?p=' + encodeURIComponent(p.id) + '&amp;n=' + m.n + '" aria-label="' + esc(api.said(p)) + '"><circle cx="' + x + '" cy="' + y + '" r="18" fill="transparent"/>';
      if (p.repeat > 1) g += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 2.6) + '" fill="none" class="k-data-line" stroke-width="1"/>';
      if (p.again) g += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 2.6 + (p.repeat > 1 ? 2.2 : 0)) + '" fill="none" class="k-data-line" stroke-width="1" stroke-dasharray="2.4 1.8"/>';
      if (isC) {
        g += '<circle cx="' + x + '" cy="' + y + '" r="24" class="tv-halo"/><circle cx="' + x + '" cy="' + y + '" r="15" fill="none" class="tv-ring" stroke-width="1.4"/>'
          + '<circle cx="' + x + '" cy="' + y + '" r="' + f(r) + '" class="tv-heart"/>';
      } else if (s.redacted || s.sparse) {
        g += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r) + '" class="tv-leaf tv-hidden" stroke-width="1.5" stroke-dasharray="1.6 1.6"/>';
      } else if (s.living) {
        g += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 5) + '" class="tv-leaf-glow tv-living"/><circle cx="' + x + '" cy="' + y + '" r="' + f(r) + '" class="tv-leaf tv-living"/>';
      } else {
        g += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 4) + '" class="tv-leaf-glow tv-gone"/><circle cx="' + x + '" cy="' + y + '" r="' + f(r - 0.3) + '" class="tv-leaf tv-gone"/>';
      }
      var L = keep[p.key];
      if (L && !s.redacted) {
        var ex = L.side > 0 ? L.x : L.side < 0 ? L.x + L.w : L.x + L.w / 2, ey = L.side ? L.y + L.h / 2 : L.y > p.y ? L.y : L.y + L.h;
        var sx = p.x + L.side * (r + 2), sy = L.side ? p.y : p.y + (L.y > p.y ? r + 2 : -r - 2);
        g += '<line class="tv-leader" x1="' + f(sx) + '" y1="' + f(sy) + '" x2="' + f(ex) + '" y2="' + f(ey) + '"/>';
        g += '<rect class="tv-panel' + (L.centre ? ' tv-panel-centre' : '') + '" x="' + f(L.x) + '" y="' + f(L.y) + '" width="' + f(L.w) + '" height="' + f(L.h) + '" rx="4"/>';
        if (L.centre) {
          g += '<text x="' + f(L.x + 10) + '" y="' + f(L.y + 15) + '" font-size="15" font-weight="600" class="tv-name tv-centre">' + esc(L.word) + '</text>'
            + '<text x="' + f(L.x + 10) + '" y="' + f(L.y + 28) + '" font-family="' + api.mono + '" font-size="10.5" class="tv-data">' + esc(s.years) + '</text>';
        } else {
          g += '<text x="' + f(L.x + 7) + '" y="' + f(L.y + 12.5) + '" font-size="11.5" class="tv-name">' + esc(L.word) + '</text>';
          if (L.both) g += '<text x="' + f(L.x + 7) + '" y="' + f(L.y + 23.5) + '" font-family="' + api.mono + '" font-size="9" class="tv-data">' + esc(s.years) + '</text>';
        }
      }
      if (rest) g += '</a>';
      people += g + '</g>';
    });
    if (rest) {
      named = {};
      Object.keys(keep).forEach(function (k) { if (!keep[k].centre) named[k] = keep[k]; });
    }
    var defs = '<linearGradient id="tv-glass" x1="0" x2="1" y1="0" y2="0"><stop offset="0" class="tv-glass-edge"/><stop offset="0.5" class="tv-glass-mid"/><stop offset="1" class="tv-glass-edge"/></linearGradient>';
    var head = '<svg class="rv-svg tv-svg" data-view="tree" viewBox="' + f(hl) + ' ' + f(vt) + ' ' + f(hw) + ' ' + f(vh) + '" width="100%" '
      + (rest ? 'role="group" aria-label="' + esc(label() + (C && C.s && !C.s.redacted ? ' — ' + C.s.names[2] : '')) + '"' : 'aria-hidden="true"')
      + ' xmlns="http://www.w3.org/2000/svg"><defs>' + defs + '</defs>';
    // The lit route lies over the branches and under the people (river.js).
    return head + out + ringsBack + far + ringsFront + tw + near + '<g class="rv-lit"></g>' + marks + people + '</svg>';
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
    // People per generation, oldest at the left: the roots' bars, the
    // fixed point's, the crown's.
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
  window.axgfView.tween = tween3;

  /* ---- the life of the laboratory ---- */
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
  var SAP = rgb('--tv-vein', [255, 255, 255]), SAP2 = rgb('--tv-vein-soft', SAP), COLD = rgb('--tv-glass', rgb('--tv-teal', SAP));
  function rgba(c, a) { return 'rgba(' + c[0] + ',' + c[1] + ',' + c[2] + ',' + a.toFixed(3) + ')'; }

  // Under the drawing, the room's motes, embers and scanners; over it, the
  // light up the branches.
  var ground = document.createElement('canvas');
  ground.className = 'rv-ground';
  ground.setAttribute('aria-hidden', 'true');
  stage.insertBefore(ground, stage.firstChild);
  var layer = document.createElement('canvas');
  layer.className = 'rv-water';
  layer.setAttribute('aria-hidden', 'true');
  stage.appendChild(layer);
  stage.classList.add('rv-alive');
  var gx = ground.getContext('2d'), ctx = layer.getContext('2d');
  if (!ctx || !gx) return;
  // The motes, the same on every visit: an angle round the glass, a floor,
  // a speed and a size.
  var MOTES = [];
  for (var mi = 0; mi < 54; mi++) {
    var h1 = Math.abs(Math.sin(mi * 12.9898) * 43758.5453) % 1, h2 = Math.abs(Math.sin(mi * 78.233) * 12345.678) % 1;
    MOTES.push({ th: h1 * 6.283, v: h2, speed: 0.12 + 0.2 * ((h1 * 7) % 1), rho: 0.96 + 0.1 * ((h2 * 3) % 1), r: 0.8 + 1.3 * ((h1 * 5) % 1), hot: mi % 3 === 0 });
  }

  var flows = [], dpr = 1, cw = 0, ch = 0, clock = 0, travelling = false;
  function size() {
    var w = stage.clientWidth, h = stage.clientHeight, r = Math.min(window.devicePixelRatio || 1, 2);
    if (w === cw && h === ch && r === dpr) return false;
    cw = w; ch = h; dpr = r;
    layer.width = ground.width = Math.max(1, Math.round(w * dpr));
    layer.height = ground.height = Math.max(1, Math.round(h * dpr));
    return true;
  }
  function view() {
    var svg = stage.querySelector('svg.rv-svg'), vb = svg && svg.viewBox && svg.viewBox.baseVal;
    if (!vb || !vb.width || !cw) return null;
    var k = Math.min(cw / vb.width, ch / vb.height);
    return { x: vb.x - (cw / k - vb.width) / 2, y: vb.y - (ch / k - vb.height) / 2, k: k };
  }
  // Light is drawn up every attested branch, from its lower end to its
  // upper, as sap rises: roots to column to crown.
  function layout(L) {
    flows = [];
    if (!L || !L.edges) return;
    L.edges.forEach(function (e) {
      if (e.conf !== 'a') return;
      var a = e.y1 >= e.y2 ? [e.x1, e.y1, e.x2, e.y2] : [e.x2, e.y2, e.x1, e.y1], my = (a[1] + a[3]) / 2;
      var c = [a[0], a[1], a[0], my, a[2], my, a[2], a[3]], n = 22, xs = [], ys = [], cum = [0];
      for (var i = 0; i <= n; i++) {
        var t = i / n, u = 1 - t;
        xs.push(u * u * u * c[0] + 3 * u * u * t * c[2] + 3 * u * t * t * c[4] + t * t * t * c[6]);
        ys.push(u * u * u * c[1] + 3 * u * u * t * c[3] + 3 * u * t * t * c[5] + t * t * t * c[7]);
        if (i) cum.push(cum[i - 1] + Math.hypot(xs[i] - xs[i - 1], ys[i] - ys[i - 1]));
      }
      if (cum[n] < 10) return;
      flows.push({ x: xs, y: ys, cum: cum, n: n, len: cum[n], k: Math.max(1, Math.round(cum[n] / 90)), phase: Math.abs(Math.sin(e.x1 * 12.9898 + e.y2 * 78.233)) % 1 });
    });
    request();
  }
  var SPEED = 30;
  function room(moving) {
    var W = ground.width, Hh = ground.height, v = view(), G = GEO, t = moving ? clock : 3.1;
    gx.setTransform(1, 0, 0, 1, 0, 0);
    gx.clearRect(0, 0, W, Hh);
    if (!v || !G) return;
    var s = v.k * dpr, ox = -v.x * s, oy = -v.y * s;
    function P(th, lv, rho) { var q = project(G, th, lv, rho); return [q.x * s + ox, q.y * s + oy, q.z]; }
    var top = G.nd + 0.75, bot = -G.na - 0.65;
    gx.globalCompositeOperation = 'lighter';
    // Motes turning round the glass, brighter at the front.
    MOTES.forEach(function (mo) {
      var lv = bot + (top - bot) * ((mo.v + t * 0.012) % 1), q = P(mo.th + t * mo.speed, lv, mo.rho), a = 0.18 + 0.42 * (q[2] + 1) / 2;
      gx.fillStyle = rgba(mo.hot ? SAP2 : COLD, a);
      gx.beginPath(); gx.arc(q[0], q[1], mo.r * dpr * (1 + 0.3 * q[2]), 0, 6.283); gx.fill();
    });
    // A scanner's arc going round each floor's ring.
    for (var l = -G.na; l <= G.nd; l++) {
      var cy = (G.y0 - l * G.H) * s + oy, rx = G.R * 0.98 * s, ry = rx * TILT, a0 = t * 0.7 + l * 1.3;
      gx.strokeStyle = rgba(l > 0 ? SAP : COLD, 0.32);
      gx.lineWidth = 1.6 * dpr;
      gx.beginPath(); gx.ellipse(G.x0 * s + ox, cy, rx, ry, 0, a0, a0 + 0.9); gx.stroke();
    }
    // Embers rising from the projector up the column.
    var plat = (G.y0 + (G.na + 0.5) * G.H + G.R * TILT + 10) * s + oy, span = plat - ((G.y0 - (G.nd + 0.75) * G.H) * s + oy);
    for (var e = 0; e < 26; e++) {
      var ph = ((e * 0.618 + t * (0.05 + (e % 5) * 0.012)) % 1), x = G.x0 * s + ox + Math.sin(e * 3.7 + t * 0.8) * G.R * 0.12 * s * (1 - ph);
      gx.fillStyle = rgba(e % 4 ? SAP : SAP2, 0.5 * (1 - ph));
      gx.beginPath(); gx.arc(x, plat - span * ph, (1.4 + (e % 3) * 0.6) * dpr, 0, 6.283); gx.fill();
    }
    gx.globalCompositeOperation = 'source-over';
  }
  function draw(moving) {
    size();
    room(moving);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, layer.width, layer.height);
    var v = view();
    if (!v || !moving || travelling) return;
    var s = v.k * dpr, ox = -v.x * s, oy = -v.y * s;
    ctx.globalCompositeOperation = 'lighter';
    flows.forEach(function (fl) {
      for (var j = 0; j < fl.k; j++) {
        var pos = ((fl.phase + j / fl.k) * fl.len + clock * SPEED) % fl.len, i = 1;
        while (i < fl.n && fl.cum[i] < pos) i++;
        var u = (pos - fl.cum[i - 1]) / ((fl.cum[i] - fl.cum[i - 1]) || 1);
        var x = (fl.x[i - 1] + (fl.x[i] - fl.x[i - 1]) * u) * s + ox, y = (fl.y[i - 1] + (fl.y[i] - fl.y[i - 1]) * u) * s + oy;
        var fade = Math.min(1, pos / 18, (fl.len - pos) / 18), r = 3 * v.k * dpr;
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
  if ('ResizeObserver' in window) new ResizeObserver(function () { if (size()) request(); if (!running) draw(false); }).observe(stage);
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
    travel: function () { travelling = true; if (!running) draw(false); },
    view: function () { if (!running) draw(false); }
  };
  size();
  draw(false);
})();
