/* Genealogy's tree, climbed (v0.5.0, docs/adr/0016; redrawn 2026-10-23).
 *
 * After the operator's frame pack: an old tree, its trunk twisted, its crown
 * in gold leaves, growing out of a parchment scroll, and the family's
 * portraits on the parchment joined to it by fine lines. The tree is a
 * picture (static/img/genealogy-tree.jpg, made in the operator's Canva in
 * the spirit of their reference, not from it), drawn in relief: a depth map
 * of it (genealogy-tree-depth.jpg) moves its near parts more than its far
 * ones as the camera moves, the crown and the roots standing off the
 * parchment and the parchment off the wall.
 *
 * The river's own people — from the same payload, through the same reader's
 * lens, in the same order from left to right — are set on the tree: the
 * person at the fixed point on its trunk, the descendants up in the crown,
 * the ancestors down among the roots, a generation a level, each further
 * one closer to the last, and each level's people spread round the trunk in
 * the river's order, the nearest at the front and larger. Each is a sepia
 * portrait in a cream mount — their initials, since the payload carries no
 * picture — with a cream card that names them; whom the drawing has no room
 * for, the pointer names, or river.js when every name is asked for. A
 * parentage is a fine line, square-cornered as on the reference, dark ink
 * edged in the parchment's light so that it reads on the leaves and the
 * paper alike: whole for an attested one, broken for an inferred one, a
 * line of knots for a speculative one.
 *
 * Travelling up to a descendant, the reader climbs: the camera rises into
 * the crown and the tree slides down past it, the person travelled to
 * coming down to the trunk; to an ancestor, the camera goes down toward the
 * roots. The blend is done in the tree's own space (`tween3`, through
 * river.js's travel), and the picture moves with the camera, in relief.
 *
 * Two things the page knows by name:
 * - `window.axgfView` (river.js): where everyone stands (`shape`), how a
 *   line runs (`edge`, which the lit route follows too), how a frame is
 *   painted at rest and in travel, which people the drawing named, the
 *   dashboard at rest, and how two frames blend (`tween`).
 * - `window.axgfRiverScene`: under the drawing, the picture in relief, with
 *   gold leaves falling from the crown and a little gold dust; over it, a
 *   glint run along every attested line. It holds still as the river's
 *   ground does: reduced motion, "Cut", the pause, a hidden tab, a tree off
 *   screen, a machine too slow for it. Still, the picture still stands in
 *   its relief; the pointer tilts it a little, unless motion is reduced.
 *
 * No colour is named here: every one is a token of the theme (`--gn-*`,
 * tokens.css), read from the page or set by class in app.css.
 */
(function () {
  'use strict';
  var root = document.querySelector('.river');
  if (!root || root.dataset.view !== 'scroll') return;
  var stage = document.getElementById('rv-canvas');

  /* ---- the tree, on its picture ---- */
  // The picture: its aspect; where its trunk stands (u, a share of its
  // width) and where on it the fixed point is set (v, a share of its
  // height); how far up the crown reaches from there and down the roots
  // (shares of its height); how quickly further generations close in
  // (ku up, kd down). Read from the picture and its depth map. And the
  // climb: how far up the picture (or down) the camera goes, at most, as the
  // reader travels up the generations (or down), and how many generations
  // take it half of that way (pace, roughly).
  // ZOOM (2026-10-27, the operator's review): the picture's height in
  // windows, large enough that the window shows the trunk at rest, the
  // leaves as the reader climbs and the roots as they go down.
  var PIC = { src: '/static/img/genealogy-tree.jpg?v=1', depth: '/static/img/genealogy-tree-depth.jpg?v=1', aspect: 1088 / 1456, u: 0.53, v: 0.57, up: 0.58, down: 0.44, ku: 2.6, kd: 2.2, climbUp: 0.36, climbDown: 0.3, pace: 2.2, zoom: 3.6 };
  // SHRINK: how much smaller a generation further away is. THETA: how far
  // round the trunk a generation's people reach, either side of the front.
  // DEPTH: how much smaller someone round the side is than someone at the
  // front.
  var SHRINK = 0.05, THETA = 1.25, DEPTH = 0.22;
  var GEO = null, world = 0, pending = 0;
  function clamp(v, a, b) { return Math.max(a, Math.min(b, v)); }
  // A level's height above the fixed point, as a share of the picture: up
  // into the crown, down into the roots, each further level closer to the
  // last, so that any number of them stays on the tree.
  function lift(l) { return l >= 0 ? PIC.up * (1 - Math.exp(-l / PIC.ku)) : -PIC.down * (1 - Math.exp(l / PIC.kd)); }
  // Round the trunk, but spaced nearly evenly: a pure sine bunched the
  // people at the sides together (the operator's review of 2026-10-27).
  function across(th) { return 0.3 * Math.sin(th) + 0.7 * Math.sin(THETA) * th / THETA; }
  function proj(G, th, l) {
    var z = Math.cos(th), kl = Math.exp(-SHRINK * Math.abs(l));
    return { x: G.x0 + G.R * across(th) * kl, y: G.y0 - lift(l) * G.H, z: z, k: kl * (1 - DEPTH * (1 - z)) };
  }
  function centreOf(fr) { return fr.persons.filter(function (p) { return p.role === 'centre'; })[0]; }
  function byKey(list) { var o = {}; list.forEach(function (p) { o[p.key] = p; }); return o; }

  /* ---- where everyone stands: a family tree in levels ---- */
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
    // The family's levels over a little more than the window's height, the
    // fixed point a little under its middle. The picture much larger (PH,
    // PW): nearly the window's width, or well over its height on a narrow
    // one, so that the window shows a part of the tree, and the camera
    // climbs it (scene, picture()). Round the trunk, the widest side
    // reaches THETA either way, a place never more than a quarter turn
    // wide.
    var H = vh * 1.06, PH = Math.max(hw * 2.2 / PIC.aspect, vh * PIC.zoom), PW = PH * PIC.aspect;
    var R = clamp(hw * 0.47, 160, 640);
    var G = { x0: hl + hw / 2, y0: vt + (vh - H) / 2 + PIC.v * H, R: R, H: H, PH: PH, PW: PW, nd: nd, na: na };
    function per(X) {
      if (!C || !(C.key in X)) return 0.4;
      var reach = 0;
      Object.keys(X).forEach(function (k) { reach = Math.max(reach, Math.abs(X[k] - X[C.key])); });
      return reach ? Math.min(0.4, THETA / reach) : 0.4;
    }
    var uD = per(XD), uA = per(XA), oD = C ? XD[C.key] : 0, oA = C ? XA[C.key] : 0;
    // Whoever neither tree reaches keeps the river's place, round the trunk.
    var reachR = 1;
    fr.persons.forEach(function (p) { reachR = Math.max(reachR, Math.abs(p.x - cx)); });
    var at = {};
    var persons = fr.persons.map(function (p) {
      var l = lev(p.y), th;
      if (p.key in XD && Math.round(l) >= 0) th = (XD[p.key] - oD) * uD;
      else if (p.key in XA) th = (XA[p.key] - oA) * uA;
      else th = (p.x - cx) / reachR * THETA;
      return { p: p, th: th, l: l };
    });
    // A crowded level hangs its portraits in quincunx, every other one a
    // little higher, so that each has room for its card beside it (the
    // operator's review of 2026-10-27: the frames stood too close).
    var rows = {};
    persons.forEach(function (o) { var r = Math.round(o.l); if (r) (rows[r] = rows[r] || []).push(o); });
    Object.keys(rows).forEach(function (r) {
      var row = rows[r].sort(function (a, b) { return a.th - b.th; });
      if (row.length < 5) return;
      row.forEach(function (o, i) { o.l += (i % 2 ? 0.24 : -0.24) * (+r > 0 ? 1 : -1); });
    });
    persons = persons.map(function (o) {
      var p = o.p, q = proj(G, o.th, o.l);
      at[p.x + ',' + p.y] = [o.th, o.l];
      return Object.assign({}, p, { x: q.x, y: q.y, z: q.z, k: q.k, th: o.th, lv: o.l });
    });
    var NK = byKey(persons);
    function mean(list) { return list.reduce(function (s, q) { return s + q.th; }, 0) / list.length; }
    var couples = fr.couples.map(function (c) {
      var ps = c.parents.map(function (k) { return NK[k]; }).filter(Boolean), ks = c.kids.map(function (k) { return NK[k]; }).filter(Boolean);
      var from = ps.length ? ps : ks, th = from.length ? mean(from) : (c.x - cx) / reachR * THETA;
      var l = lev(c.y);
      // Where a family's wood parts. Above someone, on their own wood, the
      // partner's twig joining it there — the fixed point's families at the
      // top of the trunk, where the limbs leave it. Below, where the root
      // parts toward the two parents, under the child.
      var blood = ps.filter(function (q) { return q.role === 'centre' || q.role === 'desc'; })[0];
      if (l > 0 && blood) {
        var others = ps.filter(function (q) { return q !== blood; }), mo = others.length ? mean(others) : blood.th;
        th = blood.role === 'centre' ? (blood.th + mo) / 2 * 0.15 : blood.th + (mo - blood.th) * 0.2;
      } else if (l < 0 && ks.length && ps.length) th = (mean(ks) + mean(ps)) / 2;
      var q = proj(G, th, l);
      at[c.x + ',' + c.y] = [th, l];
      var o = Object.assign({}, c, { x: q.x, y: q.y, z: q.z, k: q.k, th: th, lv: l });
      if (c.stub) {
        // Brothers and sisters not drawn: a twig off the family toward the
        // children's level, away from the trunk.
        o.stub_side = th < 0 ? -1 : 1; o.stub_up = ks.length && ks[0].lv > l ? 1 : -1;
      }
      return o;
    });
    // A line the record continues past the last level runs on along the
    // tree, with its count beyond; a line it ends, a short one.
    var tails = fr.tails.map(function (t) {
      var s = at[t.x1 + ',' + t.y1] || [(t.x1 - cx) / reachR * THETA, lev(t.y1)];
      var dir = t.y2 < t.y1 ? 1 : -1, len = t.kind === 'cont' ? 0.42 : 0.24;
      return place(Object.assign({}, t, { th1: s[0], lv1: s[1], th2: s[0], lv2: s[1] + dir * len }), G);
    });
    GEO = G;
    return { persons: persons, couples: couples, tails: tails, meta: Object.assign({}, m, { gn: G }) };
  }
  // A tail's ends, and its count, where the tree puts them.
  function place(t, G) {
    var a = proj(G, t.th1, t.lv1), b = proj(G, t.th2, t.lv2), up = t.lv2 > t.lv1;
    t.x1 = a.x; t.y1 = a.y; t.x2 = b.x; t.y2 = b.y; t.k = (a.k + b.k) / 2;
    t.count_at = t.kind === 'cont' ? { x: b.x, y: b.y + (up ? -6 : 13), anchor: 0, halo: true } : null;
    return t;
  }
  // A line runs as on the reference: straight down (or up) from one end,
  // across, and straight to the other, its corners a little rounded. The
  // points it turns at, for the glints.
  function wire(x1, y1, x2, y2) {
    var my = (y1 + y2) / 2;
    return [[x1, y1], [x1, my], [x2, my], [x2, y2]];
  }
  function edge(x1, y1, x2, y2) {
    var r = function (v) { return Math.round(v * 10) / 10; }, my = (y1 + y2) / 2, dx = x2 - x1, dy = y2 - y1;
    if (Math.abs(dx) < 0.6) return 'M' + r(x1) + ' ' + r(y1) + 'L' + r(x2) + ' ' + r(y2);
    var c = Math.min(6, Math.abs(dx) / 2, Math.abs(dy) / 4), sx = dx < 0 ? -1 : 1, sy = dy < 0 ? -1 : 1;
    return 'M' + r(x1) + ' ' + r(y1) + 'L' + r(x1) + ' ' + r(my - sy * c) + 'Q' + r(x1) + ' ' + r(my) + ' ' + r(x1 + sx * c) + ' ' + r(my)
      + 'L' + r(x2 - sx * c) + ' ' + r(my) + 'Q' + r(x2) + ' ' + r(my) + ' ' + r(x2) + ' ' + r(my + sy * c) + 'L' + r(x2) + ' ' + r(y2);
  }
  // Two frames blended as the reader climbs: each person's place round the
  // trunk and height on it blended in the tree's own space, the camera
  // rising (or going down) from the old fixed point to the new, and every
  // place drawn from where the camera is at that moment.
  function tween3(A, B, e, base) {
    var fr = base(A, B, e), G = B.meta.gn || A.meta.gn;
    if (!G) return fr;
    var PA = byKey(A.persons), PB = byKey(B.persons), to = PA[B.meta.centre];
    var lc = to ? to.lv : 0, cam = lc * e;
    function mix(a, b) { return a + (b - a) * e; }
    function at(a, b) {
      // A's heights are measured from A's fixed point, B's from B's: in A's
      // terms, B's are higher by lc.
      var th = a && b ? mix(a.th, b.th) : (a || b).th;
      var w = a && b ? mix(a.lv, b.lv + lc) : a ? a.lv : b.lv + lc;
      return proj(G, th, w - cam);
    }
    fr.persons.forEach(function (o) {
      var q = at(PA[o.key], PB[o.key]);
      o.x = q.x; o.y = q.y; o.z = q.z; o.k = q.k;
    });
    var CA = byKey(A.couples), CB = byKey(B.couples);
    fr.couples.forEach(function (o) {
      var a = CA[o.key], b = CB[o.key];
      if (!a && !b) return;
      var q = at(a, b);
      o.x = q.x; o.y = q.y; o.z = q.z; o.k = q.k;
    });
    var TA = byKey(A.tails), TB = byKey(B.tails);
    fr.tails.forEach(function (o) {
      var a = TA[o.key], b = TB[o.key], s = a || b;
      if (!s) return;
      var w1 = a && b ? mix(a.lv1, b.lv1 + lc) : a ? a.lv1 : b.lv1 + lc, w2 = a && b ? mix(a.lv2, b.lv2 + lc) : a ? a.lv2 : b.lv2 + lc;
      place(Object.assign(o, { th1: s.th1, th2: s.th2, lv1: w1 - cam, lv2: w2 - cam }), G);
    });
    fr.meta = Object.assign({}, fr.meta, { gn: Object.assign({}, G, { world: world + cam, e: e, lc: lc }) });
    pending = lc;
    return fr;
  }

  /* ---- the drawing ---- */
  // A portrait's mount: the heart's the largest, then smaller with distance
  // along the trunk and round it. Half its width and half its height.
  function frameOf(p) {
    var g = Math.abs(p.gen), base = p.role === 'centre' ? 25 : g <= 1 ? 17 : g === 2 ? 14 : 12;
    var rx = Math.max(7, base * (p.k || 1));
    return { rx: rx, ry: rx * 1.22 };
  }
  // Initials for the portrait: the first letter of the first and of the
  // last word of the name.
  function initials(name) {
    var w = String(name || '').trim().split(/\s+/).filter(Boolean);
    if (!w.length) return '';
    var a = Array.from(w[0])[0] || '', b = w.length > 1 ? Array.from(w[w.length - 1])[0] || '' : '';
    return (a + b).toUpperCase();
  }
  var named = {};
  function paint(fr, api, rest) {
    var f = api.f, esc = api.esc, PM = api.byKey(fr.persons), m = fr.meta;
    var vt = m.view[0], vh = m.view[1], hl = m.hview[0], hw = m.hview[1];
    if (rest && pending) { world += pending; pending = 0; }
    var G = m.gn || GEO || { x0: hl + hw / 2, y0: vt + vh * 0.57, R: hw / 3, H: vh, PH: vh * 1.7, PW: vh * 1.7 * PIC.aspect, nd: 0, na: 0 };
    GEO = Object.assign({}, G, { world: G.world !== undefined ? G.world : world, e: rest ? 0 : G.e || 0, lc: rest ? 0 : G.lc || 0 });
    var C = PM[m.centre] || centreOf(fr);
    var boxes = [];
    function hit(box) {
      for (var i = 0; i < boxes.length; i++) {
        var q = boxes[i];
        if (box[0] < q[2] && box[2] > q[0] && box[1] < q[3] && box[3] > q[1]) return true;
      }
      return false;
    }
    // The generations' numbers at the right, as the river's rail.
    var rail = '';
    for (var l = -G.na; l <= G.nd; l++) {
      var ly = G.y0 - lift(l) * G.H, word = l > 0 ? '+' + l : l < 0 ? '−' + (-l) : '0 ◂', gx = hl + hw - 44;
      rail += '<text x="' + f(gx) + '" y="' + f(ly + 3.5) + '" font-family="' + api.mono + '" font-size="10" paint-order="stroke" class="gn-gen' + (l === 0 ? ' gn-gen-0' : '') + '">' + word + '</text>';
      boxes.push([gx - 2, ly - 8, gx + 34, ly + 5]);
    }

    // The lines: the parchment's light under each, then its ink, as sure as
    // the parentage is known; each family a knot of gold where its lines
    // meet.
    var halo = '', ink = '', knots = '';
    api.edges(fr, PM).forEach(function (e) {
      var d = edge(e.x1, e.y1, e.x2, e.y2), o = e.a < 1 ? ' opacity="' + f(e.a) + '"' : '';
      halo += '<path d="' + d + '" class="gn-wire-halo"' + o + '/>';
      ink += '<path d="' + d + '" class="gn-wire' + (e.conf === 'd' ? ' gn-wire-d' : e.conf === 's' ? ' gn-wire-s' : '') + '"' + o + '/>';
    });
    fr.couples.forEach(function (c) {
      if (c.parents.length + c.kids.length > 1) knots += '<circle cx="' + f(c.x) + '" cy="' + f(c.y) + '" r="' + f(Math.max(2.2, 3.4 * (c.k || 1))) + '" class="gn-node" opacity="' + f(c.a) + '"/>';
    });
    // Lines the record continues past the last level, and lines it ends.
    var counts = [], tw = '';
    fr.tails.forEach(function (t) {
      var dx = t.x2 - t.x1, dy = t.y2 - t.y1, len = Math.hypot(dx, dy) || 1, ux = dx / len, uy = dy / len;
      var d = 'M' + f(t.x1) + ' ' + f(t.y1) + 'L' + f(t.x2) + ' ' + f(t.y2);
      tw += '<g opacity="' + f(t.a) + '"><path d="' + d + '" class="gn-wire-halo"/><path d="' + d + '" class="gn-wire"/>';
      if (t.kind === 'cont') {
        tw += '<polygon class="gn-tip" points="' + f(t.x2 - uy * 3.6) + ',' + f(t.y2 + ux * 3.6) + ' ' + f(t.x2 + uy * 3.6) + ',' + f(t.y2 - ux * 3.6) + ' ' + f(t.x2 + ux * 5.5) + ',' + f(t.y2 + uy * 5.5) + '"/>';
        if (t.count_at) counts.push({ mk: t.count_at, text: '+' + t.count, size: 9.5, dx: t.cref ? t.x2 - t.cref[0] : 0, dy: t.cref ? t.y2 - t.cref[1] : 0, a: t.a });
      } else {
        tw += '<circle cx="' + f(t.x2) + '" cy="' + f(t.y2) + '" r="2.8" class="gn-end"/>';
      }
      tw += '</g>';
    });
    fr.couples.forEach(function (c) {
      if (!c.stub) return;
      var sx = c.x + (c.stub_side || 1) * 22 * (c.k || 1), sy = c.y - (c.stub_up || 1) * 16 * (c.k || 1), d = edge(c.x, c.y, sx, sy);
      tw += '<g opacity="' + f(c.a) + '"><path d="' + d + '" class="gn-wire-halo"/><path d="' + d + '" class="gn-wire"/></g>';
      counts.push({ mk: { x: sx + (c.stub_side || 1) * 6, y: sy + 3.5, anchor: c.stub_side || 1, halo: true }, text: '+' + c.stub, size: 10, dx: 0, dy: 0, a: c.a });
    });

    // Cards beside the portraits, outward from the trunk: the heart's first,
    // under its portrait, then the nearest levels'. A card that would cover
    // another is left out, as the river leaves out a word it has no room
    // for; the pointer brings it back.
    var keep = {}, cs = C && C.s || {};
    var F = {};
    fr.persons.forEach(function (p) {
      var fm = frameOf(p);
      F[p.key] = fm;
      boxes.push([p.x - fm.rx - 2, p.y - fm.ry - 2, p.x + fm.rx + 2, p.y + fm.ry + 2]);
    });
    if (C && !cs.redacted) {
      var cf = F[C.key], cn = api.clip(cs.names[2], 240), cw = Math.max(api.width(cn, 15, true), api.width(cs.years, 10.5, true)) + 22;
      var spot = null, least = Infinity, tries = [];
      [8, 30].forEach(function (gap) {
        tries.push([C.x - cw / 2, C.y + cf.ry + gap, 0]);
        tries.push([C.x - cw / 2, C.y - cf.ry - gap - 36, 0]);
      });
      [12, 40].forEach(function (gap) {
        [1, -1].forEach(function (sd) {
          [0, -24, 24].forEach(function (dy) { tries.push([sd > 0 ? C.x + cf.rx + gap : C.x - cf.rx - gap - cw, C.y - 18 + dy, sd]); });
        });
      });
      tries.forEach(function (o) {
        var box = [o[0] - 2, o[1] - 2, o[0] + cw + 2, o[1] + 38], n = 0;
        if (box[0] < hl + 2 || box[2] > hl + hw - 48 || box[1] < vt + 2 || box[3] > vt + vh - 2) return;
        boxes.forEach(function (q) { if (box[0] < q[2] && box[2] > q[0] && box[1] < q[3] && box[3] > q[1]) n++; });
        if (n < least) { least = n; spot = { x: o[0], y: o[1], side: o[2], box: box }; }
      });
      if (!spot) spot = { x: C.x - cw / 2, y: C.y + cf.ry + 8, side: 0, box: [C.x - cw / 2 - 2, C.y + cf.ry + 6, C.x + cw / 2 + 2, C.y + cf.ry + 46] };
      keep[C.key] = { x: spot.x, y: spot.y, w: cw, h: 36, side: spot.side, centre: true, word: cn, both: true };
      boxes.push(spot.box);
    }
    fr.persons.slice().sort(function (a, b) { return Math.abs(a.gen) - Math.abs(b.gen) || b.z - a.z; }).forEach(function (p) {
      var s = p.s || {}, gd = Math.abs(p.gen), fm = F[p.key];
      if (p.role === 'centre' || s.redacted || gd > 4) return;
      var side = p.x < G.x0 - 3 ? -1 : 1;
      var word = gd >= 3 ? s.names[0] : s.names[1], both = gd <= 2 && !!s.years;
      var size = gd <= 1 ? 12.5 : gd === 2 ? 11.5 : 10.5;
      var w = Math.max(api.width(word, size, false), both ? api.width(s.years, 9, true) : 0) + 16, h = both ? 30 : 18;
      var inset = fm.rx * 0.45;
      for (var tries = 0; tries < 6; tries++) {
        var sd = tries < 3 ? side : -side, dy = [0, -(h / 2 + 6), h / 2 + 6][tries % 3];
        var x0 = sd > 0 ? p.x + fm.rx - inset : p.x - fm.rx + inset - w - inset;
        var y0 = p.y - h / 2 + dy, box = [sd > 0 ? p.x + fm.rx + 3 : x0 - 1, y0 - 1, sd > 0 ? x0 + w + inset + 1 : p.x - fm.rx - 3, y0 + h + 1];
        if (box[0] < hl + 2 || box[2] > hl + hw - 48 || box[1] < vt + 2 || box[3] > vt + vh - 2 || hit(box)) continue;
        boxes.push(box);
        keep[p.key] = { x: x0, y: y0, w: w + inset, h: h, side: sd, word: word, both: both, size: size, pad: sd > 0 ? inset + 8 : 8 };
        return;
      }
    });
    // The counts: never left out — the only sign of the people beyond —
    // but stepped aside from a card rather than written over it.
    var marks = '';
    counts.forEach(function (c) {
      var mk = c.mk, w = api.width(c.text, c.size, true), off = [0, 0], room = null;
      [[0, 0], [0, 11], [0, -11], [14, 0], [-14, 0], [0, 22], [0, -22], [28, 0], [-28, 0], [0, 34], [0, -34], [42, 0], [-42, 0]].some(function (o) {
        var x = mk.x + c.dx + o[0], y = mk.y + c.dy + o[1];
        var x0 = mk.anchor < 0 ? x - w : mk.anchor === 0 ? x - w / 2 : x, box = [x0 - 1, y - c.size, x0 + w + 1, y + 2];
        if (!room) room = box;
        if (hit(box)) return false;
        off = o; room = box;
        return true;
      });
      boxes.push(room);
      marks += '<g class="gn-count" opacity="' + f(c.a) + '">' + api.mark(Object.assign({}, mk, { x: mk.x + off[0], y: mk.y + off[1] }), c.text, c.size, c.dx, c.dy) + '</g>';
    });

    // People, the furthest first: a card, then the portrait over its inner
    // edge, so the two read as one — a sepia print in a cream mount with a
    // fine gold edge, as on the reference, its shadow on the parchment; the
    // person this reader may not see as an empty mount.
    var people = '';
    fr.persons.slice().sort(function (a, b) { return (a.k || 1) - (b.k || 1); }).forEach(function (p) {
      var s = p.s || {}, isC = p.role === 'centre', fm = F[p.key], x = f(p.x), y = f(p.y), g = '<g opacity="' + f(p.a) + '">';
      var bx = p.x - fm.rx, by = p.y - fm.ry, bw = fm.rx * 2, bh = fm.ry * 2, inset = Math.max(2, fm.rx * 0.16);
      if (rest) g += '<a class="rv-p" data-id="' + esc(p.id) + '" data-key="' + esc(p.key) + '" href="' + api.base + '?p=' + encodeURIComponent(p.id) + '&amp;n=' + m.n + '" aria-label="' + esc(api.said(p)) + '"><rect x="' + f(bx - 6) + '" y="' + f(by - 6) + '" width="' + f(bw + 12) + '" height="' + f(bh + 12) + '" fill="transparent"/>';
      var Lb = keep[p.key];
      if (Lb && !s.redacted) {
        g += '<g class="gn-plate">';
        g += '<rect class="gn-card-shade" x="' + f(Lb.x + 1.5) + '" y="' + f(Lb.y + 2.5) + '" width="' + f(Lb.w) + '" height="' + f(Lb.h) + '" rx="2"/>';
        g += '<rect class="gn-card' + (Lb.centre ? ' gn-card-centre' : '') + '" x="' + f(Lb.x) + '" y="' + f(Lb.y) + '" width="' + f(Lb.w) + '" height="' + f(Lb.h) + '" rx="2"/>';
        if (Lb.centre) {
          g += '<text x="' + f(Lb.x + Lb.w / 2) + '" y="' + f(Lb.y + 16) + '" text-anchor="middle" font-size="15" font-weight="600" class="gn-name gn-centre">' + esc(Lb.word) + '</text>'
            + '<text x="' + f(Lb.x + Lb.w / 2) + '" y="' + f(Lb.y + 29) + '" text-anchor="middle" font-family="' + api.mono + '" font-size="10.5" class="gn-years">' + esc(s.years) + '</text>';
        } else {
          var tx = Lb.side > 0 ? Lb.x + Lb.pad : Lb.x + 8;
          g += '<text x="' + f(tx) + '" y="' + f(Lb.y + (Lb.both ? 13 : 12.5)) + '" font-size="' + Lb.size + '" class="gn-name">' + esc(Lb.word) + '</text>';
          if (Lb.both) g += '<text x="' + f(tx) + '" y="' + f(Lb.y + 24.5) + '" font-family="' + api.mono + '" font-size="9" class="gn-years">' + esc(s.years) + '</text>';
        }
        g += '</g>';
      }
      if (p.repeat > 1) g += '<rect x="' + f(bx - 3.2) + '" y="' + f(by - 3.2) + '" width="' + f(bw + 6.4) + '" height="' + f(bh + 6.4) + '" rx="2" fill="none" class="gn-again" stroke-width="1"/>';
      if (p.again) g += '<rect x="' + f(bx - 3.2 - (p.repeat > 1 ? 2.4 : 0)) + '" y="' + f(by - 3.2 - (p.repeat > 1 ? 2.4 : 0)) + '" width="' + f(bw + 6.4 + (p.repeat > 1 ? 4.8 : 0)) + '" height="' + f(bh + 6.4 + (p.repeat > 1 ? 4.8 : 0)) + '" rx="2" fill="none" class="gn-again" stroke-width="1" stroke-dasharray="2.4 1.8"/>';
      if (isC) g += '<rect x="' + f(bx - 9) + '" y="' + f(by - 9) + '" width="' + f(bw + 18) + '" height="' + f(bh + 18) + '" rx="6" class="gn-halo"/>';
      g += '<rect x="' + f(bx + 1.5) + '" y="' + f(by + 2.5) + '" width="' + f(bw) + '" height="' + f(bh) + '" rx="1.5" class="gn-card-shade"/>';
      if (s.redacted) {
        g += '<rect x="' + f(bx) + '" y="' + f(by) + '" width="' + f(bw) + '" height="' + f(bh) + '" rx="1.5" class="gn-mount gn-hidden" stroke-dasharray="1.8 1.8"/>';
      } else {
        g += '<rect x="' + f(bx) + '" y="' + f(by) + '" width="' + f(bw) + '" height="' + f(bh) + '" rx="1.5" class="gn-mount' + (isC ? ' gn-mount-centre' : s.living ? ' gn-living' : ' gn-gone') + '"/>';
        g += '<rect x="' + f(bx + inset) + '" y="' + f(by + inset) + '" width="' + f(bw - 2 * inset) + '" height="' + f(bh - 2 * inset) + '" fill="url(#gn-print-g)" class="gn-print' + (s.sparse ? ' gn-sparse' : '') + '"/>';
        var ini = initials(s.names && s.names[2]);
        if (ini && fm.rx >= 8.5) g += '<text x="' + x + '" y="' + f(p.y + fm.rx * 0.34) + '" text-anchor="middle" font-size="' + f(fm.rx * 0.86) + '" class="gn-initials">' + esc(ini) + '</text>';
      }
      if (rest) g += '</a>';
      people += g + '</g>';
    });
    if (rest) {
      named = {};
      Object.keys(keep).forEach(function (k) { if (!keep[k].centre) named[k] = keep[k]; });
    }
    var head = '<svg class="rv-svg gn-svg" data-view="scroll" viewBox="' + f(hl) + ' ' + f(vt) + ' ' + f(hw) + ' ' + f(vh) + '" width="100%" '
      + (rest ? 'role="group" aria-label="' + esc(label() + (C && C.s && !C.s.redacted ? ' — ' + C.s.names[2] : '')) + '"' : 'aria-hidden="true"')
      + ' xmlns="http://www.w3.org/2000/svg"><defs>'
      // A print lighter at its middle, as an old photograph is.
      + '<radialGradient id="gn-print-g" cx="0.5" cy="0.42" r="0.62"><stop offset="0" class="gn-print-hi"/><stop offset="1" class="gn-print-lo"/></radialGradient></defs>';
    // While the camera climbs, the picture follows it (the scene, below).
    if (!rest && window.axgfRiverScene) window.axgfRiverScene.view();
    // The lit route lies over the lines and under the people (river.js).
    return head + rail + halo + ink + tw + knots + '<g class="rv-lit"></g>' + marks + people + '</svg>';
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
    // trunk's, the crown's.
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

  window.axgfView = { name: 'scroll', shape: shape, edge: edge, paint: paint, rest: rest, named: function () { return named; } };
  window.axgfView.tween = tween3;

  /* ---- the picture in relief, and its life ---- */
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
  var GOLD = rgb('--gn-gold', [200, 160, 80]), GLINT = rgb('--gn-glint', GOLD), LEAF = rgb('--gn-leaf', GOLD), DUST = rgb('--gn-dust', GOLD);
  function rgba(c, a) { return 'rgba(' + c[0] + ',' + c[1] + ',' + c[2] + ',' + a.toFixed(3) + ')'; }

  // Under the drawing, the picture (WebGL, in relief) and the leaves and
  // dust (a canvas over it); over the drawing, the glints along the lines.
  var ground = document.createElement('canvas');
  ground.className = 'rv-ground';
  ground.setAttribute('aria-hidden', 'true');
  stage.insertBefore(ground, stage.firstChild);
  var drift = document.createElement('canvas');
  drift.className = 'rv-ground rv-drift';
  drift.setAttribute('aria-hidden', 'true');
  stage.insertBefore(drift, ground.nextSibling);
  var layer = document.createElement('canvas');
  layer.className = 'rv-water';
  layer.setAttribute('aria-hidden', 'true');
  stage.appendChild(layer);
  stage.classList.add('rv-alive');
  var dx = drift.getContext('2d'), ctx = layer.getContext('2d');
  if (!ctx || !dx) return;
  // The same leaves and dust on every visit.
  function rnd(i, j) { var h = Math.abs(Math.sin(i * 12.9898 + j * 78.233) * 43758.5453); return h - Math.floor(h); }
  var LEAVES = [], MOTES = [];
  for (var li = 0; li < 16; li++) LEAVES.push({ u: 0.2 + 0.65 * rnd(li, 1), v: rnd(li, 2), fall: 0.012 + 0.016 * rnd(li, 3), sway: 0.6 + 1.2 * rnd(li, 4), size: 2.6 + 3 * rnd(li, 5), spin: 0.4 + rnd(li, 6) });
  for (var mi = 0; mi < 26; mi++) MOTES.push({ u: rnd(mi, 7), v: rnd(mi, 8), rise: 0.006 + 0.01 * rnd(mi, 9), r: 0.6 + 1.1 * rnd(mi, 10), ph: rnd(mi, 11) * 6.283 });

  // The picture: a quad over the window, each pixel looking the picture up
  // where the camera's shift, scaled by how near that pixel is (the depth
  // map), takes it; past the picture's edges its wall runs on.
  var gl = ground.getContext('webgl', { alpha: false, antialias: false, depth: false, stencil: false, premultipliedAlpha: false, powerPreference: 'low-power' });
  var prog = null, picT = null, depT = null, hasPic = false, hasDep = false, UL = {};
  function shader(type, src) {
    var s = gl.createShader(type);
    gl.shaderSource(s, src); gl.compileShader(s);
    return gl.getShaderParameter(s, gl.COMPILE_STATUS) ? s : null;
  }
  if (gl) {
    var vs = shader(gl.VERTEX_SHADER, 'attribute vec2 a;uniform vec2 uRes;varying vec2 vP;void main(){vP=vec2((a.x+1.0)*0.5*uRes.x,(1.0-a.y)*0.5*uRes.y);gl_Position=vec4(a,0.0,1.0);}');
    var fs = shader(gl.FRAGMENT_SHADER, 'precision mediump float;uniform sampler2D uT;uniform sampler2D uD;uniform vec4 uRect;uniform vec2 uCam;uniform float uDep;uniform vec3 uWall;varying vec2 vP;'
      + 'void main(){vec2 p=(vP-uRect.xy)/uRect.zw;vec2 s=uCam/uRect.zw;float d=uDep>0.5?texture2D(uD,clamp(p,0.0,1.0)).r:0.35;vec2 q=p-s*(d-0.35);'
      + 'if(uDep>0.5){d=texture2D(uD,clamp(q,0.0,1.0)).r;q=p-s*(d-0.35);}'
      + 'vec3 c=texture2D(uT,clamp(q,vec2(0.0),vec2(1.0))).rgb;'
      // The picture stands on the stage's own green (it was set on it, its
      // wall cut away): past its edges, that green.
      + 'float o=step(0.0,q.x)*step(q.x,1.0)*step(0.0,q.y)*step(q.y,1.0);'
      + 'gl_FragColor=vec4(mix(uWall,c,o),1.0);}');
    if (vs && fs) {
      prog = gl.createProgram();
      gl.attachShader(prog, vs); gl.attachShader(prog, fs); gl.linkProgram(prog);
      if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) prog = null;
    }
    if (prog) {
      gl.useProgram(prog);
      var buf = gl.createBuffer();
      gl.bindBuffer(gl.ARRAY_BUFFER, buf);
      gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]), gl.STATIC_DRAW);
      var aLoc = gl.getAttribLocation(prog, 'a');
      gl.enableVertexAttribArray(aLoc);
      gl.vertexAttribPointer(aLoc, 2, gl.FLOAT, false, 0, 0);
      ['uRes', 'uT', 'uD', 'uRect', 'uCam', 'uDep', 'uWall'].forEach(function (n) { UL[n] = gl.getUniformLocation(prog, n); });
      gl.uniform1i(UL.uT, 0); gl.uniform1i(UL.uD, 1);
      var WALL = rgb('--gn-stage', [20, 37, 25]);
      gl.uniform3f(UL.uWall, WALL[0] / 255, WALL[1] / 255, WALL[2] / 255);
      picT = gl.createTexture(); depT = gl.createTexture();
    } else gl = null;
  }
  var g2 = gl ? null : ground.getContext('2d');
  // The picture arrives once. For the graphics card it is redrawn at a
  // power of two each way, so that it can be filtered as it shrinks (its
  // aspect is the quad's, not the texture's).
  var pic = new Image(), dep = new Image();
  pic.onload = function () {
    hasPic = true;
    if (gl && !gl.isContextLost()) {
      // The window shows a part of the picture, magnified: the large
      // texture always.
      var big = true;
      var c = document.createElement('canvas');
      c.width = big ? 2048 : 1024; c.height = big ? 4096 : 2048;
      var cx2 = c.getContext('2d');
      cx2.imageSmoothingQuality = 'high';
      cx2.drawImage(pic, 0, 0, c.width, c.height);
      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, picT);
      gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGB, gl.RGB, gl.UNSIGNED_BYTE, c);
      gl.generateMipmap(gl.TEXTURE_2D);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    }
    if (!running) draw(false);
  };
  dep.onload = function () {
    if (!gl || gl.isContextLost()) return;
    gl.activeTexture(gl.TEXTURE1);
    gl.bindTexture(gl.TEXTURE_2D, depT);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.LUMINANCE, gl.LUMINANCE, gl.UNSIGNED_BYTE, dep);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    hasDep = true;
    if (!running) draw(false);
  };
  pic.src = PIC.src;
  if (gl) dep.src = PIC.depth;

  var flows = [], dpr = 1, cw = 0, ch = 0, clock = 0, travelling = false;
  function size() {
    var w = stage.clientWidth, h = stage.clientHeight, r = Math.min(window.devicePixelRatio || 1, 2);
    if (w === cw && h === ch && r === dpr) return false;
    cw = w; ch = h; dpr = r;
    layer.width = ground.width = drift.width = Math.max(1, Math.round(w * dpr));
    layer.height = ground.height = drift.height = Math.max(1, Math.round(h * dpr));
    return true;
  }
  function view() {
    var svg = stage.querySelector('svg.rv-svg'), vb = svg && svg.viewBox && svg.viewBox.baseVal;
    if (!vb || !vb.width || !cw) return null;
    var k = Math.min(cw / vb.width, ch / vb.height);
    return { x: vb.x - (cw / k - vb.width) / 2, y: vb.y - (ch / k - vb.height) / 2, k: k };
  }
  // A glint runs along every attested line, from its lower end to its
  // upper: roots to trunk to crown.
  function layout(L) {
    flows = [];
    if (!L || !L.edges) return;
    L.edges.forEach(function (e) {
      if (e.conf !== 'a') return;
      var P = e.y1 >= e.y2 ? wire(e.x1, e.y1, e.x2, e.y2) : wire(e.x2, e.y2, e.x1, e.y1), xs = [], ys = [], cum = [0];
      P.forEach(function (q, i) { xs.push(q[0]); ys.push(q[1]); if (i) cum.push(cum[i - 1] + Math.hypot(q[0] - P[i - 1][0], q[1] - P[i - 1][1])); });
      if (cum[3] < 12) return;
      flows.push({ x: xs, y: ys, cum: cum, n: 3, len: cum[3], phase: Math.abs(Math.sin(e.x1 * 12.9898 + e.y2 * 78.233)) % 1 });
    });
    request();
  }
  // The camera: where the pointer leans the tree (a little), a slow sway
  // while the scene lives, and the climb — up into the crown as the reader
  // travels to a descendant, down to the roots toward an ancestor, settling
  // as the person travelled to comes to the trunk.
  var lean = [0, 0], leanTo = [0, 0];
  if (!reduce && !cut) {
    stage.addEventListener('pointermove', function (ev) {
      var r = stage.getBoundingClientRect();
      leanTo = [clamp((ev.clientX - r.left) / r.width * 2 - 1, -1, 1), clamp((ev.clientY - r.top) / r.height * 2 - 1, -1, 1)];
      if (!running) draw(false);
    });
    stage.addEventListener('pointerleave', function () { leanTo = [0, 0]; if (!running) draw(false); });
  }
  var SPEED = 34;
  function picture(moving) {
    var v = view(), G = GEO;
    if (!v || !G) return;
    var s = v.k * dpr;
    lean[0] += (leanTo[0] - lean[0]) * (moving ? 0.08 : 1); lean[1] += (leanTo[1] - lean[1]) * (moving ? 0.08 : 1);
    var sway = moving ? Math.sin(clock * 0.31) * 0.35 : 0;
    // Where on the tree the camera stands: the trunk, where the reader came
    // in; up into the crown by as many generations as they have climbed
    // since, down to the roots by as many as they have gone down, step by
    // step as they travel (the world's level runs on through a climb), the
    // tree's top and foot never left behind. That height of the picture
    // stands at the fixed point's.
    var wl = G.world || 0, camV = PIC.v - (wl > 0 ? PIC.climbUp : PIC.climbDown) * Math.tanh(wl / PIC.pace);
    var px = (G.x0 - PIC.u * G.PW - v.x) * s, py = (G.y0 - camV * G.PH - v.y) * s, pw = G.PW * s, ph = G.PH * s;
    // Up in the crown the leaves stand out a little more, down in the roots
    // the roots do: the relief leans with the climb.
    var camX = (lean[0] * 9 + sway * 6) * s, camY = (lean[1] * 6 + (PIC.v - camV) * 36) * s;
    if (gl) {
      gl.viewport(0, 0, ground.width, ground.height);
      if (!hasPic) { var W0 = rgb('--gn-stage', [20, 37, 25]); gl.clearColor(W0[0] / 255, W0[1] / 255, W0[2] / 255, 1); gl.clear(gl.COLOR_BUFFER_BIT); return; }
      gl.useProgram(prog);
      gl.activeTexture(gl.TEXTURE0); gl.bindTexture(gl.TEXTURE_2D, picT);
      gl.activeTexture(gl.TEXTURE1); gl.bindTexture(gl.TEXTURE_2D, depT);
      gl.uniform2f(UL.uRes, ground.width, ground.height);
      gl.uniform4f(UL.uRect, px, py, pw, ph);
      gl.uniform2f(UL.uCam, camX, camY);
      gl.uniform1f(UL.uDep, hasDep ? 1 : 0);
      gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
    } else if (g2) {
      g2.setTransform(1, 0, 0, 1, 0, 0);
      g2.clearRect(0, 0, ground.width, ground.height);
      if (hasPic) g2.drawImage(pic, px, py, pw, ph);
    }
    return { px: px, py: py, pw: pw, ph: ph };
  }
  // Gold leaves falling from the crown across the parchment, and a little
  // gold dust in its light.
  function leaves(moving, P) {
    var Wd = drift.width, Hh = drift.height, t = moving ? clock : 4.2;
    dx.setTransform(1, 0, 0, 1, 0, 0);
    dx.clearRect(0, 0, Wd, Hh);
    if (!P || !moving) return;
    MOTES.forEach(function (mo) {
      var y = P.py + (1.02 - ((mo.v + t * mo.rise) % 1.04)) * P.ph * 0.8, x = P.px + (mo.u + 0.02 * Math.sin(t * 0.7 + mo.ph)) * P.pw;
      dx.fillStyle = rgba(DUST, 0.35 + 0.35 * (0.5 + 0.5 * Math.sin(t * 1.3 + mo.ph)));
      dx.beginPath(); dx.arc(x, y, mo.r * dpr, 0, 6.283); dx.fill();
    });
    LEAVES.forEach(function (lf) {
      var fy = (lf.v + t * lf.fall) % 1, y = P.py + (0.12 + fy * 0.84) * P.ph, x = P.px + (lf.u + 0.04 * Math.sin(t * 0.5 * lf.sway + lf.u * 9)) * P.pw;
      var a = 0.85 * Math.min(1, fy * 8, (1 - fy) * 5), r = lf.size * dpr, turn = t * lf.spin + lf.u * 6.283;
      dx.save();
      dx.translate(x, y);
      dx.rotate(turn);
      dx.scale(1, 0.45 + 0.35 * Math.abs(Math.sin(turn)));
      dx.fillStyle = rgba(LEAF, Math.max(0, a));
      dx.beginPath(); dx.ellipse(0, 0, r, r * 0.5, 0, 0, 6.283); dx.fill();
      dx.restore();
    });
  }
  function draw(moving) {
    size();
    var P = picture(moving);
    leaves(moving, P);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, layer.width, layer.height);
    var v = view();
    if (!v || !moving || travelling) return;
    var s = v.k * dpr, ox = -v.x * s, oy = -v.y * s;
    flows.forEach(function (fl) {
      var pos = (fl.phase * fl.len + clock * SPEED) % (fl.len + 80), i = 1;
      if (pos > fl.len) return;
      while (i < fl.n && fl.cum[i] < pos) i++;
      var u = (pos - fl.cum[i - 1]) / ((fl.cum[i] - fl.cum[i - 1]) || 1);
      var x = (fl.x[i - 1] + (fl.x[i] - fl.x[i - 1]) * u) * s + ox, y = (fl.y[i - 1] + (fl.y[i] - fl.y[i - 1]) * u) * s + oy;
      var fade = Math.min(1, pos / 16, (fl.len - pos) / 16), r = 2.6 * v.k * dpr;
      var g = ctx.createRadialGradient(x, y, 0, x, y, r * 2.6);
      g.addColorStop(0, rgba(GLINT, 0.95 * fade));
      g.addColorStop(0.4, rgba(GOLD, 0.5 * fade));
      g.addColorStop(1, rgba(GOLD, 0));
      ctx.fillStyle = g;
      ctx.beginPath(); ctx.arc(x, y, r * 2.6, 0, 6.283); ctx.fill();
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
