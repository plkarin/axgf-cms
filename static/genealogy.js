/* Genealogy's tree, climbed (v0.5.0, docs/adr/0016; redrawn 2026-10-23).
 *
 * The river's own people — from the same payload, through the same reader's
 * lens, in the same order from left to right — on a tree drawn in
 * perspective, which the reader climbs: the camera stands at the height of
 * the person at the fixed point, a little in front of the trunk, and the
 * trunk runs through that person. Descendants are up the tree, on branches
 * that climb and spread; ancestors are down it, toward the roots. A
 * generation further up or down is further along the trunk from the camera:
 * drawn closer to the last and smaller, as it would look from where the
 * reader stands. Each generation is a level around the trunk, its people
 * spread round it in the river's order, the nearest at the front.
 *
 * Travelling up to a descendant, the reader climbs: the camera rises along
 * the trunk, the tree slides down past it and the person travelled to grows
 * as they come level with it; travelling to an ancestor, the reader climbs
 * down. The blend is done in the tree's own space (`tween3`, through
 * river.js's travel), and the far woods behind the tree move with the
 * climb, more slowly, as distant things do.
 *
 * Each person is a portrait in an oval gold frame — their initials, since
 * the payload carries no picture — with a cream card beside it, outward
 * from the trunk, that names them; whom the drawing has no room for, the
 * pointer names, or river.js when every name is asked for. A branch is lit
 * wood edged in dark bark, as wide as it is near the trunk, and says what
 * the parentage is worth: whole for an attested one, broken for an inferred
 * one, a line of knots for a speculative one.
 *
 * Two things the page knows by name:
 * - `window.axgfView` (river.js): where everyone stands (`shape`), how a
 *   branch bends (`edge`, which the lit route follows too), how a frame is
 *   painted at rest and in travel, which people the drawing named, the
 *   dashboard at rest, and how two frames blend (`tween`).
 * - `window.axgfRiverScene`: the woods' life — the far trees behind, gold
 *   motes rising, leaves falling, and a glint run up every attested branch.
 *   It holds still as the river's ground does: reduced motion, "Cut", the
 *   pause, a hidden tab, a tree off screen, a machine too slow for it.
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
    var couples = fr.couples.map(function (c) {
      var ps = c.parents.map(function (k) { return NK[k]; }).filter(Boolean), ks = c.kids.map(function (k) { return NK[k]; }).filter(Boolean);
      var from = ps.length ? ps : ks, th = from.length ? from.reduce(function (s, q) { return s + q.th; }, 0) / from.length : (c.x - cx) / reachR * THETA;
      var l = lev(c.y), q = proj(G, th, l);
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
  // A branch leaves and reaches each end upright, as wood grows.
  function edge(x1, y1, x2, y2) {
    var r = function (v) { return Math.round(v * 10) / 10; }, my = (y1 + y2) / 2;
    return 'M' + r(x1) + ' ' + r(y1) + 'C' + r(x1) + ' ' + r(my) + ' ' + r(x2) + ' ' + r(my) + ' ' + r(x2) + ' ' + r(y2);
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
  // A branch's width: the trunk's near the fixed point, narrowing a level
  // further out each time, never under a twig's.
  function thick(gen) { return Math.max(1.6, 9 * Math.pow(0.64, Math.abs(gen))); }
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
  var named = {};
  function paint(fr, api, rest) {
    var f = api.f, esc = api.esc, PM = api.byKey(fr.persons), m = fr.meta;
    var vt = m.view[0], vh = m.view[1], hl = m.hview[0], hw = m.hview[1];
    if (rest && pending) { world += pending; pending = 0; }
    var G = m.gn || GEO || { x0: hl + hw / 2, y0: vt + vh / 2, R: hw / 3, H: 120, nd: 0, na: 0 };
    var W = G.world !== undefined ? G.world : world;
    GEO = Object.assign({}, G, { world: W });
    var climb = W * G.H;
    var ringsBack = '', ringsFront = '', trunk = '', branches = '', leaves = '', tw = '', marks = '', rail = '';
    // The levels: a faint ring round the trunk at each generation's height,
    // its back half behind the trunk and its front half before it; and the
    // generation's number at the right, as the river's rail.
    var boxes = [];
    function hit(box) {
      for (var i = 0; i < boxes.length; i++) {
        var q = boxes[i];
        if (box[0] < q[2] && box[2] > q[0] && box[1] < q[3] && box[3] > q[1]) return true;
      }
      return false;
    }
    for (var l = -G.na; l <= G.nd; l++) {
      var ly = G.y0 - lift(l) * G.H, kl = Math.exp(-SHRINK * Math.abs(l)), rx = G.R * kl * 1.02, ry = rx * 0.07;
      var cls = 'gn-ring' + (l === 0 ? ' gn-ring-0' : '');
      ringsBack += '<path class="' + cls + '" d="M' + f(G.x0 - rx) + ' ' + f(ly) + 'A' + f(rx) + ' ' + f(ry) + ' 0 0 1 ' + f(G.x0 + rx) + ' ' + f(ly) + '"/>';
      ringsFront += '<path class="' + cls + '" d="M' + f(G.x0 - rx) + ' ' + f(ly) + 'A' + f(rx) + ' ' + f(ry) + ' 0 0 0 ' + f(G.x0 + rx) + ' ' + f(ly) + '"/>';
      var word = l > 0 ? '+' + l : l < 0 ? '−' + (-l) : '0 ◂', gx = hl + hw - 44;
      rail += '<text x="' + f(gx) + '" y="' + f(ly + 3.5) + '" font-family="' + api.mono + '" font-size="10" class="gn-gen' + (l === 0 ? ' gn-gen-0' : '') + '">' + word + '</text>';
      boxes.push([gx - 2, ly - 8, gx + 34, ly + 5]);
    }
    // The trunk, the whole height of the window: wider toward the roots,
    // swaying a little, its bark lit from the left. The sway and the bark's
    // grain belong to the tree, so they slide past as the reader climbs.
    function sway(y) { return Math.sin((y + climb) / 190) * 7 + Math.sin((y + climb) / 77) * 2.5; }
    function girth(y) { var t = clamp((y - vt) / vh, 0, 1); return 22 + 46 * t * t + 10 * t; }
    var y1 = vt - 20, y2 = vt + vh + 20, L = [], Rr = [];
    for (var y = y1; y <= y2 + 0.1; y += 18) {
      var c = G.x0 + sway(y), w = girth(y);
      L.push(f(c - w / 2) + ' ' + f(y)); Rr.unshift(f(c + w / 2) + ' ' + f(y));
    }
    trunk += '<path class="gn-trunk" d="M' + L.join('L') + 'L' + Rr.join('L') + 'Z" fill="url(#gn-bark)"/>';
    // The grain: lines down the trunk, each a fraction of the way across it.
    [0.18, 0.34, 0.52, 0.68, 0.84].forEach(function (fx, i) {
      var pts = [];
      for (var y = y1; y <= y2 + 0.1; y += 14) {
        var w = girth(y), c = G.x0 + sway(y);
        pts.push(f(c - w / 2 + w * fx + Math.sin((y + climb) / (23 + i * 7) + i) * w * 0.035) + ' ' + f(y));
      }
      trunk += '<path class="gn-grain" d="M' + pts.join('L') + '"/>';
    });

    // Lines the record continues past the last level, and lines it ends.
    var counts = [];
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

    // The branches: dark bark round lit wood, as wide as they are near the
    // trunk; below the fixed point the wood is the roots' darker grain.
    api.edges(fr, PM).forEach(function (e) {
      var a = e.from ? PM[e.from] : null, k = e.to ? PM[e.to] : null, cp = e.cp || {};
      var gen = Math.max(a ? Math.abs(a.gen) : 0, k ? Math.abs(k.gen) : 0), kk = ((a ? a.k : cp.k || 1) + (k ? k.k : cp.k || 1)) / 2;
      var w = thick(gen) * (kk || 1);
      if (a && (a.role === 'spouse' || a.role === 'dspouse')) w *= 0.65;
      var low = (e.y1 + e.y2) / 2 > G.y0 + 2, d = edge(e.x1, e.y1, e.x2, e.y2), o = e.a < 1 ? ' opacity="' + f(e.a) + '"' : '';
      branches += '<path d="' + d + '" class="gn-branch' + (low ? ' gn-root' : '') + '" stroke-width="' + f(w + 2.4) + '"' + o + '/>';
      if (e.conf === 'a') branches += '<path d="' + d + '" class="gn-branch-core' + (low ? ' gn-root-core' : '') + '" stroke-width="' + f(Math.max(1.1, w * 0.55)) + '"' + o + '/>';
      else if (e.conf === 'd') branches += '<path d="' + d + '" class="gn-branch-core' + (low ? ' gn-root-core' : '') + '" stroke-width="' + f(Math.max(1.1, w * 0.55)) + '" stroke-dasharray="6 4"' + o + '/>';
      else branches += '<path d="' + d + '" class="gn-branch-core gn-faint" stroke-width="' + f(Math.max(1.6, w * 0.5)) + '" stroke-dasharray="0.1 6"' + o + '/>';
    });
    // Each family a knot where its branches meet.
    var knots = '';
    fr.couples.forEach(function (c) {
      if (c.parents.length + c.kids.length > 1) knots += '<circle cx="' + f(c.x) + '" cy="' + f(c.y) + '" r="' + f(Math.max(2.2, 4 * (c.k || 1))) + '" class="gn-knot" opacity="' + f(c.a) + '"/>';
    });
    // Leaves round every descendant, the same on every visit.
    fr.persons.forEach(function (p) {
      if (!(p.gen > 0) || (p.s && p.s.redacted)) return;
      var fm = frameOf(p), n = 7;
      for (var i = 0; i < n; i++) {
        var an = (i / n) * 6.283 + hash(p.key, i) * 0.9, rr = fm.rx * (1.55 + 0.6 * hash(p.key, i + 9));
        var lx = p.x + Math.cos(an) * rr, ly = p.y - fm.ry * 0.35 + Math.sin(an) * rr * 0.75, s = fm.rx * (0.42 + 0.3 * hash(p.key, i + 17));
        leaves += '<ellipse cx="' + f(lx) + '" cy="' + f(ly) + '" rx="' + f(s) + '" ry="' + f(s * 0.55) + '" transform="rotate(' + f(an * 57.3 + 30) + ' ' + f(lx) + ' ' + f(ly) + ')" class="gn-leaf-' + (1 + (i % 3)) + '" opacity="' + f(0.8 * p.a) + '"/>';
      }
    });

    // Cards beside the portraits, outward from the trunk: the heart's first,
    // under its portrait, then the nearest levels'. A card that would cover
    // another is left out, as the river leaves out a word it has no room
    // for; the pointer brings it back.
    var keep = {}, C = centreOf(fr), cs = C && C.s || {};
    var F = {};
    fr.persons.forEach(function (p) {
      var fm = frameOf(p);
      F[p.key] = fm;
      boxes.push([p.x - fm.rx - 2, p.y - fm.ry - 2, p.x + fm.rx + 2, p.y + fm.ry + 2]);
    });
    if (C && !cs.redacted) {
      var cf = F[C.key], cn = api.clip(cs.names[2], 240), cw = Math.max(api.width(cn, 15, true), api.width(cs.years, 10.5, true)) + 22;
      var at = null, least = Infinity, tries = [];
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
        if (n < least) { least = n; at = { x: o[0], y: o[1], side: o[2], box: box }; }
      });
      if (!at) at = { x: C.x - cw / 2, y: C.y + cf.ry + 8, side: 0, box: [C.x - cw / 2 - 2, C.y + cf.ry + 6, C.x + cw / 2 + 2, C.y + cf.ry + 46] };
      keep[C.key] = { x: at.x, y: at.y, w: cw, h: 36, side: at.side, centre: true, word: cn, both: true };
      boxes.push(at.box);
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
      marks += '<g class="gn-count" opacity="' + f(c.a) + '">' + api.mark(Object.assign({}, mk, { x: mk.x + at[0], y: mk.y + at[1] }), c.text, c.size, c.dx, c.dy) + '</g>';
    });

    // People, the furthest first: a card, then the portrait over its inner
    // edge, so the two read as one; the person this reader may not see as
    // an empty frame.
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
      if (isC) g += '<ellipse cx="' + x + '" cy="' + y + '" rx="' + f(fm.rx + 10) + '" ry="' + f(fm.ry + 10) + '" class="gn-halo"/>';
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
    var defs = '<linearGradient id="gn-bark" x1="0" x2="1" y1="0" y2="0"><stop offset="0" class="gn-bark-dark"/><stop offset="0.28" class="gn-bark-lit"/><stop offset="0.55" class="gn-bark-mid"/><stop offset="1" class="gn-bark-dark"/></linearGradient>';
    var head = '<svg class="rv-svg gn-svg" data-view="scroll" viewBox="' + f(hl) + ' ' + f(vt) + ' ' + f(hw) + ' ' + f(vh) + '" width="100%" '
      + (rest ? 'role="img" aria-label="' + esc(label() + (C && C.s && !C.s.redacted ? ' — ' + C.s.names[2] : '')) + '"' : 'aria-hidden="true"')
      + ' xmlns="http://www.w3.org/2000/svg"><defs>' + defs + '</defs>';
    // While the camera climbs, the far woods follow it (the scene, below).
    if (!rest && window.axgfRiverScene) window.axgfRiverScene.view();
    // The lit route lies over the branches and under the people (river.js).
    return head + ringsBack + trunk + ringsFront + rail + tw + branches + leaves + knots + '<g class="rv-lit"></g>' + marks + people + '</svg>';
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
  var FAR = rgb('--gn-far', [30, 50, 35]), FAR2 = rgb('--gn-far-2', FAR), MIST = rgb('--gn-mist', [150, 170, 150]);
  function rgba(c, a) { return 'rgba(' + c[0] + ',' + c[1] + ',' + c[2] + ',' + a.toFixed(3) + ')'; }

  // Under the drawing, the far woods, the mist, the motes and the leaves;
  // over it, the glints up the branches.
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
      var a = e.y1 >= e.y2 ? [e.x1, e.y1, e.x2, e.y2] : [e.x2, e.y2, e.x1, e.y1], my = (a[1] + a[3]) / 2;
      var c = [a[0], a[1], a[0], my, a[2], my, a[2], a[3]], n = 22, xs = [], ys = [], cum = [0];
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
  // The woods: two rows of far crowns behind the tree, the further darker
  // and slower, each sliding as the camera climbs; a band of mist; motes
  // rising; leaves falling.
  function woods(moving) {
    var Wd = ground.width, Hh = ground.height, t = moving ? clock : 4.2, v = view(), G = GEO;
    gx.setTransform(1, 0, 0, 1, 0, 0);
    gx.clearRect(0, 0, Wd, Hh);
    if (!v) return;
    var climb = G ? (G.world || 0) * G.H * v.k * dpr : 0;
    [[FAR2, 0.12, 150, 0.55], [FAR, 0.26, 110, 0.7]].forEach(function (row, ri) {
      var col = row[0], par = row[1], step = row[2] * dpr, alpha = row[3], span = 300 * dpr;
      var off = ((climb * par) % span + span) % span;
      for (var band = -1; band <= Math.ceil(Hh / span) + 1; band++) {
        var base = band * span + off;
        for (var x = -step; x < Wd + step; x += step * 0.55) {
          var i = Math.round(x / (step * 0.55)), h = (0.5 + 0.5 * rnd(i + ri * 31, band * 7 + 3)) * step * 1.4, cxp = x + rnd(i, band + 11) * step * 0.3;
          var g = gx.createLinearGradient(0, base - h, 0, base + span * 0.4);
          g.addColorStop(0, rgba(col, alpha));
          g.addColorStop(1, rgba(col, 0));
          gx.fillStyle = g;
          gx.beginPath();
          gx.ellipse(cxp, base, step * 0.42, h, 0, 0, 6.283);
          gx.fill();
        }
      }
    });
    // Mist across the middle of the window, drifting.
    var my = Hh * 0.58 + Math.sin(t * 0.15) * 12 * dpr, mg = gx.createLinearGradient(0, my - 90 * dpr, 0, my + 90 * dpr);
    mg.addColorStop(0, rgba(MIST, 0)); mg.addColorStop(0.5, rgba(MIST, 0.07)); mg.addColorStop(1, rgba(MIST, 0));
    gx.fillStyle = mg; gx.fillRect(0, my - 90 * dpr, Wd, 180 * dpr);
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
