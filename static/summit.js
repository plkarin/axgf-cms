/* Summit's ascent (v0.5.0, docs/adr/0010).
 *
 * The river's own layout — the same people, the same links, from the same
 * payload, through the same reader's lens — raised into a mountain at
 * sunrise, the way the river runs: the oldest at the foot, the youngest
 * toward the summit. The person at the fixed point stands at the camp. Their
 * descendants climb the mountain, one contour line for every generation,
 * each line a little narrower than the one below it, as a mountain is; their
 * ancestors stand down in the valley under the camp, the furthest back the
 * deepest in its mist. Every parentage is a trail from parent to union to
 * child, lit as surely as it is known: solid, broken or a trace. The summit
 * is the youngest generation the drawing reaches: with no descendant above,
 * the camp is on the summit.
 *
 * The mountain answers the climb:
 * - its snow comes down with the years. The nearer the present the person at
 *   the camp was born, the nearer the summit they stand, and the lower the
 *   snow line: read from the year the river itself rests on, and only when
 *   this reader may see it (`meta.era`). Without one, the snow keeps to the
 *   heights, whoever is at the camp;
 * - the sun follows the climb: travelling up to a descendant raises it,
 *   down to an ancestor lowers it, between the last stars and full light.
 *
 * The mountain is a photograph: a wall of peaks above a forest and a misty
 * valley, at dawn (static/img/summit-panorama.jpg, made in the operator's
 * Canva). It is laid under the drawing with its summit on the drawing's and
 * the camp at the foot of its rock, and every row up the mountain is drawn
 * in, about the summit's axis, until it stands under the crest: nobody
 * stands in the sky. Over the photograph the scene paints, as the river's
 * ground is painted (river-scene.js), what the climb changes:
 * - the sky behind the crest, from the night to the dawn: the last stars,
 *   high cloud, the sun rising beside the summit;
 * - the snow the years set, laid on the photograph's own rock down to their
 *   line and further down its couloirs, its relief kept under it;
 * - the light, from the blue of the night to the first sun coming down the
 *   wall;
 * - and, every frame: snow blown off the summit, mist drifting through the
 *   forest and over the valley, light climbing every attested trail.
 * Without WebGL the drawing keeps a mountain of its own (`drawn`).
 *
 * Two things the page knows by name:
 * - `window.axgfView` (river.js): where its people stand (`shape`), how a
 *   frame is painted, the curve of a trail (`edge`, which the lit route
 *   follows too), and the dashboard beside it at rest. Travel, the trail, the
 *   panel, the keyboard, the camera and every touch rule stay river.js's.
 * - `window.axgfRiverScene`: the scene, told the layout at rest and when a
 *   travel begins. It holds still as the river's ground does: reduced
 *   motion, "Cut", the pause, a hidden tab, a river off screen, a machine
 *   too slow for it.
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
  // How far the sun has risen: with how high the reader has climbed since
  // they came, a step of dawn for every generation up toward the summit and
  // one back for every generation down toward the valley, between the last
  // stars and full light. It used to count the itinerary's people, so that
  // going down to the ancestors raised the sun as well, and after six it
  // stood still (the operator's review, 2026-10-23).
  // The climb is kept within what the sky can show, so that a reader who
  // went on down in the night sees the sun rise again at their first step
  // back up, and one who went on up in full light sees it lower at their
  // first step down.
  var climbed = 0, lastCentre = null, lastGen = null, lastYear = null;
  function dawn() {
    return Math.max(0.12, Math.min(1, 0.42 + climbed * 0.15));
  }
  // A new layout: if its fixed point is someone else, how many generations
  // up (or down) that person stood in the drawing the reader left — or, if
  // they were not in it, the years between the two, a generation every 29,
  // only when this reader may see both.
  function track(fr) {
    var m = fr.meta, c = m.centre;
    if (lastCentre !== null && c !== lastCentre) {
      var d = lastGen && (c in lastGen) ? lastGen[c] : null;
      if (d === null && m.era && isFinite(m.year) && lastYear !== null) d = Math.round((m.year - lastYear) / 29);
      climbed = Math.max(-2, Math.min(4, climbed + (d || 0)));
    }
    lastCentre = c;
    lastGen = {};
    fr.persons.forEach(function (p) { if (!(p.id in lastGen)) lastGen[p.id] = p.gen; });
    lastYear = m.era && isFinite(m.year) ? m.year : null;
  }
  // How far the snow comes down, 0.12 (the summit's tip) to 1 (down to the
  // camp), from the year the river rests on: the nearer the present, the
  // nearer the summit, the snowier. Only a year this reader may see: a year
  // it may not see must not show through the weather.
  function snowiness(m) {
    if (!m || !m.era || !isFinite(m.year)) return 0.3;
    return Math.max(0.12, Math.min(1, (m.year - 1705) / 300));
  }
  // A fixed sequence of numbers, so that the same mountain is drawn on every
  // visit and in every frame of a travel.
  function seeded(seed) {
    var s = seed % 2147483647 || 1;
    return function () { s = (s * 16807) % 2147483647; return s / 2147483647; };
  }

  /* ---- the photograph ---- */
  // Made at 1776 × 896 and shown at that aspect; served at 4096 × 2048 so
  // that the graphics card can filter it, and so that it stays sharp on a
  // dense screen (2026-10-23: enlarged four times by a super-resolution
  // model, then brought back to the original's tones). Where its summit is, where a camp
  // stands on it (the foot of its rock, as a share of its height), from
  // where down its crest leaves the whole wall to stand on (`wall`), and its
  // skyline: in each of 1024 columns, the first row that is no longer sky,
  // as a share of its height, 16 bits little-endian. It was read from the
  // photograph itself: the first row of each column that departs from the
  // sky above it, and stays departed.
  var PHOTO = { src: '/static/img/summit-panorama.jpg?v=2', aspect: 1776 / 896, su: 0.4907, sv: 0.0739, camp: 0.58, wall: 0.27 };
  var SKYLINE = 'tze3N7c3tzcAOAM4gTi3OCQ5bDm1OSA6nzrbOoM7HDybPFg94T0XPtw+Xj+xP/A/AEAAQP4/AEAAQIY/JD/bPpw+HT7nPWg9' +
    '6Ty0PDU8tjs3OyQ7FTvbOmA6STpJOgg6QDkkOR452zhpOEk4Rzg3OG047DhrOek54Dm3Obc55TlkOuM6YTsKPJI82zzbPNo8' +
    'kjxLPEk8STxJPEk8ADy3O2U75jqwOkk6/Dm3OW05JDncOF04JzgAOLw3hjckN9I2kjZmNmI2lzYWN5U3FDjcOCQ5uDg5OLs3' +
    'PDe9Nnk2WjUkNcI0jDQNNI4zWDMkMyQzADPLMgYybjFOMSQxJDHbMPYwLDGrMSoypDJwM7czbjNuM7UzaDRFNbc1tzUkNSQ1' +
    'JDVjNeI1YDbfNl43JDcRNyM37za6NpI2jzZJNuM1rTUuNSQ12zSSNFc0ADTsM20z7jK4MoMyBDK3MZgx5TDCMKcweDAAMMQv' +
    'ji9YL9su2y5uLu8tcC2ELNErPSujKjkqCCqUKQ0p7yjbKJIokihtKEkokCjGKI4pxCkAKmUq0ysALD4sdCzbLCQt6yy1LEks' +
    'SSyCKyQrxioWKuEpgyk8KbEoeyhxKIMnDCfMJnIm9iXFJZQlFiUAJdUkGCUkJWUlSCZMJ84n2yfbJ/sn/ie3J9snACi6J24n' +
    'aiduJ68nACgAKLcneiduJ24ntye3J7cnyCcAKAAoACgAKAAoeCgkKbcpOiq1KsEqkipYKtopyikAKkkqGCvFKwAstyu3K8Ar' +
    'ACwALBgsUywWLZUtSS7cLlsvlC9uL24vbi9PLxov2S5lLuYtbi0yLYosSSz+K2Mr3CppKkwqSSoqKgAqACpJKpIq4ypiK+Ar' +
    'diwnLZ8tYC4kLyQvJC/PL0kwSTDQL1Iv0y4WLsMs6CtqK+sqgSnlKAUn6iV3JUwlKCUkJQcl2yTbJAclhiW8JcUljyXbJAgk' +
    '3iPYI7cjrCOTI5ojtyO3I7cjtyMAJAAkKCSnJCUlJiaSJjgnOCgzKR8qPSoAKjkq7Cu1LLct+yxnLI8sDyy3K8grRyx9LHIs' +
    'PCy9KzkqXSmlKbUp4il4Ktsq2yq1Kn8qACrIKCMojSdIJ5AmCibjJbEloyVkJTgl6SSSJCAkpSOrI24jXSMkI64iviLbItgi' +
    'iiILItshFyJhIqci3yJtIzgjTiOuI0QkICV0Jf0l2ybbJuImJCfbJpImjyaDJdQjdiMmIyoi+SGIIb8glSAnIPcfeB8kH9Ae' +
    'kh6SHiMekR0kHdsccBzxG18bDBu9GpIaUxrUGVUZGBk1GPIX0BdfF7sWeBY6FgAWfxXHFCQUJxT7E48TbhMyEyQT7hLrEgAT' +
    'bRMeFNMVKRZyFsMW7RjnGa0aCRssG60bURykHXseLR9bH3UgOSHyISkiRCPfI5Mk7yTiJZIm8yZyJyoolCgTKUgp/CmSKpIq' +
    'kio8K/Erwiy3L0kwMDBjL6AuSS5JLkguNi5JLqIuZi8yMIYwOjEqMnYyjzLUMn8zkjTlNG817jUANhA2STbFNiQ3MDcsN5I2' +
    'rTbbNp429jVXNR81kjRtNMMzlDMYM+AytDKKMusxLTH0MJQwujDbMNsw2zAbMSQxZDE1MkkyVzKSMtsyJDN3M/YzdTT0NCo1' +
    'qDVwNjg3tzftN2w42zjbOC84mDckNyQ3wTYvNrA1bjVuNSQ1HTWRMyUzGTMkMyQzJDMwM24z4TMbNJo0GDWXNRY2lTYUN203' +
    'DjjZONs4cThJOAY4SThlOJI4VDjVN583JDfrNpI2Nja3NSQ1JDXbNNs0kjQANAA0ADRJNJI0lzQWNbw1ujYlN6Q3ADhYOBw5' +
    'nznzOfE6bTuJO207dDvzO948bT0hPVs83Du3O3A7OzubOvQ5tzm3OTI4tzd5NyQ3LjbgNOM0kjQONLczJTPnMtAy2zLbMiQz' +
    'LTNuM+IzADRNNJI0AjXKNUg2wzaEN7c3tzfnN8w4dzk1OhY7IzwpPW09bT1ZPSQ9pTxJPPA7cTt2OgA6ADq/OYk5tDkGOoU6' +
    'kjqnOts6bDqTORQ5RjgHOPI3tze3N7c3UzfbNts2kjaSNos2gTaVNjQ3tze3N7c3xDcAOAA4ADgAOPE35DewN203bTdLN203' +
    'tje3N7c3qDcHN7c2PDbRNeE1cTarNik3qDcFOF04kzhWOSM6nDqsOnY6JDoJOkU6RjoqOgw61DmzOSs5tTg2OMM3uTc9N4Q2' +
    'SjYMNo81ZTVJNUU1SDUkNYQ16DUwN9w3Rzi7N6Q3jDcXN5g2CDYANrc1rTWxNbU1rTW0NfU1ADYANjU2azagNts2JDckNxU3' +
    '8DZEN203tze3Nyc4pjjaODc4JDgkOHU42zgkOXg5tzlJOpI62zrbOgQ7DzvWOtQ6hTpGOkk6STqxOuc6ZjuIO5I7pTt5O5c7' +
    'tzu3O7c7ADxkPGw91TykPJI8uzzWPNs82zwZPZg9AD48Pqg+kT+aPyU/Pj+MQCRBJEGcQBVA7j/+P/U/2T/EP19AdUAlQJs/' +
    'bD8BP4o+kj6SPrA+2z4cP1E/hz8DQPZAbUHMQUtCyUJIQ8dDRkT1RFtFYUZfRy5ISUhJSAxIt0e3R21HNUf/Rm9GakWnRChE' +
    'qUMrQ3xC5EH2QNpAZ0DoP2M/fT7rPbs9rj1tPW09bT2xPQA+Zj7lPmQ/bT/PPwpAzECVQV1Ck0JbQ/xDsUQkRZ9Fy0XmRiRH' +
    'm0czSL9IoEhBSLdHbEckRwBHvkYmRr9Fl0UGRSxErUM=';
  var SKY = (function () {
    var bin = atob(SKYLINE), out = new Float32Array(bin.length / 2);
    for (var i = 0; i < out.length; i++) out[i] = (bin.charCodeAt(2 * i) | bin.charCodeAt(2 * i + 1) << 8) / 65535;
    return out;
  })();
  // The photograph under a layout: its summit on the drawing's, the camp at
  // the foot of its rock (or above it, when the valley needs the room), and
  // never narrower than the window river.js opens on the drawing (its
  // `frame`, whose KMAX this is).
  var KMAX = 1.15;
  function placing(sx, campY, summitY, hl, hw, foot) {
    var sw = (stage && stage.clientWidth) || hw, fw = Math.max(hw, sw / Math.min(sw / hw, KMAX)), fx = hl - (fw - hw) / 2;
    var ax = sx + 18, ay = summitY + 2;
    var cover = Math.max((ax - fx) / PHOTO.su, (fx + fw - ax) / (1 - PHOTO.su)) * 1.02 / PHOTO.aspect;
    // Its valley floor reaches most of the way down the frame: below it the
    // valley goes on in mist, never for long.
    var h = Math.max((campY - ay) / (PHOTO.camp - PHOTO.sv), cover, (foot - ay) / (1 - PHOTO.sv) * 0.92);
    return { x: ax - PHOTO.su * h * PHOTO.aspect, y: ay - PHOTO.sv * h, w: h * PHOTO.aspect, h: h };
  }
  // How far the rock reaches either side of the summit's axis at the height
  // y, `margin` under the crest; past the photograph's edges, as far as it
  // likes (the scene folds the photograph over there).
  function rockAt(pl, y, margin) {
    var v = (y - margin - pl.y) / pl.h, n = SKY.length, l = Math.floor(PHOTO.su * n), r = l;
    while (l > 0 && SKY[l - 1] < v) l--;
    while (r < n - 1 && SKY[r + 1] < v) r++;
    return [l === 0 ? -Infinity : pl.x + l / n * pl.w, r === n - 1 ? Infinity : pl.x + (r + 1) / n * pl.w];
  }
  // What the scene needs of a layout: the photograph's place, the camp, the
  // summit, a generation's height, the snow line and the contours.
  function scenery(m) {
    return { pl: m.pano, camp: m.camp[1], summit: m.summit, rise: m.rise, up: m.up || 0,
      snow: m.summit + (m.camp[1] - m.summit) * snowiness(m) };
  }
  // The scenery at rest (from the frame painted) and the one the next travel
  // leaves (from the frame shaped); whether there is a scene at all (WebGL),
  // set before river.js first shapes.
  var REST = null, NEXT = null, SCENE = false;

  /* ---- where everyone stands ---- */
  function shape(fr) {
    track(fr);
    var m = fr.meta, hl = m.hview[0], hw = m.hview[1], vt = m.view[0], vh = m.view[1];
    var C = fr.persons.filter(function (p) { return p.role === 'centre'; })[0];
    var cx = C ? C.x : hl + hw / 2, cy = C ? C.y : m.cy, step0 = m.step || 100;
    // How far the drawing reaches each way, in generations, as the river
    // draws them: down to the ancestors (below the river's centre) and up to
    // the descendants (above it), with the lines the record continues past
    // them and their counts — so that a count is never cut by the frame.
    var up = 0, down = 0;
    function reach(y, extra) {
      var g = (y - cy) / step0;
      if (g > 0) down = Math.max(down, g + (extra || 0)); else up = Math.max(up, -g + (extra || 0));
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
    var sx = hl + hw / 2, bottom = vt + vh - 30, summitY = vt + 26, span = up + down + 0.5;
    // The rows from `top` down, a step for each generation. The camp's name
    // and years take some 46 units under it: when the steps are too short to
    // leave them that, the valley's first row stands NAME under the camp
    // (`under` more than a step) and the rest a step apart.
    var NAME = 56;
    function lay(top) {
      var avail = bottom - top, step = Math.min(step0 * 1.25, avail / Math.max(1, span)), under = 0;
      if (down > 0.25 && step < NAME) { step = Math.min(step, (avail - NAME) / Math.max(1, span - 1)); under = NAME - step; }
      return { top: top, step: step, under: under, campY: top + up * step };
    }
    var L = lay(vt + vh * (0.18 + 0.06 * Math.max(0, 3 - up))), pl = null;
    if (SCENE) {
      pl = placing(sx, L.campY, summitY, hl, hw, vt + vh);
      // On the photograph the heights begin no higher than where its crest
      // leaves the whole wall to stand on: above that a row would have only
      // the summit's peak for its rock. Laying the rows lower moves the
      // photograph, so the two are settled together.
      for (var pass = 0; pass < 4 && up > 0.25; pass++) {
        var wall = pl.y + PHOTO.wall * pl.h;
        if (L.top >= wall - 0.5) break;
        L = lay(Math.min(wall, bottom - span * 22));
        pl = placing(sx, L.campY, summitY, hl, hw, vt + vh);
      }
    }
    var step = L.step, under = L.under, campY = L.campY;
    // Each half takes the river's order from left to right over its own
    // breadth, as Galaxy's halves do: the valley's (0, the ancestors) and the
    // heights' (1, the descendants), which narrow as they rise.
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
    // On the photograph, every row up the mountain must stand on its rock:
    // each is drawn in about the summit's axis as much as it needs to stand
    // under the crest where it is, and never less than the row below it.
    var fit = [1];
    if (pl) {
      var ext = [];
      var note = function (x, y) {
        var g = Math.round((cy - y) / step0);
        if (g < 1) return;
        var d = (x - mid[1]) * k[1], e = ext[g] || (ext[g] = [0, 0]);
        e[0] = Math.min(e[0], d); e[1] = Math.max(e[1], d);
      };
      fr.persons.forEach(function (p) { note(p.x, p.y); });
      fr.tails.forEach(function (t) { note(t.x2, t.y2); });
      fr.couples.forEach(function (c) { note(c.x, c.y); });
      for (var g = 1; g < ext.length; g++) {
        var e = ext[g] || [0, 0], R = rockAt(pl, campY - g * step, 24), nar = 1 - steep * Math.min(g, Math.max(1, up)), c = fit[g - 1];
        if (e[0] < 0) c = Math.min(c, Math.max(0, sx - 30 - R[0]) / (-e[0] * nar));
        if (e[1] > 0) c = Math.min(c, Math.max(0, R[1] - 30 - sx) / (e[1] * nar));
        fit.push(Math.max(0.12, c));
      }
    }
    function fitAt(g) {
      var i = Math.min(fit.length - 1, Math.floor(g)), j = Math.min(fit.length - 1, i + 1);
      return fit[i] + (fit[j] - fit[i]) * (g - Math.floor(g));
    }
    function place(x, y) {
      var g = (y - cy) / step0, ny = campY + g * step + (g > 0 ? under * Math.min(1, g) : 0);
      if (Math.abs(g) < 0.25) {
        var dx = x - cx;
        if (Math.abs(dx) < 1) return [sx, ny];
        return [sx + (dx < 0 ? -1 : 1) * (28 + Math.min(1, Math.abs(dx) / 140) * 62), ny];
      }
      var h = g > 0 ? 0 : 1, narrow = h === 1 ? (1 - steep * Math.min(-g, Math.max(1, up))) * fitAt(-g) : 1;
      return [sx + (x - mid[h]) * k[h] * narrow, ny];
    }
    function moved(o) { var q = place(o.x, o.y); return Object.assign({}, o, { x: q[0], y: q[1] }); }
    function markAt(mk) {
      if (!mk) return mk;
      var q = place(mk.x, mk.y), out = Object.assign({}, mk, { x: q[0], y: q[1] });
      if (mk.leader) { var a = place(mk.leader[0], mk.leader[1]), b = place(mk.leader[2], mk.leader[3]); out.leader = [a[0], a[1], b[0], b[1]]; }
      return out;
    }
    var meta = Object.assign({}, m, { camp: [sx, campY], summit: summitY, rise: step, under: under, up: Math.round(up), down: Math.round(down), pano: pl });
    if (pl) NEXT = scenery(meta);
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
      meta: meta
    };
  }
  // A trail climbs as a path does: up the slope, across, and up again.
  function edge(x1, y1, x2, y2) {
    var my = (y1 + y2) / 2, r = function (v) { return Math.round(v * 10) / 10; };
    return 'M' + r(x1) + ' ' + r(y1) + 'C' + r(x1) + ' ' + r(my) + ' ' + r(x2) + ' ' + r(my) + ' ' + r(x2) + ' ' + r(y2);
  }

  /* ---- the drawing's own mountain, without a scene ---- */
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
  // The far ranges, the massif with its relief and its snow, the contours,
  // and the valley with its forest edge and its mist: the mountain the
  // drawing carries when there is no scene to lay the photograph.
  function drawn(fr, f, m, sx, campY, rise, summitY, up) {
    var hl = m.hview[0], hw = m.hview[1], vt = m.view[0], vh = m.view[1], out = '';
    var S = snowiness(m), D = dawn(), valleyY = campY + rise * 0.55;
    var X0 = hl - hw, X1 = hl + 2 * hw, below = vt + vh * 2;
    // The far ranges, the farther one higher and paler, each with the snow
    // of its own heights.
    var horizon = Math.min(campY - rise * 0.2, vt + vh * 0.62);
    var far = ridge(11, X0, X1, horizon, vh * 0.2, 240, hw), near = ridge(23, X0, X1, horizon + vh * 0.08, vh * 0.16, 240, hw * 1.3);
    var closeAt = function (pts) { return polyline(pts) + 'L' + f(X1) + ' ' + f(below) + 'L' + f(X0) + ' ' + f(below) + 'Z'; };
    var farTop = Math.min.apply(null, far.map(function (p) { return p[1]; }));
    out += '<path d="' + closeAt(far) + '" class="sm-range sm-range-far"/>';
    out += '<path d="' + closeAt(far) + '" class="sm-range-snow" clip-path="url(#sm-far-snow)"/>';
    out += '<path d="' + closeAt(near) + '" class="sm-range sm-range-mid"/>';
    // The mountain: rock in the shade, its sunward side warming with the
    // dawn, then its snow down to the line the years set.
    var mass = massif(fr, sx, campY, rise, summitY, valleyY + rise * 0.6);
    var peakOf = mass.peaks.filter(function (pk) { return pk[2]; })[0];
    var md = polyline(mass.outline) + 'L' + f(X1) + ' ' + f(below) + 'L' + f(X0) + ' ' + f(below) + 'Z';
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
    // The contour of every generation up the mountain.
    for (var g = 1; g <= up; g++) {
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
    var farSnow = farTop + (horizon - farTop) * (0.25 + 0.5 * S);
    var defs = '<linearGradient id="sm-lit" x1="0" y1="0" x2="1" y2="0"><stop offset="0.35" class="sm-lit-0"/><stop offset="1" class="sm-lit-1"/></linearGradient>'
      + '<linearGradient id="sm-snow-fade" gradientUnits="userSpaceOnUse" x1="0" y1="' + f(summitY) + '" x2="0" y2="' + f(snowY + 20) + '"><stop offset="0" class="sm-snow-0"/><stop offset="0.7" class="sm-snow-1"/><stop offset="1" class="sm-snow-2"/></linearGradient>'
      + '<linearGradient id="sm-glow" x1="0" y1="0" x2="1" y2="0"><stop offset="0.4" class="sm-glow-0"/><stop offset="1" class="sm-glow-1"/></linearGradient>'
      + '<linearGradient id="sm-mist" x1="0" y1="0" x2="0" y2="1"><stop offset="0" class="sm-mist-0"/><stop offset="0.5" class="sm-mist-1"/><stop offset="1" class="sm-mist-0"/></linearGradient>'
      + '<clipPath id="sm-snowline"><path d="' + clip + '"/></clipPath>'
      + '<clipPath id="sm-on-rock"><path d="' + md + '"/></clipPath>'
      + '<clipPath id="sm-far-snow"><rect x="' + f(X0) + '" y="' + f(vt - vh) + '" width="' + f(X1 - X0) + '" height="' + f(farSnow - vt + vh) + '"/></clipPath>';
    return { out: out, defs: defs };
  }

  /* ---- the drawing ---- */
  var named = {};
  function paint(fr, api, rest) {
    var f = api.f, esc = api.esc, m = fr.meta, PM = api.byKey(fr.persons), out = '', links = '', nodes = '', defs = '';
    var hl = m.hview[0], hw = m.hview[1], vt = m.view[0], vh = m.view[1];
    var camp = m.camp || [hl + hw / 2, vt + vh * 0.6], sx = camp[0], campY = camp[1];
    var rise = m.rise || 60, summitY = m.summit || vt + 20, up = m.up || 0, down = m.down || 0;
    // Under a scene the mountain is the photograph, its contours and its
    // weather the scene's; without one, the drawing's own.
    if (rest && m.pano) REST = scenery(m);
    if (!SCENE) { var dm = drawn(fr, f, m, sx, campY, rise, summitY, up); out += dm.out; defs = dm.defs; }
    // Lines the record continues past the drawing, and lines it ends. How
    // many more each one leads to is written once the names are placed.
    var counts = [], boxes = [], g;
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
    // The summit's flag, over the youngest generation the drawing reaches:
    // on the camp itself when nobody stands above it.
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
    for (g = 1; g <= down; g++) number('▼' + g, campY + g * rise + (m.under || 0) - 3);
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
    // From the valley (the oldest generation, the smallest `gen`) to the
    // summit (the youngest), left to right, as a climb is read.
    var most = 1, pts = [], W = 160, H = 40, i = 0, count = hi - lo + 1;
    for (var g = lo; g <= hi; g++) most = Math.max(most, per[g] || 0);
    for (g = lo; g <= hi; g++, i++) {
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

  /* ---- the scene ---- */
  if (!stage || !window.WebGLRenderingContext) return;
  var reduce = window.matchMedia && matchMedia('(prefers-reduced-motion: reduce)').matches;
  var cut = root.dataset.riverMotion === 'cut';
  var STORE = 'axgf-river-still', paused = false;
  try { paused = localStorage.getItem(STORE) === '1'; } catch (e) { /* private window */ }
  var css = getComputedStyle(root);
  function colour(name, fallback) {
    var v = css.getPropertyValue(name).trim();
    if (v.charAt(0) !== '#') return fallback;
    var h = v.slice(1);
    if (h.length === 3) h = h.charAt(0) + h.charAt(0) + h.charAt(1) + h.charAt(1) + h.charAt(2) + h.charAt(2);
    return [0, 2, 4].map(function (i) { return parseInt(h.slice(i, i + 2), 16) / 255; });
  }
  var GREY = [0.12, 0.14, 0.2], WHITE = [1, 1, 1];
  var P = {
    night: colour('--sm-sky-night', GREY), dusk: colour('--sm-sky-dawn', GREY), glow: colour('--sm-sky-glow', GREY),
    sun: colour('--sm-sun', WHITE), rock: colour('--sm-rock', GREY), snow: colour('--sm-snow', WHITE),
    snowSh: colour('--sm-snow-shade', WHITE), snowGl: colour('--sm-snow-glow', WHITE), valley: colour('--sm-valley', GREY),
    mist: colour('--sm-mist', WHITE), trail: colour('--sm-trail', WHITE), hot: colour('--sm-trail-hot', WHITE)
  };

  var VERT = 'attribute vec2 a;void main(){gl_Position=vec4(a,0.0,1.0);}';
  var NOISE = [
    'float hash(vec2 p){return fract(sin(dot(p,vec2(127.1,311.7)))*43758.5453);}',
    'float noise(vec2 p){vec2 i=floor(p),f=fract(p),u=f*f*(3.0-2.0*f);',
    ' return mix(mix(hash(i),hash(i+vec2(1.0,0.0)),u.x),mix(hash(i+vec2(0.0,1.0)),hash(i+vec2(1.0,1.0)),u.x),u.y);}',
    'float fbm(vec2 p){float s=0.0,a=0.5;for(int i=0;i<5;i++){s+=a*noise(p);p=p*2.03+vec2(17.3,9.1);a*=0.5;}return s;}'
  ].join('\n');
  // What holds still between two changes: the photograph in its place, in
  // the light of the dawn reached, under the snow of the years, behind its
  // contours, against the sky. Drawn into a texture for every new layout,
  // dawn or window, never for every frame. Its alpha is how much of each
  // pixel is sky.
  var LAND = [
    'precision highp float;',
    'uniform vec2 uRes;uniform vec4 uView;uniform vec4 uPl;uniform vec4 uGeo;uniform vec3 uSun;',
    'uniform float uDawn,uHas,uUp;uniform sampler2D uPhoto,uSky;',
    'uniform vec3 uNight,uDusk,uGlow,uSunC,uRock,uSnowC,uSnowSh,uSnowGl,uValley,uMist;',
    NOISE,
    'float dec(vec4 c){return (c.r*65280.0+c.g*255.0)/65535.0;}',
    'float lum(vec3 c){return dot(c,vec3(0.299,0.587,0.114));}',
    // The skyline at u, folded past the photograph's edges as the photograph is.
    'float crest(float u){float m=mod(u,2.0);m=m>1.0?2.0-m:m;float x=m*1024.0-0.5;',
    ' float i=clamp(floor(x),0.0,1023.0),j=min(i+1.0,1023.0);',
    ' return mix(dec(texture2D(uSky,vec2((i+0.5)/1024.0,0.5))),dec(texture2D(uSky,vec2((j+0.5)/1024.0,0.5))),clamp(x-floor(x),0.0,1.0));}',
    'void main(){',
    ' vec2 px=vec2(gl_FragCoord.x,uRes.y-gl_FragCoord.y);vec2 w=uView.xy+px*uView.zw;',
    ' float campY=uGeo.x,sumY=uGeo.y,rise=uGeo.z,snowY=uGeo.w,D=uDawn,k=1.0/uView.z;',
    ' vec2 uv=(w-uPl.xy)/uPl.zw;float cr=crest(uv.x);',
    // The sky: the night overhead, the dawn along the crest, warmer as the
    // climb goes on; the sun's glow and, once risen, its disc.
    ' float top=uPl.y-0.2*uPl.w,hz=uPl.y+0.3*uPl.w;float t=clamp((w.y-top)/(hz-top),0.0,1.0);',
    ' vec3 sky=mix(uNight,uDusk,smoothstep(0.0,1.0,t)*(0.3+0.7*D));',
    ' sky=mix(sky,uGlow,smoothstep(0.5,1.0,t)*(0.1+0.9*D)*0.8);',
    ' float ds=length((w-uSun.xy)*vec2(1.0,1.2));',
    ' sky+=uSunC*exp(-ds/(uSun.z*(2.5+8.0*D)))*(0.1+0.9*D);',
    ' sky=mix(sky,uSunC*1.06,(1.0-smoothstep(uSun.z*0.9,uSun.z*1.1,ds))*smoothstep(0.3,0.6,D));',
    // The photograph, a little inside its own crest, so that none of the sky
    // it was taken under is left along the edge.
    ' float e=max(1.0/(k*uPl.w),1.3/1024.0);',
    ' float a=clamp((uv.y-cr-1.6/1024.0)/e+0.5,0.0,1.0);',
    ' vec2 q=vec2(uv.x,clamp(uv.y,0.0,1.0));',
    // Its pixel, its faces (a few pixels across) and its slopes (a few
    // dozen): a face lighter than its slope is a ledge or a couloir, one
    // darker a cliff.
    ' vec3 ph=texture2D(uPhoto,q).rgb,cb=texture2D(uPhoto,q,4.5).rgb;float l0=lum(ph),lm=lum(texture2D(uPhoto,q,1.5).rgb),lb=lum(cb);',
    // Until it has arrived: its outline, in the theme's rock.
    ' vec3 base=uRock*(0.8+0.4*fbm(w*0.01))*mix(1.0,0.55,smoothstep(0.3,0.9,uv.y));',
    ' ph=mix(base,ph,uHas);l0=mix(0.2,l0,uHas);lm=mix(0.2,lm,uHas);lb=mix(0.2,lb,uHas);',
    // Past its foot, the valley goes on under its mist: the photograph's
    // last rows, broadly, fading into the mist's own colour.
    ' if(uv.y>0.975){vec3 fl=mix(base,texture2D(uPhoto,vec2(uv.x,0.97),6.5).rgb,uHas);',
    '  vec3 deep=mix(fl,uMist*0.32+uValley*0.6,0.35+0.4*smoothstep(1.0,1.3,uv.y));',
    '  ph=mix(ph,deep,smoothstep(0.975,1.005,uv.y));',
    '  ph*=1.0-0.3*smoothstep(1.05,1.6,uv.y);}',
    // The snow of the years, down to their line, ragged along it. Just under
    // the line it takes the ledges and couloirs the photograph already holds
    // snow in; higher, the faces too; never the cliffs, too steep to keep it.
    // In the shade of the wall it is the snow's shade, its relief the rock's.
    ' float n1=fbm(vec2(w.x*0.011,3.7)),n2=noise(w*0.045);',
    ' float line=snowY+(n1-0.5)*rise*0.9;',
    ' float above=1.0-smoothstep(line-rise*0.3,line+rise*0.3,w.y);',
    ' float deep=smoothstep(line,line-rise*2.5,w.y);',
    ' float hold=0.45*l0+0.55*lm+1.3*(lm-lb)+(n2-0.5)*0.08;',
    ' float thr=mix(0.46,0.3,deep);',
    ' float cover=above*smoothstep(thr-0.07,thr+0.09,hold);',
    ' cover=max(cover,smoothstep(0.42,0.56,hold)*(1.0-smoothstep(line,line+rise*1.4,w.y)));',
    ' cover=clamp(cover,0.0,0.95)*step(uv.y,1.0);',
    ' float glowOn=smoothstep(0.0,0.12,cb.r-cb.b)*uHas;',
    ' vec3 sc=mix(uSnowSh,uSnowGl,glowOn*0.75)*(0.72+0.34*smoothstep(0.2,0.5,lm))*clamp(1.0+1.1*(l0-lb),0.72,1.22);',
    ' ph=mix(ph,sc,cover);',
    // The light: in the night darker and bluer, the first light on the
    // summit not come yet; at sunrise the sun coming down the wall, warm on
    // the snow and on the rock it reaches.
    ' float night=1.0-smoothstep(0.12,0.62,D),l1=lum(ph);',
    ' float warm=smoothstep(0.02,0.16,ph.r-ph.b);',
    ' ph=mix(ph,l1*vec3(0.8,0.9,1.12),warm*night*0.9);',
    ' ph*=mix(1.0,0.56,night);',
    ' ph=mix(ph,ph*vec3(0.82,0.9,1.1),night*0.6);',
    ' float day=smoothstep(0.5,1.0,D);',
    ' float litY=sumY+(campY-sumY)*(day*1.2-0.08);',
    ' float lit=day*(1.0-smoothstep(litY-rise*0.5,litY+rise*0.5,w.y))*step(uv.y,0.62);',
    ' ph*=mix(vec3(1.0),vec3(1.0)+0.42*(uSunC*0.6+uSnowGl*0.4),lit);',
    ' ph*=1.0+0.08*day;',
    ' ph+=uSunC*exp(-ds/(uSun.z*7.0))*0.25*D;',
    // The contour of every generation up the mountain, one pixel wide.
    ' float ct=0.0;',
    ' for(int i=1;i<=12;i++){if(float(i)>uUp)break;float d=abs(w.y-(campY-float(i)*rise))*k;',
    '  ct=max(ct,(1.0-smoothstep(0.35,1.25,d))*step(fract(w.x/7.5),0.22));}',
    ' ph=mix(ph,uSnowSh,ct*0.3);',
    ' gl_FragColor=vec4(mix(sky,ph,a),1.0-a);',
    '}'
  ].join('\n');
  // Every frame: the still landscape, with what moves over it.
  var LIVE = [
    'precision highp float;',
    'uniform sampler2D uLand;uniform vec2 uRes;uniform vec4 uView;uniform vec4 uPl;uniform vec4 uGeo;uniform vec2 uTop;',
    'uniform float uTime,uDawn;uniform vec3 uMist,uGlow,uSunC,uSnowC,uSnowSh;',
    NOISE,
    'void main(){',
    ' vec4 l=texture2D(uLand,gl_FragCoord.xy/uRes);vec3 col=l.rgb;float sk=l.a;',
    ' vec2 px=vec2(gl_FragCoord.x,uRes.y-gl_FragCoord.y);vec2 w=uView.xy+px*uView.zw;vec2 uv=(w-uPl.xy)/uPl.zw;',
    ' float campY=uGeo.x,rise=uGeo.z,D=uDawn,T=uTime;',
    // The last stars, twinkling, gone with the dawn; high cloud drifting,
    // lit by it.
    ' if(sk>0.01){vec2 c=floor(w*0.5);float h=hash(c);',
    '  if(h>0.9955){vec2 o=fract(w*0.5)-vec2(hash(c+1.7),hash(c+3.1));float st=exp(-dot(o,o)*120.0);',
    '   col+=vec3(st*(1.0-D)*(0.55+0.45*sin(T*(0.8+h*3.0)+h*40.0)))*sk;}',
    '  float cl=fbm(vec2(w.x*0.0015+T*0.004,w.y*0.011-T*0.0003));',
    '  float kc=smoothstep(0.56,0.86,cl)*(1.0-smoothstep(uPl.y+0.12*uPl.w,uPl.y+0.3*uPl.w,w.y));',
    '  col=mix(col,mix(uMist*0.45,mix(uGlow,uSunC,0.4),0.15+0.85*D),kc*0.38*sk);}',
    // Snow blown off the summit, streaming west on the wind.
    ' vec2 d=w-uTop;float al=-d.x;',
    ' if(al>-12.0&&al<uPl.z*0.2){float ac=d.y+al*0.06;float wd=4.0+al*0.24;',
    '  float env=exp(-ac*ac/(wd*wd))*smoothstep(-12.0,10.0,al)*(1.0-smoothstep(uPl.z*0.04,uPl.z*0.19,al));',
    '  if(env>0.003){float n=fbm(vec2((al+T*26.0)*0.03,ac*0.07+T*0.1));',
    '   col=mix(col,mix(uSnowSh,mix(uSnowC,uGlow,0.3),0.25+0.75*D),env*smoothstep(0.25,0.7,n)*0.6*(0.4+0.6*D));}}',
    // Mist drifting through the forest and over the valley floor, and a veil
    // of it between the camp and the valley under it.
    ' float low=smoothstep(0.62,0.9,uv.y),band=exp(-pow((w.y-(campY+rise*0.55))/(rise*0.4),2.0));',
    ' if(low>0.001||band>0.01){float m1=fbm(vec2(w.x*0.0026-T*0.011,w.y*0.015+T*0.0018));',
    '  if(low>0.001){float m2=fbm(vec2(w.x*0.0061+T*0.017,w.y*0.028-T*0.003));',
    '   col=mix(col,uMist*(0.5+0.35*D),low*smoothstep(0.38,0.82,m1*0.7+m2*0.3)*0.42);}',
    '  col=mix(col,uMist*(0.5+0.3*D),band*smoothstep(0.42,0.85,m1)*0.22);}',
    ' col+=(hash(gl_FragCoord.xy+fract(T)*91.7)-0.5)/255.0;',
    ' gl_FragColor=vec4(col,1.0);',
    '}'
  ].join('\n');

  var ground = document.createElement('canvas');
  ground.className = 'rv-ground';
  ground.setAttribute('aria-hidden', 'true');
  var gl = ground.getContext('webgl', { alpha: false, antialias: false, depth: false, stencil: false, premultipliedAlpha: false, powerPreference: 'low-power' });
  if (!gl) return;
  function program(fsrc) {
    function shader(type, src) {
      var sh = gl.createShader(type);
      gl.shaderSource(sh, src); gl.compileShader(sh);
      if (gl.getShaderParameter(sh, gl.COMPILE_STATUS)) return sh;
      if (window.console) console.warn('summit.js: the scene does not compile here', gl.getShaderInfoLog(sh));
      return null;
    }
    var vs = shader(gl.VERTEX_SHADER, VERT), fs = shader(gl.FRAGMENT_SHADER, fsrc);
    if (!vs || !fs) return null;
    var pr = gl.createProgram();
    gl.attachShader(pr, vs); gl.attachShader(pr, fs);
    gl.bindAttribLocation(pr, 0, 'a');
    gl.linkProgram(pr);
    return gl.getProgramParameter(pr, gl.LINK_STATUS) ? pr : null;
  }
  var landP = program(LAND), liveP = program(LIVE);
  if (!landP || !liveP) return;
  var buf = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, buf);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 3, -1, -1, 3]), gl.STATIC_DRAW);
  gl.enableVertexAttribArray(0);
  gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
  function uniforms(pr, names) {
    var u = {};
    names.forEach(function (k) { u[k] = gl.getUniformLocation(pr, k); });
    return u;
  }
  var UL = uniforms(landP, ['uRes', 'uView', 'uPl', 'uGeo', 'uSun', 'uDawn', 'uHas', 'uUp', 'uPhoto', 'uSky',
    'uNight', 'uDusk', 'uGlow', 'uSunC', 'uRock', 'uSnowC', 'uSnowSh', 'uSnowGl', 'uValley', 'uMist']);
  var UV = uniforms(liveP, ['uLand', 'uRes', 'uView', 'uPl', 'uGeo', 'uTop', 'uTime', 'uDawn', 'uMist', 'uGlow', 'uSunC', 'uSnowC', 'uSnowSh']);
  gl.useProgram(landP);
  [['uNight', 'night'], ['uDusk', 'dusk'], ['uGlow', 'glow'], ['uSunC', 'sun'], ['uRock', 'rock'], ['uSnowC', 'snow'],
    ['uSnowSh', 'snowSh'], ['uSnowGl', 'snowGl'], ['uValley', 'valley'], ['uMist', 'mist']].forEach(function (q) { gl.uniform3fv(UL[q[0]], P[q[1]]); });
  gl.uniform1i(UL.uPhoto, 1); gl.uniform1i(UL.uSky, 2);
  gl.useProgram(liveP);
  [['uMist', 'mist'], ['uGlow', 'glow'], ['uSunC', 'sun'], ['uSnowC', 'snow'], ['uSnowSh', 'snowSh']].forEach(function (q) { gl.uniform3fv(UV[q[0]], P[q[1]]); });
  gl.uniform1i(UV.uLand, 0);
  function texture(filter, wrap) {
    var t = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, t);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, wrap);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, filter);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, filter);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 1, 1, 0, gl.RGBA, gl.UNSIGNED_BYTE, new Uint8Array([0, 0, 0, 255]));
    return t;
  }
  var landT = texture(gl.NEAREST, gl.CLAMP_TO_EDGE), skyT = texture(gl.NEAREST, gl.CLAMP_TO_EDGE);
  // The photograph folds over at its sides, should a window ever be wider.
  var photoT = texture(gl.LINEAR, gl.MIRRORED_REPEAT);
  var fbo = gl.createFramebuffer();
  // The skyline, two bytes to a column, as the shader reads it back.
  var skyData = new Uint8Array(SKY.length * 4);
  for (var si = 0; si < SKY.length; si++) {
    var sq = Math.round(SKY[si] * 65535);
    skyData[si * 4] = sq >> 8; skyData[si * 4 + 1] = sq & 255; skyData[si * 4 + 3] = 255;
  }
  gl.bindTexture(gl.TEXTURE_2D, skyT);
  gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, SKY.length, 1, 0, gl.RGBA, gl.UNSIGNED_BYTE, skyData);
  SCENE = true;

  /* ---- the light down the trails: a 2D canvas over the drawing ---- */
  var over = document.createElement('canvas');
  over.className = 'rv-water';
  over.setAttribute('aria-hidden', 'true');
  var go = over.getContext('2d');
  stage.insertBefore(ground, stage.firstChild);
  stage.appendChild(over);
  stage.classList.add('rv-alive');
  function rgba(c, a) { return 'rgba(' + Math.round(c[0] * 255) + ',' + Math.round(c[1] * 255) + ',' + Math.round(c[2] * 255) + ',' + a.toFixed(3) + ')'; }

  var flows = [], dpr = 1, cw = 0, ch = 0, cap = 2, clock = 0, lastClock = 0, travelling = false, landKey = '';
  // The photograph arrives once, and fades in over its outline.
  var photo = new Image(), has = false, hasAt = 0;
  photo.onload = function () {
    if (gl.isContextLost()) return;
    gl.bindTexture(gl.TEXTURE_2D, photoT);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGB, gl.RGB, gl.UNSIGNED_BYTE, photo);
    gl.generateMipmap(gl.TEXTURE_2D);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR);
    has = true; hasAt = performance.now(); landKey = '';
    if (!running) draw(false);
  };
  photo.src = PHOTO.src;
  function shown() { return !has ? 0 : still() ? 1 : Math.min(1, (performance.now() - hasAt) / 700); }
  function size() {
    var w = stage.clientWidth, h = stage.clientHeight, r = Math.min(window.devicePixelRatio || 1, cap);
    if (w === cw && h === ch && r === dpr) return false;
    cw = w; ch = h; dpr = r;
    ground.width = over.width = Math.max(1, Math.round(w * dpr));
    ground.height = over.height = Math.max(1, Math.round(h * dpr));
    gl.bindTexture(gl.TEXTURE_2D, landT);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, ground.width, ground.height, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
    gl.bindFramebuffer(gl.FRAMEBUFFER, fbo);
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, landT, 0);
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    landKey = '';
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

  /* ---- the scenery, and its way from one layout to the next ----
   * A travel moves the photograph with the people, in step with river.js's
   * own (560 ms, the same easing), so that everyone keeps their rock. */
  var sc = { from: null, to: null, t0: 0, dur: 0 };
  function ease(t) { return t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2; }
  function between(a, b, t) {
    var m = function (x, y) { return x + (y - x) * t; };
    return { pl: { x: m(a.pl.x, b.pl.x), y: m(a.pl.y, b.pl.y), w: m(a.pl.w, b.pl.w), h: m(a.pl.h, b.pl.h) },
      camp: m(a.camp, b.camp), summit: m(a.summit, b.summit), rise: m(a.rise, b.rise), snow: m(a.snow, b.snow), up: t < 0.5 ? a.up : b.up };
  }
  function same(a, b) {
    return !!a && !!b && ['camp', 'summit', 'rise', 'snow', 'up'].every(function (k) { return Math.abs(a[k] - b[k]) < 0.01; })
      && ['x', 'y', 'w', 'h'].every(function (k) { return Math.abs(a.pl[k] - b.pl[k]) < 0.01; });
  }
  function current() {
    if (!sc.from) return sc.to;
    var t = (performance.now() - sc.t0) / sc.dur;
    if (t >= 1 || still()) { sc.from = null; return sc.to; }
    return between(sc.from, sc.to, ease(Math.max(0, t)));
  }
  function aim(next, dur) {
    if (!next) return;
    var now = current();
    if (!now || !dur || still()) { sc.from = null; sc.to = next; return; }
    sc.from = now; sc.to = next; sc.t0 = performance.now(); sc.dur = dur;
  }
  // The dawn shown comes on toward the dawn reached, rather than at a stroke.
  var dawnShown = -1;
  function dawnNow(dt) {
    var D = dawn();
    if (dawnShown < 0 || !dt || still()) dawnShown = D;
    else dawnShown += (D - dawnShown) * Math.min(1, dt * 1.4);
    return dawnShown;
  }
  // A new layout at rest: its scenery (reached already when the travel
  // moved toward it), and light climbing its attested trails, from the
  // valley toward the summit.
  function layout(L) {
    flows = [];
    if (REST && !same(sc.to, REST)) aim(REST, sc.to ? 600 : 0);
    if (L && L.edges) L.edges.forEach(function (e) {
      if (e.conf !== 'a') return;
      var a = e.y1 >= e.y2 ? [e.x1, e.y1, e.x2, e.y2] : [e.x2, e.y2, e.x1, e.y1];
      var n = 20, xs = [], ys = [], cum = [0];
      for (var i = 0; i <= n; i++) {
        var q = cpoint(a[0], a[1], a[2], a[3], i / n);
        xs.push(q[0]); ys.push(q[1]);
        if (i) cum.push(cum[i - 1] + Math.hypot(xs[i] - xs[i - 1], ys[i] - ys[i - 1]));
      }
      if (cum[n] < 8) return;
      flows.push({ x: xs, y: ys, cum: cum, n: n, len: cum[n], phase: Math.abs(Math.sin(e.x1 * 12.9898 + e.y2 * 78.233)) % 1 });
    });
    request();
    if (!running) draw(false);
  }
  // The sun, rising beside the summit, from behind the crest to over it.
  function sunAt(s, D) {
    var u = PHOTO.su + 0.16, v = SKY[Math.round(u * SKY.length)];
    return [s.pl.x + u * s.pl.w, s.pl.y + (v + 0.09 - 0.21 * (D - 0.12) / 0.88) * s.pl.h, 0.011 * s.pl.h];
  }
  // The still landscape, redrawn only when something it shows has changed.
  function land(v, s, D, H) {
    var key = [v.x, v.y, v.k, cw, ch, dpr, D.toFixed(3), H.toFixed(3), s.pl.x, s.pl.y, s.pl.w, s.camp, s.snow, s.up].join(',');
    if (key === landKey) return;
    landKey = key;
    var sun = sunAt(s, D);
    gl.bindFramebuffer(gl.FRAMEBUFFER, fbo);
    gl.viewport(0, 0, ground.width, ground.height);
    gl.useProgram(landP);
    gl.uniform2f(UL.uRes, ground.width, ground.height);
    gl.uniform4f(UL.uView, v.x, v.y, 1 / (v.k * dpr), 1 / (v.k * dpr));
    gl.uniform4f(UL.uPl, s.pl.x, s.pl.y, s.pl.w, s.pl.h);
    gl.uniform4f(UL.uGeo, s.camp, s.summit, s.rise, s.snow);
    gl.uniform3f(UL.uSun, sun[0], sun[1], sun[2]);
    gl.uniform1f(UL.uDawn, D); gl.uniform1f(UL.uHas, H); gl.uniform1f(UL.uUp, s.up);
    gl.activeTexture(gl.TEXTURE1); gl.bindTexture(gl.TEXTURE_2D, photoT);
    gl.activeTexture(gl.TEXTURE2); gl.bindTexture(gl.TEXTURE_2D, skyT);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
  }
  var SPEED = 26;
  function draw(moving) {
    size();
    var v = view(), s = current();
    if (!v || !s) return;
    var D = dawnNow(moving ? clock - lastClock : 0);
    lastClock = clock;
    land(v, s, D, shown());
    gl.viewport(0, 0, ground.width, ground.height);
    gl.useProgram(liveP);
    gl.uniform2f(UV.uRes, ground.width, ground.height);
    gl.uniform4f(UV.uView, v.x, v.y, 1 / (v.k * dpr), 1 / (v.k * dpr));
    gl.uniform4f(UV.uPl, s.pl.x, s.pl.y, s.pl.w, s.pl.h);
    gl.uniform4f(UV.uGeo, s.camp, s.summit, s.rise, s.snow);
    gl.uniform2f(UV.uTop, s.pl.x + PHOTO.su * s.pl.w, s.pl.y + PHOTO.sv * s.pl.h);
    gl.uniform1f(UV.uTime, clock); gl.uniform1f(UV.uDawn, D);
    gl.activeTexture(gl.TEXTURE0); gl.bindTexture(gl.TEXTURE_2D, landT);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
    go.setTransform(1, 0, 0, 1, 0, 0);
    go.clearRect(0, 0, over.width, over.height);
    if (!moving || travelling) return;
    var k = v.k * dpr, ox = -v.x * k, oy = -v.y * k;
    go.globalCompositeOperation = 'lighter';
    flows.forEach(function (fl) {
      var pos = (fl.phase * fl.len + clock * SPEED) % fl.len, j = 1;
      while (j < fl.n && fl.cum[j] < pos) j++;
      var t = (pos - fl.cum[j - 1]) / ((fl.cum[j] - fl.cum[j - 1]) || 1);
      var x = (fl.x[j - 1] + (fl.x[j] - fl.x[j - 1]) * t) * k + ox, y = (fl.y[j - 1] + (fl.y[j] - fl.y[j - 1]) * t) * k + oy;
      var fade = Math.min(1, pos / 14, (fl.len - pos) / 14), r = 6 * v.k * dpr;
      var pg = go.createRadialGradient(x, y, 0, x, y, r);
      pg.addColorStop(0, rgba(P.hot, 0.85 * fade)); pg.addColorStop(0.4, rgba(P.trail, 0.35 * fade)); pg.addColorStop(1, rgba(P.trail, 0));
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
        if (dpr > 1) { cap = 1; size(); }
        else if (FRAME < 60) FRAME = 1000 / 15;
        else { tired = true; request(); return; }
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
    else if (!goOn) { running = false; if (raf) cancelAnimationFrame(raf); raf = 0; sc.from = null; draw(false); }
  }
  if ('IntersectionObserver' in window) {
    new IntersectionObserver(function (es) { onscreen = es[0].isIntersecting; request(); }).observe(stage);
  }
  document.addEventListener('visibilitychange', request);
  if ('ResizeObserver' in window) new ResizeObserver(function () { if (size()) request(); if (!running) draw(false); }).observe(stage);
  ground.addEventListener('webglcontextlost', function (e) {
    // Back to the theme's own sky rather than a black ground.
    e.preventDefault(); running = false; stage.classList.remove('rv-alive');
    ground.remove(); over.remove();
  });
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
    travel: function () { travelling = true; aim(NEXT, 560); if (!running) draw(false); },
    view: function () { if (!running) draw(false); }
  };
  size();
})();
