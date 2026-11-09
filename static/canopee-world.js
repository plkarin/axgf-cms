/* Canopée's world (docs/adr/0043): the family's land, in 3D, one scene.
 *
 * The land, its water, its forests, meadows, screes and herds, the birds,
 * the people's lights and the sky are one WebGL 2 scene, read from
 * /canopee/world.bin — built by the server once for each shape of the
 * family and kept by the browser under its fingerprint. The page builds its
 * buffers once; after that only the camera, the light of the hour and the
 * living things move. Choosing a person (here, in the itinerary, in the
 * search) flies the camera to them, and the river travels there as it always
 * has, its record beside it.
 *
 * The water is the land's own: the server let it run down the slope the
 * family carved, so it joins and never crosses. Here it is drawn where it
 * lies — the rivers in their beds, the pools where the people stand, the
 * lake to the horizon — clear where it is shallow, dark where it is deep,
 * running with its flow, white where it meets and falls, with the sky in it.
 * The light is the reader's hour: dawn, day, dusk, a night under the moon.
 *
 * Decoration over the river's own data: without WebGL 2, or if the world
 * cannot be read, the river and its photograph stay as they were
 * (river-scene.js). prefers-reduced-motion, "Cut" and the river's pause
 * button hold the water, the wind and the living things still and the
 * camera jumps instead of flying; nothing is drawn while the river is off
 * screen or the tab is hidden. The page lowers its own detail when frames
 * come slowly. `?world=fps` shows how fast they come; `h<hour>` holds the
 * light at that hour and `q<0–3>` the detail at that level (`?world=fps,q1`).
 */
(function () {
  'use strict';
  var root = document.querySelector('.river[data-world]');
  var stage = document.getElementById('rv-canvas');
  if (!root || !stage) return;
  var canvas = document.createElement('canvas');
  var gl = canvas.getContext('webgl2', { antialias: true, alpha: false, powerPreference: 'high-performance' });
  if (!gl) return;

  var asked = ((/[?&]world=([^&#]*)/.exec(location.search) || [])[1] || '').split(',');
  var fixedHour = null, fixedTier = null;
  asked.forEach(function (a) {
    var m = /^h(\d+(?:\.\d+)?)$/.exec(a);
    if (m) fixedHour = Math.min(24, parseFloat(m[1]));
    m = /^q([0-3])$/.exec(a);
    if (m) fixedTier = parseInt(m[1], 10);
  });

  // The photograph's scene stands down while the world is read; it comes
  // back if the world cannot be.
  root.classList.add('rv-world-on');
  canvas.className = 'rv-world';
  canvas.setAttribute('role', 'img');
  canvas.setAttribute('aria-label', root.getAttribute('data-world-label') || '');
  canvas.tabIndex = 0;
  var labels = document.createElement('div');
  labels.className = 'rv-world-labels';
  labels.setAttribute('aria-hidden', 'true');
  stage.insertBefore(labels, stage.firstChild);
  stage.insertBefore(canvas, stage.firstChild);
  var meter = null;
  if (asked.indexOf('fps') >= 0) {
    meter = document.createElement('div');
    meter.className = 'rv-world-meter';
    meter.setAttribute('aria-hidden', 'true');
    stage.appendChild(meter);
  }

  var reduce = window.matchMedia && matchMedia('(prefers-reduced-motion: reduce)').matches;
  var cut = root.dataset.riverMotion === 'cut';
  var STORE = 'axgf-river-still';
  var paused = false;
  try { paused = localStorage.getItem(STORE) === '1'; } catch (e) { /* private window */ }

  function fallBack() {
    root.classList.remove('rv-world-on', 'rv-world-ready');
    canvas.remove();
    labels.remove();
    if (meter) meter.remove();
    var src = root.getAttribute('data-world-fallback');
    if (src) {
      var s = document.createElement('script');
      s.src = src;
      document.body.appendChild(s);
    }
  }

  /* ---- the world, from its bytes (src/world.rs, "AXW2") ---- */
  function parse(buf) {
    var dv = new DataView(buf), o = 0;
    function u8() { return dv.getUint8(o++); }
    function u16() { var v = dv.getUint16(o, true); o += 2; return v; }
    function u32() { var v = dv.getUint32(o, true); o += 4; return v; }
    function f32() { var v = dv.getFloat32(o, true); o += 4; return v; }
    function floats(count) {
      var a = new Float32Array(count);
      for (var q = 0; q < count; q++) a[q] = f32();
      return a;
    }
    if (String.fromCharCode(u8(), u8(), u8(), u8()) !== 'AXW2') throw new Error('not a world');
    var w = { n: u32(), x0: f32(), z0: f32(), sx: f32(), sz: f32(), water: f32() };
    var lo = f32(), hi = f32(), n = w.n, i, k;
    w.lo = lo;
    w.hi = hi;
    w.h = new Float32Array(n * n);
    for (i = 0; i < n * n; i++) w.h[i] = lo + (hi - lo) * u16() / 65535;
    w.forest = new Uint8Array(buf, o, n * n).slice();
    o += n * n;
    var np = u32();
    w.persons = [];
    w.byId = {};
    for (i = 0; i < np; i++) {
      var len = u8(), id = '';
      for (k = 0; k < len; k++) id += String.fromCharCode(u8());
      var p = { id: id, x: f32(), z: f32(), y: f32(), cls: u8(), gen: u16(), index: i };
      w.persons.push(p);
      w.byId[id] = p;
    }
    w.rivers = [];
    for (i = u32(); i > 0; i--) w.rivers.push(floats(u32() * 5));
    w.ponds = [];
    for (i = u32(); i > 0; i--) {
      var pond = { x: f32(), z: f32(), level: f32(), radius: f32(), spans: [] };
      for (k = u32(); k > 0; k--) pond.spans.push([u16(), u16(), u16()]);
      w.ponds.push(pond);
    }
    w.paths = [];
    for (i = u32(); i > 0; i--) w.paths.push(floats(u32() * 3));
    w.trees = floats(u32() * 6);
    w.rocks = floats(u32() * 4);
    w.herds = floats(u32() * 4);
    return w;
  }

  /* ---- a little linear algebra ---- */
  function perspective(fovy, aspect, near, far) {
    var f = 1 / Math.tan(fovy / 2), nf = 1 / (near - far);
    return [f / aspect, 0, 0, 0, 0, f, 0, 0, 0, 0, (far + near) * nf, -1, 0, 0, 2 * far * near * nf, 0];
  }
  function ortho(l, r, b, t, n, f) {
    return [2 / (r - l), 0, 0, 0, 0, 2 / (t - b), 0, 0, 0, 0, -2 / (f - n), 0,
      -(r + l) / (r - l), -(t + b) / (t - b), -(f + n) / (f - n), 1];
  }
  function lookAt(e, c, up) {
    var zx = e[0] - c[0], zy = e[1] - c[1], zz = e[2] - c[2];
    var l = Math.hypot(zx, zy, zz) || 1;
    zx /= l; zy /= l; zz /= l;
    var xx = up[1] * zz - up[2] * zy, xy = up[2] * zx - up[0] * zz, xz = up[0] * zy - up[1] * zx;
    l = Math.hypot(xx, xy, xz) || 1;
    xx /= l; xy /= l; xz /= l;
    var yx = zy * xz - zz * xy, yy = zz * xx - zx * xz, yz = zx * xy - zy * xx;
    return [xx, yx, zx, 0, xy, yy, zy, 0, xz, yz, zz, 0,
      -(xx * e[0] + xy * e[1] + xz * e[2]), -(yx * e[0] + yy * e[1] + yz * e[2]), -(zx * e[0] + zy * e[1] + zz * e[2]), 1];
  }
  function mul(a, b) {
    var o = new Array(16);
    for (var c = 0; c < 4; c++) {
      for (var r = 0; r < 4; r++) {
        o[c * 4 + r] = a[r] * b[c * 4] + a[4 + r] * b[c * 4 + 1] + a[8 + r] * b[c * 4 + 2] + a[12 + r] * b[c * 4 + 3];
      }
    }
    return o;
  }
  function invert(m) {
    var a00 = m[0], a01 = m[1], a02 = m[2], a03 = m[3], a10 = m[4], a11 = m[5], a12 = m[6], a13 = m[7];
    var a20 = m[8], a21 = m[9], a22 = m[10], a23 = m[11], a30 = m[12], a31 = m[13], a32 = m[14], a33 = m[15];
    var b00 = a00 * a11 - a01 * a10, b01 = a00 * a12 - a02 * a10, b02 = a00 * a13 - a03 * a10, b03 = a01 * a12 - a02 * a11;
    var b04 = a01 * a13 - a03 * a11, b05 = a02 * a13 - a03 * a12, b06 = a20 * a31 - a21 * a30, b07 = a20 * a32 - a22 * a30;
    var b08 = a20 * a33 - a23 * a30, b09 = a21 * a32 - a22 * a31, b10 = a21 * a33 - a23 * a31, b11 = a22 * a33 - a23 * a32;
    var det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
    det = det ? 1 / det : 0;
    return [(a11 * b11 - a12 * b10 + a13 * b09) * det, (a02 * b10 - a01 * b11 - a03 * b09) * det,
      (a31 * b05 - a32 * b04 + a33 * b03) * det, (a22 * b04 - a21 * b05 - a23 * b03) * det,
      (a12 * b08 - a10 * b11 - a13 * b07) * det, (a00 * b11 - a02 * b08 + a03 * b07) * det,
      (a32 * b02 - a30 * b05 - a33 * b01) * det, (a20 * b05 - a22 * b02 + a23 * b01) * det,
      (a10 * b10 - a11 * b08 + a13 * b06) * det, (a01 * b08 - a00 * b10 - a03 * b06) * det,
      (a30 * b04 - a31 * b02 + a33 * b00) * det, (a21 * b02 - a20 * b04 - a23 * b00) * det,
      (a11 * b07 - a10 * b09 - a12 * b06) * det, (a00 * b09 - a01 * b07 + a02 * b06) * det,
      (a31 * b01 - a30 * b03 - a32 * b00) * det, (a20 * b03 - a21 * b01 + a22 * b00) * det];
  }
  // The six planes of what a matrix sees, to leave out what it does not.
  function planesOf(m) {
    var out = [];
    [[0, 1], [0, -1], [1, 1], [1, -1], [2, 1], [2, -1]].forEach(function (q) {
      var r = q[0], s = q[1];
      out.push([m[3] + s * m[r], m[7] + s * m[4 + r], m[11] + s * m[8 + r], m[15] + s * m[12 + r]]);
    });
    return out;
  }
  function seen(planes, b) {
    for (var i = 0; i < 6; i++) {
      var p = planes[i];
      if (p[0] * (p[0] > 0 ? b.max[0] : b.min[0]) + p[1] * (p[1] > 0 ? b.max[1] : b.min[1]) +
        p[2] * (p[2] > 0 ? b.max[2] : b.min[2]) + p[3] < 0) return false;
    }
    return true;
  }
  function boxDistance(e, b) {
    return Math.hypot(Math.max(b.min[0] - e[0], 0, e[0] - b.max[0]), Math.max(b.min[1] - e[1], 0, e[1] - b.max[1]),
      Math.max(b.min[2] - e[2], 0, e[2] - b.max[2]));
  }
  function mix(a, b, t) { return a.map(function (v, i) { return v + (b[i] - v) * t; }); }
  function smooth(e0, e1, x) { var t = Math.max(0, Math.min(1, (x - e0) / (e1 - e0))); return t * t * (3 - 2 * t); }
  function norm(v) { var l = Math.hypot(v[0], v[1], v[2]) || 1; return [v[0] / l, v[1] / l, v[2] / l]; }
  function hash2(x, z) { var h = Math.sin(x * 127.1 + z * 311.7) * 43758.5453; return h - Math.floor(h); }
  function noise2(x, z) {
    var xi = Math.floor(x), zi = Math.floor(z), fx = x - xi, fz = z - zi;
    fx = fx * fx * (3 - 2 * fx);
    fz = fz * fz * (3 - 2 * fz);
    var a = hash2(xi, zi), b = hash2(xi + 1, zi), c = hash2(xi, zi + 1), d = hash2(xi + 1, zi + 1);
    return a + (b - a) * fx + (c - a) * fz + (a - b - c + d) * fx * fz;
  }
  function ridges(x, z) {
    var s = 0, a = 0.5, f = 1;
    for (var o = 0; o < 4; o++) {
      var v = 1 - Math.abs(noise2(x * f, z * f) * 2 - 1);
      s += v * v * a;
      a *= 0.5;
      f *= 2.03;
    }
    return s;
  }

  /* ---- the light of the hour ----
   * The reader's own clock: the sun rises at half past six on one side of
   * the valley, crosses it, sets at half past seven on the other; the hours
   * around are gold, the night is blue under the moon and the stars. */
  var LIGHT = {};
  function hourLight(date) {
    var h = fixedHour !== null ? fixedHour : date.getHours() + date.getMinutes() / 60;
    var t = (h - 6.5) / 13;
    var elev = Math.sin(Math.PI * Math.max(-0.15, Math.min(1.15, t)));
    var az = Math.PI * Math.max(-0.1, Math.min(1.1, t));
    var sun = norm([Math.cos(az), elev * 0.92, 0.28 - Math.sin(az) * 0.55]);
    var e = Math.asin(Math.max(-1, Math.min(1, sun[1]))) * 57.3;
    var night = 1 - smooth(-10, -1, e), gold = 1 - smooth(3, 26, e);
    var day = {
      sunCol: [1.0, 0.95, 0.86], sky: [0.40, 0.48, 0.60], ground: [0.22, 0.20, 0.15],
      zenith: [0.26, 0.47, 0.78], horizon: [0.76, 0.83, 0.89], fog: [0.74, 0.81, 0.87], exposure: [1.0]
    };
    var dusk = {
      sunCol: [1.0, 0.66, 0.38], sky: [0.40, 0.36, 0.44], ground: [0.22, 0.16, 0.11],
      zenith: [0.27, 0.36, 0.62], horizon: [0.98, 0.70, 0.48], fog: [0.88, 0.70, 0.56], exposure: [1.15]
    };
    var dark = {
      sunCol: [0.36, 0.45, 0.70], sky: [0.09, 0.12, 0.21], ground: [0.03, 0.035, 0.05],
      zenith: [0.02, 0.035, 0.085], horizon: [0.08, 0.11, 0.19], fog: [0.07, 0.09, 0.15], exposure: [1.3]
    };
    var L = {};
    Object.keys(day).forEach(function (k) { L[k] = mix(mix(day[k], dusk[k], gold), dark[k], night); });
    L.exposure = L.exposure[0];
    L.sun = sun;
    // At night the light comes from the moon, high over the other side; by
    // day never so low that the mountains' shadows drown the valley.
    var from = night > 0.5 ? [-sun[0] * 0.5, 0.8, 0.35] : [sun[0], Math.max(sun[1], 0.22), sun[2]];
    L.dir = norm(from);
    L.night = night;
    L.strength = night > 0.5 ? 0.5 : 0.35 + 0.65 * smooth(-2, 12, e);
    return L;
  }

  /* ---- shaders ---- */
  var HEAD = '#version 300 es\nprecision highp float;\nprecision highp sampler2DShadow;\n';
  var NOISE = [
    'float hash(vec2 p) { vec3 q = fract(vec3(p.xyx) * 0.1031); q += dot(q, q.yzx + 33.33); return fract((q.x + q.y) * q.z); }',
    'float vnoise(vec2 p) { vec2 i = floor(p), f = fract(p); f = f * f * (3.0 - 2.0 * f);',
    '  return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), f.x), mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), f.x), f.y); }',
    'float fbm(vec2 p) { float s = 0.0, a = 0.5; for (int i = 0; i < 4; i++) { s += a * vnoise(p); p = p * 2.07 + 17.1; a *= 0.5; } return s; }',
    ''
  ].join('\n');
  // Light as it is: colours in linear light, the sun and the sky's light,
  // the sun's shadows, a filmic curve back to the screen, then the haze.
  var LIT = [
    'uniform vec3 uDir; uniform vec3 uSunCol; uniform vec3 uSky; uniform vec3 uGround; uniform vec3 uFog; uniform vec3 uCam;',
    'uniform vec2 uFogRange; uniform float uStrength; uniform float uExposure;',
    'uniform sampler2DShadow uShadow; uniform mat4 uShadowVP; uniform float uShadowOn;',
    'float shade(vec3 p, vec3 n) {',
    '  if (uShadowOn < 0.5) return 1.0;',
    '  vec4 s = uShadowVP * vec4(p + n * 0.3, 1.0); vec3 c = s.xyz / s.w * 0.5 + 0.5;',
    '  if (c.x <= 0.0 || c.x >= 1.0 || c.y <= 0.0 || c.y >= 1.0 || c.z >= 1.0) return 1.0;',
    '  vec2 tx = 1.0 / vec2(textureSize(uShadow, 0)); float lit = 0.0;',
    '  for (int x = -1; x <= 1; x++) for (int y = -1; y <= 1; y++) lit += texture(uShadow, vec3(c.xy + vec2(float(x), float(y)) * tx * 1.3, c.z - 0.0004));',
    '  vec2 e = min(c.xy, 1.0 - c.xy);',
    '  return mix(1.0, lit / 9.0, smoothstep(0.0, 0.06, min(e.x, e.y))); }',
    'vec3 lin(vec3 c) { return c * c; }',
    'vec3 lit(vec3 albedo, vec3 n, float sh, float ao) {',
    '  float d = max(dot(n, uDir), 0.0);',
    '  vec3 amb = mix(uGround, uSky, n.y * 0.5 + 0.5);',
    '  return lin(albedo) * (uSunCol * d * sh * 1.6 * uStrength + amb * ao * 0.9); }',
    'vec3 finish(vec3 c) { c *= uExposure; c = (c * (2.51 * c + 0.03)) / (c * (2.43 * c + 0.59) + 0.14); return pow(clamp(c, 0.0, 1.0), vec3(1.0 / 2.2)); }',
    'vec3 fogged(vec3 col, vec3 p) {',
    '  vec3 v = normalize(p - uCam); float f = smoothstep(uFogRange.x, uFogRange.y, distance(uCam, p));',
    '  vec3 tint = mix(uFog, min(uSunCol * 0.95 + uFog * 0.2, vec3(1.0)), pow(max(dot(v, uDir), 0.0), 8.0) * 0.45);',
    '  return mix(col, tint, f); }',
    ''
  ].join('\n');

  var SRC = {
    sky: {
      vs: HEAD + 'out vec2 vNdc; void main() { vec2 p = vec2(float((gl_VertexID << 1) & 2), float(gl_VertexID & 2)) * 2.0 - 1.0; vNdc = p; gl_Position = vec4(p, 0.9999, 1.0); }',
      fs: HEAD + NOISE + [
        'in vec2 vNdc; uniform mat4 uInv; uniform vec3 uCam; uniform vec3 uSun; uniform vec3 uDir; uniform vec3 uZenith; uniform vec3 uHorizon;',
        'uniform vec3 uSunCol; uniform vec3 uFog; uniform float uNight; uniform float uTime; out vec4 o;',
        'void main() {',
        '  vec4 q = uInv * vec4(vNdc, 1.0, 1.0); vec3 d = normalize(q.xyz / q.w - uCam);',
        '  vec3 c = mix(uHorizon, uZenith, smoothstep(-0.02, 0.6, d.y));',
        '  c = mix(c, uFog, 1.0 - smoothstep(-0.06, 0.06, d.y));',
        '  float s = max(dot(d, uSun), 0.0);',
        '  c += uSunCol * (pow(s, 1400.0) * 5.0 + pow(s, 14.0) * 0.22 + pow(s, 3.0) * 0.06) * (1.0 - uNight);',
        '  if (d.y > 0.0) {',
        '    vec2 cp = d.xz / (d.y + 0.12) * 1.6 + vec2(uTime * 0.003, uTime * 0.0012);',
        '    float cl = smoothstep(0.5, 0.82, fbm(cp) * 0.75 + fbm(cp * 3.1) * 0.35);',
        '    vec3 cloud = mix(vec3(1.0, 0.99, 0.97), uSunCol, 0.35) * (1.0 - uNight * 0.86);',
        '    cloud *= 0.82 + 0.18 * fbm(cp * 2.0 + 4.0);',
        '    c = mix(c, cloud, cl * smoothstep(0.0, 0.25, d.y) * 0.7);',
        '    vec2 sp = floor(d.xz / (d.y + 0.35) * 300.0); float st = step(0.9972, hash(sp)) * smoothstep(0.06, 0.35, d.y);',
        '    c += vec3(0.82, 0.88, 1.0) * st * uNight * (0.55 + 0.45 * sin(uTime * 1.7 + hash(sp + 3.0) * 40.0)) * (1.0 - cl);',
        '    float m = max(dot(d, uDir), 0.0); c += vec3(0.9, 0.92, 0.98) * (step(0.99965, m) * 1.4 + pow(m, 60.0) * 0.12) * uNight;',
        '  }',
        '  o = vec4(c, 1.0); }'
      ].join('\n')
    },
    ground: {
      vs: HEAD + 'in vec3 aPos; in vec3 aNor; in vec4 aMix; uniform mat4 uVP; out vec3 vPos; out vec3 vNor; out vec4 vMix;\n' +
        'void main() { vPos = aPos; vNor = aNor; vMix = aMix; gl_Position = uVP * vec4(aPos, 1.0); }',
      // aMix: the forest's thickness, the path, the water near, the shelter.
      fs: HEAD + NOISE + LIT + [
        'in vec3 vPos; in vec3 vNor; in vec4 vMix; uniform float uWater; uniform float uSnow;',
        'uniform sampler2D uLevel; uniform vec2 uOrigin; uniform vec2 uSize; uniform float uN; out vec4 o;',
        'void main() {',
        '  vec3 n0 = normalize(vNor); vec3 n = n0; vec2 p = vPos.xz; float dist = distance(uCam, vPos);',
        '  if (dist < 220.0) {',
        '    float e = 0.3, h0 = fbm(p * 0.8), hx = fbm((p + vec2(e, 0.0)) * 0.8), hz = fbm((p + vec2(0.0, e)) * 0.8);',
        '    n = normalize(n0 + vec3(h0 - hx, 0.0, h0 - hz) * 1.5 * (1.0 - smoothstep(90.0, 220.0, dist)));',
        '  }',
        '  float slope = 1.0 - n0.y;',
        '  float g1 = fbm(p * 0.035), g2 = vnoise(p * 0.3), g3 = vnoise(p * 1.9);',
        '  vec3 meadow = mix(vec3(0.33, 0.40, 0.19), vec3(0.49, 0.50, 0.28), smoothstep(0.3, 0.72, g1));',
        '  meadow = mix(meadow, vec3(0.60, 0.56, 0.36), smoothstep(0.64, 0.92, g2) * 0.4);',
        '  meadow *= 0.88 + g3 * 0.22;',
        '  vec2 fc = floor(p * 2.5); vec2 ff = fract(p * 2.5) - 0.5;',
        '  float fl = step(0.986, hash(fc)) * smoothstep(0.32, 0.12, length(ff)) * (1.0 - smoothstep(0.2, 0.5, vMix.x)) * (1.0 - smoothstep(0.12, 0.3, slope)) * (1.0 - vMix.y);',
        '  meadow = mix(meadow, mix(vec3(0.96, 0.90, 0.56), vec3(0.86, 0.55, 0.78), hash(fc + 7.0)), fl);',
        '  vec3 litter = mix(vec3(0.12, 0.14, 0.07), vec3(0.25, 0.20, 0.12), g2) * (0.85 + g3 * 0.3);',
        '  vec3 c = mix(meadow, litter, smoothstep(0.2, 0.75, vMix.x));',
        '  vec3 scree = mix(vec3(0.45, 0.42, 0.37), vec3(0.64, 0.60, 0.52), g3) * (0.78 + step(0.62, vnoise(p * 3.1)) * 0.3);',
        '  vec3 rock = mix(vec3(0.37, 0.36, 0.34), vec3(0.58, 0.56, 0.52), vnoise(vec2(p.x * 0.07 + p.y * 0.05, vPos.y * 0.5)));',
        '  c = mix(c, scree, smoothstep(0.27, 0.42, slope + (g2 - 0.5) * 0.14));',
        '  c = mix(c, rock, smoothstep(0.5, 0.68, slope + (g3 - 0.5) * 0.08));',
        '  vec3 dirt = mix(vec3(0.46, 0.38, 0.27), vec3(0.58, 0.50, 0.37), g3);',
        '  c = mix(c, dirt, smoothstep(0.3, 0.8, vMix.y + (g3 - 0.5) * 0.35));',
        '  float sub = texture(uLevel, ((p - uOrigin) / uSize * (uN - 1.0) + 0.5) / uN).r - vPos.y, near = smoothstep(0.02, 0.2, vMix.z);',
        '  c = mix(c, c * vec3(0.66, 0.68, 0.60), near * (1.0 - smoothstep(0.0, 0.8, -sub + (g3 - 0.5) * 0.2)) * 0.85);',
        '  vec3 bed = mix(vec3(0.33, 0.31, 0.26), vec3(0.55, 0.51, 0.43), smoothstep(0.3, 0.7, vnoise(p * 4.3) * 0.6 + vnoise(p * 9.7) * 0.4)) * (0.82 + g3 * 0.22);',
        '  c = mix(c, bed, near * smoothstep(-0.03, 0.12, sub));',
        '  float shore = 1.0 - smoothstep(uWater + 0.1, uWater + 1.6, vPos.y + g3 * 0.4);',
        '  c = mix(c, vec3(0.72, 0.66, 0.52) * (0.85 + g3 * 0.2), shore);',
        '  c = mix(c, vec3(0.32, 0.33, 0.27), 1.0 - smoothstep(uWater - 2.5, uWater - 0.2, vPos.y));',
        '  float snow = smoothstep(uSnow, uSnow + 18.0, vPos.y + (g1 - 0.5) * 22.0) * (1.0 - smoothstep(0.5, 0.8, slope));',
        '  c = mix(c, vec3(0.93, 0.95, 0.98), snow);',
        '  vec3 col = lit(c, n, shade(vPos, n0), vMix.w * (1.0 - vMix.x * 0.3));',
        '  o = vec4(fogged(finish(col), vPos), 1.0); }'
      ].join('\n')
    },
    water: {
      vs: HEAD + 'in vec3 aPos; in vec4 aFlow; uniform mat4 uVP; out vec3 vPos; out vec4 vFlow;\n' +
        'void main() { vPos = aPos; vFlow = aFlow; gl_Position = uVP * vec4(aPos, 1.0); }',
      // aFlow: the flow along x and z, the foam.
      fs: HEAD + NOISE + LIT + [
        'in vec3 vPos; in vec4 vFlow; uniform float uTime; uniform vec3 uZenith; uniform vec3 uHorizon; uniform float uNight;',
        'uniform sampler2D uHeight; uniform vec2 uOrigin; uniform vec2 uSize; uniform float uN; out vec4 o;',
        'vec2 slope2(vec2 uv) { float a = vnoise(uv), b = vnoise(uv + vec2(0.07, 0.0)), c = vnoise(uv + vec2(0.0, 0.07)); return vec2(a - b, a - c) / 0.07; }',
        'vec2 waves(vec2 uv) { return slope2(uv) * 0.5 + slope2(uv * 2.3 + 5.1) * 0.3 + slope2(uv * 5.3 + 9.7) * 0.16; }',
        'void main() {',
        '  vec2 uvh = ((vPos.xz - uOrigin) / uSize * (uN - 1.0) + 0.5) / uN;',
        '  float depth = vPos.y - texture(uHeight, uvh).r;',
        '  if (depth < -0.02) discard;',
        '  vec2 flow = vFlow.xy; float speed = length(flow); float dist = distance(uCam, vPos);',
        '  float t = uTime * 0.22, p0 = fract(t), p1 = fract(t + 0.5), w = abs(1.0 - 2.0 * p0);',
        '  vec2 uv = vPos.xz * 0.42, drift = vec2(uTime * 0.035, uTime * 0.024);',
        '  vec2 g = mix(waves(uv - flow * p0 * 2.2 + drift), waves(uv - flow * p1 * 2.2 + drift + 3.7), w);',
        '  float calm = (0.55 + min(speed, 1.5) * 0.3) * (1.0 - smoothstep(120.0, 700.0, dist));',
        '  vec3 n = normalize(vec3(-g.x * 0.16 * calm, 1.0, -g.y * 0.16 * calm));',
        '  vec3 v = normalize(uCam - vPos);',
        '  float fr = 0.02 + 0.98 * pow(1.0 - max(dot(n, v), 0.0), 5.0);',
        '  vec3 r = reflect(-v, n);',
        '  vec3 skyc = lin(mix(uHorizon, uZenith, smoothstep(-0.02, 0.6, r.y)));',
        '  vec3 body = mix(lin(vec3(0.30, 0.35, 0.24)), lin(vec3(0.035, 0.12, 0.13)), smoothstep(0.0, 2.0, depth));',
        '  body *= uSky * 1.3 + uSunCol * 0.55 * uStrength;',
        '  vec3 c = mix(body, skyc * 1.15, fr);',
        '  float sh = shade(vPos, vec3(0.0, 1.0, 0.0));',
        '  float hn = max(dot(n, normalize(uDir + v)), 0.0);',
        '  c += uSunCol * (pow(hn, 500.0) * 9.0 + pow(hn, 70.0) * 0.3) * sh * uStrength;',
        '  vec2 fuv = vPos.xz * 0.9 - flow * uTime * 0.7;',
        '  float streak = smoothstep(0.55, 0.85, fbm(fuv) * 0.7 + vnoise(fuv * 3.0) * 0.3);',
        '  float edge = (1.0 - smoothstep(0.02, 0.12, depth)) * min(1.0, speed * 1.2);',
        '  float foam = clamp(vFlow.z * (0.3 + streak * 0.8) + edge * (0.15 + streak * 0.5) + streak * min(speed, 2.0) * 0.05, 0.0, 1.0);',
        '  c = mix(c, lin(vec3(0.93, 0.95, 0.93)) * (uSky * 1.2 + uSunCol * uStrength * (0.4 + 0.6 * sh)), foam * 0.8);',
        '  float a = clamp(0.5 + smoothstep(0.0, 1.2, depth) * 0.45 + fr * 0.2 + foam * 0.4, 0.0, 0.97) * smoothstep(-0.02, 0.05, depth);',
        '  o = vec4(fogged(finish(c), vPos), a); }'
      ].join('\n')
    },
    solid: {
      vs: HEAD + [
        'in vec3 aPos; in vec3 aNor; in vec3 aCol; in vec4 aInst; in vec2 aExtra;',
        'uniform mat4 uVP; uniform float uTime; uniform float uMode; uniform vec3 uCam; uniform vec2 uAt; out vec3 vPos; out vec3 vNor; out vec3 vCol; out float vLeaf;',
        'float turn(vec2 p) { vec3 q = fract(vec3(p.xyx) * 0.1031); q += dot(q, q.yzx + 33.33); return fract((q.x + q.y) * q.z) * 6.2831853; }',
        'void main() {',
        '  float s = aInst.w; vec3 p = aPos * s; vec3 nn = aNor; vec3 base = aInst.xyz; float yaw = turn(aInst.xz);',
        '  vec3 col = aCol; vLeaf = 0.0;',
        '  if (uMode > 1.5) {',
        // The herds wander a little and graze.
        '    float ph = aExtra.y, t = uTime * 0.05 + ph;',
        '    base.xz += vec2(cos(t), sin(t * 1.3)) * (1.0 + fract(ph * 0.37) * 1.6);',
        '    vec2 dir = vec2(-sin(t), 1.3 * cos(t * 1.3)); yaw = atan(dir.x, dir.y);',
        '    float graze = smoothstep(0.2, 0.9, sin(uTime * 0.35 + ph * 3.0));',
        '    if (aPos.z > 0.42 && aPos.y > 0.7) p.y -= graze * 0.5 * s * (aPos.z - 0.42) * 2.0;',
        '  }',
        '  float c = cos(yaw), si = sin(yaw);',
        '  p = vec3(c * p.x + si * p.z, p.y, -si * p.x + c * p.z); nn = vec3(c * nn.x + si * nn.z, nn.y, -si * nn.x + c * nn.z);',
        '  if (uMode > 0.5 && uMode < 1.5) {',
        // A tree between the camera and the person it looks at steps aside.
        '    vec2 ab = uAt - uCam.xz; float t = clamp(dot(base.xz - uCam.xz, ab) / max(dot(ab, ab), 1.0), 0.0, 1.0);',
        '    p *= smoothstep(3.0, 8.0, distance(base.xz, uCam.xz + ab * t) + (1.0 - step(t, 0.93)) * 99.0);',
        // The trees bend in the wind, their tops most, in gusts.
        '    float sway = max(aPos.y - 1.0, 0.0) * s, gust = 0.55 + 0.45 * sin(uTime * 0.31 + base.x * 0.011 + base.z * 0.007);',
        '    p.x += sin(uTime * 1.3 + base.x * 0.07 + base.z * 0.05) * 0.035 * sway * gust;',
        '    p.z += cos(uTime * 1.05 + base.z * 0.06) * 0.022 * sway * gust;',
        '    if (aCol.g > aCol.r * 1.1) { vLeaf = 1.0; col = mix(col, col * vec3(1.3, 1.12, 0.7), aExtra.y * 0.7); col *= 0.82 + aExtra.y * 0.36; }',
        '  }',
        '  vPos = base + p; vNor = nn; vCol = col; gl_Position = uVP * vec4(vPos, 1.0); }'
      ].join('\n'),
      fs: HEAD + NOISE + LIT + [
        'in vec3 vPos; in vec3 vNor; in vec3 vCol; in float vLeaf; out vec4 o;',
        'void main() {',
        '  vec3 n = normalize(vNor); float g = vnoise(vPos.xz * 1.7 + vPos.y * 2.3) * 0.18 + 0.91;',
        '  if (vLeaf > 0.5) { vec2 q = vPos.xz * 2.4 + vec2(vPos.y * 1.9, -vPos.y * 1.3);',
        '    n = normalize(n + (vec3(vnoise(q), vnoise(q + 5.2), vnoise(q + 9.1)) - 0.5) * 1.1); g *= 0.72 + 0.4 * vnoise(q * 1.8 + 3.3); }',
        '  float sh = shade(vPos, n);',
        '  vec3 col = lit(vCol * g, n, sh, 0.62 + 0.38 * smoothstep(-0.4, 0.7, n.y));',
        // The sun through the leaves, when it is behind them.
        '  vec3 v = normalize(vPos - uCam);',
        '  col += lin(vCol) * uSunCol * pow(max(dot(v, uDir), 0.0), 5.0) * 0.5 * vLeaf * sh * uStrength;',
        '  o = vec4(fogged(finish(col), vPos), 1.0); }'
      ].join('\n')
    },
    grass: {
      vs: HEAD + NOISE + [
        'in vec3 aPos; in vec2 aUV; uniform mat4 uVP; uniform float uTime; uniform vec3 uCam; uniform vec2 uAt; uniform float uSide; uniform float uStep;',
        'uniform float uWater; uniform sampler2D uHeight; uniform sampler2D uCover; uniform vec2 uOrigin; uniform vec2 uSize; uniform float uN;',
        'out vec3 vPos; out vec2 vUV; out float vTint; out float vDry;',
        'vec2 texel(vec2 xz) { return ((xz - uOrigin) / uSize * (uN - 1.0) + 0.5) / uN; }',
        'void main() {',
        '  float id = float(gl_InstanceID); vec2 cell = vec2(mod(id, uSide), floor(id / uSide)) - uSide * 0.5;',
        '  vec2 base = (floor(uAt / uStep) + cell) * uStep; float r = hash(base);',
        '  vec2 xz = base + (vec2(r, hash(base + 7.31)) - 0.5) * uStep * 0.96; vec2 uv = texel(xz);',
        '  float y = texture(uHeight, uv).r, e = 1.0 / uN;',
        '  vec2 g = vec2(texture(uHeight, uv + vec2(e, 0.0)).r - texture(uHeight, uv - vec2(e, 0.0)).r,',
        '                texture(uHeight, uv + vec2(0.0, e)).r - texture(uHeight, uv - vec2(0.0, e)).r) * (uN - 1.0) / (2.0 * uSize);',
        '  vec4 cover = texture(uCover, uv);',
        '  float k = (1.0 - smoothstep(0.75, 1.0, max(abs(cell.x), abs(cell.y)) / (uSide * 0.5))) * (1.0 - smoothstep(70.0, 130.0, distance(uCam.xz, xz)));',
        '  k *= (1.0 - smoothstep(0.32, 0.55, length(g))) * (1.0 - smoothstep(0.3, 0.6, cover.r)) * (1.0 - smoothstep(0.15, 0.45, cover.g));',
        '  k *= (1.0 - smoothstep(0.1, 0.45, cover.b)) * step(uWater + 0.5, y) * step(0.0, uv.x) * step(uv.x, 1.0) * step(0.0, uv.y) * step(uv.y, 1.0);',
        '  float s = (0.5 + hash(base + 3.7) * 0.75) * k;',
        '  float yaw = r * 43.98, c = cos(yaw), si = sin(yaw);',
        '  vec3 p = aPos * s; p = vec3(c * p.x + si * p.z, p.y, -si * p.x + c * p.z);',
        '  float wind = sin(uTime * 1.7 + xz.x * 0.35 + xz.y * 0.25) * 0.5 + sin(uTime * 2.9 + xz.x * 0.8) * 0.2;',
        '  p.x += wind * 0.14 * aUV.y * s; p.z += wind * 0.06 * aUV.y * s;',
        '  vPos = vec3(xz.x, y - 0.04, xz.y) + p; vUV = aUV; vTint = fract(r * 13.7);',
        '  vDry = smoothstep(0.3, 0.72, fbm(xz * 0.035));',
        '  gl_Position = uVP * vec4(vPos, 1.0); }'
      ].join('\n'),
      fs: HEAD + NOISE + LIT + [
        'in vec3 vPos; in vec2 vUV; in float vTint; in float vDry; out vec4 o;',
        'void main() { float x = vUV.x * 5.0; float blade = abs(fract(x) - 0.5) * 2.0; float hgt = 0.55 + 0.45 * hash(vec2(floor(x), vTint * 91.0));',
        '  if (blade > 1.0 - vUV.y / hgt * 0.95 || vUV.y > hgt) discard;',
        '  vec3 tip = mix(mix(vec3(0.31, 0.42, 0.17), vec3(0.46, 0.49, 0.26), vDry), vec3(0.56, 0.55, 0.33), vTint * 0.3);',
        '  vec3 col = mix(mix(vec3(0.16, 0.24, 0.09), vec3(0.26, 0.29, 0.15), vDry), tip, vUV.y / hgt);',
        '  if (vTint > 0.95 && vUV.y > hgt * 0.82) col = mix(vec3(0.95, 0.86, 0.40), vec3(0.84, 0.50, 0.72), fract(vTint * 37.0));',
        '  vec3 up = vec3(0.0, 1.0, 0.0);',
        '  o = vec4(fogged(finish(lit(col, up, shade(vPos, up), 0.6 + vUV.y * 0.4)), vPos), 1.0); }'
      ].join('\n')
    },
    bird: {
      vs: HEAD + [
        // aBird: phase, radius of its round, height, size (an eagle when large).
        'in vec3 aPos; in vec4 aBird; in vec3 aHome; uniform mat4 uVP; uniform float uTime; uniform vec3 uCentre; out vec3 vPos;',
        'void main() { float eagle = step(2.0, aBird.w); float t = uTime * mix(0.11, 0.045, eagle) + aBird.x;',
        '  vec3 home = mix(aHome + vec3(sin(uTime * 0.013 + aBird.x) * 40.0, 0.0, cos(uTime * 0.011 + aBird.x) * 30.0), uCentre, eagle);',
        '  vec2 at = home.xz + vec2(cos(t), sin(t)) * aBird.y; vec2 dir = vec2(-sin(t), cos(t));',
        '  float flap = mix(sin(uTime * 9.0 + aBird.x * 5.0), sin(uTime * 2.4 + aBird.x) * step(0.8, sin(uTime * 0.27 + aBird.x)), eagle);',
        '  vec3 p = aPos * aBird.w; p.y += abs(p.x) * flap * mix(0.55, 0.3, eagle);',
        '  float bank = mix(0.0, 0.25, eagle); vec3 fw = vec3(dir.x, 0.0, dir.y), rt = vec3(dir.y, 0.0, -dir.x);',
        '  vec3 w = vec3(at.x, home.y + aBird.z + sin(t * 3.0) * 1.5, at.y) + rt * p.x + vec3(0.0, p.y - p.x * bank, 0.0) + fw * p.z;',
        '  vPos = w; gl_Position = uVP * vec4(w, 1.0); }'
      ].join('\n'),
      fs: HEAD + [
        'in vec3 vPos; uniform vec3 uFog; uniform vec3 uCam; uniform vec2 uFogRange; uniform float uNight; out vec4 o;',
        'void main() { vec3 c = mix(vec3(0.11, 0.09, 0.08), vec3(0.03, 0.03, 0.05), uNight);',
        '  o = vec4(mix(c, uFog, smoothstep(uFogRange.x, uFogRange.y, distance(uCam, vPos)) * 0.85), 1.0); }'
      ].join('\n')
    },
    marks: {
      vs: HEAD + [
        'in vec2 aDisc; in vec4 aInst; uniform mat4 uVP; uniform float uFocus; uniform float uTime; out vec2 vUV; out float vFocus;',
        'void main() { vFocus = abs(aInst.w - uFocus) < 0.5 ? 1.0 : 0.0; float r = (2.3 + vFocus * 1.5) * (1.0 + vFocus * 0.07 * sin(uTime * 2.4));',
        '  vUV = aDisc; gl_Position = uVP * vec4(aInst.xyz + vec3(aDisc.x * r, 0.0, aDisc.y * r), 1.0); }'
      ].join('\n'),
      fs: HEAD + [
        'in vec2 vUV; in float vFocus; uniform float uNight; out vec4 o;',
        'void main() { float d = length(vUV); float ring = smoothstep(1.0, 0.84, d) * smoothstep(0.58, 0.8, d); float core = smoothstep(0.62, 0.0, d) * 0.3;',
        '  vec3 c = mix(vec3(1.0, 0.93, 0.80), vec3(1.0, 0.78, 0.34), vFocus);',
        '  float a = (ring * (0.5 + vFocus * 0.5) + core * (0.3 + vFocus * 0.45)) * (0.7 + uNight * 0.55); if (a < 0.01) discard;',
        '  o = vec4(c * a, a); }'
      ].join('\n')
    },
    depth: {
      vs: HEAD + [
        'in vec3 aPos; in vec4 aInst; uniform mat4 uVP; uniform float uInst;',
        'float turn(vec2 p) { vec3 q = fract(vec3(p.xyx) * 0.1031); q += dot(q, q.yzx + 33.33); return fract((q.x + q.y) * q.z) * 6.2831853; }',
        'void main() { vec3 p = aPos;',
        '  if (uInst > 0.5) { float yaw = turn(aInst.xz), c = cos(yaw), s = sin(yaw); p = aPos * aInst.w; p = vec3(c * p.x + s * p.z, p.y, -s * p.x + c * p.z) + aInst.xyz; }',
        '  gl_Position = uVP * vec4(p, 1.0); }'
      ].join('\n'),
      fs: HEAD + 'void main() {}'
    }
  };

  function program(src) {
    function sh(type, text) {
      var s = gl.createShader(type);
      gl.shaderSource(s, text);
      gl.compileShader(s);
      if (!gl.getShaderParameter(s, gl.COMPILE_STATUS)) throw new Error(gl.getShaderInfoLog(s));
      return s;
    }
    var p = gl.createProgram();
    gl.attachShader(p, sh(gl.VERTEX_SHADER, src.vs));
    gl.attachShader(p, sh(gl.FRAGMENT_SHADER, src.fs));
    gl.linkProgram(p);
    if (!gl.getProgramParameter(p, gl.LINK_STATUS)) throw new Error(gl.getProgramInfoLog(p));
    var u = {}, count = gl.getProgramParameter(p, gl.ACTIVE_UNIFORMS);
    for (var i = 0; i < count; i++) {
      var name = gl.getActiveUniform(p, i).name;
      u[name] = gl.getUniformLocation(p, name);
    }
    return { p: p, u: u, a: function (name) { return gl.getAttribLocation(p, name); } };
  }
  function buffer(data, target) {
    var b = gl.createBuffer();
    gl.bindBuffer(target || gl.ARRAY_BUFFER, b);
    gl.bufferData(target || gl.ARRAY_BUFFER, data, gl.STATIC_DRAW);
    return b;
  }
  function attrib(prog, name, buf, size, divisor, offset) {
    var loc = prog.a(name);
    if (loc < 0) return;
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    gl.enableVertexAttribArray(loc);
    gl.vertexAttribPointer(loc, size, gl.FLOAT, false, 0, offset || 0);
    gl.vertexAttribDivisor(loc, divisor || 0);
  }

  /* ---- the shapes the instances wear ---- */
  var seedR = 7;
  function rnd() { seedR = (seedR * 16807) % 2147483647; return seedR / 2147483647; }
  function mesh() { return { p: [], n: [], c: [] }; }
  function put(m, v, nr, col) {
    m.p.push(v[0], v[1], v[2]);
    m.n.push(nr[0], nr[1], nr[2]);
    m.c.push(col[0], col[1], col[2]);
  }
  // A triangle facing away from `inside`, with its own normals or its face's.
  function tri(m, a, b, c, inside, cols, nrs) {
    var ux = b[0] - a[0], uy = b[1] - a[1], uz = b[2] - a[2], vx = c[0] - a[0], vy = c[1] - a[1], vz = c[2] - a[2];
    var f = norm([uy * vz - uz * vy, uz * vx - ux * vz, ux * vy - uy * vx]);
    var mid = [(a[0] + b[0] + c[0]) / 3 - inside[0], (a[1] + b[1] + c[1]) / 3 - inside[1], (a[2] + b[2] + c[2]) / 3 - inside[2]];
    var pts = [a, b, c], cl = Array.isArray(cols[0]) ? cols : [cols, cols, cols], ns = nrs || [f, f, f];
    if (f[0] * mid[0] + f[1] * mid[1] + f[2] * mid[2] < 0) {
      pts = [a, c, b];
      cl = [cl[0], cl[2], cl[1]];
      ns = [ns[0], ns[2], ns[1]];
      if (!nrs) ns = ns.map(function (q) { return [-q[0], -q[1], -q[2]]; });
    }
    for (var i = 0; i < 3; i++) put(m, pts[i], ns[i], cl[i]);
  }
  function scaled(c, k) { return [c[0] * k, c[1] * k, c[2] * k]; }
  function icosphere(sub) {
    var t = (1 + Math.sqrt(5)) / 2;
    var v = [[-1, t, 0], [1, t, 0], [-1, -t, 0], [1, -t, 0], [0, -1, t], [0, 1, t], [0, -1, -t], [0, 1, -t],
      [t, 0, -1], [t, 0, 1], [-t, 0, -1], [-t, 0, 1]].map(norm);
    var f = [[0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11], [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8],
      [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9], [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1]];
    for (var s = 0; s < sub; s++) {
      var cache = {}, next = [];
      var mid = function (a, b) {
        var key = a < b ? a + ':' + b : b + ':' + a;
        if (cache[key] === undefined) {
          v.push(norm([(v[a][0] + v[b][0]) / 2, (v[a][1] + v[b][1]) / 2, (v[a][2] + v[b][2]) / 2]));
          cache[key] = v.length - 1;
        }
        return cache[key];
      };
      f.forEach(function (q) {
        var a = mid(q[0], q[1]), b = mid(q[1], q[2]), c = mid(q[2], q[0]);
        next.push([q[0], a, c], [q[1], b, a], [q[2], c, b], [a, b, c]);
      });
      f = next;
    }
    return { v: v, f: f };
  }
  // A clump of leaves: a lumpy ball, lit softly, lighter on top.
  function crown(m, at, r, squash, col, sub, rough) {
    var ico = icosphere(sub), bumps = ico.v.map(function () { return 1 + (rnd() - 0.5) * rough; });
    var pt = ico.v.map(function (d, i) {
      return [at[0] + d[0] * r * squash[0] * bumps[i], at[1] + d[1] * r * squash[1] * bumps[i], at[2] + d[2] * r * squash[2] * bumps[i]];
    });
    var nr = ico.v.map(function (d) { return norm([d[0] / squash[0], d[1] / squash[1], d[2] / squash[2]]); });
    var cl = ico.v.map(function (d) { return scaled(col, (0.74 + 0.3 * (d[1] * 0.5 + 0.5)) * (0.9 + rnd() * 0.2)); });
    ico.f.forEach(function (q) { tri(m, pt[q[0]], pt[q[1]], pt[q[2]], at, [cl[q[0]], cl[q[1]], cl[q[2]]], [nr[q[0]], nr[q[1]], nr[q[2]]]); });
  }
  // A trunk: a tapering post with round light.
  function trunk(m, h, r0, r1, sides, col) {
    for (var i = 0; i < sides; i++) {
      var a0 = i / sides * Math.PI * 2, a1 = (i + 1) / sides * Math.PI * 2;
      var n0 = [Math.cos(a0), 0.15, Math.sin(a0)], n1 = [Math.cos(a1), 0.15, Math.sin(a1)];
      var b0 = [n0[0] * r0, 0, n0[2] * r0], b1 = [n1[0] * r0, 0, n1[2] * r0], t0 = [n0[0] * r1, h, n0[2] * r1], t1 = [n1[0] * r1, h, n1[2] * r1];
      tri(m, b0, t0, b1, [0, h / 2, 0], col, [n0, n0, n1]);
      tri(m, b1, t0, t1, [0, h / 2, 0], col, [n1, n0, n1]);
    }
  }
  // A tier of a fir: a ragged cone, its branches drooping at the rim.
  function tier(m, y0, y1, r, sides, col) {
    var rim = [], h = y1 - y0;
    for (var i = 0; i < sides; i++) {
      var a = (i + rnd() * 0.4) / sides * Math.PI * 2, rr = r * (0.8 + rnd() * 0.35);
      rim.push({ p: [Math.cos(a) * rr, y0 - rnd() * 0.35, Math.sin(a) * rr], n: norm([Math.cos(a) * h, rr, Math.sin(a) * h]) });
    }
    var apex = [0, y1, 0], under = [0, y0 + h * 0.18, 0];
    for (i = 0; i < sides; i++) {
      var p = rim[i], q = rim[(i + 1) % sides], top = norm([(p.n[0] + q.n[0]) / 2, 1.2, (p.n[2] + q.n[2]) / 2]);
      tri(m, p.p, apex, q.p, [0, y0 + h * 0.3, 0], [scaled(col, 0.8), scaled(col, 1.18), scaled(col, 0.8)], [p.n, top, q.n]);
      tri(m, p.p, q.p, under, [0, y0 + h * 0.5, 0], scaled(col, 0.45), [[p.n[0], -0.6, p.n[2]], [q.n[0], -0.6, q.n[2]], [0, -1, 0]]);
    }
  }
  var BARK = [0.30, 0.22, 0.15];
  function conifer(far) {
    var m = mesh(), g = [0.11, 0.21, 0.13];
    if (far) { tier(m, 0.9, 6.9, 1.7, 7, g); return m; }
    trunk(m, 2.0, 0.24, 0.12, 6, BARK);
    tier(m, 1.0, 3.9, 1.9, 12, g);
    tier(m, 2.3, 5.0, 1.5, 11, scaled(g, 1.08));
    tier(m, 3.5, 6.0, 1.12, 10, scaled(g, 1.16));
    tier(m, 4.6, 7.0, 0.72, 8, scaled(g, 1.24));
    return m;
  }
  function broadleaf(far) {
    var m = mesh(), g = [0.23, 0.33, 0.14];
    if (far) {
      trunk(m, 2.4, 0.3, 0.2, 4, BARK);
      crown(m, [0, 3.9, 0], 2.3, [1.1, 0.9, 1.1], g, 0, 0.2);
      return m;
    }
    trunk(m, 2.8, 0.32, 0.16, 6, BARK);
    crown(m, [0, 3.7, 0], 2.0, [1.15, 0.85, 1.1], g, 1, 0.28);
    crown(m, [1.1, 4.2, 0.5], 1.35, [1, 0.9, 1], scaled(g, 1.08), 1, 0.3);
    crown(m, [-0.9, 4.5, -0.4], 1.3, [1, 0.9, 1], scaled(g, 1.04), 1, 0.3);
    crown(m, [0.2, 5.1, -0.8], 1.1, [1, 0.9, 1], scaled(g, 1.14), 1, 0.3);
    return m;
  }
  function boulder() {
    var m = mesh(), ico = icosphere(1), at = [0, 0.15, 0], sq = [1.3, 0.7, 1.0];
    var pt = ico.v.map(function (d) { var k = 1 + (rnd() - 0.5) * 0.5; return [d[0] * sq[0] * k, 0.15 + d[1] * sq[1] * k, d[2] * sq[2] * k]; });
    ico.f.forEach(function (q) {
      var up = (ico.v[q[0]][1] + ico.v[q[1]][1] + ico.v[q[2]][1]) / 3, shade = 0.85 + rnd() * 0.25;
      var col = up > 0.55 ? mix([0.48, 0.47, 0.42], [0.36, 0.42, 0.22], (up - 0.55) * 1.8) : [0.50, 0.48, 0.44];
      tri(m, pt[q[0]], pt[q[1]], pt[q[2]], at, scaled(col, shade));
    });
    return m;
  }
  function box(m, x, y, z, w, h, d, col) {
    var X = [x - w / 2, x + w / 2], Y = [y, y + h], Z = [z - d / 2, z + d / 2], at = [x, y + h / 2, z];
    var c = function (i, j, k) { return [X[i], Y[j], Z[k]]; };
    [[c(0, 0, 1), c(1, 0, 1), c(1, 1, 1), c(0, 1, 1)], [c(1, 0, 0), c(0, 0, 0), c(0, 1, 0), c(1, 1, 0)],
      [c(0, 0, 0), c(0, 0, 1), c(0, 1, 1), c(0, 1, 0)], [c(1, 0, 1), c(1, 0, 0), c(1, 1, 0), c(1, 1, 1)],
      [c(0, 1, 1), c(1, 1, 1), c(1, 1, 0), c(0, 1, 0)]].forEach(function (q) {
      tri(m, q[0], q[1], q[2], at, col);
      tri(m, q[0], q[2], q[3], at, col);
    });
  }
  function beast() {
    var m = mesh(), fur = [0.52, 0.38, 0.24], dark = [0.24, 0.18, 0.12], light = [0.82, 0.74, 0.62];
    box(m, 0, 0.64, 0, 0.38, 0.42, 1.08, fur);
    box(m, 0, 0.6, -0.1, 0.3, 0.1, 0.72, light);
    box(m, 0, 0.9, 0.52, 0.17, 0.38, 0.22, fur);
    box(m, 0, 1.1, 0.72, 0.17, 0.17, 0.34, dark);
    box(m, -0.06, 1.26, 0.66, 0.03, 0.2, 0.03, dark);
    box(m, 0.06, 1.26, 0.66, 0.03, 0.2, 0.03, dark);
    box(m, 0, 0.82, -0.56, 0.06, 0.08, 0.14, light);
    [[-0.12, -0.4], [0.12, -0.4], [-0.12, 0.4], [0.12, 0.4]].forEach(function (l) { box(m, l[0], 0, l[1], 0.07, 0.66, 0.07, dark); });
    return m;
  }
  function tuft() {
    // Three crossed cards of blades.
    var p = [], uv = [];
    for (var i = 0; i < 3; i++) {
      var a = i / 3 * Math.PI, c = Math.cos(a) * 0.5, s = Math.sin(a) * 0.5;
      [[-c, 0, -s, 0, 0], [c, 0, s, 1, 0], [c, 0.75, s, 1, 1], [-c, 0, -s, 0, 0], [c, 0.75, s, 1, 1], [-c, 0.75, -s, 0, 1]]
        .forEach(function (v) { p.push(v[0], v[1], v[2]); uv.push(v[3], v[4]); });
    }
    return { p: p, uv: uv };
  }
  function birdShape() {
    return [0, 0, 0.35, -1, 0.06, -0.1, 0, 0, -0.2, 0, 0, 0.35, 0, 0, -0.2, 1, 0.06, -0.1, 0, 0, -0.2, -0.16, 0, -0.5, 0.16, 0, -0.5];
  }

  /* ---- the scene, built once ---- */
  var W, progs = {}, draws = {}, centre = root.getAttribute('data-river-centre');
  var cam = { tx: 0, ty: 0, tz: 0, yaw: Math.PI, pitch: 0.5, dist: 64 };
  var MAX_DIST = 320, CH = 8;
  var fly = null, words = {}, mvp = null, eye = [0, 0, 0], planes = null;
  var shadow = { size: 2048, fb: null, tex: null, vp: null, on: true, planes: null };
  var heightTex = null, levelTex = null, coverTex = null, marksY = [];

  function heightAt(x, z) {
    var n = W.n, fx = Math.max(0, Math.min(n - 1.001, (x - W.x0) / W.sx * (n - 1)));
    var fz = Math.max(0, Math.min(n - 1.001, (z - W.z0) / W.sz * (n - 1)));
    var i = Math.floor(fx), j = Math.floor(fz), tx = fx - i, tz = fz - j, h = W.h;
    var a = h[j * n + i] + (h[j * n + i + 1] - h[j * n + i]) * tx;
    var b = h[(j + 1) * n + i] + (h[(j + 1) * n + i + 1] - h[(j + 1) * n + i]) * tx;
    return a + (b - a) * tz;
  }

  // Instances sorted into a grid of chunks, so that what the camera does
  // not see is not drawn, and what is far is drawn simpler.
  function chunked(inst, extra, reach, tall, kinds) {
    var count = inst.length / 4, cells = [], i, k;
    for (k = 0; k < CH * CH; k++) cells.push([]);
    for (i = 0; i < count; i++) {
      var ci = Math.max(0, Math.min(CH - 1, Math.floor((inst[i * 4] - W.x0) / W.sx * CH)));
      var cj = Math.max(0, Math.min(CH - 1, Math.floor((inst[i * 4 + 2] - W.z0) / W.sz * CH)));
      cells[cj * CH + ci].push(i);
    }
    var si = new Float32Array(inst.length), se = new Float32Array(count * 2), at = 0, chunks = [];
    cells.forEach(function (list) {
      if (!list.length) return;
      var b = { start: at, count: list.length, min: [1e9, 1e9, 1e9], max: [-1e9, -1e9, -1e9] };
      list.forEach(function (q) {
        var x = inst[q * 4], y = inst[q * 4 + 1], z = inst[q * 4 + 2], s = inst[q * 4 + 3];
        si.set([x, y, z, s], at * 4);
        se.set([extra[q * 2], extra[q * 2 + 1]], at * 2);
        b.min = [Math.min(b.min[0], x - reach * s), Math.min(b.min[1], y - 1), Math.min(b.min[2], z - reach * s)];
        b.max = [Math.max(b.max[0], x + reach * s), Math.max(b.max[1], y + tall * s), Math.max(b.max[2], z + reach * s)];
        at++;
      });
      chunks.push(b);
    });
    var ib = buffer(si), eb = buffer(se);
    // For each kind of shape (near, far) and each pass (seen, shadow).
    var shapes = kinds.map(function (m) {
      return { m: m, pb: buffer(new Float32Array(m.p)), nb: buffer(new Float32Array(m.n)), cb: buffer(new Float32Array(m.c)), count: m.p.length / 3 };
    });
    chunks.forEach(function (b) {
      b.vao = [];
      b.depth = [];
      shapes.forEach(function (sh) {
        var v = gl.createVertexArray();
        gl.bindVertexArray(v);
        attrib(progs.solid, 'aPos', sh.pb, 3);
        attrib(progs.solid, 'aNor', sh.nb, 3);
        attrib(progs.solid, 'aCol', sh.cb, 3);
        attrib(progs.solid, 'aInst', ib, 4, 1, b.start * 16);
        attrib(progs.solid, 'aExtra', eb, 2, 1, b.start * 8);
        b.vao.push(v);
        var d = gl.createVertexArray();
        gl.bindVertexArray(d);
        attrib(progs.depth, 'aPos', sh.pb, 3);
        attrib(progs.depth, 'aInst', ib, 4, 1, b.start * 16);
        b.depth.push(d);
      });
    });
    gl.bindVertexArray(null);
    return { chunks: chunks, counts: shapes.map(function (sh) { return sh.count; }), total: count, ib: ib };
  }

  function build() {
    var n = W.n, N = n * n, i, j, k;
    Object.keys(SRC).forEach(function (key) { progs[key] = program(SRC[key]); });
    var dx = W.sx / (n - 1), dz = W.sz / (n - 1);

    // Where the water lies — the rivers and the pools — and, at each point
    // of the grid, the level, the flow and the foam of the water nearest it.
    // The ground hides it wherever the ground is higher, so its shore is the
    // land's own.
    var near = new Float32Array(N).fill(1e9), level = new Float32Array(N), fx = new Float32Array(N), fz = new Float32Array(N);
    var foam = new Float32Array(N), wet = new Float32Array(N), path = new Float32Array(N);
    function stamp(x, z, r, lv, vx, vz, fo, grow) {
      var reach = r + grow;
      var i0 = Math.max(0, Math.floor((x - reach - W.x0) / dx)), i1 = Math.min(n - 1, Math.ceil((x + reach - W.x0) / dx));
      var j0 = Math.max(0, Math.floor((z - reach - W.z0) / dz)), j1 = Math.min(n - 1, Math.ceil((z + reach - W.z0) / dz));
      for (var jj = j0; jj <= j1; jj++) {
        var pz = W.z0 + jj * dz;
        for (var ii = i0; ii <= i1; ii++) {
          var d = Math.hypot(W.x0 + ii * dx - x, pz - z);
          if (d > reach) continue;
          var q = jj * n + ii, e = Math.max(0, d - r);
          if (e < near[q] || (e === near[q] && lv > level[q])) {
            near[q] = e;
            level[q] = lv;
            fx[q] = vx;
            fz[q] = vz;
          }
          var w = W.h[q] < lv - 0.02 ? 1 : 0.85 * Math.max(0, 1 - e / (grow * 0.8));
          if (w > wet[q]) wet[q] = w;
          var f = fo * Math.max(0, 1 - d / reach);
          if (f > foam[q]) foam[q] = f;
        }
      }
    }
    // A pool fills its own cells, and a little under its banks.
    function pondCell(ci, cj, lv) {
      for (var jj = Math.max(0, cj - 2); jj <= Math.min(n - 1, cj + 2); jj++) {
        for (var ii = Math.max(0, ci - 2); ii <= Math.min(n - 1, ci + 2); ii++) {
          var e = Math.hypot((ii - ci) * dx, (jj - cj) * dz), q = jj * n + ii;
          if (e > 2.05 * dx) continue;
          if (e < near[q] || (e === near[q] && lv > level[q])) {
            near[q] = e;
            level[q] = lv;
            fx[q] = 0;
            fz[q] = 0;
          }
          var w = W.h[q] < lv - 0.02 ? 1 : 0.85 * Math.max(0, 1 - e / (2 * dx));
          if (w > wet[q]) wet[q] = w;
        }
      }
    }
    W.ponds.forEach(function (pond) {
      pond.spans.forEach(function (s) { for (var ci = s[1]; ci <= s[2]; ci++) pondCell(ci, s[0], pond.level); });
    });
    W.rivers.forEach(function (pts) {
      var m = pts.length / 5;
      for (var t = 0; t < m - 1; t++) {
        var o = t * 5, x0 = pts[o], z0 = pts[o + 1], x1 = pts[o + 5], z1 = pts[o + 6];
        var len = Math.hypot(x1 - x0, z1 - z0) || 1e-3, steps = Math.max(1, Math.ceil(len / (dx * 0.5)));
        var speed = 0.5 + Math.min(2.5, (pts[o + 2] - pts[o + 7]) / len * 14) + Math.min(1, pts[o + 3] * 0.15);
        for (var s = 0; s <= steps; s++) {
          var u = s / steps;
          stamp(x0 + (x1 - x0) * u, z0 + (z1 - z0) * u, pts[o + 3] + (pts[o + 8] - pts[o + 3]) * u,
            pts[o + 2] + (pts[o + 7] - pts[o + 2]) * u, (x1 - x0) / len * speed, (z1 - z0) / len * speed,
            pts[o + 4] + (pts[o + 9] - pts[o + 4]) * u, dx * 2.2);
        }
      }
    });
    W.paths.forEach(function (pp) {
      var m = pp.length / 3;
      for (var t = 0; t < m - 1; t++) {
        var x0 = pp[t * 3], z0 = pp[t * 3 + 1], x1 = pp[t * 3 + 3], z1 = pp[t * 3 + 4], hw = pp[t * 3 + 2];
        var steps = Math.max(1, Math.ceil(Math.hypot(x1 - x0, z1 - z0) / (dx * 0.5)));
        for (var s = 0; s <= steps; s++) {
          var u = s / steps, x = x0 + (x1 - x0) * u, z = z0 + (z1 - z0) * u;
          var ci = Math.round((x - W.x0) / dx), cj = Math.round((z - W.z0) / dz);
          for (var jj = Math.max(0, cj - 2); jj <= Math.min(n - 1, cj + 2); jj++) {
            for (var ii = Math.max(0, ci - 2); ii <= Math.min(n - 1, ci + 2); ii++) {
              var d = Math.hypot(W.x0 + ii * dx - x, W.z0 + jj * dz - z), q = jj * n + ii;
              path[q] = Math.max(path[q], 1 - smooth(hw * 0.5, hw * 1.9, d));
            }
          }
        }
      }
    });

    // The ground: the grid, its normals, what covers it — forest, path, the
    // wet of the water near — and how much of the sky each point sees.
    var pos = new Float32Array(N * 3), nor = new Float32Array(N * 3), mixes = new Float32Array(N * 4);
    for (j = 0; j < n; j++) {
      for (i = 0; i < n; i++) {
        k = j * n + i;
        pos[k * 3] = W.x0 + i * dx;
        pos[k * 3 + 1] = W.h[k];
        pos[k * 3 + 2] = W.z0 + j * dz;
        var hl = W.h[j * n + Math.max(0, i - 1)], hr = W.h[j * n + Math.min(n - 1, i + 1)];
        var hd = W.h[Math.max(0, j - 1) * n + i], hu = W.h[Math.min(n - 1, j + 1) * n + i];
        var nv = norm([(hl - hr) / (2 * dx), 1, (hd - hu) / (2 * dz)]);
        nor.set(nv, k * 3);
        var above = 0, cnt = 0;
        for (var r = 2; r <= 11; r += 3) {
          for (var a = 0; a < 6; a++) {
            var ang = a * 1.047 + r;
            var i2 = Math.max(0, Math.min(n - 1, i + Math.round(Math.cos(ang) * r))), j2 = Math.max(0, Math.min(n - 1, j + Math.round(Math.sin(ang) * r)));
            above += Math.max(0, W.h[j2 * n + i2] - W.h[k]) / (r * dx);
            cnt++;
          }
        }
        mixes[k * 4] = W.forest[k] / 255;
        mixes[k * 4 + 1] = path[k];
        mixes[k * 4 + 2] = Math.min(1, wet[k]);
        mixes[k * 4 + 3] = Math.max(0.25, 1 - above / cnt * 1.4);
      }
    }
    var idx = new Uint32Array((n - 1) * (n - 1) * 6), q = 0;
    for (j = 0; j < n - 1; j++) {
      for (i = 0; i < n - 1; i++) {
        var a0 = j * n + i, b0 = a0 + 1, c0 = a0 + n, d0 = c0 + 1;
        idx[q++] = a0; idx[q++] = c0; idx[q++] = b0; idx[q++] = b0; idx[q++] = c0; idx[q++] = d0;
      }
    }
    var g = gl.createVertexArray();
    gl.bindVertexArray(g);
    var groundPos = buffer(pos);
    attrib(progs.ground, 'aPos', groundPos, 3);
    attrib(progs.ground, 'aNor', buffer(nor), 3);
    attrib(progs.ground, 'aMix', buffer(mixes), 4);
    var groundIdx = buffer(idx, gl.ELEMENT_ARRAY_BUFFER);
    draws.ground = { vao: g, count: idx.length };
    var gd = gl.createVertexArray();
    gl.bindVertexArray(gd);
    attrib(progs.depth, 'aPos', groundPos, 3);
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, groundIdx);
    draws.groundDepth = { vao: gd, count: idx.length };
    draws.skirt = skirt(dx, dz);

    // The ground's heights, for the water to know how deep it is and the
    // grass where to stand.
    heightTex = grid(gl.R16F, gl.RED, gl.FLOAT, W.h);

    // The water: the cells of the grid where it shows, at its own level;
    // the lake is one surface to the horizon.
    var wp = [], wf = [], wi = [], vmap = new Int32Array(N).fill(-1);
    function wv(kk) {
      if (vmap[kk] < 0) {
        vmap[kk] = wp.length / 3;
        wp.push(W.x0 + (kk % n) * dx, level[kk], W.z0 + Math.floor(kk / n) * dz);
        wf.push(fx[kk], fz[kk], foam[kk], 0);
      }
      return vmap[kk];
    }
    // A corner the water does not reach dips just under the ground there,
    // so that the water's edge lies on the land between.
    var dry = new Uint8Array(N);
    for (k = 0; k < N; k++) {
      if (near[k] >= 1e8) {
        dry[k] = 1;
        level[k] = W.h[k] - 0.05;
      }
    }
    for (j = 0; j < n - 1; j++) {
      for (i = 0; i < n - 1; i++) {
        var ks = [j * n + i, j * n + i + 1, (j + 1) * n + i, (j + 1) * n + i + 1], shows = false, lake = true;
        for (var c = 0; c < 4; c++) {
          var kk = ks[c];
          if (dry[kk]) continue;
          if (W.h[kk] < level[kk]) shows = true;
          if (level[kk] > W.water + 0.03) lake = false;
        }
        if (!shows || lake) continue;
        var v = ks.map(wv);
        wi.push(v[0], v[2], v[1], v[1], v[2], v[3]);
      }
    }
    // The water's level and what covers the ground, for the banks and the
    // grass.
    levelTex = grid(gl.R16F, gl.RED, gl.FLOAT, level);
    var cover = new Uint8Array(N * 4);
    for (k = 0; k < N; k++) {
      cover[k * 4] = W.forest[k];
      cover[k * 4 + 1] = Math.round(path[k] * 255);
      cover[k * 4 + 2] = Math.round(Math.min(1, wet[k]) * 255);
      cover[k * 4 + 3] = 255;
    }
    coverTex = grid(gl.RGBA8, gl.RGBA, gl.UNSIGNED_BYTE, cover);
    var wvao = gl.createVertexArray();
    gl.bindVertexArray(wvao);
    attrib(progs.water, 'aPos', buffer(new Float32Array(wp)), 3);
    attrib(progs.water, 'aFlow', buffer(new Float32Array(wf)), 4);
    buffer(new Uint32Array(wi), gl.ELEMENT_ARRAY_BUFFER);
    draws.water = { vao: wvao, count: wi.length };
    var L = W.water, X0 = W.x0 - 4000, Z0 = W.z0 - 4000, X1 = W.x0 + W.sx + 4000, Z1 = W.z0 + W.sz + 4000;
    var lv = gl.createVertexArray();
    gl.bindVertexArray(lv);
    attrib(progs.water, 'aPos', buffer(new Float32Array([X0, L, Z0, X1, L, Z0, X0, L, Z1, X1, L, Z1])), 3);
    attrib(progs.water, 'aFlow', buffer(new Float32Array([0.03, 0.015, 0, 0, 0.03, 0.015, 0, 0, 0.03, 0.015, 0, 0, 0.03, 0.015, 0, 0])), 4);
    buffer(new Uint32Array([0, 2, 1, 1, 2, 3]), gl.ELEMENT_ARRAY_BUFFER);
    draws.lake = { vao: lv, count: 6 };

    // The people's lights: on the water where they stand in a pool, or just
    // over the ground.
    marksY = W.persons.map(function (p) {
      var y = -1e9;
      for (var a = 0; a < 9; a++) {
        var rr = a ? 2.2 : 0, an = a * Math.PI / 4;
        y = Math.max(y, heightAt(p.x + Math.cos(an) * rr, p.z + Math.sin(an) * rr));
      }
      var ci = Math.round((p.x - W.x0) / dx), cj = Math.round((p.z - W.z0) / dz);
      var kk = Math.max(0, Math.min(n - 1, cj)) * n + Math.max(0, Math.min(n - 1, ci));
      var floor = heightAt(p.x, p.z);
      if (near[kk] === 0 && level[kk] > floor) y = Math.max(floor, level[kk]);
      return Math.max(y, W.water) + 0.1;
    });

    // The trees, by kind; the boulders; the herds; the grass.
    var con = [], conX = [], brd = [], brdX = [];
    for (i = 0; i < W.trees.length; i += 6) {
      var inst = [W.trees[i], W.trees[i + 2] - 0.3, W.trees[i + 1], W.trees[i + 3]];
      if (W.trees[i + 4] < 0.5) { Array.prototype.push.apply(con, inst); conX.push(0, W.trees[i + 5]); }
      else { Array.prototype.push.apply(brd, inst); brdX.push(1, W.trees[i + 5]); }
    }
    draws.conifers = chunked(new Float32Array(con), new Float32Array(conX), 2.3, 7.4, [conifer(false), conifer(true)]);
    draws.broadleaves = chunked(new Float32Array(brd), new Float32Array(brdX), 3.0, 6.5, [broadleaf(false), broadleaf(true)]);
    var rk = [], rx = [];
    for (i = 0; i < W.rocks.length; i += 4) {
      rk.push(W.rocks[i], W.rocks[i + 2] - 0.2 * W.rocks[i + 3], W.rocks[i + 1], W.rocks[i + 3]);
      rx.push(0, 0);
    }
    draws.rocks = chunked(new Float32Array(rk), new Float32Array(rx), 1.5, 1.2, [boulder()]);
    var an2 = [], ax = [];
    for (i = 0; i < W.herds.length; i += 4) {
      for (k = 0; k < W.herds[i + 3]; k++) {
        var bx = W.herds[i] + (k % 3) * 2.6 - 2.6, bz = W.herds[i + 1] + Math.floor(k / 3) * 2.8 - 2.8;
        an2.push(bx, heightAt(bx, bz), bz, 1.5 + (k % 2) * 0.2);
        ax.push(2, k * 0.83 + i * 0.1);
      }
    }
    draws.beasts = chunked(new Float32Array(an2), new Float32Array(ax), 6, 2, [beast()]);
    var tf = tuft(), gv = gl.createVertexArray();
    gl.bindVertexArray(gv);
    attrib(progs.grass, 'aPos', buffer(new Float32Array(tf.p)), 3);
    attrib(progs.grass, 'aUV', buffer(new Float32Array(tf.uv)), 2);
    draws.grass = { vao: gv, count: tf.p.length / 3 };

    // The birds: flocks along the valley, and the eagle over whoever the
    // camera looks at.
    var lo = [1e9, 1e9], hi = [-1e9, -1e9];
    W.persons.forEach(function (p) { lo = [Math.min(lo[0], p.x), Math.min(lo[1], p.z)]; hi = [Math.max(hi[0], p.x), Math.max(hi[1], p.z)]; });
    if (!W.persons.length) { lo = [-50, -50]; hi = [50, 50]; }
    var bs = [], bh = [];
    for (var f = 0; f < 5; f++) {
      var hx = (lo[0] + hi[0]) / 2 + (f % 2 ? -0.22 : 0.22) * (hi[0] - lo[0]), hz = lo[1] + (hi[1] - lo[1]) * (f + 0.5) / 5;
      var hy = Math.max(heightAt(hx, hz), W.water);
      for (k = 0; k < 7; k++) {
        bs.push(f * 2.3 + k * 0.045, 62 + f * 9 + (k % 3) * 2.2, 30 + f * 3 + (k % 4) * 1.3, 0.85);
        bh.push(hx, hy, hz);
      }
    }
    bs.push(0.3, 30, 26, 2.6);
    bh.push(0, 0, 0);
    var bv = gl.createVertexArray();
    gl.bindVertexArray(bv);
    attrib(progs.bird, 'aPos', buffer(new Float32Array(birdShape())), 3);
    attrib(progs.bird, 'aBird', buffer(new Float32Array(bs)), 4, 1);
    attrib(progs.bird, 'aHome', buffer(new Float32Array(bh)), 3, 1);
    draws.birds = { vao: bv, count: 9, instances: bs.length / 4 };

    var disc = [];
    for (k = 0; k < 40; k++) {
      var a1 = k / 40 * Math.PI * 2, a2 = (k + 1) / 40 * Math.PI * 2;
      disc.push(0, 0, Math.cos(a1), Math.sin(a1), Math.cos(a2), Math.sin(a2));
    }
    var mi = [];
    W.persons.forEach(function (p, pi) { mi.push(p.x, marksY[pi], p.z, p.index); });
    var mv = gl.createVertexArray();
    gl.bindVertexArray(mv);
    attrib(progs.marks, 'aDisc', buffer(new Float32Array(disc)), 2);
    attrib(progs.marks, 'aInst', buffer(new Float32Array(mi)), 4, 1);
    draws.marks = { vao: mv, count: disc.length / 2, instances: W.persons.length };
    draws.sky = { vao: gl.createVertexArray() };
    gl.bindVertexArray(null);

    // The sun's shadows: a depth map around whoever the camera looks at.
    shadow.tex = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, shadow.tex);
    gl.texStorage2D(gl.TEXTURE_2D, 1, gl.DEPTH_COMPONENT24, shadow.size, shadow.size);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_COMPARE_MODE, gl.COMPARE_REF_TO_TEXTURE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_COMPARE_FUNC, gl.LEQUAL);
    shadow.fb = gl.createFramebuffer();
    gl.bindFramebuffer(gl.FRAMEBUFFER, shadow.fb);
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, gl.TEXTURE_2D, shadow.tex, 0);
    gl.drawBuffers([gl.NONE]);
    gl.readBuffer(gl.NONE);
    if (gl.checkFramebufferStatus(gl.FRAMEBUFFER) !== gl.FRAMEBUFFER_COMPLETE) shadow.on = false;
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
  }

  // A texture of the grid: one texel per point, read between them.
  function grid(internal, format, type, data) {
    var t = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, t);
    gl.pixelStorei(gl.UNPACK_ALIGNMENT, 1);
    gl.texImage2D(gl.TEXTURE_2D, 0, internal, W.n, W.n, 0, format, type, data);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    return t;
  }

  // Beyond the grid the land goes on: the mountains rise on, the lake
  // reaches the horizon, so that the world has no edge to see.
  function skirt(dx, dz) {
    var n = W.n, per = [], STEP = 4, i, j;
    for (i = 0; i < n - 1; i += STEP) per.push([i, 0]);
    for (j = 0; j < n - 1; j += STEP) per.push([n - 1, j]);
    for (i = n - 1; i > 0; i -= STEP) per.push([i, n - 1]);
    for (j = n - 1; j > 0; j -= STEP) per.push([0, j]);
    var D = [0, 14, 34, 66, 115, 190, 310, 500, 800, 1300, 2100, 3400], P = per.length, R = D.length;
    var pos = new Float32Array(P * R * 3), mixes = new Float32Array(P * R * 4);
    per.forEach(function (e, pi) {
      var ox = e[0] === 0 ? -1 : e[0] === n - 1 ? 1 : 0, oz = e[1] === 0 ? -1 : e[1] === n - 1 ? 1 : 0, l = Math.hypot(ox, oz);
      ox /= l; oz /= l;
      var x = W.x0 + e[0] * dx, z = W.z0 + e[1] * dz, k = e[1] * n + e[0], he = W.h[k], forest = W.forest[k] / 255;
      var rise = smooth(W.water + 2, W.water + 30, he);
      D.forEach(function (d, ri) {
        var px = x + ox * d, pz = z + oz * d, q = pi * R + ri;
        var h = he + rise * (d * 0.3 + ridges(px / 150, pz / 150) * Math.min(d * 0.45, 170)) - (ri === 0 ? 0.4 : 0);
        pos.set([px, h, pz], q * 3);
        mixes.set([forest * Math.max(0, 1 - d / 700) * (1 - smooth(60, 140, h - W.water)), 0, 0, 1], q * 4);
      });
    });
    var idx = [];
    for (var pi = 0; pi < P; pi++) {
      var pn = (pi + 1) % P;
      for (var ri = 0; ri < R - 1; ri++) {
        var a = pi * R + ri, b = pn * R + ri, c = pi * R + ri + 1, d = pn * R + ri + 1;
        idx.push(a, b, c, b, d, c);
      }
    }
    // Normals from the faces around each point; faces turned to the sky.
    var nor = new Float32Array(P * R * 3);
    for (var t = 0; t < idx.length; t += 3) {
      var A = idx[t] * 3, B = idx[t + 1] * 3, C = idx[t + 2] * 3;
      var ux = pos[B] - pos[A], uy = pos[B + 1] - pos[A + 1], uz = pos[B + 2] - pos[A + 2];
      var vx = pos[C] - pos[A], vy = pos[C + 1] - pos[A + 1], vz = pos[C + 2] - pos[A + 2];
      var nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
      if (ny < 0) { nx = -nx; ny = -ny; nz = -nz; var sw = idx[t + 1]; idx[t + 1] = idx[t + 2]; idx[t + 2] = sw; }
      [A, B, C].forEach(function (o) { nor[o] += nx; nor[o + 1] += ny; nor[o + 2] += nz; });
    }
    for (t = 0; t < nor.length; t += 3) nor.set(norm([nor[t], nor[t + 1], nor[t + 2]]), t);
    var v = gl.createVertexArray();
    gl.bindVertexArray(v);
    attrib(progs.ground, 'aPos', buffer(pos), 3);
    attrib(progs.ground, 'aNor', buffer(nor), 3);
    attrib(progs.ground, 'aMix', buffer(mixes), 4);
    buffer(new Uint32Array(idx), gl.ELEMENT_ARRAY_BUFFER);
    gl.bindVertexArray(null);
    return { vao: v, count: idx.length };
  }

  /* ---- the camera ---- */
  var size = { w: 1, h: 1, dpr: 1 };
  var quality = { tier: fixedTier === null ? 0 : fixedTier, scale: 1 };
  var TIERS = [
    { scale: 1, shadows: true, grass: 180, lod: 240 },
    { scale: 0.85, shadows: true, grass: 140, lod: 170 },
    { scale: 0.72, shadows: false, grass: 100, lod: 120 },
    { scale: 0.58, shadows: false, grass: 0, lod: 80 }
  ];
  function resize() {
    var r = stage.getBoundingClientRect();
    quality.scale = TIERS[quality.tier].scale;
    size.dpr = Math.min(window.devicePixelRatio || 1, 2) * quality.scale;
    size.w = Math.max(1, Math.round(r.width));
    size.h = Math.max(1, Math.round(r.height));
    canvas.width = Math.max(1, Math.round(size.w * size.dpr));
    canvas.height = Math.max(1, Math.round(size.h * size.dpr));
    dirty = true;
    wake();
  }
  // What stands under a point: the ground, its trees, and beyond the grid
  // the mountains that go on.
  function above(x, z) {
    var n = W.n, i = Math.round((Math.max(W.x0, Math.min(W.x0 + W.sx, x)) - W.x0) / W.sx * (n - 1));
    var j = Math.round((Math.max(W.z0, Math.min(W.z0 + W.sz, z)) - W.z0) / W.sz * (n - 1));
    var h = heightAt(x, z), out = Math.hypot(Math.max(W.x0 - x, 0, x - W.x0 - W.sx), Math.max(W.z0 - z, 0, z - W.z0 - W.sz));
    if (out > 0) h += smooth(W.water + 2, W.water + 30, h) * (out * 0.3 + Math.min(out * 0.45, 170));
    return Math.max(h + 3 + (out > 0 ? 0 : W.forest[j * n + i] / 255 * 9), W.water + 2.5);
  }
  function matrices() {
    // Never in the ground, the trees or the water: the camera rises round
    // its target rather than go through them.
    var pitch = cam.pitch, floor = 0;
    for (var it = 0; it < 24; it++) {
      var cp = Math.cos(pitch), sp = Math.sin(pitch);
      eye = [cam.tx + Math.sin(cam.yaw) * cp * cam.dist, cam.ty + sp * cam.dist, cam.tz + Math.cos(cam.yaw) * cp * cam.dist];
      floor = above(eye[0], eye[2]);
      if (eye[1] >= floor || pitch >= 1.45) break;
      pitch = Math.min(1.45, pitch + 0.05);
    }
    if (eye[1] < floor) eye[1] = floor;
    var view = lookAt(eye, [cam.tx, cam.ty + 1.5, cam.tz], [0, 1, 0]);
    mvp = mul(perspective(0.85, size.w / size.h, 0.6, 6000), view);
    planes = planesOf(mvp);
    // The shadow map follows the camera's target, as large as what it sees.
    var reach = Math.max(90, Math.min(420, cam.dist * 2.6)), L = LIGHT.dir, c = [cam.tx, cam.ty, cam.tz];
    var from = [c[0] + L[0] * 600, c[1] + L[1] * 600, c[2] + L[2] * 600];
    shadow.vp = mul(ortho(-reach, reach, -reach, reach, 100, 1400), lookAt(from, c, Math.abs(L[1]) > 0.98 ? [0, 0, 1] : [0, 1, 0]));
    shadow.planes = planesOf(shadow.vp);
  }
  function focusOn(p, ms) {
    var y = marksY[p.index];
    var to = { tx: p.x, ty: y, tz: p.z, dist: Math.min(Math.max(cam.dist, 40), 110) };
    if (!ms || reduce || cut) { cam.tx = to.tx; cam.ty = to.ty; cam.tz = to.tz; cam.dist = to.dist; dirty = true; wake(); return; }
    fly = { from: { tx: cam.tx, ty: cam.ty, tz: cam.tz, dist: cam.dist }, to: to, t0: performance.now(), ms: ms };
    wake();
  }

  /* ---- the names, over the scene ---- */
  var pool = [], allNames = false;
  function project(x, y, z) {
    var m = mvp, cx = m[0] * x + m[4] * y + m[8] * z + m[12], cy = m[1] * x + m[5] * y + m[9] * z + m[13];
    var cw = m[3] * x + m[7] * y + m[11] * z + m[15];
    if (cw <= 0.1) return null;
    return { x: (cx / cw * 0.5 + 0.5) * size.w, y: (1 - (cy / cw * 0.5 + 0.5)) * size.h, w: cw };
  }
  function drawLabels() {
    var near = [];
    W.persons.forEach(function (p) {
      var wd = words[p.id];
      if (!wd) return;
      var s = project(p.x, marksY[p.index] + 3.2, p.z);
      if (!s || s.x < -40 || s.x > size.w + 40 || s.y < -20 || s.y > size.h + 20 || s.w > (allNames ? 800 : 380)) return;
      near.push({ p: p, s: s, wd: wd, focus: p.id === centre });
    });
    near.sort(function (a, b) { return (b.focus - a.focus) || (a.s.w - b.s.w); });
    // The nearest first, and none written over another: a name that would
    // cover one already placed waits until the camera comes closer.
    var shown = [], boxes = [];
    near.forEach(function (it) {
      if (shown.length >= (allNames ? 160 : 60)) return;
      var name = (it.wd[0] + ' ' + it.wd[1]).trim();
      var w = Math.max(name.length * (it.focus ? 9 : 7), it.wd[2].length * 6) + 10, h = it.focus ? 36 : 30;
      var bx = { l: it.s.x - w / 2, r: it.s.x + w / 2, t: it.s.y - h, b: it.s.y };
      if (!it.focus && boxes.some(function (o) { return bx.l < o.r && bx.r > o.l && bx.t < o.b && bx.b > o.t; })) return;
      boxes.push(bx);
      shown.push(it);
    });
    while (pool.length < shown.length) {
      var el = document.createElement('span');
      el.className = 'rv-world-label';
      el.appendChild(document.createElement('b'));
      el.appendChild(document.createElement('small'));
      // A name is a way to its person, as on the river.
      el.addEventListener('click', function (ev) {
        var id = ev.currentTarget.getAttribute('data-id');
        if (id && id !== centre && window.axgfRiver) window.axgfRiver.travel(id);
      });
      labels.appendChild(el);
      pool.push(el);
    }
    pool.forEach(function (el, i) {
      var it = shown[i];
      if (!it) { el.hidden = true; return; }
      el.hidden = false;
      el.setAttribute('data-id', it.p.id);
      var name = (it.wd[0] + ' ' + it.wd[1]).trim();
      if (el.firstChild.textContent !== name) el.firstChild.textContent = name;
      if (el.lastChild.textContent !== it.wd[2]) el.lastChild.textContent = it.wd[2];
      el.classList.toggle('is-focus', it.focus);
      el.style.transform = 'translate(' + Math.round(it.s.x) + 'px,' + Math.round(it.s.y) + 'px) translate(-50%,-100%)';
      el.style.opacity = String(Math.max(0.4, 1 - it.s.w / 400));
    });
  }

  /* ---- drawing ---- */
  var dirty = true, raf = null, onscreen = true, t0 = performance.now(), frozenAt = 0, lightAt = 0;
  var perf = { ema: 16, js: 0, frames: 0, last: 0, changed: 0, slow: 0 };
  var renderer = '';
  function still() { return reduce || cut || paused || document.hidden || !onscreen; }
  function uniforms(pr, time, fogRange) {
    gl.useProgram(pr.p);
    var u = pr.u;
    if (u.uVP) gl.uniformMatrix4fv(u.uVP, false, mvp);
    if (u.uDir) gl.uniform3fv(u.uDir, LIGHT.dir);
    if (u.uSun) gl.uniform3fv(u.uSun, LIGHT.sun);
    if (u.uSunCol) gl.uniform3fv(u.uSunCol, LIGHT.sunCol);
    if (u.uSky) gl.uniform3fv(u.uSky, LIGHT.sky);
    if (u.uGround) gl.uniform3fv(u.uGround, LIGHT.ground);
    if (u.uFog) gl.uniform3fv(u.uFog, LIGHT.fog);
    if (u.uZenith) gl.uniform3fv(u.uZenith, LIGHT.zenith);
    if (u.uHorizon) gl.uniform3fv(u.uHorizon, LIGHT.horizon);
    if (u.uNight) gl.uniform1f(u.uNight, LIGHT.night);
    if (u.uStrength) gl.uniform1f(u.uStrength, LIGHT.strength);
    if (u.uExposure) gl.uniform1f(u.uExposure, LIGHT.exposure);
    if (u.uCam) gl.uniform3fv(u.uCam, eye);
    if (u.uFogRange) gl.uniform2fv(u.uFogRange, fogRange);
    if (u.uTime) gl.uniform1f(u.uTime, time);
    if (u.uShadowOn) gl.uniform1f(u.uShadowOn, shadow.on && TIERS[quality.tier].shadows ? 1 : 0);
    if (u.uShadowVP) gl.uniformMatrix4fv(u.uShadowVP, false, shadow.vp);
    if (u.uShadow) { gl.activeTexture(gl.TEXTURE0); gl.bindTexture(gl.TEXTURE_2D, shadow.tex); gl.uniform1i(u.uShadow, 0); }
    if (u.uWater) gl.uniform1f(u.uWater, W.water);
    if (u.uOrigin) {
      gl.uniform2f(u.uOrigin, W.x0, W.z0);
      gl.uniform2f(u.uSize, W.sx, W.sz);
      gl.uniform1f(u.uN, W.n);
    }
    [['uHeight', heightTex, 1], ['uLevel', levelTex, 2], ['uCover', coverTex, 3]].forEach(function (t) {
      if (!u[t[0]]) return;
      gl.activeTexture(gl.TEXTURE0 + t[2]);
      gl.bindTexture(gl.TEXTURE_2D, t[1]);
      gl.uniform1i(u[t[0]], t[2]);
    });
    gl.activeTexture(gl.TEXTURE0);
  }
  // The chunks of a set the eye (or the sun) sees; near or far shape.
  function drawSet(set, pl, lod, pass) {
    set.chunks.forEach(function (b) {
      if (!seen(pl, b)) return;
      var far = set.counts.length > 1 && (pass === 'shadow' ? quality.tier > 0 : boxDistance(eye, b) > lod) ? 1 : 0;
      gl.bindVertexArray(pass === 'shadow' ? b.depth[far] : b.vao[far]);
      gl.drawArraysInstanced(gl.TRIANGLES, 0, set.counts[far], b.count);
    });
  }
  function frame(now) {
    raf = null;
    if (!W) return;
    if (fly) {
      var t = Math.min(1, (now - fly.t0) / fly.ms), e = t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2;
      ['tx', 'ty', 'tz', 'dist'].forEach(function (k) { cam[k] = fly.from[k] + (fly.to[k] - fly.from[k]) * e; });
      if (t >= 1) fly = null;
      dirty = true;
    }
    if (now - lightAt > 60000) { LIGHT = hourLight(new Date()); lightAt = now; dirty = true; }
    var animate = !still();
    if (!dirty && !animate) return;
    var time = animate ? (now - t0) / 1000 : frozenAt, began = performance.now();
    if (animate) frozenAt = time;
    var tier = TIERS[quality.tier];
    matrices();
    var fogRange = [cam.dist * 1.2 + 110, cam.dist * 2.8 + 700];

    // The sun's shadows first.
    var shadows = shadow.on && tier.shadows;
    if (shadows) {
      gl.bindFramebuffer(gl.FRAMEBUFFER, shadow.fb);
      gl.viewport(0, 0, shadow.size, shadow.size);
      gl.clear(gl.DEPTH_BUFFER_BIT);
      gl.enable(gl.DEPTH_TEST);
      gl.enable(gl.POLYGON_OFFSET_FILL);
      gl.polygonOffset(2, 4);
      gl.disable(gl.CULL_FACE);
      gl.useProgram(progs.depth.p);
      gl.uniformMatrix4fv(progs.depth.u.uVP, false, shadow.vp);
      gl.uniform1f(progs.depth.u.uInst, 0);
      gl.bindVertexArray(draws.groundDepth.vao);
      gl.drawElements(gl.TRIANGLES, draws.groundDepth.count, gl.UNSIGNED_INT, 0);
      gl.uniform1f(progs.depth.u.uInst, 1);
      drawSet(draws.conifers, shadow.planes, 0, 'shadow');
      drawSet(draws.broadleaves, shadow.planes, 0, 'shadow');
      drawSet(draws.rocks, shadow.planes, 0, 'shadow');
      gl.disable(gl.POLYGON_OFFSET_FILL);
      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    }

    gl.viewport(0, 0, canvas.width, canvas.height);
    gl.clearColor(LIGHT.fog[0], LIGHT.fog[1], LIGHT.fog[2], 1);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    gl.enable(gl.DEPTH_TEST);
    // The sky, behind everything.
    gl.disable(gl.CULL_FACE);
    gl.depthMask(false);
    uniforms(progs.sky, time, fogRange);
    gl.uniformMatrix4fv(progs.sky.u.uInv, false, invert(mvp));
    gl.bindVertexArray(draws.sky.vao);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
    gl.depthMask(true);
    // The ground, and the land beyond it.
    gl.enable(gl.CULL_FACE);
    gl.cullFace(gl.BACK);
    uniforms(progs.ground, time, fogRange);
    gl.uniform1f(progs.ground.u.uWater, W.water);
    gl.uniform1f(progs.ground.u.uSnow, W.hi - 45);
    gl.bindVertexArray(draws.ground.vao);
    gl.drawElements(gl.TRIANGLES, draws.ground.count, gl.UNSIGNED_INT, 0);
    gl.bindVertexArray(draws.skirt.vao);
    gl.drawElements(gl.TRIANGLES, draws.skirt.count, gl.UNSIGNED_INT, 0);
    // Its trees, boulders and herds.
    uniforms(progs.solid, time, fogRange);
    gl.uniform2f(progs.solid.u.uAt, cam.tx, cam.tz);
    gl.uniform1f(progs.solid.u.uMode, 1);
    drawSet(draws.conifers, planes, tier.lod, 'seen');
    drawSet(draws.broadleaves, planes, tier.lod, 'seen');
    gl.uniform1f(progs.solid.u.uMode, 0);
    drawSet(draws.rocks, planes, tier.lod, 'seen');
    gl.uniform1f(progs.solid.u.uMode, 2);
    drawSet(draws.beasts, planes, tier.lod, 'seen');
    // The grass, thick around what the camera looks at.
    gl.disable(gl.CULL_FACE);
    if (tier.grass > 0 && cam.dist < 170) {
      uniforms(progs.grass, time, fogRange);
      gl.uniform2f(progs.grass.u.uAt, cam.tx + (eye[0] - cam.tx) * 0.35, cam.tz + (eye[2] - cam.tz) * 0.35);
      gl.uniform1f(progs.grass.u.uSide, tier.grass);
      gl.uniform1f(progs.grass.u.uStep, 0.72);
      gl.bindVertexArray(draws.grass.vao);
      gl.drawArraysInstanced(gl.TRIANGLES, 0, draws.grass.count, tier.grass * tier.grass);
    }
    // The birds.
    uniforms(progs.bird, time, fogRange);
    gl.uniform3fv(progs.bird.u.uCentre, [cam.tx, cam.ty, cam.tz]);
    gl.bindVertexArray(draws.birds.vao);
    gl.drawArraysInstanced(gl.TRIANGLES, 0, draws.birds.count, draws.birds.instances);
    // The water, over its beds, the pools and the lake.
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
    gl.depthMask(false);
    uniforms(progs.water, time, fogRange);
    gl.bindVertexArray(draws.lake.vao);
    gl.drawElements(gl.TRIANGLES, draws.lake.count, gl.UNSIGNED_INT, 0);
    if (draws.water.count) {
      gl.bindVertexArray(draws.water.vao);
      gl.drawElements(gl.TRIANGLES, draws.water.count, gl.UNSIGNED_INT, 0);
    }
    // The people's lights.
    gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
    uniforms(progs.marks, time, fogRange);
    var f = W.byId[centre];
    gl.uniform1f(progs.marks.u.uFocus, f ? f.index : -1);
    gl.bindVertexArray(draws.marks.vao);
    gl.drawArraysInstanced(gl.TRIANGLES, 0, draws.marks.count, draws.marks.instances);
    gl.depthMask(true);
    gl.disable(gl.BLEND);
    gl.bindVertexArray(null);
    drawLabels();
    dirty = false;
    perf.js = perf.js * 0.9 + (performance.now() - began) * 0.1;

    // How fast the frames come; the detail lowered when they come slowly,
    // raised again when there is room.
    if (perf.last) {
      var dt = now - perf.last;
      if (dt < 500) perf.ema = perf.ema * 0.92 + dt * 0.08;
      perf.frames++;
      if (fixedTier === null && animate && perf.frames > 60 && now - perf.changed > 4000) {
        if (perf.ema > 28 && quality.tier < 3) { setTier(quality.tier + 1); perf.changed = now; perf.slow = now; }
        else if (perf.ema < 17.5 && quality.tier > 0 && now - perf.slow > 20000) { setTier(quality.tier - 1); perf.changed = now; }
      }
      if (meter) {
        meter.textContent = perf.ema.toFixed(1) + ' ms · ' + Math.round(1000 / perf.ema) + ' fps · js ' + perf.js.toFixed(1) + ' ms · q' + quality.tier + ' · ' +
          canvas.width + '×' + canvas.height + (renderer ? ' · ' + renderer : '');
      }
    }
    perf.last = animate ? now : 0;
    if (fly || animate) wake();
  }
  function setTier(t) {
    quality.tier = t;
    perf.frames = 0;
    resize();
  }
  function wake() {
    if (!raf) raf = requestAnimationFrame(frame);
  }

  /* ---- moving in it ---- */
  var drag = null;
  function clampTarget() {
    cam.tx = Math.max(W.x0, Math.min(W.x0 + W.sx, cam.tx));
    cam.tz = Math.max(W.z0, Math.min(W.z0 + W.sz, cam.tz));
    cam.ty = Math.max(heightAt(cam.tx, cam.tz), W.water);
  }
  canvas.addEventListener('pointerdown', function (e) {
    if (!W) return;
    drag = { x: e.clientX, y: e.clientY, yaw: cam.yaw, pitch: cam.pitch, tx: cam.tx, tz: cam.tz, pan: e.button === 2 || e.shiftKey, moved: 0 };
    canvas.setPointerCapture(e.pointerId);
  });
  canvas.addEventListener('pointermove', function (e) {
    if (!drag) return;
    var mx = e.clientX - drag.x, my = e.clientY - drag.y;
    drag.moved = Math.max(drag.moved, Math.abs(mx) + Math.abs(my));
    if (drag.pan) {
      var k = cam.dist / size.h * 1.6, s = Math.sin(cam.yaw), c = Math.cos(cam.yaw);
      cam.tx = drag.tx - (mx * c - my * s) * k;
      cam.tz = drag.tz + (mx * s + my * c) * k;
      clampTarget();
    } else {
      cam.yaw = drag.yaw - mx * 0.006;
      cam.pitch = Math.max(0.08, Math.min(1.42, drag.pitch + my * 0.005));
    }
    fly = null;
    dirty = true;
    wake();
  });
  canvas.addEventListener('pointerup', function (e) {
    var d = drag;
    drag = null;
    if (!d || d.moved > 5) return;
    // A tap on a person's light travels to them, as a tap on the river does.
    var r = canvas.getBoundingClientRect(), x = e.clientX - r.left, y = e.clientY - r.top, best = null, bd = 26;
    W.persons.forEach(function (p) {
      var s = project(p.x, marksY[p.index], p.z);
      if (!s) return;
      var dd = Math.hypot(s.x - x, s.y - y);
      if (dd < bd) { bd = dd; best = p; }
    });
    if (best && best.id !== centre && window.axgfRiver) window.axgfRiver.travel(best.id);
  });
  canvas.addEventListener('contextmenu', function (e) { e.preventDefault(); });
  canvas.addEventListener('wheel', function (e) {
    e.preventDefault();
    if (!W) return;
    cam.dist = Math.max(12, Math.min(MAX_DIST, cam.dist * Math.exp(e.deltaY * 0.0012)));
    fly = null;
    dirty = true;
    wake();
  }, { passive: false });
  canvas.addEventListener('keydown', function (e) {
    if (!W) return;
    var step = cam.dist * 0.08, s = Math.sin(cam.yaw), c = Math.cos(cam.yaw), moved = true;
    if (e.key === 'ArrowUp') { cam.tx -= s * step; cam.tz -= c * step; }
    else if (e.key === 'ArrowDown') { cam.tx += s * step; cam.tz += c * step; }
    else if (e.key === 'ArrowLeft') { cam.tx -= c * step; cam.tz += s * step; }
    else if (e.key === 'ArrowRight') { cam.tx += c * step; cam.tz -= s * step; }
    else if (e.key === '+' || e.key === '=') cam.dist = Math.max(12, cam.dist * 0.85);
    else if (e.key === '-') cam.dist = Math.min(MAX_DIST, cam.dist * 1.18);
    else moved = false;
    if (!moved) return;
    e.preventDefault();
    clampTarget();
    fly = null;
    dirty = true;
    wake();
  });

  /* ---- the river's own controls ---- */
  var btn = root.querySelector('[data-rv-still]');
  if (btn && !reduce && !cut) {
    btn.hidden = false;
    btn.setAttribute('aria-pressed', paused ? 'true' : 'false');
    btn.addEventListener('click', function () {
      paused = !paused;
      btn.setAttribute('aria-pressed', paused ? 'true' : 'false');
      try { localStorage.setItem(STORE, paused ? '1' : '0'); } catch (e) { /* private window */ }
      dirty = true;
      wake();
    });
  }
  // The river's zoom brings the camera closer, and "all names" writes more
  // of them (river.js binds the same controls for its own drawing).
  var zoom = root.querySelector('[data-rv-zoom]');
  if (zoom) {
    root.querySelectorAll('[data-rv-zoom-wrap]').forEach(function (w) { w.hidden = false; });
    zoom.addEventListener('input', function () {
      if (!W) return;
      cam.dist = Math.max(12, Math.min(MAX_DIST, 130 / Math.max(0.2, parseFloat(zoom.value) || 1)));
      fly = null;
      dirty = true;
      wake();
    });
  }
  var namesBtn = root.querySelector('[data-rv-names]');
  if (namesBtn) {
    namesBtn.hidden = false;
    namesBtn.addEventListener('click', function () {
      allNames = namesBtn.getAttribute('aria-pressed') !== 'true';
      namesBtn.setAttribute('aria-pressed', allNames ? 'true' : 'false');
      dirty = true;
      wake();
    });
  }
  document.addEventListener('visibilitychange', function () { dirty = true; perf.last = 0; wake(); });
  if (window.IntersectionObserver) {
    new IntersectionObserver(function (es) { onscreen = es[0].isIntersecting; dirty = true; perf.last = 0; wake(); }).observe(canvas);
  }
  if (window.ResizeObserver) new ResizeObserver(resize).observe(stage);
  else window.addEventListener('resize', resize);

  // The river travels: the camera follows (river.js calls this).
  window.axgfRiverWorld = {
    focus: function (id) {
      centre = id;
      if (!W) return;
      var p = W.byId[id];
      if (p) focusOn(p, 1300);
      else { dirty = true; wake(); }
    }
  };

  /* ---- reading it ---- */
  LIGHT = hourLight(new Date());
  lightAt = performance.now();
  Promise.all([
    fetch(root.getAttribute('data-world'), { credentials: 'same-origin' }).then(function (r) { if (!r.ok) throw new Error('world ' + r.status); return r.arrayBuffer(); }),
    fetch('/canopee/words.json', { credentials: 'same-origin' }).then(function (r) { return r.ok ? r.json() : { people: {} }; }).catch(function () { return { people: {} }; })
  ]).then(function (got) {
    var started = performance.now();
    W = parse(got[0]);
    words = got[1].people || {};
    build();
    if (meter) {
      var info = gl.getExtension('WEBGL_debug_renderer_info');
      var r = (info && gl.getParameter(info.UNMASKED_RENDERER_WEBGL)) || gl.getParameter(gl.RENDERER) || '';
      renderer = String(r).replace(/^ANGLE \(|\)$/g, '').slice(0, 70);
      meter.title = 'built in ' + Math.round(performance.now() - started) + ' ms';
    }
    resize();
    var p = W.byId[centre] || W.persons[0];
    if (p) focusOn(p, 0);
    root.classList.add('rv-world-ready');
    wake();
  }).catch(function (e) {
    if (window.console) console.warn('Canopée world:', e && e.message ? e.message : e);
    fallBack();
  });
})();
