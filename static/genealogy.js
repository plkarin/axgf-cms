/* Genealogy's tree, climbed (v0.5.0, docs/adr/0016; redrawn 2026-10-23).
 *
 * The river's own people — from the same payload, through the same reader's
 * lens, in the same order from left to right — on a tree the reader climbs.
 * The person at the fixed point is set into its trunk, a little over the
 * ground. Descendants are up in the crown, ancestors down among the roots,
 * under the ground: a generation a level, each further one drawn closer to
 * the last and smaller, as it would look from where the reader stands, and
 * each level's people spread round the trunk in the river's order.
 *
 * The family's own wood makes the tree. Where a family parts, above
 * someone, is a fork on their wood, their partner's twig joining it there;
 * the fixed point's families fork at the top of the trunk, so that their
 * children's limbs leave the trunk itself. Below, each root parts toward
 * the two parents. Every stem is about as thick as all it bears — a twig
 * for one person, a limb for a family, the trunk for everyone — and grows
 * as wood does, leaving its fork at a slant and bending up toward the
 * light (or, a root, down toward the deep). A crown of leaves stands
 * behind the wood, as wide as the family reaches; a crown with fewer than
 * three limbs gets the trunk's own, bearing no one.
 *
 * Travelling up to a descendant, the reader climbs: the camera rises along
 * the tree, which slides down past it, the person travelled to growing as
 * they come level with it; travelling to an ancestor, the reader climbs
 * down. The blend is done in the tree's own space (`tween3`, through
 * river.js's travel); the ground holds still, the tree coming up out of it
 * or going down into it, and the woods behind move with the climb, more
 * slowly, as distant things do.
 *
 * Each person is a portrait in an oval gold frame — their initials, since
 * the payload carries no picture — with a cream card beside it, outward
 * from the trunk, that names them; whom the drawing has no room for, the
 * pointer names, or river.js when every name is asked for. A stem is lit
 * wood edged in dark bark; a parentage not attested runs along it as a
 * seam, broken for an inferred one, a line of knots for a speculative one.
 *
 * Two things the page knows by name:
 * - `window.axgfView` (river.js): where everyone stands (`shape`), how a
 *   stem bends (`edge`, which the lit route follows too), how a frame is
 *   painted at rest and in travel, which people the drawing named, the
 *   dashboard at rest, and how two frames blend (`tween`).
 * - `window.axgfRiverScene`: under the drawing, the woods (a picture), the
 *   ground and the crown's leaves, painted once for each drawing of the
 *   tree; over them, mist, gold motes rising, leaves falling, and a glint
 *   run up every attested stem. It holds still as the river's ground does:
 *   reduced motion, "Cut", the pause, a hidden tab, a tree off screen, a
 *   machine too slow for it.
 *
 * No colour is named here: every one is a token of the theme (`--gn-*`,
 * tokens.css), read from the page or set by class in app.css.
 */
(function () {
  'use strict';
  var root = document.querySelector('.river');
  if (!root || root.dataset.view !== 'scroll') return;
  var stage = document.getElementById('rv-canvas');

  /* ---- the tree in perspective ---- */
  // FALL: how a generation's height above or below the camera is drawn —
  // the first nearly a whole step, each further one less. SHRINK: how much
  // smaller a generation further away is. THETA: how far round the trunk a
  // generation's people reach, either side of the front. DEPTH: how much
  // smaller someone round the side is than someone at the front.
  var FALL = 2.4, SHRINK = 0.14, THETA = 1.25, DEPTH = 0.26;
  var GEO = null, world = 0, pending = 0;
  function clamp(v, a, b) { return Math.max(a, Math.min(b, v)); }
  function lift(l) { var a = Math.abs(l); return (l < 0 ? -1 : 1) * FALL * (1 - Math.exp(-a / FALL)); }
  function proj(G, th, l) {
    var z = Math.cos(th), kl = Math.exp(-SHRINK * Math.abs(l));
    return { x: G.x0 + G.R * Math.sin(th) * kl, y: G.y0 - lift(l) * G.H, z: z, k: kl * (1 - DEPTH * (1 - z)) };
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
    // Round the trunk: the widest side reaches THETA either way, a place
    // never more than a quarter turn wide.
    var R = clamp(hw * 0.46, 160, 640);
    var top = lift(nd), bottom = lift(na), H = clamp((vh - 150) / Math.max(1, top + bottom), 60, 210);
    var y0 = vt + 74 + top * H + Math.max(0, (vh - 150 - (top + bottom) * H) / 2);
    var G = { x0: hl + hw / 2, y0: y0, R: R, H: H, nd: nd, na: na };
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
      var q = proj(G, th, l);
      at[p.x + ',' + p.y] = [th, l];
      return Object.assign({}, p, { x: q.x, y: q.y, z: q.z, k: q.k, th: th, lv: l });
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
  // How a stem grows: it leaves the wood it grows from at a slant and bends
  // toward the light — or, a root, toward the deep — arriving upright (or
  // straight down). Its base is the end nearer the camera's height, the
  // trunk's side. The control points, in the order the ends were given.
  function bend(x1, y1, x2, y2) {
    var y0 = GEO ? GEO.y0 : (y1 + y2) / 2, flip = Math.abs(y2 - y0) < Math.abs(y1 - y0);
    var bx = flip ? x2 : x1, by = flip ? y2 : y1, vx = (flip ? x1 : x2) - bx, vy = (flip ? y1 : y2) - by;
    var c1 = [bx + vx * 0.46, by + vy * 0.14], c2 = [bx + vx * 0.94, by + vy * 0.5];
    return flip ? [x1, y1, c2[0], c2[1], c1[0], c1[1], x2, y2] : [x1, y1, c1[0], c1[1], c2[0], c2[1], x2, y2];
  }
  function edge(x1, y1, x2, y2) {
    var c = bend(x1, y1, x2, y2), r = function (v) { return Math.round(v * 10) / 10; };
    return 'M' + r(c[0]) + ' ' + r(c[1]) + 'C' + r(c[2]) + ' ' + r(c[3]) + ' ' + r(c[4]) + ' ' + r(c[5]) + ' ' + r(c[6]) + ' ' + r(c[7]);
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
    fr.meta = Object.assign({}, fr.meta, { gn: Object.assign({}, G, { world: world + cam }) });
    pending = lc;
    return fr;
  }

  /* ---- the drawing ---- */
  // Wood: a stem about as thick as all it bears together, as a tree's wood
  // is — a twig for one person, a limb for a family, the trunk for all.
  function wood(load) { return clamp(3.4 * Math.pow(load, 0.68), 3.4, 40); }
  // Where the light comes from: the upper left, as in the woods behind.
  var LX = -0.55, LY = -0.83;
  function at3(c, t) {
    var u = 1 - t;
    return [u * u * u * c[0] + 3 * u * u * t * c[2] + 3 * u * t * t * c[4] + t * t * t * c[6],
      u * u * u * c[1] + 3 * u * u * t * c[3] + 3 * u * t * t * c[5] + t * t * t * c[7],
      3 * u * u * (c[2] - c[0]) + 6 * u * t * (c[4] - c[2]) + 3 * t * t * (c[6] - c[4]),
      3 * u * u * (c[3] - c[1]) + 6 * u * t * (c[5] - c[3]) + 3 * t * t * (c[7] - c[5])];
  }
  // A stem along its curve, from width wa at its first end to wb at its
  // last: its outline, its lit side, its shaded side and two lines of its
  // grain. The lit side is the one the light falls on at its middle.
  function stem(c, wa, wb, f) {
    var n = 14, m = at3(c, 0.5), md = Math.hypot(m[2], m[3]) || 1, side = (-m[3] * LX + m[2] * LY) / md < 0 ? -1 : 1;
    var L = [], R = [], lit = [], litB = [], sh = [], shB = [], g1 = [], g2 = [];
    for (var i = 0; i <= n; i++) {
      var t = i / n, q = at3(c, t), d = Math.hypot(q[2], q[3]) || 1, nx = -q[3] / d * side, ny = q[2] / d * side;
      var w = (wa + (wb - wa) * t) / 2, x = q[0], y = q[1];
      function pt(o) { return f(x + nx * w * o) + ' ' + f(y + ny * w * o); }
      L.push(pt(1)); R.unshift(pt(-1));
      lit.push(pt(0.8)); litB.unshift(pt(0.1));
      sh.push(pt(-0.38)); shB.unshift(pt(-1));
      g1.push(pt(-0.55 + 0.12 * Math.sin(t * 9 + wa))); g2.push(pt(0.42 + 0.1 * Math.sin(t * 7 + wb)));
    }
    return { body: 'M' + L.join('L') + 'L' + R.join('L') + 'Z', lit: 'M' + lit.join('L') + 'L' + litB.join('L') + 'Z',
      shade: 'M' + sh.join('L') + 'L' + shB.join('L') + 'Z', grain: 'M' + g1.join('L') + 'M' + g2.join('L') };
  }
  // A portrait's frame: the heart's the largest, then smaller with distance
  // along the trunk and round it, never wider than the places allow.
  function frameOf(p) {
    var g = Math.abs(p.gen), base = p.role === 'centre' ? 23 : g <= 1 ? 17 : g === 2 ? 14 : 12;
    var rx = Math.max(7, base * (p.k || 1));
    return { rx: rx, ry: rx * 1.2 };
  }
  // Initials for the portrait: the first letter of the first and of the
  // last word of the name.
  function initials(name) {
    var w = String(name || '').trim().split(/\s+/).filter(Boolean);
    if (!w.length) return '';
    var a = Array.from(w[0])[0] || '', b = w.length > 1 ? Array.from(w[w.length - 1])[0] || '' : '';
    return (a + b).toUpperCase();
  }
  // The same small numbers on every visit, from a person's key.
  function hash(s, i) {
    var h = 2166136261;
    for (var j = 0; j < s.length; j++) { h ^= s.charCodeAt(j); h = Math.imul(h, 16777619); }
    h ^= i * 374761393; h = Math.imul(h ^ (h >>> 13), 1274126177);
    return ((h ^ (h >>> 16)) >>> 0) / 4294967296;
  }
  var named = {}, leafage = 0;
  function paint(fr, api, rest) {
    var f = api.f, esc = api.esc, PM = api.byKey(fr.persons), m = fr.meta;
    var vt = m.view[0], vh = m.view[1], hl = m.hview[0], hw = m.hview[1];
    if (rest && pending) { world += pending; pending = 0; }
    var G = m.gn || GEO || { x0: hl + hw / 2, y0: vt + vh / 2, R: hw / 3, H: 120, nd: 0, na: 0 };
    var W = G.world !== undefined ? G.world : world;
    GEO = Object.assign({}, G, { world: W });
    var climb = W * G.H, C = PM[m.centre] || centreOf(fr);
    function yAt(l) { return G.y0 - lift(l) * G.H; }
    var crownTop = G.y0 - G.H * 0.12, rootTop = G.y0 + G.H * 0.12;
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
      var ly = yAt(l), word = l > 0 ? '+' + l : l < 0 ? '−' + (-l) : '0 ◂', gx = hl + hw - 44;
      rail += '<text x="' + f(gx) + '" y="' + f(ly + 3.5) + '" font-family="' + api.mono + '" font-size="10" class="gn-gen' + (l === 0 ? ' gn-gen-0' : '') + '">' + word + '</text>';
      boxes.push([gx - 2, ly - 8, gx + 34, ly + 5]);
    }

    // What each stem bears: above someone, all the people its wood carries
    // up to the last level drawn; below, all the ancestors its root reaches.
    var E = api.edges(fr, PM), upF = {}, downF = {};
    fr.couples.forEach(function (c) {
      c.parents.forEach(function (k) { var p = PM[k]; if (p && c.y < p.y) (upF[k] = upF[k] || []).push(c); });
      c.kids.forEach(function (k) { var p = PM[k]; if (p && c.y > p.y) (downF[k] = downF[k] || []).push(c); });
    });
    var LU = {}, LD = {};
    function famUp(c, seen) { return c.kids.reduce(function (s, k) { return s + (PM[k] ? loadUp(k, seen) : 0); }, 0); }
    function loadUp(k, seen) {
      if (k in LU) return LU[k];
      if (seen[k]) return 1;
      seen[k] = 1;
      var n = 1;
      (upF[k] || []).forEach(function (c) { n += famUp(c, seen); });
      return (LU[k] = n);
    }
    function loadDown(k, seen) {
      if (k in LD) return LD[k];
      if (seen[k]) return 1;
      seen[k] = 1;
      var n = 1;
      (downF[k] || []).forEach(function (c) { c.parents.forEach(function (q) { if (PM[q]) n += loadDown(q, seen); }); });
      return (LD[k] = n);
    }
    var Wt = clamp(wood(C ? loadUp(C.key, {}) + loadDown(C.key, {}) : 4) * 3, 78, Math.max(78, Math.min(132, hw * 0.12)));

    // The trunk: from where the roots part, under the ground, to where the
    // limbs leave it over the fixed point; wider toward the ground and
    // flaring into it, swaying a little, its bark lit from the left. The
    // ground lies a little under the fixed point.
    var yG = yAt(-0.3), yTop = yAt(0.62), yBot = yAt(-0.62);
    fr.couples.forEach(function (c) {
      if (Math.abs(c.x - G.x0) > Wt * 0.8) return;
      if (c.y < G.y0 && c.y > yAt(0.9)) yTop = Math.max(yTop, c.y);
      if (c.y > G.y0 && c.y < yAt(-0.9)) yBot = Math.min(yBot, c.y);
    });
    function sway(y) { return Math.sin((y + climb) / 170) * 4 + Math.sin((y + climb) / 61) * 1.4; }
    function girth(y) {
      var t = clamp((y - yTop) / (yG - yTop), 0, 1), w = Wt * (0.72 + 0.26 * t);
      if (y > yG - 30) w += Wt * 0.42 * Math.pow(clamp((y - (yG - 30)) / (yBot - yG + 30), 0, 1), 1.6);
      return w;
    }
    var L = [], Rr = [], trunk = '', cap = Wt * 0.22;
    for (var y = yTop - cap; y <= yBot + 6.1; y += 6) {
      var c0 = G.x0 + sway(y), w = girth(y) * (y < yTop ? 0.45 + 0.55 * Math.sqrt(Math.max(0, 1 - Math.pow((yTop - y) / cap, 2))) : 1);
      L.push(f(c0 - w / 2) + ' ' + f(y)); Rr.unshift(f(c0 + w / 2) + ' ' + f(y));
    }
    trunk += '<path class="gn-trunk" d="M' + L.join('L') + 'L' + Rr.join('L') + 'Z" fill="url(#gn-bark)"/>';
    // Its bark: fissures down it, lit on one lip, the same on every visit,
    // sliding past as the reader climbs.
    var span = yBot - yTop + 16, fis = '', ridge = '';
    for (var i = 0; i < 46; i++) {
      var fx = (i % 9) / 9 + hash('bark', i) * 0.09, len = 12 + 22 * hash('bark', i + 50);
      var y1 = yTop - 8 + ((hash('bark', i + 100) * span + climb * 0.999) % span + span) % span, y2 = Math.min(yBot, y1 + len);
      if (y2 - y1 < 6) continue;
      var w1 = girth(y1), x1 = G.x0 + sway(y1) - w1 / 2 + w1 * (0.08 + fx * 0.84), w2 = girth(y2), x2 = G.x0 + sway(y2) - w2 / 2 + w2 * (0.08 + fx * 0.84);
      var mx = (x1 + x2) / 2 + (hash('bark', i + 150) - 0.5) * 4;
      fis += 'M' + f(x1) + ' ' + f(y1) + 'Q' + f(mx) + ' ' + f((y1 + y2) / 2) + ' ' + f(x2) + ' ' + f(y2);
      ridge += 'M' + f(x1 - 1.6) + ' ' + f(y1 + 2) + 'Q' + f(mx - 1.6) + ' ' + f((y1 + y2) / 2) + ' ' + f(x2 - 1.6) + ' ' + f(y2 - 2);
    }
    trunk += '<path class="gn-ridge" d="' + ridge + '"/><path class="gn-fissure" d="' + fis + '"/>';

    // The ground lies a little under the fixed point: the scene draws it,
    // the whole width of the window, soil from there down (static/genealogy.js,
    // woods()), rising in a mound round the trunk.
    GEO.yG = yG; GEO.Wt = Wt;

    // The stems: each family's wood from the person it grows from, as thick
    // as all it bears; the partner's a twig joining it. Below the ground,
    // roots. The two stems inside the trunk are the trunk's.
    var crownW = '', rootW = '', knots = '', rootlets = '', limbs = [];
    function grow(e, base, tip, rooted) {
      var c = bend(e.x1, e.y1, e.x2, e.y2), flip = Math.abs(e.y2 - G.y0) < Math.abs(e.y1 - G.y0);
      var wa = flip ? tip : base, wb = flip ? base : tip, s = stem(c, wa, wb, f), o = e.a < 1 ? e.a : 1;
      if (e.conf === 's') o *= 0.62;
      var g = '<g class="gn-stem' + (rooted ? ' gn-rooted' : '') + '" opacity="' + f(o) + '"><path d="' + s.body + '" class="gn-wood"/><path d="' + s.shade + '" class="gn-wood-shade"/><path d="' + s.lit + '" class="gn-wood-lit"/>';
      if (Math.max(wa, wb) > 9) g += '<path d="' + s.grain + '" class="gn-grain"/>';
      if (e.conf === 'd') g += '<path d="' + edge(e.x1, e.y1, e.x2, e.y2) + '" class="gn-seam" stroke-dasharray="6 4"/>';
      else if (e.conf === 's') g += '<path d="' + edge(e.x1, e.y1, e.x2, e.y2) + '" class="gn-seam" stroke-dasharray="0.1 5"/>';
      g += '</g>';
      if (rooted) {
        // Rootlets off a root, into the soil.
        [0.45, 0.75].forEach(function (t, j) {
          var q = at3(c, t), d = Math.hypot(q[2], q[3]) || 1, sd = j ? 1 : -1, ux = q[2] / d, uy = q[3] / d;
          var rx = ux * 0.8 - uy * 0.6 * sd, ry = Math.abs(uy * 0.8 + ux * 0.6 * sd) + 0.3, ln = Math.min(10 + 12 * hash(String(e.from || e.to) + (e.cp ? e.cp.key : ''), j), wa + wb + 8);
          rootlets += 'M' + f(q[0]) + ' ' + f(q[1]) + 'q' + f(rx * ln * 0.5) + ' ' + f(ry * ln * 0.3) + ' ' + f(rx * ln) + ' ' + f(ry * ln);
        });
      }
      return g;
    }
    E.forEach(function (e) {
      var cp = e.cp || {}, a = e.from ? PM[e.from] : null, kd = e.to ? PM[e.to] : null;
      var kk = ((a ? a.k : cp.k || 1) + (kd ? kd.k : cp.k || 1)) / 2 || 1, crown = cp.y < G.y0;
      if (e.from) {
        if (crown) {
          if (a === C) return;
          var blood = a.role === 'centre' || a.role === 'desc', ww = blood ? wood(famUp(cp, {})) * kk : Math.max(2.4, 3.2 * kk);
          crownW += grow(e, ww * (blood ? 1.12 : 1), ww, false);
        } else {
          var wr = wood(loadDown(e.from, {})) * kk, near = Math.abs(cp.x - G.x0) < Wt * 0.8 && Math.abs(cp.y - yBot) < 2;
          // A root off the trunk leaves it from inside its flare.
          rootW += grow(near ? Object.assign({}, e, { y2: e.y2 - Wt * 0.3 }) : e, near ? Math.max(wr * 1.5, Wt * 0.46) : wr * 1.4, wr * 0.8, true);
        }
      } else {
        if (crown) {
          var wc = wood(loadUp(e.to, {})) * kk, fork = Math.abs(cp.x - G.x0) < Wt * 0.8 && Math.abs(cp.y - yTop) < 2;
          // A limb off the trunk leaves it from a little under its top.
          if (fork) limbs.push(e.x2 - G.x0);
          crownW += grow(fork ? Object.assign({}, e, { y1: e.y1 + Wt * 0.32 }) : e, fork ? Math.max(wc * 1.45, Wt * 0.42) : wc * 1.35, wc * 0.82, false);
        } else {
          if (kd === C) return;
          var wd = wood(loadDown(e.to, {})) * kk;
          rootW += grow(e, wd * 1.1, wd, true);
        }
      }
    });
    // Each family a knot where its wood parts, but where the trunk parts.
    fr.couples.forEach(function (c) {
      if (Math.abs(c.x - G.x0) < Wt * 0.6 && (Math.abs(c.y - yTop) < 2 || Math.abs(c.y - yBot) < 2)) return;
      if (c.parents.length + c.kids.length > 1) knots += '<circle cx="' + f(c.x) + '" cy="' + f(c.y) + '" r="' + f(Math.max(2, 3.2 * (c.k || 1))) + '" class="gn-knot" opacity="' + f(c.a) + '"/>';
    });

    // Lines the record continues past the last level, and lines it ends.
    var counts = [], tw = '';
    fr.tails.forEach(function (t) {
      var dx = t.x2 - t.x1, dy = t.y2 - t.y1, len = Math.hypot(dx, dy) || 1, ux = dx / len, uy = dy / len, w = Math.max(1.4, 2.4 * (t.k || 1));
      tw += '<g opacity="' + f(t.a) + '"><path d="M' + f(t.x1) + ' ' + f(t.y1) + 'L' + f(t.x2) + ' ' + f(t.y2) + '" class="gn-branch" stroke-width="' + f(w + 1.6) + '"/>'
        + '<path d="M' + f(t.x1) + ' ' + f(t.y1) + 'L' + f(t.x2) + ' ' + f(t.y2) + '" class="gn-branch-core" stroke-width="' + f(w * 0.55) + '"/>';
      if (t.kind === 'cont') {
        tw += '<polygon class="gn-tip" points="' + f(t.x2 - uy * 4) + ',' + f(t.y2 + ux * 4) + ' ' + f(t.x2 + uy * 4) + ',' + f(t.y2 - ux * 4) + ' ' + f(t.x2 + ux * 6) + ',' + f(t.y2 + uy * 6) + '"/>';
        if (t.count_at) counts.push({ mk: t.count_at, text: '+' + t.count, size: 9.5, dx: t.cref ? t.x2 - t.cref[0] : 0, dy: t.cref ? t.y2 - t.cref[1] : 0, a: t.a });
      } else {
        tw += '<circle cx="' + f(t.x2) + '" cy="' + f(t.y2) + '" r="3" class="gn-end"/>';
      }
      tw += '</g>';
    });
    fr.couples.forEach(function (c) {
      if (!c.stub) return;
      var sx = c.x + (c.stub_side || 1) * 22 * (c.k || 1), sy = c.y - (c.stub_up || 1) * 16 * (c.k || 1);
      tw += '<g opacity="' + f(c.a) + '"><path d="' + edge(c.x, c.y, sx, sy) + '" class="gn-branch" stroke-width="3"/><path d="' + edge(c.x, c.y, sx, sy) + '" class="gn-branch-core" stroke-width="1.4"/></g>';
      counts.push({ mk: { x: sx + (c.stub_side || 1) * 6, y: sy + 3.5, anchor: c.stub_side || 1, halo: true }, text: '+' + c.stub, size: 10, dx: 0, dy: 0, a: c.a });
    });

    // The crown: clusters of leaves round and over everyone up the tree,
    // each in the round — its shade under it, its body, the light on its
    // upper left — behind the wood, so that every stem is seen; with no
    // one up the tree, the crown is the tree's own. The scene paints them,
    // under the drawing (woods()): [x, y, radius, tone, opacity], tones
    // from the shade to the sun, then the dome's two.
    var FOL = [];
    function cluster(cx, cy, r, seed, a) {
      FOL.push([cx + r * 0.1, cy + r * 0.16, r * 0.98, 0, a]);
      for (var i = 0; i < 7; i++) {
        var an = i / 7 * 6.283 + hash(seed, i) * 0.8, d = r * (0.4 + 0.2 * hash(seed, i + 7));
        FOL.push([cx + Math.cos(an) * d, cy + Math.sin(an) * d * 0.82, r * (0.4 + 0.16 * hash(seed, i + 13)), 1, a]);
      }
      for (i = 0; i < 4; i++) {
        an = -2.2 + (hash(seed, i + 20) - 0.5) * 1.5; d = r * (0.28 + 0.26 * hash(seed, i + 27));
        FOL.push([cx + Math.cos(an) * d, cy + Math.sin(an) * d, r * (0.22 + 0.14 * hash(seed, i + 33)), 2, a]);
      }
      for (i = 0; i < 6; i++) {
        an = -2.3 + (hash(seed, i + 40) - 0.5) * 1.9; d = r * (0.5 + 0.38 * hash(seed, i + 47));
        FOL.push([cx + Math.cos(an) * d, cy + Math.sin(an) * d, r * (0.05 + 0.06 * hash(seed, i + 53)), 3, a]);
      }
    }
    var crownP = fr.persons.filter(function (p) { return p.y < crownTop; });
    var topY = crownP.reduce(function (s, p) { return Math.min(s, p.y); }, Infinity);
    // Behind it all, the crown's mass: a dome of foliage from the top of the
    // trunk to over the highest people, as wide as they reach, darker inside
    // and lit along its upper left.
    var reachX = crownP.reduce(function (s, p) { return Math.max(s, Math.abs(p.x - G.x0) + frameOf(p).rx); }, 0);
    var dRx = Math.max(reachX + 40, Math.min(hw * 0.3, 280)), dTop = crownP.length ? topY - 96 : yTop - G.H * 1.25, dBot = yTop + 24;
    var dRy = (dBot - dTop) / 2, dCy = dTop + dRy;
    for (var gyi = 0; gyi < 40; gyi++) {
      var yy = dTop + 30 + gyi * 46;
      if (yy > dBot - 6) break;
      var half = dRx * Math.sqrt(Math.max(0, 1 - Math.pow((yy - dCy) / dRy, 2)));
      for (var xx = G.x0 - half + 24; xx <= G.x0 + half - 24 + 0.1; xx += 58) {
        var key = 'dome' + gyi + ':' + Math.round(xx - G.x0), jx = (hash(key, 1) - 0.5) * 26, jy = (hash(key, 2) - 0.5) * 20, r = 40 + 22 * hash(key, 3);
        var rim = (xx - G.x0) / dRx * LX + (yy - dCy) / dRy * LY > 0.55 ? 5 : 4;
        FOL.push([xx + jx, yy + jy, r, 0, 1], [xx + jx - r * 0.18, yy + jy - r * 0.2, r * 0.72, rim, 1]);
        // Its leaves: specks of light on the upper left of each mass, of
        // shade on its lower right.
        for (var li = 0; li < 7; li++) {
          var la = -2.2 + (hash(key, li + 10) - 0.5) * 2.2, ld = r * (0.25 + 0.5 * hash(key, li + 20)), lit = li < 5;
          FOL.push([xx + jx + Math.cos(la) * ld * (lit ? 1 : -1), yy + jy + Math.sin(la) * ld * (lit ? 1 : -1), r * (0.05 + 0.07 * hash(key, li + 30)), lit ? 3 : 0, lit ? 0.55 : 0.8]);
        }
      }
    }
    crownP.forEach(function (p) {
      var fm = frameOf(p), out = p.x < G.x0 - 2 ? -1 : p.x > G.x0 + 2 ? 1 : (hash(p.key, 1) < 0.5 ? -1 : 1), k = p.k || 1;
      cluster(p.x + out * fm.rx * 1.1, p.y - fm.ry * 0.7, 44 * k, p.key, 0.96 * p.a);
      cluster(p.x - out * fm.rx * 0.9, p.y - fm.ry * 1.7, 38 * k, p.key + '+', 0.94 * p.a);
      if (p.y < topY + 12) cluster(p.x + out * fm.rx * 0.3, p.y - fm.ry * 2.9, 48 * k, p.key + '^', 0.95 * p.a);
    });
    fr.couples.forEach(function (c) {
      if (c.y >= crownTop || !(c.kids.length)) return;
      var out = c.x < G.x0 ? -1 : 1, k = c.k || 1;
      cluster(c.x + out * 26 * k, c.y - 10 * k, 30 * k, c.key, 0.9 * c.a);
    });
    // A tree has more than one or two limbs: where its family has fewer,
    // the trunk's own grow into the crown, bearing no one.
    if (crownP.length && limbs.length < 3) {
      [-0.62, 0, 0.62].forEach(function (u, j) {
        var tx = u * dRx;
        if (limbs.some(function (lx) { return Math.abs(lx - tx) < dRx * 0.3; })) return;
        crownW += grow({ x1: G.x0 + u * Wt * 0.2, y1: yTop + Wt * 0.3, x2: G.x0 + tx * 0.8, y2: yTop - G.H * (j === 1 ? 0.95 : 0.7), a: 1, conf: 'a', from: 'own' + j }, Wt * 0.32, 5, false);
      });
    }
    if (!crownP.length) {
      for (var ci = 0; ci < 7; ci++) {
        var an = Math.PI * (1.08 + 0.84 * ci / 6), rx = Math.min(hw * 0.26, 260), ry = G.H * 0.62;
        cluster(G.x0 + Math.cos(an) * rx, yTop - ry * 0.55 + Math.sin(an) * ry, 58, 'crown' + ci, 0.97);
      }
      cluster(G.x0, yTop - G.H * 0.62, 72, 'crown-top', 0.97);
      [-1, 0, 1].forEach(function (sd, j) {
        var e = { x1: G.x0 + sd * Wt * 0.2, y1: yTop, x2: G.x0 + sd * Math.min(hw * 0.2, 200), y2: yTop - G.H * (0.55 + 0.25 * (j === 1)), a: 1, conf: 'a' };
        crownW += grow(e, Wt * 0.42, 7, false);
      });
    }
    if (!fr.persons.some(function (p) { return p.y > rootTop; })) {
      [-1, 1].forEach(function (sd) {
        rootW += grow({ x1: G.x0 + sd * Wt * 0.25, y1: yBot, x2: G.x0 + sd * Math.min(hw * 0.2, 180), y2: yBot + G.H * 0.5, a: 1, conf: 'a' }, Wt * 0.5, 6, true);
      });
    }

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
    // A few leaves on the stems in front, never over a portrait or a card.
    var leavesFront = '';
    crownP.forEach(function (p) {
      var fm = F[p.key];
      for (var i = 0; i < 5; i++) {
        var an = (i / 5) * 6.283 + hash(p.key, i) * 0.9, rr = fm.rx * (1.5 + 0.7 * hash(p.key, i + 9));
        var lx = p.x + Math.cos(an) * rr, ly = p.y + Math.sin(an) * rr * 0.8, sz = fm.rx * (0.36 + 0.26 * hash(p.key, i + 17));
        if (hit([lx - sz, ly - sz, lx + sz, ly + sz])) continue;
        leavesFront += '<ellipse cx="' + f(lx) + '" cy="' + f(ly) + '" rx="' + f(sz) + '" ry="' + f(sz * 0.5) + '" transform="rotate(' + f(an * 57.3 + 30) + ' ' + f(lx) + ' ' + f(ly) + ')" class="gn-leaf-' + (1 + (i % 3)) + '" opacity="' + f(0.85 * p.a) + '"/>';
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
    // edge, so the two read as one; the person this reader may not see as
    // an empty frame. The heart's portrait is set into the trunk.
    var people = '';
    fr.persons.slice().sort(function (a, b) { return (a.k || 1) - (b.k || 1); }).forEach(function (p) {
      var s = p.s || {}, isC = p.role === 'centre', fm = F[p.key], x = f(p.x), y = f(p.y), g = '<g opacity="' + f(p.a) + '">';
      if (rest) g += '<a class="rv-p" data-id="' + esc(p.id) + '" data-key="' + esc(p.key) + '" href="' + api.base + '?p=' + encodeURIComponent(p.id) + '&amp;n=' + m.n + '"><ellipse cx="' + x + '" cy="' + y + '" rx="' + f(fm.rx + 6) + '" ry="' + f(fm.ry + 6) + '" fill="transparent"/>';
      var Lb = keep[p.key];
      if (Lb && !s.redacted) {
        g += '<g class="gn-plate">';
        g += '<rect class="gn-card-shade" x="' + f(Lb.x + 1.5) + '" y="' + f(Lb.y + 2.5) + '" width="' + f(Lb.w) + '" height="' + f(Lb.h) + '" rx="3"/>';
        g += '<rect class="gn-card' + (Lb.centre ? ' gn-card-centre' : '') + '" x="' + f(Lb.x) + '" y="' + f(Lb.y) + '" width="' + f(Lb.w) + '" height="' + f(Lb.h) + '" rx="3"/>';
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
      if (p.repeat > 1) g += '<ellipse cx="' + x + '" cy="' + y + '" rx="' + f(fm.rx + 3.2) + '" ry="' + f(fm.ry + 3.2) + '" fill="none" class="gn-again" stroke-width="1"/>';
      if (p.again) g += '<ellipse cx="' + x + '" cy="' + y + '" rx="' + f(fm.rx + 3.2 + (p.repeat > 1 ? 2.4 : 0)) + '" ry="' + f(fm.ry + 3.2 + (p.repeat > 1 ? 2.4 : 0)) + '" fill="none" class="gn-again" stroke-width="1" stroke-dasharray="2.4 1.8"/>';
      if (isC) g += '<ellipse cx="' + x + '" cy="' + y + '" rx="' + f(fm.rx + 10) + '" ry="' + f(fm.ry + 10) + '" class="gn-halo"/><ellipse cx="' + x + '" cy="' + y + '" rx="' + f(fm.rx + 4.5) + '" ry="' + f(fm.ry + 4.5) + '" class="gn-mount"/>';
      if (s.redacted) {
        g += '<ellipse cx="' + x + '" cy="' + y + '" rx="' + f(fm.rx) + '" ry="' + f(fm.ry) + '" class="gn-portrait gn-hidden" stroke-dasharray="1.8 1.8"/>';
      } else {
        g += '<ellipse cx="' + x + '" cy="' + y + '" rx="' + f(fm.rx) + '" ry="' + f(fm.ry) + '" class="gn-portrait' + (isC ? ' gn-portrait-centre' : s.living ? ' gn-living' : ' gn-gone') + (s.sparse ? ' gn-sparse' : '') + '"/>';
        g += '<ellipse cx="' + x + '" cy="' + y + '" rx="' + f(fm.rx - 2.6) + '" ry="' + f(fm.ry - 2.6) + '" class="gn-portrait-inner"/>';
        var ini = initials(s.names && s.names[2]);
        if (ini && fm.rx >= 8.5) g += '<text x="' + x + '" y="' + f(p.y + fm.rx * 0.36) + '" text-anchor="middle" font-size="' + f(fm.rx * 0.9) + '" class="gn-initials">' + esc(ini) + '</text>';
      }
      if (rest) g += '</a>';
      people += g + '</g>';
    });
    if (rest) {
      named = {};
      Object.keys(keep).forEach(function (k) { if (!keep[k].centre) named[k] = keep[k]; });
    }
    var defs = '<linearGradient id="gn-bark" x1="0" x2="1" y1="0" y2="0"><stop offset="0" class="gn-bark-dark"/><stop offset="0.22" class="gn-bark-lit"/><stop offset="0.5" class="gn-bark-mid"/><stop offset="1" class="gn-bark-dark"/></linearGradient>';
    var head = '<svg class="rv-svg gn-svg" data-view="scroll" viewBox="' + f(hl) + ' ' + f(vt) + ' ' + f(hw) + ' ' + f(vh) + '" width="100%" '
      + (rest ? 'role="img" aria-label="' + esc(label() + (C && C.s && !C.s.redacted ? ' — ' + C.s.names[2] : '')) + '"' : 'aria-hidden="true"')
      + ' xmlns="http://www.w3.org/2000/svg"><defs>' + defs + '</defs>';
    // While the camera climbs, the far woods follow it (the scene, below).
    if (!rest && window.axgfRiverScene) window.axgfRiverScene.view();
    // The lit route lies over the wood and under the people (river.js).
    GEO.foliage = FOL; GEO.fv = ++leafage;
    return head + '<path class="gn-rootlet" d="' + rootlets + '"/>' + rootW + crownW + trunk + rail + tw + knots + leavesFront + '<g class="rv-lit"></g>' + marks + people + '</svg>';
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

  /* ---- the woods' life ---- */
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
  var GOLD = rgb('--gn-gold', [200, 160, 80]), GLINT = rgb('--gn-glint', GOLD), LEAF = rgb('--gn-leaf', GOLD);
  var MIST = rgb('--gn-mist', [150, 170, 150]), SHADE = rgb('--gn-woods-shade', [6, 16, 11]);
  var SOIL = rgb('--gn-soil-top', [46, 36, 22]), DEEP = rgb('--gn-soil-deep', [15, 12, 7]), PEBBLE = rgb('--gn-pebble', SOIL);
  var MOSS = rgb('--gn-moss', [78, 107, 44]), GRASS = rgb('--gn-grass', MOSS);
  function rgba(c, a) { return 'rgba(' + c[0] + ',' + c[1] + ',' + c[2] + ',' + a.toFixed(3) + ')'; }

  // Under the drawing, the far woods, the mist, the motes and the leaves;
  // over it, the glints up the branches. The woods are a picture (made in
  // the operator's Canva, docs/adr/0016), the tree in front of them drawn.
  var WOODS = new Image(), woodsOk = false;
  WOODS.onload = function () { woodsOk = true; if (!running) draw(false); };
  WOODS.src = '/static/img/genealogy-woods.jpg?v=1';
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
  // The same woods, motes and leaves on every visit.
  function rnd(i, j) { var h = Math.abs(Math.sin(i * 12.9898 + j * 78.233) * 43758.5453); return h - Math.floor(h); }
  var LEAVES = [], MOTES = [];
  for (var li = 0; li < 18; li++) LEAVES.push({ u: rnd(li, 1), v: rnd(li, 2), fall: 0.016 + 0.02 * rnd(li, 3), sway: 0.6 + 1.2 * rnd(li, 4), size: 3 + 3.5 * rnd(li, 5), spin: 0.4 + rnd(li, 6) });
  for (var mi = 0; mi < 40; mi++) MOTES.push({ u: rnd(mi, 7), v: rnd(mi, 8), rise: 0.008 + 0.014 * rnd(mi, 9), r: 0.7 + 1.4 * rnd(mi, 10), ph: rnd(mi, 11) * 6.283 });

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
  // A glint runs up every attested branch, from its lower end to its upper:
  // roots to trunk to crown.
  function layout(L) {
    flows = [];
    if (!L || !L.edges) return;
    L.edges.forEach(function (e) {
      if (e.conf !== 'a') return;
      var b = bend(e.x1, e.y1, e.x2, e.y2), c = e.y1 >= e.y2 ? b : [b[6], b[7], b[4], b[5], b[2], b[3], b[0], b[1]], n = 22, xs = [], ys = [], cum = [0];
      for (var i = 0; i <= n; i++) {
        var t = i / n, u = 1 - t;
        xs.push(u * u * u * c[0] + 3 * u * u * t * c[2] + 3 * u * t * t * c[4] + t * t * t * c[6]);
        ys.push(u * u * u * c[1] + 3 * u * u * t * c[3] + 3 * u * t * t * c[5] + t * t * t * c[7]);
        if (i) cum.push(cum[i - 1] + Math.hypot(xs[i] - xs[i - 1], ys[i] - ys[i - 1]));
      }
      if (cum[n] < 12) return;
      flows.push({ x: xs, y: ys, cum: cum, n: n, len: cum[n], phase: Math.abs(Math.sin(e.x1 * 12.9898 + e.y2 * 78.233)) % 1 });
    });
    request();
  }
  var SPEED = 30;
  // What does not move between two drawings of the tree — the woods, the
  // ground, the crown — is painted once into a still layer, and only the
  // mist, the motes and the leaves over it at every frame.
  var calm = document.createElement('canvas'), sx = calm.getContext('2d'), stillKey = '';
  var TONES = ['--gn-clump-0', '--gn-clump-1', '--gn-clump-2', '--gn-clump-3', '--gn-dome-1', '--gn-dome-2'].map(function (k) { return rgb(k, [30, 60, 35]); });
  var TONE_A = [0.9, 1, 0.9, 0.6, 1, 1];
  // The woods: the picture over the whole window, a little taller than it,
  // sinking as the reader climbs and rising as they go down, slowly, as
  // distant things do, and shaded toward its edges so that the tree stands
  // out. The ground, the whole width of the window: soil from a little
  // under the fixed point down, rising in a mound round the trunk, with its
  // stones, moss and grass along its edge; it holds still as the reader
  // climbs, the tree coming up out of it or going down into it. Then the
  // crown's leaves, behind the tree's wood.
  function paintStill(v, G, Wd, Hh) {
    if (calm.width !== Wd || calm.height !== Hh) { calm.width = Wd; calm.height = Hh; }
    var g = sx;
    g.setTransform(1, 0, 0, 1, 0, 0);
    g.clearRect(0, 0, Wd, Hh);
    if (woodsOk) {
      var iw = WOODS.naturalWidth, ih = WOODS.naturalHeight, sc = Math.max(Wd / iw, Hh * 1.16 / ih), dw = iw * sc, dh = ih * sc;
      var par = Math.tanh((G && G.world || 0) * 0.16) * (dh - Hh) / 2;
      g.imageSmoothingQuality = 'high';
      g.drawImage(WOODS, (Wd - dw) / 2, (Hh - dh) / 2 + par, dw, dh);
    }
    var sh = g.createRadialGradient(Wd * 0.5, Hh * 0.42, Math.min(Wd, Hh) * 0.2, Wd * 0.5, Hh * 0.5, Math.max(Wd, Hh) * 0.75);
    sh.addColorStop(0, rgba(SHADE, 0.28)); sh.addColorStop(1, rgba(SHADE, 0.72));
    g.fillStyle = sh; g.fillRect(0, 0, Wd, Hh);
    var s = v.k * dpr, gy = G && G.yG !== undefined ? (G.yG - v.y) * s : null;
    if (gy !== null && gy < Hh) {
      var edge = [];
      for (var px = -16 * dpr; px <= Wd + 16 * dpr; px += 12 * dpr) {
        var wx = v.x + px / s;
        edge.push([px, gy + (-16 * Math.exp(-Math.pow((wx - G.x0) / (G.Wt * 2.4), 2)) + (rnd(Math.round(wx / 12), 3) - 0.5) * 3) * s]);
      }
      g.beginPath();
      g.moveTo(edge[0][0], edge[0][1]);
      edge.forEach(function (p) { g.lineTo(p[0], p[1]); });
      g.lineTo(Wd + 16 * dpr, Hh + 16); g.lineTo(-16 * dpr, Hh + 16); g.closePath();
      var sg = g.createLinearGradient(0, gy - 16 * s, 0, Hh);
      sg.addColorStop(0, rgba(SOIL, 0.97)); sg.addColorStop(1, rgba(DEEP, 0.99));
      g.fillStyle = sg; g.fill();
      g.fillStyle = rgba(PEBBLE, 0.55);
      for (var pi = 0; pi < 70; pi++) {
        var ppx = ((rnd(pi, 21) * 1.4 - 0.2) * Wd), ppy = gy + (14 + rnd(pi, 22) * 900) * s;
        if (ppy > Hh + 8) continue;
        g.beginPath(); g.ellipse(ppx, ppy, (2 + 4 * rnd(pi, 23)) * s, (1.3 + 2 * rnd(pi, 24)) * s, rnd(pi, 25) * 3, 0, 6.283); g.fill();
      }
      g.lineJoin = 'round'; g.lineCap = 'round';
      g.strokeStyle = rgba(MOSS, 1); g.lineWidth = 5 * s;
      g.beginPath(); g.moveTo(edge[0][0], edge[0][1]); edge.forEach(function (p) { g.lineTo(p[0], p[1]); }); g.stroke();
      g.strokeStyle = rgba(GRASS, 0.95); g.lineWidth = 1.3 * s;
      g.beginPath();
      for (var gi = 0; gi < edge.length - 1; gi++) {
        for (var b = 0; b < 3; b++) {
          var u = (b + rnd(gi, b + 30)) / 3, bx = edge[gi][0] + (edge[gi + 1][0] - edge[gi][0]) * u, by = edge[gi][1] + (edge[gi + 1][1] - edge[gi][1]) * u + 1.5 * s;
          var bh = (5 + 9 * rnd(gi, b + 40)) * s, lean = (rnd(gi, b + 50) - 0.5) * 7 * s;
          g.moveTo(bx, by); g.quadraticCurveTo(bx + lean * 0.3, by - bh * 0.6, bx + lean, by - bh);
        }
      }
      g.stroke();
    }
    (G && G.foliage || []).forEach(function (q) {
      g.fillStyle = rgba(TONES[q[3]], TONE_A[q[3]] * q[4]);
      g.beginPath(); g.arc((q[0] - v.x) * s, (q[1] - v.y) * s, Math.max(0.5, q[2] * s), 0, 6.283); g.fill();
    });
    return gy;
  }
  var groundY = null;
  function woods(moving) {
    var Wd = ground.width, Hh = ground.height, t = moving ? clock : 4.2, v = view(), G = GEO;
    gx.setTransform(1, 0, 0, 1, 0, 0);
    gx.clearRect(0, 0, Wd, Hh);
    if (!v) return;
    var key = [v.x, v.y, v.k, Wd, Hh, G && G.world, G && G.fv, woodsOk].join(',');
    if (key !== stillKey) { groundY = paintStill(v, G, Wd, Hh); stillKey = key; }
    gx.drawImage(calm, 0, 0);
    var gy = groundY;
    // Mist across the middle of the window, drifting.
    var my = Hh * 0.58 + Math.sin(t * 0.15) * 12 * dpr, mg = gx.createLinearGradient(0, my - 90 * dpr, 0, my + 90 * dpr);
    mg.addColorStop(0, rgba(MIST, 0)); mg.addColorStop(0.5, rgba(MIST, 0.07)); mg.addColorStop(1, rgba(MIST, 0));
    gx.fillStyle = mg; gx.fillRect(0, my - 90 * dpr, Wd, Math.max(0, Math.min(180 * dpr, (gy === null ? Hh : gy) - (my - 90 * dpr))));
    // Motes rising, and leaves falling across the whole window.
    gx.globalCompositeOperation = 'lighter';
    MOTES.forEach(function (mo) {
      var y = (1.05 - ((mo.v + t * mo.rise) % 1.1)) * Hh, x = (mo.u + 0.02 * Math.sin(t * 0.7 + mo.ph)) * Wd;
      gx.fillStyle = rgba(GOLD, 0.25 + 0.3 * (0.5 + 0.5 * Math.sin(t * 1.3 + mo.ph)));
      gx.beginPath(); gx.arc(x, y, mo.r * dpr, 0, 6.283); gx.fill();
    });
    gx.globalCompositeOperation = 'source-over';
    LEAVES.forEach(function (lf) {
      var y = ((lf.v + t * lf.fall) % 1.1 - 0.05) * Hh, x = (lf.u + 0.03 * Math.sin(t * 0.5 * lf.sway + lf.u * 9)) * Wd;
      var a = 0.55 * Math.min(1, (y / Hh + 0.05) * 6, (1.05 - y / Hh) * 4), r = lf.size * dpr, turn = t * lf.spin + lf.u * 6.283;
      gx.save();
      gx.translate(x, y);
      gx.rotate(turn);
      gx.scale(1, 0.45 + 0.35 * Math.abs(Math.sin(turn)));
      gx.fillStyle = rgba(LEAF, Math.max(0, a));
      gx.beginPath(); gx.ellipse(0, 0, r, r * 0.48, 0, 0, 6.283); gx.fill();
      gx.restore();
    });
  }
  function draw(moving) {
    size();
    woods(moving);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, layer.width, layer.height);
    var v = view();
    if (!v || !moving || travelling) return;
    var s = v.k * dpr, ox = -v.x * s, oy = -v.y * s;
    ctx.globalCompositeOperation = 'lighter';
    flows.forEach(function (fl) {
      var pos = (fl.phase * fl.len + clock * SPEED) % (fl.len + 80), i = 1;
      if (pos > fl.len) return;
      while (i < fl.n && fl.cum[i] < pos) i++;
      var u = (pos - fl.cum[i - 1]) / ((fl.cum[i] - fl.cum[i - 1]) || 1);
      var x = (fl.x[i - 1] + (fl.x[i] - fl.x[i - 1]) * u) * s + ox, y = (fl.y[i - 1] + (fl.y[i] - fl.y[i - 1]) * u) * s + oy;
      var fade = Math.min(1, pos / 16, (fl.len - pos) / 16), r = 2.8 * v.k * dpr;
      var g = ctx.createRadialGradient(x, y, 0, x, y, r * 2.6);
      g.addColorStop(0, rgba(GLINT, 0.95 * fade));
      g.addColorStop(0.4, rgba(GOLD, 0.45 * fade));
      g.addColorStop(1, rgba(GOLD, 0));
      ctx.fillStyle = g;
      ctx.beginPath(); ctx.arc(x, y, r * 2.6, 0, 6.283); ctx.fill();
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
