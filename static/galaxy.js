/* Galaxy's solar system (v0.5.0, docs/adr/0007).
 *
 * The river's own layout — the same people, the same links, from the same
 * payload, through the same reader's lens — bent into a solar system. The
 * person at the fixed point is the sun, in the middle of the window. Every
 * generation is an orbit, one further out for each generation away: the
 * descendants on the upper half of the sky, the ancestors on the lower half,
 * each orbit's people in the river's own order from left to right. Partners
 * stand beside the sun. Every parentage is a filament from parent to union to
 * child, lit as surely as it is known: solid, broken or a trace; violet on
 * the upper sky, toward the descendants, white on the lower, toward the
 * ancestors (the operator's review, 2026-10-27), so that up and down read
 * at a glance. Every star holds a field round it, wider as more descendants
 * come from it. The sky is a galaxy made in Canva (static/img/galaxy-sky.jpg).
 *
 * Two things the page knows by name:
 * - `window.axgfView` (river.js): where its people stand (`shape`), how a
 *   frame is painted, the curve of a link (`edge`, which the lit route
 *   follows too), and the dashboard beside it at rest. Travel, the trail, the
 *   panel, the keyboard, the camera and every touch rule stay river.js's.
 * - `window.axgfRiverScene`: the sky — stars that twinkle under the drawing,
 *   light running along every attested link from the ancestors toward the
 *   living, a comet on every orbit, the sun breathing. It holds still as the
 *   river's ground does: reduced motion, "Cut", the pause, a hidden tab, a
 *   river off screen, a machine too slow for it.
 *
 * No colour is named here: every one is a token of the theme (`--gx-*`,
 * tokens.css), read from the page or set by class in app.css.
 */
(function () {
  'use strict';
  var root = document.querySelector('.river');
  if (!root || root.dataset.view !== 'galaxy') return;
  var stage = document.getElementById('rv-canvas');

  /* ---- where everyone stands ---- */
  var SUN = [450, 320];
  function shape(fr) {
    var m = fr.meta, hl = m.hview[0], hw = m.hview[1], vt = m.view[0], vh = m.view[1];
    var C = fr.persons.filter(function (p) { return p.role === 'centre'; })[0];
    var cx = C ? C.x : hl + hw / 2, cy = C ? C.y : m.cy, step0 = m.step || 100;
    var sx = hl + hw / 2, sy = vt + vh / 2, n = Math.max(1, m.n);
    // The outermost orbit leaves room for what goes beyond it: a line the
    // record continues runs on past the last generation drawn, and its count
    // is written past its end. Measured from the frame — how far out its
    // farthest line and count reach — so that a sky with none keeps its size.
    var reach = n;
    fr.tails.forEach(function (t) {
      reach = Math.max(reach, Math.abs(t.y2 - cy) / step0);
      if (t.count_at) reach = Math.max(reach, Math.abs(t.count_at.y - cy) / step0 + 0.12);
    });
    fr.couples.forEach(function (c) {
      if (!c.stub) return;
      reach = Math.max(reach, Math.abs(c.y + c.stub_dy - cy) / step0);
      if (c.stub_at) reach = Math.max(reach, Math.abs(c.stub_at.y - cy) / step0 + 0.12);
    });
    var Rmax = Math.min(hw, vh) / 2 - 30, R0 = Math.min(40, Rmax * 0.17);
    var step = reach > n ? (Rmax + 12 - R0) / (reach + 0.1) : (Rmax - R0) / n;
    SUN = [sx, sy];
    // Each half of the sky takes the river's order from left to right over
    // its own breadth, not the window's: the river draws the ancestors over
    // part of its width only, and they would crowd one quarter of theirs.
    var lo = [Infinity, Infinity], hi = [-Infinity, -Infinity];
    fr.persons.forEach(function (p) {
      var g = (p.y - cy) / step0;
      if (Math.abs(g) < 0.25) return;
      var h = g < 0 ? 0 : 1;
      lo[h] = Math.min(lo[h], p.x); hi[h] = Math.max(hi[h], p.x);
    });
    function place(x, y) {
      var g = (y - cy) / step0;
      if (Math.abs(g) < 0.25) {
        var dx = x - cx;
        if (Math.abs(dx) < 1) return [sx, sy];
        return [sx + (dx < 0 ? -1 : 1) * (R0 + Math.min(1, Math.abs(dx) / 140) * step * 0.4), sy];
      }
      var h = g < 0 ? 0 : 1, span = hi[h] - lo[h];
      var r = R0 + Math.abs(g) * step, u = span > 1 ? Math.min(1, Math.max(0, (x - lo[h]) / span)) : 0.5, M = 0.09;
      var th = g < 0 ? Math.PI * (1 + M + u * (1 - 2 * M)) : Math.PI * (1 - M - u * (1 - 2 * M));
      return [sx + r * Math.cos(th), sy + r * Math.sin(th)];
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
      meta: Object.assign({}, m, { sun: [sx, sy], r0: R0, orbit: step })
    };
  }
  // A link bends a little toward the sun, as an orbit would pull it.
  function edge(x1, y1, x2, y2) {
    var mx = (x1 + x2) / 2, my = (y1 + y2) / 2, qx = mx + (SUN[0] - mx) * 0.16, qy = my + (SUN[1] - my) * 0.16;
    var r = function (v) { return Math.round(v * 10) / 10; };
    return 'M' + r(x1) + ' ' + r(y1) + 'Q' + r(qx) + ' ' + r(qy) + ' ' + r(x2) + ' ' + r(y2);
  }

  /* ---- the drawing ---- */
  var named = {};
  function paint(fr, api, rest) {
    var f = api.f, esc = api.esc, m = fr.meta, PM = api.byKey(fr.persons), out = '', links = '', nodes = '';
    var hl = m.hview[0], hw = m.hview[1], vt = m.view[0], vh = m.view[1];
    var S = m.sun || [hl + hw / 2, vt + vh / 2], R0 = m.r0 || 30, step = m.orbit || 50;
    SUN = S;
    out += '<rect class="gx-bg" x="' + f(hl) + '" y="' + f(vt) + '" width="' + f(hw) + '" height="' + f(vh) + '"/>';
    // The sky is the stage's picture (app.css); a dark well round the sun
    // keeps the drawing readable over the galaxy's bright core.
    out += '<circle class="gx-well" cx="' + f(S[0]) + '" cy="' + f(S[1]) + '" r="' + f(R0 + (m.n + 0.6) * step) + '" fill="url(#gx-well)"/>';
    // Orbits: the partners' close one, then one per generation each way.
    out += '<circle class="gx-orbit gx-orbit-0" cx="' + f(S[0]) + '" cy="' + f(S[1]) + '" r="' + f(R0) + '"/>';
    for (var g = 1; g <= m.n; g++) {
      out += '<circle class="gx-orbit" cx="' + f(S[0]) + '" cy="' + f(S[1]) + '" r="' + f(R0 + g * step) + '"/>';
    }
    // Lines the record continues past the outermost orbit, and lines it ends.
    // How many more each one leads to is written once the names are placed.
    var counts = [];
    fr.tails.forEach(function (t) {
      out += '<g opacity="' + f(t.a) + '"><path d="' + edge(t.x1, t.y1, t.x2, t.y2) + '" fill="none" class="gx-tail"/>';
      out += t.kind === 'cont'
        ? '<circle cx="' + f(t.x2) + '" cy="' + f(t.y2) + '" r="2.2" class="gx-tail-tip"/>'
        : '<circle cx="' + f(t.x2) + '" cy="' + f(t.y2) + '" r="3.2" class="gx-end"/>';
      out += '</g>';
      if (t.kind === 'cont' && t.count_at) counts.push({ mk: t.count_at, text: '+' + t.count, size: 9.5, dx: t.cref ? t.x2 - t.cref[0] : 0, dy: t.cref ? t.y2 - t.cref[1] : 0, a: t.a });
    });
    fr.couples.forEach(function (c) {
      if (!c.stub) return;
      out += '<g opacity="' + f(c.a) + '"><path d="' + edge(c.x, c.y, c.x + c.stub_dx, c.y + c.stub_dy) + '" fill="none" class="gx-tail gx-stub"/></g>';
      if (c.stub_at) counts.push({ mk: c.stub_at, text: '+' + c.stub, size: 10, dx: c.sref ? c.x - c.sref[0] : 0, dy: c.sref ? c.y - c.sref[1] : 0, a: c.a });
    });
    // Every star's field: a disc of faint light and its edge, wider as more
    // descendants come from the star (the river's own count, `d`, is the
    // descendants and the person), never wider than most of an orbit's step.
    fr.persons.forEach(function (p) {
      if (p.role === 'centre' || (p.s && p.s.redacted)) return;
      var desc = Math.max(0, (p.d || 1) - 1);
      if (!desc) return;
      var rf = Math.min(step * 0.56, 9 + 4 * Math.sqrt(desc)), o = ' opacity="' + f(p.a) + '"';
      out += '<circle class="gx-field" cx="' + f(p.x) + '" cy="' + f(p.y) + '" r="' + f(rf) + '" fill="url(#gx-field)"' + o + '/>'
        + '<circle class="gx-field-edge" cx="' + f(p.x) + '" cy="' + f(p.y) + '" r="' + f(rf) + '"' + o + '/>';
      if (desc >= 6) out += '<circle class="gx-field-edge gx-field-inner" cx="' + f(p.x) + '" cy="' + f(p.y) + '" r="' + f(rf * 0.62) + '"' + o + '/>';
    });
    // Filaments: parent to union, union to child — violet on the upper sky,
    // toward the descendants, white on the lower, toward the ancestors; a
    // halo, the filament, and a thread of light down an attested one.
    api.edges(fr, PM).forEach(function (e) {
      var d = edge(e.x1, e.y1, e.x2, e.y2), w = 0.7 + api.widths[api.cls(e.d)] * 0.32, o = e.a < 1 ? ' opacity="' + f(e.a) + '"' : '';
      var side = (e.y1 + e.y2) / 2 > S[1] ? ' gx-down' : ' gx-up';
      links += '<path d="' + d + '" fill="none" class="gx-link-halo' + side + '" stroke-width="' + f(w + 4) + '" stroke-linecap="round"' + o + '/>';
      if (e.conf === 'a') links += '<path d="' + d + '" fill="none" class="gx-link' + side + '" stroke-width="' + f(w) + '" stroke-linecap="round"' + o + '/>'
        + '<path d="' + d + '" fill="none" class="gx-thread' + side + '" stroke-width="' + f(Math.max(0.6, w * 0.35)) + '" stroke-linecap="round"' + o + '/>';
      else if (e.conf === 'd') links += '<path d="' + d + '" fill="none" class="gx-link gx-link-d' + side + '" stroke-width="' + f(w) + '" stroke-dasharray="5 3.5"' + o + '/>';
      else links += '<path d="' + d + '" fill="none" class="gx-link gx-link-s' + side + '" stroke-width="2" stroke-linecap="round" stroke-dasharray="0.1 5"' + o + '/>';
    });
    // Where a couple's filaments meet: a small four-pointed spark.
    fr.couples.forEach(function (c) {
      var x = c.x, y = c.y, side = c.y > S[1] ? ' gx-down' : ' gx-up';
      links += '<path class="gx-union' + side + '" d="M' + f(x) + ' ' + f(y - 4) + 'L' + f(x + 1.2) + ' ' + f(y - 1.2) + 'L' + f(x + 4) + ' ' + f(y) + 'L' + f(x + 1.2) + ' ' + f(y + 1.2) + 'L' + f(x) + ' ' + f(y + 4) + 'L' + f(x - 1.2) + ' ' + f(y + 1.2) + 'L' + f(x - 4) + ' ' + f(y) + 'L' + f(x - 1.2) + ' ' + f(y - 1.2) + 'Z" opacity="' + f(c.a) + '"/>';
    });
    // Names stand outward from the sun, along the line from it: the near
    // orbits named first, the next ones while there is room, and a name that
    // would cover one already placed is dropped, as the river drops a word it
    // has no room for. The pointer brings it back (river.js `light`).
    var keep = {}, boxes = [];
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
      boxes.push([C0.x - cw0 / 2, C0.y + 36, C0.x + cw0 / 2, C0.y + 68]);
    }
    fr.persons.slice().sort(function (a, b) { return Math.abs(a.gen) - Math.abs(b.gen) || a.x - b.x; }).forEach(function (p) {
      var s = p.s || {}, gd = Math.abs(p.gen);
      if (p.role === 'centre' || s.redacted || !(gd <= 2 || p.role === 'spouse')) return;
      var dx = p.x - S[0], dy = p.y - S[1], l = Math.hypot(dx, dy) || 1, ux = dx / l, uy = dy / l;
      var anchor = ux < -0.3 ? 'end' : ux > 0.3 ? 'start' : 'middle', lx = p.x + ux * 11, ly = p.y + uy * 11 + (uy > 0.3 ? 9 : uy < -0.3 ? -2 : 4);
      var word = gd <= 1 || p.role === 'spouse' ? s.names[1] : s.names[0], both = gd <= 1;
      var w = Math.max(api.width(word, 12, false), both ? api.width(s.years, 9.5, true) : 0);
      var x0 = anchor === 'end' ? lx - w : anchor === 'middle' ? lx - w / 2 : lx, box = [x0 - 2, ly - 11, x0 + w + 2, ly + (both ? 14 : 3)];
      if (hit(box)) return;
      boxes.push(box);
      keep[p.key] = { x: lx, y: ly, anchor: anchor, word: word, both: both };
    });
    // The counts, then: a count never dropped, since it is the only sign of
    // the people beyond, but stepped aside from a name rather than written
    // over it, a line lower, higher, or to one side.
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
      var moved = Object.assign({}, mk, { x: mk.x + at[0], y: mk.y + at[1] });
      if (mk.leader) moved.leader = [mk.leader[0], mk.leader[1], mk.leader[2] + at[0], mk.leader[3] + at[1]];
      out += '<g opacity="' + f(c.a) + '">' + api.mark(moved, c.text, c.size, c.dx, c.dy) + '</g>';
    });
    // Each orbit numbered where the names leave room, in the narrow band
    // either side of the sun that no planet takes (`shape`, M): on the right
    // as the river's rail is, else on the left, else a little further from
    // the axis. A partner stands on that axis, so a number may have to move;
    // one with nowhere to go is left out, as a name would be.
    for (g = 1; g <= m.n; g++) {
      [1, -1].forEach(function (up) {
        var R = R0 + g * step, word = (up > 0 ? '+' : '−') + g, w = api.width(word, 9.5, true);
        var spots = [[1, 0.035], [-1, 0.035], [1, 0.065], [-1, 0.065]];
        for (var j = 0; j < spots.length; j++) {
          var side = spots[j][0], a = spots[j][1] * Math.PI;
          var tx = S[0] + side * (R * Math.cos(a) + 4), ty = S[1] - up * R * Math.sin(a) + (up > 0 ? 0 : 8);
          var x0 = side > 0 ? tx : tx - w, box = [x0 - 1, ty - 8.5, x0 + w + 1, ty + 2];
          if (hit(box)) continue;
          boxes.push(box);
          out += '<text x="' + f(tx) + '" y="' + f(ty) + '"' + (side < 0 ? ' text-anchor="end"' : '') + ' font-family="' + api.mono + '" font-size="9.5" class="k-band-text gx-orbit-n">' + word + '</text>';
          return;
        }
      });
    }
    // Planets, and the sun.
    fr.persons.forEach(function (p) {
      var s = p.s || {}, isC = p.role === 'centre', x = f(p.x), y = f(p.y), r = 5;
      var k = '<g opacity="' + f(p.a) + '">';
      if (rest) k += '<a class="rv-p" data-id="' + esc(p.id) + '" data-key="' + esc(p.key) + '" href="' + api.base + '?p=' + encodeURIComponent(p.id) + '&amp;n=' + m.n + '" aria-label="' + esc(api.said(p)) + '"><circle cx="' + x + '" cy="' + y + '" r="20" fill="transparent"/>';
      if (isC) {
        // The sun: its corona, its rays, its disc and its heart.
        var rays = '';
        for (var ri = 0; ri < 8; ri++) {
          var ra = ri * Math.PI / 4, rl = ri % 2 ? 30 : 46;
          rays += 'M' + f(p.x + Math.cos(ra) * 18) + ' ' + f(p.y + Math.sin(ra) * 18) + 'L' + f(p.x + Math.cos(ra) * rl) + ' ' + f(p.y + Math.sin(ra) * rl);
        }
        k += '<circle cx="' + x + '" cy="' + y + '" r="34" class="gx-corona gx-corona-3"/><circle cx="' + x + '" cy="' + y + '" r="24" class="gx-corona gx-corona-2"/>'
          + '<path class="gx-sun-rays" d="' + rays + '"/>'
          + '<circle cx="' + x + '" cy="' + y + '" r="16" class="gx-sun" fill="url(#gx-sun-disc)"/><circle cx="' + x + '" cy="' + y + '" r="7" class="gx-sun-core"/>';
      } else {
        if (p.repeat > 1) k += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 3) + '" fill="none" class="k-data-line" stroke-width="1"/>';
        if (p.again) k += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 3 + (p.repeat > 1 ? 2.2 : 0)) + '" fill="none" class="k-data-line" stroke-width="1" stroke-dasharray="2.4 1.8"/>';
        if (s.redacted || s.sparse) k += '<circle cx="' + x + '" cy="' + y + '" r="' + r + '" class="gx-planet gx-hidden" stroke-width="1.5" stroke-dasharray="1.6 1.6"/>';
        else if (s.living) {
          // A living star: its glow, four rays, and its lit sphere.
          var sl = r + 9;
          k += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 5) + '" class="gx-planet-glow gx-living"/>'
            + '<path class="gx-spikes gx-living" d="M' + f(p.x - sl) + ' ' + y + 'L' + f(p.x + sl) + ' ' + y + 'M' + x + ' ' + f(p.y - sl) + 'L' + x + ' ' + f(p.y + sl) + '"/>'
            + '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 0.6) + '" class="gx-planet gx-living" fill="url(#gx-sphere-living)"/>';
        } else {
          // One that is gone: a cooler sphere with its ring, tilted.
          k += '<circle cx="' + x + '" cy="' + y + '" r="' + f(r + 3.5) + '" class="gx-planet-glow gx-gone"/>'
            + '<circle cx="' + x + '" cy="' + y + '" r="' + r + '" class="gx-planet gx-gone" fill="url(#gx-sphere-gone)"/>'
            + '<ellipse class="gx-planet-ring" cx="' + x + '" cy="' + y + '" rx="' + f(r + 4.5) + '" ry="' + f((r + 4.5) * 0.32) + '" transform="rotate(-22 ' + x + ' ' + y + ')"/>';
        }
      }
      var halo = ' paint-order="stroke" stroke-width="4" stroke-linejoin="round"';
      if (isC && !s.redacted) {
        k += '<text x="' + x + '" y="' + f(p.y + 50) + '" text-anchor="middle" font-size="16" font-weight="600" class="gx-name gx-centre k-halo"' + halo + '>' + esc(api.clip(s.names[2], 260)) + '</text>'
          + '<text x="' + x + '" y="' + f(p.y + 64) + '" text-anchor="middle" font-family="' + api.mono + '" font-size="11" class="gx-data k-halo"' + halo + '>' + esc(s.years) + '</text>';
      } else if (keep[p.key]) {
        var L = keep[p.key], an = L.anchor !== 'start' ? ' text-anchor="' + L.anchor + '"' : '';
        k += '<text x="' + f(L.x) + '" y="' + f(L.y) + '"' + an + ' font-size="12" class="gx-name k-halo"' + halo + '>' + esc(L.word) + '</text>';
        if (L.both) k += '<text x="' + f(L.x) + '" y="' + f(L.y + 11.5) + '"' + an + ' font-family="' + api.mono + '" font-size="9.5" class="gx-data k-halo"' + halo + '>' + esc(s.years) + '</text>';
      }
      if (rest) k += '</a>';
      nodes += k + '</g>';
    });
    if (rest) named = keep;
    var defs = '<radialGradient id="gx-well"><stop offset="0" class="gx-well-in"/><stop offset="0.72" class="gx-well-mid"/><stop offset="1" class="gx-well-out"/></radialGradient>'
      + '<radialGradient id="gx-field"><stop offset="0" class="gx-field-in"/><stop offset="0.7" class="gx-field-mid"/><stop offset="1" class="gx-field-out"/></radialGradient>'
      + '<radialGradient id="gx-sphere-living" cx="0.36" cy="0.34" r="0.75"><stop offset="0" class="gx-sph-hi"/><stop offset="0.5" class="gx-sph-living"/><stop offset="1" class="gx-sph-living-deep"/></radialGradient>'
      + '<radialGradient id="gx-sphere-gone" cx="0.36" cy="0.34" r="0.75"><stop offset="0" class="gx-sph-hi"/><stop offset="0.5" class="gx-sph-gone"/><stop offset="1" class="gx-sph-gone-deep"/></radialGradient>'
      + '<radialGradient id="gx-sun-disc" cx="0.42" cy="0.4" r="0.7"><stop offset="0" class="gx-sun-hi"/><stop offset="1" class="gx-sun-edge"/></radialGradient>';
    var C = fr.persons.filter(function (p) { return p.role === 'centre'; })[0], cn = C && C.s && !C.s.redacted ? C.s.names[2] : '';
    var lbl = (root.querySelector('.rv-stage') || root).getAttribute('aria-label') || '';
    return '<svg class="rv-svg gx-svg" data-view="galaxy" viewBox="' + f(hl) + ' ' + f(vt) + ' ' + f(hw) + ' ' + f(vh) + '" width="100%" '
      + (rest ? 'role="group" aria-label="' + esc(lbl + (cn ? ' — ' + cn : '')) + '"' : 'aria-hidden="true"')
      + ' xmlns="http://www.w3.org/2000/svg"><defs>' + defs + '</defs>' + out + links + '<g class="rv-lit"></g>' + nodes + '</svg>';
  }

  /* ---- the dashboard: figures read from the system on screen ---- */
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

  window.axgfView = { name: 'galaxy', shape: shape, edge: edge, paint: paint, rest: rest, named: function () { return named; } };

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
  var WHITE = rgb('--text-primary', [255, 255, 255]), LINK = rgb('--gx-link', WHITE), HOT = rgb('--gx-link-hot', LINK);
  // The two sides of the sun (2026-10-27): violet toward the descendants,
  // white toward the ancestors.
  var UP = rgb('--gx-up', LINK), DOWN = rgb('--gx-down', WHITE);
  var SUNC = rgb('--gx-sun', WHITE), ORBIT = rgb('--gx-planet-living', LINK);
  function rgba(c, a) { return 'rgba(' + c[0] + ',' + c[1] + ',' + c[2] + ',' + a.toFixed(3) + ')'; }

  var under = document.createElement('canvas'), over = document.createElement('canvas');
  under.className = 'rv-ground'; over.className = 'rv-water';
  under.setAttribute('aria-hidden', 'true'); over.setAttribute('aria-hidden', 'true');
  stage.insertBefore(under, stage.firstChild);
  stage.appendChild(over);
  stage.classList.add('rv-alive');
  var gu = under.getContext('2d'), go = over.getContext('2d');
  if (!gu || !go) return;

  // Stars: placed once, from a fixed seed, so the sky is the same sky on
  // every visit and between two frames.
  var stars = [];
  (function () {
    var seed = 7;
    function rnd() { seed = (seed * 16807) % 2147483647; return seed / 2147483647; }
    for (var i = 0; i < 220; i++) stars.push({ u: rnd(), v: rnd(), r: 0.35 + rnd() * 1.1, a: 0.25 + rnd() * 0.6, tw: rnd() * 6.283, sp: 0.6 + rnd() * 1.6 });
  })();
  var flows = [], orbits = [], sun = null, dpr = 1, cw = 0, ch = 0, clock = 0, travelling = false;
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
  function qpoint(x1, y1, x2, y2, t) {
    var mx = (x1 + x2) / 2, my = (y1 + y2) / 2, qx = mx + (SUN[0] - mx) * 0.16, qy = my + (SUN[1] - my) * 0.16, u = 1 - t;
    return [u * u * x1 + 2 * u * t * qx + t * t * x2, u * u * y1 + 2 * u * t * qy + t * t * y2];
  }
  // Light runs from the ancestors toward the living: from the lower end of
  // a link to its upper one, inward on the lower sky, outward on the upper;
  // white on the lower, violet on the upper.
  function layout(L) {
    flows = []; orbits = []; sun = null;
    if (!L || !L.edges) return;
    L.persons.forEach(function (p) { if (p.role === 'centre') sun = [p.x, p.y]; });
    L.edges.forEach(function (e) {
      if (e.conf !== 'a') return;
      var up = e.y1 >= e.y2, a = up ? [e.x1, e.y1, e.x2, e.y2] : [e.x2, e.y2, e.x1, e.y1];
      var n = 20, xs = [], ys = [], cum = [0];
      for (var i = 0; i <= n; i++) {
        var p = qpoint(a[0], a[1], a[2], a[3], i / n);
        xs.push(p[0]); ys.push(p[1]);
        if (i) cum.push(cum[i - 1] + Math.hypot(xs[i] - xs[i - 1], ys[i] - ys[i - 1]));
      }
      if (cum[n] < 8) return;
      flows.push({ x: xs, y: ys, cum: cum, n: n, len: cum[n], phase: Math.abs(Math.sin(e.x1 * 12.9898 + e.y2 * 78.233)) % 1, my: (a[1] + a[3]) / 2 });
    });
    // One comet per orbit, each at its own pace, the outer ones slower.
    var gens = {};
    if (sun) L.persons.forEach(function (p) { if (p.gen) gens[Math.abs(p.gen)] = Math.max(gens[Math.abs(p.gen)] || 0, Math.hypot(p.x - sun[0], p.y - sun[1])); });
    Object.keys(gens).forEach(function (g, i) { orbits.push({ r: gens[g], w: 0.22 / Math.sqrt(+g), phase: i * 1.7 }); });
    request();
  }
  var SPEED = 30;
  function draw(moving) {
    size();
    var W = under.width, H = under.height;
    gu.setTransform(1, 0, 0, 1, 0, 0); go.setTransform(1, 0, 0, 1, 0, 0);
    gu.clearRect(0, 0, W, H); go.clearRect(0, 0, W, H);
    // The sky, still or twinkling.
    gu.globalCompositeOperation = 'lighter';
    stars.forEach(function (st) {
      var a = st.a * (moving ? 0.65 + 0.35 * Math.sin(clock * st.sp + st.tw) : 0.8);
      gu.fillStyle = rgba(WHITE, a);
      gu.beginPath(); gu.arc(st.u * W, st.v * H, st.r * dpr, 0, 6.283); gu.fill();
    });
    gu.globalCompositeOperation = 'source-over';
    var v = view();
    if (!v || !moving || travelling) return;
    var s = v.k * dpr, ox = -v.x * s, oy = -v.y * s;
    go.globalCompositeOperation = 'lighter';
    if (sun) {
      var sxp = sun[0] * s + ox, syp = sun[1] * s + oy, R = (40 + 6 * Math.sin(clock * 1.1)) * v.k * dpr;
      var g = go.createRadialGradient(sxp, syp, 0, sxp, syp, R);
      g.addColorStop(0, rgba(SUNC, 0.28)); g.addColorStop(1, rgba(SUNC, 0));
      go.fillStyle = g; go.beginPath(); go.arc(sxp, syp, R, 0, 6.283); go.fill();
      orbits.forEach(function (o) {
        var th = o.phase + clock * o.w, x = sxp + o.r * s * Math.cos(th), y = syp + o.r * s * Math.sin(th), r = 7 * v.k * dpr;
        var cg = go.createRadialGradient(x, y, 0, x, y, r);
        cg.addColorStop(0, rgba(ORBIT, 0.55)); cg.addColorStop(1, rgba(ORBIT, 0));
        go.fillStyle = cg; go.beginPath(); go.arc(x, y, r, 0, 6.283); go.fill();
      });
    }
    flows.forEach(function (fl) {
      var pos = (fl.phase * fl.len + clock * SPEED) % fl.len, i = 1;
      while (i < fl.n && fl.cum[i] < pos) i++;
      var t = (pos - fl.cum[i - 1]) / ((fl.cum[i] - fl.cum[i - 1]) || 1);
      var x = (fl.x[i - 1] + (fl.x[i] - fl.x[i - 1]) * t) * s + ox, y = (fl.y[i - 1] + (fl.y[i] - fl.y[i - 1]) * t) * s + oy;
      var fade = Math.min(1, pos / 14, (fl.len - pos) / 14), r = 6 * v.k * dpr;
      var pg = go.createRadialGradient(x, y, 0, x, y, r), c = sun && fl.my > sun[1] ? DOWN : UP;
      pg.addColorStop(0, rgba(WHITE, 0.85 * fade)); pg.addColorStop(0.4, rgba(c, 0.45 * fade)); pg.addColorStop(1, rgba(c, 0));
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
