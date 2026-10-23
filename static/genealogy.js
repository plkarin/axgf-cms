/* Genealogy's scroll (v0.5.0, docs/adr/0016).
 *
 * The river's own people — from the same payload, through the same reader's
 * lens, in the same order from left to right — set out on an old parchment
 * scroll that a tree grows from (static/img/genealogy-tree.jpg, made in the
 * operator's Canva after their reference, without the building in its
 * trunk). The person at the fixed point stands on the trunk. The family is a
 * hierarchy in rows, a generation a row: descendants above, toward the
 * crown, each family's children side by side over it with the parents
 * centred under them; ancestors below, toward the roots, each person's
 * father and mother under them. The river's order from left to right decides
 * who comes first; the spacing is the scroll's own.
 *
 * Each person is a portrait in an oval frame — their initials, since the
 * payload carries no picture — with a cream card beside it, outward from the
 * trunk, that names them; whom the drawing has no room for, the pointer
 * names, or river.js when every name is asked for. The lines between them
 * are thin and gold and turn square corners, as an engraved family tree's
 * do: an attested parentage a whole line, an inferred one a broken line, a
 * speculative one a trace of dots, and each family a gold knot.
 *
 * Two things the page knows by name:
 * - `window.axgfView` (river.js): where everyone stands (`shape`), how a
 *   line turns (`edge`, which the lit route follows too), how a frame is
 *   painted at rest and in travel, which people the drawing named, and the
 *   dashboard at rest.
 * - `window.axgfRiverScene`: the life of the scroll — a few gold leaves
 *   falling from the crown, and a glint of light run along every attested
 *   line, older to newer. It holds still as the river's ground does:
 *   reduced motion, "Cut", the pause, a hidden tab, a scroll off screen, a
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

  /* ---- the scroll ---- */
  // The picture: the tree's trunk at its middle, the crown above the
  // scroll's band, the roots on its lower roll. `mid` is where the row of
  // the person at the fixed point goes in it.
  var TREE = { src: '/static/img/genealogy-tree.jpg?v=1', aspect: 1680 / 944, trunk: 0.5, mid: 0.6 };
  var GEO = null;
  function clamp(v, a, b) { return Math.max(a, Math.min(b, v)); }
  function centreOf(fr) { return fr.persons.filter(function (p) { return p.role === 'centre'; })[0]; }

  /* ---- where everyone stands: a family tree in rows ---- */
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
    // Each side spread over the scroll's width from the trunk, its places as
    // wide as the side allows and never wider than a card needs; a row per
    // generation, as tall as the window allows.
    var R = Math.max(140, hw * 0.46);
    var span = nd + na, H = clamp((vh - 120) / Math.max(1, span), 44, 132);
    var y0 = vt + (vh - span * H) / 2 + nd * H;
    var G = { x0: hl + hw / 2, y0: y0, R: R, H: H, nd: nd, na: na };
    function unit(X) {
      if (!C || !(C.key in X)) return 0;
      var reach = 0;
      Object.keys(X).forEach(function (k) { reach = Math.max(reach, Math.abs(X[k] - X[C.key])); });
      return reach ? Math.min(176, R * 0.94 / reach) : 176;
    }
    var ud = unit(XD), ua = unit(XA), oD = C ? XD[C.key] : 0, oA = C ? XA[C.key] : 0;
    G.ud = ud || 176; G.ua = ua || 176;
    // Whoever neither tree reaches keeps the river's place, scaled to the scroll.
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
      return Object.assign({}, p, { x: x, y: y });
    });
    var NK = byKey(persons);
    var couples = fr.couples.map(function (c) {
      var ps = c.parents.map(function (k) { return NK[k]; }).filter(Boolean), ks = c.kids.map(function (k) { return NK[k]; }).filter(Boolean);
      var from = ps.length ? ps : ks, x = from.length ? from.reduce(function (s, q) { return s + q.x; }, 0) / from.length : G.x0 + (c.x - cx) * sr;
      var y = G.y0 - lev(c.y) * G.H, o = Object.assign({}, c, { x: x, y: y });
      at[c.x + ',' + c.y] = [x, y];
      if (c.stub) {
        // Brothers and sisters not drawn: a short line off the family
        // toward the children's row, away from the trunk.
        var side = x < G.x0 ? -1 : 1, vert = ks.length && ks[0].y < y ? -1 : 1;
        o.stub_dx = side * 20; o.stub_dy = vert * 14;
        o.stub_at = markAt(x + side * 20, y + vert * 14, side, vert);
      }
      return o;
    });
    // A line the record continues past the last row runs on up (or down),
    // with its count beyond; a line it ends, a short one.
    var tails = fr.tails.map(function (t) {
      var s = at[t.x1 + ',' + t.y1] || [G.x0 + (t.x1 - cx) * sr, G.y0 - lev(t.y1) * G.H];
      var dir = t.y2 < t.y1 ? -1 : 1, len = (t.kind === 'cont' ? 0.42 : 0.24) * G.H;
      var e = [s[0], s[1] + dir * len];
      return Object.assign({}, t, { x1: s[0], y1: s[1], x2: e[0], y2: e[1], count_at: t.kind === 'cont' ? markAt(e[0], e[1], 0, dir) : null });
    });
    GEO = G;
    return { persons: persons, couples: couples, tails: tails, meta: Object.assign({}, m, { gn: G }) };
  }
  // Where a count stands at the end of a line: beside it, on its side.
  function markAt(x, y, side, vertical) {
    return { x: x + side * 6, y: y + (side ? 3.5 : vertical > 0 ? 13 : -6), anchor: side, halo: true };
  }
  // A line between two rows turns square corners, as an engraved tree's:
  // straight down (or up) to half way, across, and on.
  function edge(x1, y1, x2, y2) {
    var r = function (v) { return Math.round(v * 10) / 10; }, my = r((y1 + y2) / 2);
    if (Math.abs(x1 - x2) < 0.5) return 'M' + r(x1) + ' ' + r(y1) + 'V' + r(y2);
    return 'M' + r(x1) + ' ' + r(y1) + 'V' + my + 'H' + r(x2) + 'V' + r(y2);
  }

  /* ---- the drawing ---- */
  // A portrait's frame: the heart's the largest, the near rows' next, then
  // smaller a row further out each time, never wider than the places allow.
  function frameOf(p, G) {
    var g = Math.abs(p.gen), rx = p.role === 'centre' ? 21 : g <= 1 ? 15 : g === 2 ? 12 : 9.5;
    var room = (p.gen >= 0 ? G.ud : G.ua) * 0.34;
    if (p.role !== 'centre') rx = Math.max(7, Math.min(rx, room));
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
  var named = {};
  function paint(fr, api, rest) {
    var f = api.f, esc = api.esc, PM = api.byKey(fr.persons), m = fr.meta;
    var vt = m.view[0], vh = m.view[1], hl = m.hview[0], hw = m.hview[1];
    var G = m.gn || GEO || { x0: hl + hw / 2, y0: vt + vh / 2, R: hw / 3, H: 90, nd: 0, na: 0, ud: 120, ua: 120 };
    GEO = G;
    var out = '', lines = '', tw = '', marks = '', knots = '';
    var top = G.y0 - G.nd * G.H, bottom = G.y0 + G.na * G.H;
    // The scroll: the picture across the whole window, the trunk under the
    // person at the fixed point and their row where the trunk leaves the
    // scroll, and a veil that keeps the words readable over it.
    var Hi = Math.max(vh * 1.04, hw / TREE.aspect), Wi = Hi * TREE.aspect;
    var iy = clamp(G.y0 - TREE.mid * Hi, vt + vh - Hi, vt);
    out += '<image class="gn-tree" href="' + TREE.src + '" x="' + f(G.x0 - TREE.trunk * Wi) + '" y="' + f(iy) + '" width="' + f(Wi) + '" height="' + f(Hi) + '" preserveAspectRatio="none"/>';
    out += '<rect class="gn-veil" x="' + f(hl) + '" y="' + f(vt) + '" width="' + f(hw) + '" height="' + f(vh) + '"/>';
    var boxes = [];
    function hit(box) {
      for (var i = 0; i < boxes.length; i++) {
        var q = boxes[i];
        if (box[0] < q[2] && box[2] > q[0] && box[1] < q[3] && box[3] > q[1]) return true;
      }
      return false;
    }
    // A ruled line per generation, faint, and its number at the scroll's
    // right, as the river's rail.
    for (var l = -G.na; l <= G.nd; l++) {
      var ly = G.y0 - l * G.H;
      out += '<line class="gn-row' + (l === 0 ? ' gn-row-0' : '') + '" x1="' + f(hl + 18) + '" y1="' + f(ly) + '" x2="' + f(hl + hw - 52) + '" y2="' + f(ly) + '"/>';
      var word = l > 0 ? '+' + l : l < 0 ? '−' + (-l) : '0 ◂', gx = hl + hw - 44;
      out += '<text x="' + f(gx) + '" y="' + f(ly + 3.5) + '" font-family="' + api.mono + '" font-size="10" class="gn-gen' + (l === 0 ? ' gn-gen-0' : '') + '">' + word + '</text>';
      boxes.push([gx - 2, ly - 8, gx + 34, ly + 5]);
    }

    // Lines the record continues past the last row, and lines it ends.
    var counts = [];
    fr.tails.forEach(function (t) {
      var dx = t.x2 - t.x1, dy = t.y2 - t.y1, len = Math.hypot(dx, dy) || 1, ux = dx / len, uy = dy / len;
      tw += '<g opacity="' + f(t.a) + '"><path d="M' + f(t.x1) + ' ' + f(t.y1) + 'L' + f(t.x2) + ' ' + f(t.y2) + '" class="gn-line gn-twig"/>';
      if (t.kind === 'cont') {
        tw += '<polygon class="gn-tip" points="' + f(t.x2 - uy * 4) + ',' + f(t.y2 + ux * 4) + ' ' + f(t.x2 + uy * 4) + ',' + f(t.y2 - ux * 4) + ' ' + f(t.x2 + ux * 6) + ',' + f(t.y2 + uy * 6) + '"/>';
        if (t.count_at) counts.push({ mk: t.count_at, text: '+' + t.count, size: 9.5, dx: t.cref ? t.x2 - t.cref[0] : 0, dy: t.cref ? t.y2 - t.cref[1] : 0, a: t.a });
      } else {
        tw += '<circle cx="' + f(t.x2) + '" cy="' + f(t.y2) + '" r="3" class="gn-end"/>';
      }
      tw += '</g>';
    });
    fr.couples.forEach(function (c) {
      if (c.stub) {
        tw += '<g opacity="' + f(c.a) + '"><path d="M' + f(c.x) + ' ' + f(c.y) + 'H' + f(c.x + c.stub_dx) + 'V' + f(c.y + c.stub_dy) + '" class="gn-line gn-twig"/></g>';
        if (c.stub_at) counts.push({ mk: c.stub_at, text: '+' + c.stub, size: 10, dx: c.sref ? c.x - c.sref[0] : 0, dy: c.sref ? c.y - c.sref[1] : 0, a: c.a });
      }
      // Each family a gold knot, where its lines meet.
      if (c.parents.length + c.kids.length > 1) knots += '<circle cx="' + f(c.x) + '" cy="' + f(c.y) + '" r="3.2" class="gn-knot" opacity="' + f(c.a) + '"/>';
    });

    // The gold lines: what the parentage is worth, in how the line is drawn.
    api.edges(fr, PM).forEach(function (e) {
      var d = edge(e.x1, e.y1, e.x2, e.y2), o = e.a < 1 ? ' opacity="' + f(e.a) + '"' : '';
      if (e.conf === 'a') lines += '<path d="' + d + '" class="gn-line"' + o + '/>';
      else if (e.conf === 'd') lines += '<path d="' + d + '" class="gn-line gn-line-d" stroke-dasharray="5 3.5"' + o + '/>';
      else lines += '<path d="' + d + '" class="gn-line gn-line-s" stroke-linecap="round" stroke-dasharray="0.1 4.5"' + o + '/>';
    });

    // Cards beside the portraits, outward from the trunk: the heart's first,
    // under its portrait, then the nearest rows'. A card that would cover
    // another is left out, as the river leaves out a word it has no room
    // for; the pointer brings it back.
    var keep = {}, C = centreOf(fr), cs = C && C.s || {};
    var F = {};
    fr.persons.forEach(function (p) {
      var fm = frameOf(p, G);
      F[p.key] = fm;
      boxes.push([p.x - fm.rx - 2, p.y - fm.ry - 2, p.x + fm.rx + 2, p.y + fm.ry + 2]);
    });
    if (C && !cs.redacted) {
      var cf = F[C.key], cn = api.clip(cs.names[2], 240), cw = Math.max(api.width(cn, 15, true), api.width(cs.years, 10.5, true)) + 22;
      var at = null, least = Infinity, tries = [];
      [6, 30].forEach(function (gap) {
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
      if (!at) at = { x: C.x - cw / 2, y: C.y + cf.ry + 6, side: 0, box: [C.x - cw / 2 - 2, C.y + cf.ry + 4, C.x + cw / 2 + 2, C.y + cf.ry + 44] };
      keep[C.key] = { x: at.x, y: at.y, w: cw, h: 36, side: at.side, centre: true, word: cn, both: true };
      boxes.push(at.box);
    }
    fr.persons.slice().sort(function (a, b) { return Math.abs(a.gen) - Math.abs(b.gen) || Math.abs(a.x - G.x0) - Math.abs(b.x - G.x0); }).forEach(function (p) {
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

    // People: a card, then the portrait over its inner edge, so the two
    // read as one; the person this reader may not see as an empty frame.
    var people = '';
    fr.persons.forEach(function (p) {
      var s = p.s || {}, isC = p.role === 'centre', fm = F[p.key], x = f(p.x), y = f(p.y), g = '<g opacity="' + f(p.a) + '">';
      if (rest) g += '<a class="rv-p" data-id="' + esc(p.id) + '" data-key="' + esc(p.key) + '" href="' + api.base + '?p=' + encodeURIComponent(p.id) + '&amp;n=' + m.n + '"><ellipse cx="' + x + '" cy="' + y + '" rx="' + f(fm.rx + 6) + '" ry="' + f(fm.ry + 6) + '" fill="transparent"/>';
      var L = keep[p.key];
      if (L && !s.redacted) {
        g += '<g class="gn-plate">';
        g += '<rect class="gn-card-shade" x="' + f(L.x + 1.5) + '" y="' + f(L.y + 2) + '" width="' + f(L.w) + '" height="' + f(L.h) + '" rx="3"/>';
        g += '<rect class="gn-card' + (L.centre ? ' gn-card-centre' : '') + '" x="' + f(L.x) + '" y="' + f(L.y) + '" width="' + f(L.w) + '" height="' + f(L.h) + '" rx="3"/>';
        if (L.centre) {
          g += '<text x="' + f(L.x + L.w / 2) + '" y="' + f(L.y + 16) + '" text-anchor="middle" font-size="15" font-weight="600" class="gn-name gn-centre">' + esc(L.word) + '</text>'
            + '<text x="' + f(L.x + L.w / 2) + '" y="' + f(L.y + 29) + '" text-anchor="middle" font-family="' + api.mono + '" font-size="10.5" class="gn-years">' + esc(s.years) + '</text>';
        } else {
          var tx = L.side > 0 ? L.x + L.pad : L.x + 8;
          g += '<text x="' + f(tx) + '" y="' + f(L.y + (L.both ? 13 : 12.5)) + '" font-size="' + L.size + '" class="gn-name">' + esc(L.word) + '</text>';
          if (L.both) g += '<text x="' + f(tx) + '" y="' + f(L.y + 24.5) + '" font-family="' + api.mono + '" font-size="9" class="gn-years">' + esc(s.years) + '</text>';
        }
        g += '</g>';
      }
      if (p.repeat > 1) g += '<ellipse cx="' + x + '" cy="' + y + '" rx="' + f(fm.rx + 3.2) + '" ry="' + f(fm.ry + 3.2) + '" fill="none" class="gn-again" stroke-width="1"/>';
      if (p.again) g += '<ellipse cx="' + x + '" cy="' + y + '" rx="' + f(fm.rx + 3.2 + (p.repeat > 1 ? 2.4 : 0)) + '" ry="' + f(fm.ry + 3.2 + (p.repeat > 1 ? 2.4 : 0)) + '" fill="none" class="gn-again" stroke-width="1" stroke-dasharray="2.4 1.8"/>';
      if (isC) g += '<ellipse cx="' + x + '" cy="' + y + '" rx="' + f(fm.rx + 9) + '" ry="' + f(fm.ry + 9) + '" class="gn-halo"/>';
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
    var head = '<svg class="rv-svg gn-svg" data-view="scroll" viewBox="' + f(hl) + ' ' + f(vt) + ' ' + f(hw) + ' ' + f(vh) + '" width="100%" '
      + (rest ? 'role="img" aria-label="' + esc(label() + (C && C.s && !C.s.redacted ? ' — ' + C.s.names[2] : '')) + '"' : 'aria-hidden="true"')
      + ' xmlns="http://www.w3.org/2000/svg">';
    // The lit route lies over the lines and under the people (river.js).
    return head + out + tw + lines + knots + '<g class="rv-lit"></g>' + marks + people + '</svg>';
  }
  function label() {
    var s = root.querySelector('.rv-stage');
    return (s && s.getAttribute('aria-label')) || '';
  }

  /* ---- the dashboard: figures read from the scroll on screen ---- */
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

  /* ---- the life of the scroll ---- */
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
  function rgba(c, a) { return 'rgba(' + c[0] + ',' + c[1] + ',' + c[2] + ',' + a.toFixed(3) + ')'; }

  // Under the drawing, the falling leaves; over it, the glints along the lines.
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
  // The leaves, the same on every visit: where each starts across the
  // window, how fast it falls and sways, its size and its turn.
  var LEAVES = [];
  for (var li = 0; li < 22; li++) {
    var h1 = Math.abs(Math.sin(li * 12.9898) * 43758.5453) % 1, h2 = Math.abs(Math.sin(li * 78.233) * 12345.678) % 1;
    LEAVES.push({ u: h1, v: h2, fall: 0.018 + 0.022 * ((h1 * 7) % 1), sway: 0.6 + 1.2 * ((h2 * 3) % 1), size: 3 + 3.5 * ((h1 * 5) % 1), spin: 0.4 + ((h2 * 11) % 1) });
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
  // A glint runs along every attested line, from its older end to its
  // newer: roots to trunk to crown, along the line's square corners.
  function layout(L) {
    flows = [];
    if (!L || !L.edges) return;
    L.edges.forEach(function (e) {
      if (e.conf !== 'a') return;
      var a = e.y1 >= e.y2 ? [e.x1, e.y1, e.x2, e.y2] : [e.x2, e.y2, e.x1, e.y1], my = (a[1] + a[3]) / 2;
      var xs = [a[0], a[0], a[2], a[2]], ys = [a[1], my, my, a[3]], cum = [0];
      for (var i = 1; i < 4; i++) cum.push(cum[i - 1] + Math.hypot(xs[i] - xs[i - 1], ys[i] - ys[i - 1]));
      if (cum[3] < 12) return;
      flows.push({ x: xs, y: ys, cum: cum, len: cum[3], phase: Math.abs(Math.sin(e.x1 * 12.9898 + e.y2 * 78.233)) % 1 });
    });
    request();
  }
  var SPEED = 26;
  function leaves(moving) {
    var W = ground.width, Hh = ground.height, t = moving ? clock : 4.2;
    gx.setTransform(1, 0, 0, 1, 0, 0);
    gx.clearRect(0, 0, W, Hh);
    if (!view()) return;
    // Leaves drift down from the crown across the whole window, turning
    // as they sway.
    LEAVES.forEach(function (lf) {
      var y = ((lf.v + t * lf.fall) % 1.1 - 0.05) * Hh, x = (lf.u + 0.03 * Math.sin(t * 0.5 * lf.sway + lf.u * 9)) * W;
      var a = 0.45 * Math.min(1, (y / Hh + 0.05) * 6, (1.05 - y / Hh) * 4), r = lf.size * dpr, turn = t * lf.spin + lf.u * 6.283;
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
    leaves(moving);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, layer.width, layer.height);
    var v = view();
    if (!v || !moving || travelling) return;
    var s = v.k * dpr, ox = -v.x * s, oy = -v.y * s;
    flows.forEach(function (fl) {
      var pos = (fl.phase * fl.len + clock * SPEED) % (fl.len + 60), i = 1;
      if (pos > fl.len) return;
      while (i < 3 && fl.cum[i] < pos) i++;
      var u = (pos - fl.cum[i - 1]) / ((fl.cum[i] - fl.cum[i - 1]) || 1);
      var x = (fl.x[i - 1] + (fl.x[i] - fl.x[i - 1]) * u) * s + ox, y = (fl.y[i - 1] + (fl.y[i] - fl.y[i - 1]) * u) * s + oy;
      var fade = Math.min(1, pos / 16, (fl.len - pos) / 16), r = 2.6 * v.k * dpr;
      var g = ctx.createRadialGradient(x, y, 0, x, y, r * 2.6);
      g.addColorStop(0, rgba(GLINT, 0.95 * fade));
      g.addColorStop(0.4, rgba(GOLD, 0.45 * fade));
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
