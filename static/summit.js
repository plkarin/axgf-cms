/* Summit's ascent (v0.5.0, docs/adr/0010).
 *
 * The river's own layout — the same people, the same links, from the same
 * payload, through the same reader's lens — raised into a mountain at
 * sunrise. The person at the fixed point stands at the camp. Their ancestors
 * go up the mountain, one contour line for every generation, each line a
 * little narrower than the one below it, as a mountain is; their
 * descendants go down into the valley under the camp. Every parentage is a
 * trail from parent to union to child, lit as surely as it is known: solid,
 * broken or a trace. The summit is the farthest the record reaches: with no
 * ancestor above, the camp is on the summit.
 *
 * The mountain answers the climb:
 * - its snow comes down with the years. The further back the person at the
 *   camp was born, the lower the snow line: read from the year the river
 *   itself rests on, and only when this reader may see it (`meta.era`).
 *   Without one, the snow keeps to the heights, whoever is at the camp;
 * - the sun rises with the itinerary. Every person travelled to brings the
 *   dawn on, from the last stars to full light.
 *
 * Two things the page knows by name:
 * - `window.axgfView` (river.js): where its people stand (`shape`), how a
 *   frame is painted, the curve of a trail (`edge`, which the lit route
 *   follows too), and the dashboard beside it at rest. Travel, the trail, the
 *   panel, the keyboard, the camera and every touch rule stay river.js's.
 * - `window.axgfRiverScene`: the sky — the last stars, the sun's glow, mist
 *   drifting in the valley, light running down every attested trail from
 *   the heights toward the living. It holds still as the river's ground
 *   does: reduced motion, "Cut", the pause, a hidden tab, a river off
 *   screen, a machine too slow for it.
 *
 * No colour is named here: every one is a token of the theme (`--sm-*`,
 * tokens.css), read from the page or set by class in app.css.
 */
(function () {
  'use strict';
  var root = document.querySelector('.river');
  if (!root || root.dataset.view !== 'summit') return;
  var stage = document.getElementById('rv-canvas');

  /* ---- the climb's two readings ---- */
  // How far the sun has risen: one more step of dawn for every person on
  // the itinerary (river.js keeps a chip for each), up to full light.
  function dawn() {
    var n = root.querySelectorAll('.rv-trail li').length || 1;
    return Math.min(1, 0.12 + (n - 1) * 0.18);
  }
  // How far the snow comes down, 0.12 (the summit's tip) to 1 (down to the
  // camp), from the year the river rests on, and only one this reader may
  // see: a year it may not see must not show through the weather.
  function snowiness(m) {
    if (!m || !m.era || !isFinite(m.year)) return 0.3;
    return Math.max(0.12, Math.min(1, (2005 - m.year) / 300));
  }
  // A fixed sequence of numbers, so that the same mountain is drawn on every
  // visit and in every frame of a travel.
  function seeded(seed) {
    var s = seed % 2147483647 || 1;
    return function () { s = (s * 16807) % 2147483647; return s / 2147483647; };
  }
  // Where the drawing put the camp and the summit, for the sky.
  var GEO = null;

  /* ---- where everyone stands ---- */
  function shape(fr) {
    var m = fr.meta, hl = m.hview[0], hw = m.hview[1], vt = m.view[0], vh = m.view[1];
    var C = fr.persons.filter(function (p) { return p.role === 'centre'; })[0];
    var cx = C ? C.x : hl + hw / 2, cy = C ? C.y : m.cy, step0 = m.step || 100;
    // How far the drawing reaches each way, in generations: up (the
    // ancestors, below the river's centre) and down (the descendants, above
    // it), with the lines the record continues past them and their counts —
    // so that a count is never cut by the frame.
    var up = 0, down = 0;
    function reach(y, extra) {
      var g = (y - cy) / step0;
      if (g > 0) up = Math.max(up, g + (extra || 0)); else down = Math.max(down, -g + (extra || 0));
    }
    fr.persons.forEach(function (p) { reach(p.y); });
    fr.tails.forEach(function (t) { reach(t.y2); if (t.count_at) reach(t.count_at.y, 0.18); });
    fr.couples.forEach(function (c) {
      if (!c.stub) return;
      reach(c.y + c.stub_dy);
      if (c.stub_at) reach(c.stub_at.y, 0.18);
    });
    // The heights take what the summit leaves; the camp's name needs half a
    // step under it; the valley takes the rest.
    // A short climb leaves the summit more sky to stand in: with few
    // contours, the peak stands taller over the last of them.
    var sx = hl + hw / 2, topContent = vt + vh * (0.18 + 0.06 * Math.max(0, 3 - up)), bottom = vt + vh - 30;
    var span = up + down + 0.5;
    var step = Math.min(step0 * 1.25, (bottom - topContent) / Math.max(1, span));
    var campY = topContent + up * step;
    var summitY = vt + 26;
    // Each half takes the river's order from left to right over its own
    // breadth, as Galaxy's halves do; the heights narrow as they rise.
    var lo = [Infinity, Infinity], hi = [-Infinity, -Infinity];
    function widen(x, y) {
      var g = (y - cy) / step0;
      if (Math.abs(g) < 0.25) return;
      var h = g > 0 ? 0 : 1;
      lo[h] = Math.min(lo[h], x); hi[h] = Math.max(hi[h], x);
    }
    fr.persons.forEach(function (p) { widen(p.x, p.y); });
    fr.tails.forEach(function (t) { widen(t.x2, t.y2); });
    var room = hw / 2 - 70, k = [1, 1], mid = [cx, cx];
    [0, 1].forEach(function (h) {
      if (!(hi[h] > lo[h])) return;
      mid[h] = (lo[h] + hi[h]) / 2;
      k[h] = Math.min(1.2, room / Math.max(1, (hi[h] - lo[h]) / 2));
    });
    var steep = up > 0 ? 0.32 / Math.max(1, up) : 0;
    function place(x, y) {
      var g = (y - cy) / step0, ny = campY - g * step;
      if (Math.abs(g) < 0.25) {
        var dx = x - cx;
        if (Math.abs(dx) < 1) return [sx, ny];
        return [sx + (dx < 0 ? -1 : 1) * (28 + Math.min(1, Math.abs(dx) / 140) * 62), ny];
      }
      var h = g > 0 ? 0 : 1, narrow = h === 0 ? 1 - steep * Math.min(g, Math.max(1, up)) : 1;
      return [sx + (x - mid[h]) * k[h] * narrow, ny];
    }
    function moved(o) { var q = place(o.x, o.y); return Object.assign({}, o, { x: q[0], y: q[1] }); }
    function markAt(mk) {
      if (!mk) return mk;
      var q = place(mk.x, mk.y), out = Object.assign({}, mk, { x: q[0], y: q[1] });
      if (mk.leader) { var a = place(mk.leader[0], mk.leader[1]), b = place(mk.leader[2], mk.leader[3]); out.leader = [a[0], a[1], b[0], b[1]]; }
      return out;
    }
    return {
      persons: fr.persons.map(moved),
      couples: fr.couples.map(function (c) {
        var o = moved(c);
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
      meta: Object.assign({}, m, { camp: [sx, campY], summit: summitY, rise: step, up: Math.round(up), down: Math.round(down) })
    };
  }
  // A trail climbs as a path does: up the slope, across, and up again.
  function edge(x1, y1, x2, y2) {
    var my = (y1 + y2) / 2, r = function (v) { return Math.round(v * 10) / 10; };
    return 'M' + r(x1) + ' ' + r(y1) + 'C' + r(x1) + ' ' + r(my) + ' ' + r(x2) + ' ' + r(my) + ' ' + r(x2) + ' ' + r(y2);
  }

  /* ---- the mountain ---- */
  // A ridge line across [x0, x1]: a few waves of different lengths, from a
  // seed, so that the far ranges are the same ranges in every frame.
  function ridge(seed, x0, x1, base, height, n, span) {
    var rnd = seeded(seed), waves = [], total = 0;
    for (var i = 0; i < 5; i++) {
      var a = Math.pow(0.55, i) * (0.7 + rnd() * 0.6);
      waves.push({ f: (2 + i * 2.6 + rnd() * 2) / span * 3.1416, p: rnd() * 6.283, a: a });
      total += a;
    }
    var pts = [];
    for (var j = 0; j <= n; j++) {
      var x = x0 + (x1 - x0) * j / n, v = 0;
      // Ridged: 1 − |sin| peaks sharply and bottoms out round, as a range
      // of mountains does against the sky.
      waves.forEach(function (w) { v += w.a * (1 - Math.abs(Math.sin(x * w.f + w.p))); });
      pts.push([x, base - height * (0.2 + 0.95 * v / total)]);
    }
    return pts;
  }
  function polyline(pts) {
    var f = function (v) { return Math.round(v * 10) / 10; };
    return pts.map(function (p, i) { return (i ? 'L' : 'M') + f(p[0]) + ' ' + f(p[1]); }).join('');
  }
  // The massif under the climb. Every contour is as wide as the people on
  // it need, and from each one the flanks spread as they go down, as a
  // mountain's do, so nobody stands in the sky; over the highest contour, a
  // crest of peaks, the summit the highest of them. Drawn from where the
  // frame put everyone, so it rises with them as they travel. Returns the
  // outline and the peaks.
  var SLOPE = 0.78;
  function massif(fr, sx, campY, rise, summitY, valleyY) {
    var need = {}, top = 0;
    function note(x, y) {
      var g = Math.round((campY - y) / rise);
      if (g < 0) return;
      top = Math.max(top, g);
      need[g] = Math.max(need[g] || 0, Math.abs(x - sx) + 46);
    }
    fr.persons.forEach(function (p) { note(p.x, p.y); });
    fr.tails.forEach(function (t) { note(t.x2, t.y2); });
    fr.couples.forEach(function (c) { note(c.x, c.y); });
    function half(y) {
      var w = 64;
      for (var g = 0; g <= top; g++) {
        var yg = campY - g * rise;
        if (y >= yg - rise * 0.45) w = Math.max(w, (need[g] || 64) + Math.max(0, y - yg) * SLOPE);
      }
      return w;
    }
    var crestY = campY - top * rise - rise * 0.32, rnd = seeded(97), left = [], right = [], y;
    for (y = valleyY; y > crestY; y -= 14) left.push([sx - half(y) + (rnd() - 0.5) * 8, y]);
    // The crest: saddles and peaks across the top contour's breadth, the
    // summit a little right of the axis and the highest.
    var w0 = half(crestY), K = w0 > 220 ? 6 : 4, crest = [], peaks = [];
    for (var i = 0; i <= 2 * K; i++) {
      var u = i / (2 * K), x = sx - w0 + 2 * w0 * u;
      if (i % 2 === 0) { crest.push([x, crestY - (i === 0 || i === 2 * K ? 0 : 2 + rnd() * 12)]); continue; }
      var main = i === K + (K % 2 ? 0 : 1);
      var py = main ? summitY : summitY + (crestY - summitY) * (0.18 + rnd() * 0.42);
      var px = main ? sx + 18 : x + (rnd() - 0.5) * w0 * 0.12;
      crest.push([px, py]);
      peaks.push([px, py, main]);
    }
    for (y = crestY; y <= valleyY; y += 14) right.push([sx + half(y) + (rnd() - 0.5) * 8, y]);
    return { outline: left.concat(crest, right), peaks: peaks, half: half, crest: crestY };
  }

  /* ---- the drawing ---- */
  var named = {};
  function paint(fr, api, rest) {
    var f = api.f, esc = api.esc, m = fr.meta, PM = api.byKey(fr.persons), out = '', links = '', nodes = '';
    var hl = m.hview[0], hw = m.hview[1], vt = m.view[0], vh = m.view[1];
    var camp = m.camp || [hl + hw / 2, vt + vh * 0.6], sx = camp[0], campY = camp[1];
    var rise = m.rise || 60, summitY = m.summit || vt + 20, up = m.up || 0, down = m.down || 0;
    var S = snowiness(m), D = dawn(), valleyY = campY + rise * 0.55;
    var X0 = hl - hw, X1 = hl + 2 * hw, below = vt + vh * 2;
    GEO = { camp: camp, summit: summitY, sx: sx, rise: rise, valley: valleyY, hl: hl, hw: hw, vt: vt, vh: vh, flank: 0 };
    // The far ranges, the farther one higher and paler, each with the snow
    // of its own heights.
    var horizon = Math.min(campY - rise * 0.2, vt + vh * 0.62);
    var far = ridge(11, X0, X1, horizon, vh * 0.2, 240, hw), near = ridge(23, X0, X1, horizon + vh * 0.08, vh * 0.16, 240, hw * 1.3);
    var closeAt = function (pts) { return polyline(pts) + 'L' + f(X1) + ' ' + f(below) + 'L' + f(X0) + ' ' + f(below) + 'Z'; };
    out += '<path d="' + closeAt(far) + '" class="sm-range sm-range-far"/>';
    var farTop = Math.min.apply(null, far.map(function (p) { return p[1]; }));
    out += '<path d="' + closeAt(far) + '" class="sm-range-snow" clip-path="url(#sm-far-snow)"/>';
    out += '<path d="' + closeAt(near) + '" class="sm-range sm-range-mid"/>';
    // The mountain: rock in the shade, its sunward side warming with the
    // dawn, then its snow down to the line the years set.
    var mass = massif(fr, sx, campY, rise, summitY, valleyY + rise * 0.6);
    var peakOf = mass.peaks.filter(function (pk) { return pk[2]; })[0];
    var md = polyline(mass.outline) + 'L' + f(X1) + ' ' + f(below) + 'L' + f(X0) + ' ' + f(below) + 'Z';
    GEO.flank = mass.half(mass.crest + (campY - mass.crest) * 0.3);
    out += '<path d="' + md + '" class="sm-rock"/>';
    out += '<path d="' + md + '" fill="url(#sm-lit)" opacity="' + f(0.25 + 0.6 * D) + '"/>';
    // The relief: an arête down from every peak each way, its sunward side
    // catching the light.
    var ridges = '', rr = seeded(71);
    mass.peaks.filter(function (pk, i) { return pk[2] || Math.abs(i - mass.peaks.indexOf(peakOf)) === 1; }).forEach(function (pk) {
      [-1, 1].forEach(function (side) {
        var len = (campY - pk[1]) * (0.45 + rr() * 0.35), bend = (rr() - 0.5) * 30;
        var ex = pk[0] + side * len * (0.55 + rr() * 0.3), ey = pk[1] + len;
        ridges += 'M' + f(pk[0]) + ' ' + f(pk[1]) + 'Q' + f((pk[0] + ex) / 2 + bend) + ' ' + f((pk[1] + ey) / 2) + ' ' + f(ex) + ' ' + f(ey);
      });
    });
    out += '<path d="' + ridges + '" class="sm-ridge" clip-path="url(#sm-on-rock)"/>';
    // The snow line: lower the further back the camp's year, with tongues
    // down the couloirs, never a ruled line.
    var snowY = summitY + (campY - summitY) * S, rnd = seeded(31), edgePts = [], tongue = Math.min(60, rise * 0.9);
    for (var x = X0, ph = 0; x <= X1; x += 14, ph += 0.55) {
      var t = Math.sin(ph) * 0.5 + Math.sin(ph * 0.37 + 1.3) * 0.5;
      edgePts.push([x, snowY + Math.max(0, t) * tongue + (rnd() - 0.5) * 10]);
    }
    var clip = 'M' + f(X0) + ' ' + f(vt - vh) + 'L' + f(X1) + ' ' + f(vt - vh) + polyline(edgePts.slice().reverse()).replace(/^M/, 'L') + 'Z';
    out += '<path d="' + md + '" class="sm-snow" fill="url(#sm-snow-fade)" clip-path="url(#sm-snowline)"/>';
    out += '<path d="' + md + '" fill="url(#sm-glow)" opacity="' + f(0.15 + 0.7 * D) + '" clip-path="url(#sm-snowline)"/>';
    // The contour of every generation up the mountain, and its number at the
    // edge of the frame: ▲ for the heights, ▼ for the valley.
    var g, boxes = [];
    for (g = 1; g <= up; g++) {
      var cyG = campY - g * rise;
      out += '<line x1="' + f(X0) + '" y1="' + f(cyG) + '" x2="' + f(X1) + '" y2="' + f(cyG) + '" class="sm-contour" clip-path="url(#sm-on-rock)"/>';
    }
    // The valley under the camp: its forest edge, its floor, its mist.
    var trees = '', tr = seeded(53);
    for (var tx = X0; tx < X1; tx += 9 + tr() * 10) {
      var th = 10 + tr() * 16, ty = valleyY + rise * 0.15 + tr() * 8;
      trees += 'M' + f(tx - th * 0.32) + ' ' + f(ty) + 'L' + f(tx) + ' ' + f(ty - th) + 'L' + f(tx + th * 0.32) + ' ' + f(ty) + 'Z';
    }
    out += '<path d="M' + f(X0) + ' ' + f(valleyY + rise * 0.2) + 'L' + f(X1) + ' ' + f(valleyY + rise * 0.2) + 'L' + f(X1) + ' ' + f(below) + 'L' + f(X0) + ' ' + f(below) + 'Z" class="sm-valley"/>';
    out += '<path d="' + trees + '" class="sm-pines"/>';
    out += '<rect x="' + f(X0) + '" y="' + f(valleyY - rise * 0.2) + '" width="' + f(X1 - X0) + '" height="' + f(rise * 0.9) + '" fill="url(#sm-mist)"/>';
    // Lines the record continues past the drawing, and lines it ends. How
    // many more each one leads to is written once the names are placed.
    var counts = [];
    fr.tails.forEach(function (t) {
      out += '<g opacity="' + f(t.a) + '"><path d="' + edge(t.x1, t.y1, t.x2, t.y2) + '" fill="none" class="sm-tail"/>';
      out += t.kind === 'cont'
        ? '<circle cx="' + f(t.x2) + '" cy="' + f(t.y2) + '" r="2.2" class="sm-tail-tip"/>'
        : '<circle cx="' + f(t.x2) + '" cy="' + f(t.y2) + '" r="3.2" class="sm-end"/>';
      out += '</g>';
      if (t.kind === 'cont' && t.count_at) counts.push({ mk: t.count_at, text: '+' + t.count, size: 9.5, dx: t.cref ? t.x2 - t.cref[0] : 0, dy: t.cref ? t.y2 - t.cref[1] : 0, a: t.a });
    });
    fr.couples.forEach(function (c) {
      if (!c.stub) return;
      out += '<g opacity="' + f(c.a) + '"><path d="' + edge(c.x, c.y, c.x + c.stub_dx, c.y + c.stub_dy) + '" fill="none" class="sm-tail sm-stub"/></g>';
      if (c.stub_at) counts.push({ mk: c.stub_at, text: '+' + c.stub, size: 10, dx: c.sref ? c.x - c.sref[0] : 0, dy: c.sref ? c.y - c.sref[1] : 0, a: c.a });
    });
    // Trails: parent to union, union to child.
    api.edges(fr, PM).forEach(function (e) {
      var d = edge(e.x1, e.y1, e.x2, e.y2), w = f(0.8 + api.widths[api.cls(e.d)] * 0.3), o = e.a < 1 ? ' opacity="' + f(e.a) + '"' : '';
      if (e.conf === 'a') links += '<path d="' + d + '" fill="none" class="sm-trail" stroke-width="' + w + '" stroke-linecap="round"' + o + '/>';
      else if (e.conf === 'd') links += '<path d="' + d + '" fill="none" class="sm-trail sm-trail-d" stroke-width="' + w + '" stroke-dasharray="5 3.5"' + o + '/>';
      else links += '<path d="' + d + '" fill="none" class="sm-trail sm-trail-s" stroke-width="2" stroke-linecap="round" stroke-dasharray="0.1 5"' + o + '/>';
    });
    fr.couples.forEach(function (c) {
      out += '<circle cx="' + f(c.x) + '" cy="' + f(c.y) + '" r="1.8" class="sm-union" opacity="' + f(c.a) + '"/>';
    });
    // The summit's flag, where the record stops climbing: on the camp itself
    // when nobody stands above it.
    if (up > 0) {
      var fx = sx + 18, fy = summitY + 1;
      out += '<path d="M' + f(fx) + ' ' + f(fy) + 'V' + f(fy - 16) + '" class="sm-pole"/><path d="M' + f(fx) + ' ' + f(fy - 16) + 'L' + f(fx + 11) + ' ' + f(fy - 12.5) + 'L' + f(fx) + ' ' + f(fy - 9) + 'Z" class="sm-flag"/>';
    }
    // Names stand beside their camp, away from the mountain's axis: the
    // nearest generations first, the next ones while there is room, and a
    // name that would cover one already placed is dropped, as the river drops
    // a word it has no room for. The pointer brings it back (river.js
    // `light`).
    var keep = {};
    function hit(box) {
      for (var i = 0; i < boxes.length; i++) {
        var b = boxes[i];
        if (box[0] < b[2] && box[2] > b[0] && box[1] < b[3] && box[3] > b[1]) return true;
      }
      return false;
    }
    var C0 = fr.persons.filter(function (p) { return p.role === 'centre'; })[0];
    if (C0 && C0.s && !C0.s.redacted) {
      var cw0 = api.width(api.clip(C0.s.names[2], 260), 16, true);
      boxes.push([C0.x - cw0 / 2, C0.y + 12, C0.x + cw0 / 2, C0.y + 44]);
    }
    if (up > 0) boxes.push([sx + 10, summitY - 20, sx + 34, summitY + 2]);
    fr.persons.slice().sort(function (a, b) { return Math.abs(a.gen) - Math.abs(b.gen) || a.x - b.x; }).forEach(function (p) {
      var s = p.s || {}, gd = Math.abs(p.gen);
      if (p.role === 'centre' || s.redacted || !(gd <= 2 || p.role === 'spouse')) return;
      var side = p.x < sx - 4 ? -1 : p.x > sx + 4 ? 1 : 0;
      var anchor = side < 0 ? 'end' : side > 0 ? 'start' : 'middle';
      var lx = p.x + side * 10, ly = side ? p.y + 4 : p.y - 10;
      var word = gd <= 1 || p.role === 'spouse' ? s.names[1] : s.names[0], both = gd <= 1;
      var w = Math.max(api.width(word, 12, false), both ? api.width(s.years, 9.5, true) : 0);
      var x0 = anchor === 'end' ? lx - w : anchor === 'middle' ? lx - w / 2 : lx, box = [x0 - 2, ly - 11, x0 + w + 2, ly + (both ? 14 : 3)];
      if (hit(box)) return;
      boxes.push(box);
      keep[p.key] = { x: lx, y: ly, anchor: anchor, word: word, both: both };
    });
    // The counts: never dropped, since each is the only sign of the people
    // beyond, but stepped aside from a name rather than written over it.
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
      var mv = Object.assign({}, mk, { x: mk.x + at[0], y: mk.y + at[1] });
      if (mk.leader) mv.leader = [mk.leader[0], mk.leader[1], mk.leader[2] + at[0], mk.leader[3] + at[1]];
      out += '<g opacity="' + f(c.a) + '">' + api.mark(mv, c.text, c.size, c.dx, c.dy) + '</g>';
    });
    // Each generation numbered at the right edge of the drawing, where the
    // names leave room; one with nowhere to go is left out, as a name would be.
    var edgeX = hl + hw - 10;
    function number(word, y) {
      var w = api.width(word, 9.5, true), box = [edgeX - w - 1, y - 8.5, edgeX + 1, y + 2];
      if (hit(box)) return;
      boxes.push(box);
      out += '<text x="' + f(edgeX) + '" y="' + f(y) + '" text-anchor="end" font-family="' + api.mono + '" font-size="9.5" class="k-band-text sm-alt">' + word + '</text>';
    }
    for (g = 1; g <= up; g++) number('▲' + g, campY - g * rise - 3);
    for (g = 1; g <= down; g++) number('▼' + g, campY + g * rise - 3);
    // The camps, and the one at the fixed point.
    fr.persons.forEach(function (p) {
      var s = p.s || {}, isC = p.role === 'centre', x = f(p.x), y = f(p.y), r = 4.8;
      var k = '<g opacity="' + f(p.a) + '">';
      if (rest) k += '<a class="rv-p" data-id="' + esc(p.id) + '" data-key="' + esc(p.key) + '" href="' + api.base + '?p=' + encodeURIComponent(p.id) + '&amp;n=' + m.n + '"><circle cx="' + x + '" cy="' + y + '" r="20" fill="transparent"/>';
      if (isC) {
        k += '<circle cx="' + x + '" cy="' + y + '" r="26" class="sm-camp-halo"/><circle cx="' + x + '" cy="' + y + '" r="14" class="sm-camp-halo sm-camp-halo-2"/>'
          + '<path d="M' + x + ' ' + y + 'V' + f(p.y - 22) + '" class="sm-pole sm-pole-camp"/>'
          + '<path d="M' + x + ' ' + f(p.y - 22) + 'L' + f(p.x + 15) + ' ' + f(p.y - 17.5) + 'L' + x + ' ' + f(p.y - 13) + 'Z" class="sm-flag sm-flag-camp"/>'
          + '<circle cx="' + x + '" cy="' + y + '" r="5.5" class="sm-camp-core"/>';
      } else {
        if (p.repeat > 1) k += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 3) + '" fill="none" class="k-data-line" stroke-width="1"/>';
        if (p.again) k += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 3 + (p.repeat > 1 ? 2.2 : 0)) + '" fill="none" class="k-data-line" stroke-width="1" stroke-dasharray="2.4 1.8"/>';
        if (s.redacted || s.sparse) k += '<circle cx="' + x + '" cy="' + y + '" r="' + r + '" class="sm-camp sm-hidden" stroke-width="1.5" stroke-dasharray="1.6 1.6"/>';
        else if (s.living) k += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 5) + '" class="sm-camp-glow sm-living"/><circle cx="' + x + '" cy="' + y + '" r="' + f(r + 0.4) + '" class="sm-camp sm-living"/>';
        else k += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 3) + '" class="sm-camp-glow sm-gone"/><circle cx="' + x + '" cy="' + y + '" r="' + r + '" class="sm-camp sm-gone"/>';
      }
      var halo = ' paint-order="stroke" stroke-width="4" stroke-linejoin="round"';
      if (isC && !s.redacted) {
        k += '<text x="' + x + '" y="' + f(p.y + 26) + '" text-anchor="middle" font-size="16" font-weight="600" class="sm-name sm-centre k-halo"' + halo + '>' + esc(api.clip(s.names[2], 260)) + '</text>'
          + '<text x="' + x + '" y="' + f(p.y + 40) + '" text-anchor="middle" font-family="' + api.mono + '" font-size="11" class="sm-data k-halo"' + halo + '>' + esc(s.years) + '</text>';
      } else if (keep[p.key]) {
        var L = keep[p.key], an = L.anchor !== 'start' ? ' text-anchor="' + L.anchor + '"' : '';
        k += '<text x="' + f(L.x) + '" y="' + f(L.y) + '"' + an + ' font-size="12" class="sm-name k-halo"' + halo + '>' + esc(L.word) + '</text>';
        if (L.both) k += '<text x="' + f(L.x) + '" y="' + f(L.y + 11.5) + '"' + an + ' font-family="' + api.mono + '" font-size="9.5" class="sm-data k-halo"' + halo + '>' + esc(s.years) + '</text>';
      }
      if (rest) k += '</a>';
      nodes += k + '</g>';
    });
    if (rest) named = keep;
    var farSnow = farTop + (horizon - farTop) * (0.25 + 0.5 * S);
    var defs = '<linearGradient id="sm-lit" x1="0" y1="0" x2="1" y2="0"><stop offset="0.35" class="sm-lit-0"/><stop offset="1" class="sm-lit-1"/></linearGradient>'
      + '<linearGradient id="sm-snow-fade" gradientUnits="userSpaceOnUse" x1="0" y1="' + f(summitY) + '" x2="0" y2="' + f(snowY + 20) + '"><stop offset="0" class="sm-snow-0"/><stop offset="0.7" class="sm-snow-1"/><stop offset="1" class="sm-snow-2"/></linearGradient>'
      + '<linearGradient id="sm-glow" x1="0" y1="0" x2="1" y2="0"><stop offset="0.4" class="sm-glow-0"/><stop offset="1" class="sm-glow-1"/></linearGradient>'
      + '<linearGradient id="sm-mist" x1="0" y1="0" x2="0" y2="1"><stop offset="0" class="sm-mist-0"/><stop offset="0.5" class="sm-mist-1"/><stop offset="1" class="sm-mist-0"/></linearGradient>'
      + '<clipPath id="sm-snowline"><path d="' + clip + '"/></clipPath>'
      + '<clipPath id="sm-on-rock"><path d="' + md + '"/></clipPath>'
      + '<clipPath id="sm-far-snow"><rect x="' + f(X0) + '" y="' + f(vt - vh) + '" width="' + f(X1 - X0) + '" height="' + f(farSnow - vt + vh) + '"/></clipPath>';
    var C = fr.persons.filter(function (p) { return p.role === 'centre'; })[0], cn = C && C.s && !C.s.redacted ? C.s.names[2] : '';
    var lbl = (root.querySelector('.rv-stage') || root).getAttribute('aria-label') || '';
    return '<svg class="rv-svg sm-svg" data-view="summit" viewBox="' + f(hl) + ' ' + f(vt) + ' ' + f(hw) + ' ' + f(vh) + '" width="100%" '
      + (rest ? 'role="img" aria-label="' + esc(lbl + (cn ? ' — ' + cn : '')) + '"' : 'aria-hidden="true"')
      + ' xmlns="http://www.w3.org/2000/svg"><defs>' + defs + '</defs>' + out + links + '<g class="rv-lit"></g>' + nodes + '</svg>';
  }

  /* ---- the dashboard: figures read from the mountain on screen ----
   * The same five figures as every view's, and the climb's profile: how
   * many people stand at each height, from the valley to the summit, as a
   * line with its ground under it. */
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
    var box = hud.querySelector('.rv-hud-bars') || hud.appendChild(document.createElement('div'));
    box.className = 'rv-hud-bars sm-profile';
    box.setAttribute('aria-hidden', 'true');
    // From the valley (the youngest generation, the largest `gen`) to the
    // summit (the oldest), left to right, as a climb is read.
    var most = 1, pts = [], W = 160, H = 40, i = 0, count = hi - lo + 1;
    for (var g = hi; g >= lo; g--) most = Math.max(most, per[g] || 0);
    for (g = hi; g >= lo; g--, i++) {
      var x = count > 1 ? 6 + (W - 12) * i / (count - 1) : W / 2;
      pts.push([x, H - 5 - (H - 12) * (per[g] || 0) / most, g === 0]);
    }
    var line = '', f1 = function (v) { return v.toFixed(1); };
    pts.forEach(function (p, j) {
      if (!j) { line = 'M' + f1(p[0]) + ' ' + f1(p[1]); return; }
      var q = pts[j - 1], mx = (q[0] + p[0]) / 2;
      line += 'C' + f1(mx) + ' ' + f1(q[1]) + ' ' + f1(mx) + ' ' + f1(p[1]) + ' ' + f1(p[0]) + ' ' + f1(p[1]);
    });
    var area = pts.length ? line + 'L' + f1(pts[pts.length - 1][0]) + ' ' + H + 'L' + f1(pts[0][0]) + ' ' + H + 'Z' : '';
    box.innerHTML = '<svg viewBox="0 0 ' + W + ' ' + H + '" preserveAspectRatio="none" class="sm-prof">'
      + '<path d="' + area + '" class="sm-prof-area"/><path d="' + line + '" class="sm-prof-line" vector-effect="non-scaling-stroke"/>'
      + pts.map(function (p) { return '<circle cx="' + f1(p[0]) + '" cy="' + f1(p[1]) + '" r="' + (p[2] ? 3 : 2) + '" class="sm-prof-dot' + (p[2] ? ' sm-prof-here' : '') + '"/>'; }).join('')
      + '</svg>';
    hud.hidden = false;
  }

  window.axgfView = { name: 'summit', shape: shape, edge: edge, paint: paint, rest: rest, named: function () { return named; } };

  /* ---- the sky ---- */
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
  var WHITE = rgb('--text-primary', [255, 255, 255]), NIGHT = rgb('--sm-sky-night', [0, 0, 0]), DAWN = rgb('--sm-sky-dawn', NIGHT);
  var GLOW = rgb('--sm-sky-glow', DAWN), SUN = rgb('--sm-sun', WHITE), CORE = rgb('--sm-sun-core', SUN);
  var TRAIL = rgb('--sm-trail', SUN), HOT = rgb('--sm-trail-hot', TRAIL), MIST = rgb('--sm-mist', WHITE);
  function rgba(c, a) { return 'rgba(' + c[0] + ',' + c[1] + ',' + c[2] + ',' + a.toFixed(3) + ')'; }
  function mix(a, b, t) { return [0, 1, 2].map(function (i) { return Math.round(a[i] + (b[i] - a[i]) * t); }); }

  var under = document.createElement('canvas'), over = document.createElement('canvas');
  under.className = 'rv-ground'; over.className = 'rv-water';
  under.setAttribute('aria-hidden', 'true'); over.setAttribute('aria-hidden', 'true');
  stage.insertBefore(under, stage.firstChild);
  stage.appendChild(over);
  stage.classList.add('rv-alive');
  var gu = under.getContext('2d'), go = over.getContext('2d');
  if (!gu || !go) return;

  // The last stars: placed once, from a fixed seed, so the sky is the same
  // sky on every visit and between two frames.
  var stars = [];
  (function () {
    var rnd = seeded(7);
    for (var i = 0; i < 160; i++) stars.push({ u: rnd(), v: rnd() * 0.6, r: 0.35 + rnd() * 1, a: 0.25 + rnd() * 0.6, tw: rnd() * 6.283, sp: 0.6 + rnd() * 1.4 });
  })();
  var flows = [], dpr = 1, cw = 0, ch = 0, clock = 0, travelling = false;
  function size() {
    var w = stage.clientWidth, h = stage.clientHeight, r = Math.min(window.devicePixelRatio || 1, 2);
    if (w === cw && h === ch && r === dpr) return false;
    cw = w; ch = h; dpr = r;
    under.width = over.width = Math.max(1, Math.round(w * dpr));
    under.height = over.height = Math.max(1, Math.round(h * dpr));
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
  // Light runs down the attested trails, from the heights toward the
  // living: from the upper end of a trail to its lower one.
  function layout(L) {
    flows = [];
    if (!L || !L.edges) return;
    L.edges.forEach(function (e) {
      if (e.conf !== 'a') return;
      var a = e.y1 <= e.y2 ? [e.x1, e.y1, e.x2, e.y2] : [e.x2, e.y2, e.x1, e.y1];
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
  var SPEED = 26;
  function draw(moving) {
    size();
    var W = under.width, H = under.height, D = dawn(), v = view();
    gu.setTransform(1, 0, 0, 1, 0, 0); go.setTransform(1, 0, 0, 1, 0, 0);
    gu.clearRect(0, 0, W, H); go.clearRect(0, 0, W, H);
    // The sky, from the night at the top to the dawn at the horizon, warmer
    // the further the climb has gone.
    var s = v ? v.k * dpr : 1, ox = v ? -v.x * s : 0, oy = v ? -v.y * s : 0;
    var horizonPx = GEO && v ? GEO.camp[1] * s + oy : H * 0.6;
    var sky = gu.createLinearGradient(0, 0, 0, Math.max(1, horizonPx));
    sky.addColorStop(0, rgba(mix(NIGHT, DAWN, 0.25 + 0.45 * D), 1));
    sky.addColorStop(0.62, rgba(mix(DAWN, GLOW, 0.15 + 0.35 * D), 1));
    sky.addColorStop(1, rgba(mix(DAWN, GLOW, 0.45 + 0.55 * D), 1));
    gu.fillStyle = sky; gu.fillRect(0, 0, W, H);
    // The sun, rising behind the mountain's sunward shoulder as the dawn
    // comes on: under the drawing, so the ridges stand in front of it.
    if (GEO && v) {
      var sxp = (GEO.sx + Math.min(GEO.hw * 0.42, GEO.flank + 70)) * s + ox, low = GEO.camp[1] + GEO.rise * 0.2, high = GEO.summit + (GEO.camp[1] - GEO.summit) * 0.2;
      var syp = (low + (high - low) * D) * s + oy, R = (60 + 120 * D) * v.k * dpr;
      var pulse = moving ? 1 + 0.04 * Math.sin(clock * 0.9) : 1;
      var halo = gu.createRadialGradient(sxp, syp, 0, sxp, syp, R * 3.2 * pulse);
      halo.addColorStop(0, rgba(SUN, 0.55 * (0.4 + 0.6 * D))); halo.addColorStop(0.35, rgba(GLOW, 0.25 * (0.4 + 0.6 * D))); halo.addColorStop(1, rgba(GLOW, 0));
      gu.fillStyle = halo; gu.beginPath(); gu.arc(sxp, syp, R * 3.2 * pulse, 0, 6.283); gu.fill();
      gu.fillStyle = rgba(CORE, 0.9); gu.beginPath(); gu.arc(sxp, syp, 14 * v.k * dpr, 0, 6.283); gu.fill();
    }
    // The last stars, fading as the sun comes up.
    var starA = Math.pow(1 - D, 1.4);
    if (starA > 0.02) {
      gu.globalCompositeOperation = 'lighter';
      stars.forEach(function (st) {
        var a = st.a * starA * (moving ? 0.65 + 0.35 * Math.sin(clock * st.sp + st.tw) : 0.8);
        gu.fillStyle = rgba(WHITE, a);
        gu.beginPath(); gu.arc(st.u * W, st.v * Math.max(1, horizonPx), st.r * dpr, 0, 6.283); gu.fill();
      });
      gu.globalCompositeOperation = 'source-over';
    }
    if (!v || !moving || travelling) return;
    // Over the drawing: mist drifting in the valley, and light running down
    // the trails.
    if (GEO) {
      var vy = GEO.valley * s + oy;
      for (var i = 0; i < 3; i++) {
        var mx = ((clock * (6 + i * 3) + i * 330) % (W / dpr + 400) - 200) * dpr, my = vy + (i - 1) * 18 * v.k * dpr;
        var mr = (180 + i * 60) * v.k * dpr, mg = go.createRadialGradient(mx, my, 0, mx, my, mr);
        mg.addColorStop(0, rgba(MIST, 0.07)); mg.addColorStop(1, rgba(MIST, 0));
        go.fillStyle = mg; go.beginPath(); go.ellipse(mx, my, mr, mr * 0.28, 0, 0, 6.283); go.fill();
      }
    }
    go.globalCompositeOperation = 'lighter';
    flows.forEach(function (fl) {
      var pos = (fl.phase * fl.len + clock * SPEED) % fl.len, j = 1;
      while (j < fl.n && fl.cum[j] < pos) j++;
      var t = (pos - fl.cum[j - 1]) / ((fl.cum[j] - fl.cum[j - 1]) || 1);
      var x = (fl.x[j - 1] + (fl.x[j] - fl.x[j - 1]) * t) * s + ox, y = (fl.y[j - 1] + (fl.y[j] - fl.y[j - 1]) * t) * s + oy;
      var fade = Math.min(1, pos / 14, (fl.len - pos) / 14), r = 6 * v.k * dpr;
      var pg = go.createRadialGradient(x, y, 0, x, y, r);
      pg.addColorStop(0, rgba(HOT, 0.85 * fade)); pg.addColorStop(0.4, rgba(TRAIL, 0.35 * fade)); pg.addColorStop(1, rgba(TRAIL, 0));
      go.fillStyle = pg; go.beginPath(); go.arc(x, y, r, 0, 6.283); go.fill();
    });
    go.globalCompositeOperation = 'source-over';
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
