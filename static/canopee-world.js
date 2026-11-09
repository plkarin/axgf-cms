/* Canopée's world (docs/adr/0043): the family's land, in 3D, one scene.
 *
 * The land, its water, its trees, rocks and herds, the people's marks and
 * the sky are one WebGL 2 scene, read from /canopee/world.bin — built by the
 * server once for each shape of the family and kept by the browser under its
 * fingerprint. The page builds its buffers once; after that only the camera
 * moves. Choosing a person (here, in the itinerary, in the search) flies the
 * camera to them, and the river travels there as it always has, its record
 * beside it.
 *
 * Decoration over the river's own data: without WebGL 2, or if the world
 * cannot be read, the river and its photograph stay as they were
 * (river-scene.js). prefers-reduced-motion, "Cut" and the river's pause
 * button hold the water, the wind and the herds still and the camera jumps
 * instead of flying; nothing is drawn while the river is off screen or the
 * tab is hidden.
 */
(function () {
  'use strict';
  var root = document.querySelector('.river[data-world]');
  var stage = document.getElementById('rv-canvas');
  if (!root || !stage) return;
  var canvas = document.createElement('canvas');
  var gl = canvas.getContext('webgl2', { antialias: true, alpha: false, powerPreference: 'high-performance' });
  if (!gl) return;

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

  var reduce = window.matchMedia && matchMedia('(prefers-reduced-motion: reduce)').matches;
  var cut = root.dataset.riverMotion === 'cut';
  var STORE = 'axgf-river-still';
  var paused = false;
  try { paused = localStorage.getItem(STORE) === '1'; } catch (e) { /* private window */ }

  function fallBack() {
    root.classList.remove('rv-world-on');
    canvas.remove();
    labels.remove();
    var src = root.getAttribute('data-world-fallback');
    if (src) {
      var s = document.createElement('script');
      s.src = src;
      document.body.appendChild(s);
    }
  }

  /* ---- the world, from its bytes ---- */
  function parse(buf) {
    var dv = new DataView(buf), o = 0;
    function u8() { return dv.getUint8(o++); }
    function u16() { var v = dv.getUint16(o, true); o += 2; return v; }
    function u32() { var v = dv.getUint32(o, true); o += 4; return v; }
    function f32() { var v = dv.getFloat32(o, true); o += 4; return v; }
    if (String.fromCharCode(u8(), u8(), u8(), u8()) !== 'AXW1') throw new Error('not a world');
    var w = { n: u32(), x0: f32(), z0: f32(), sx: f32(), sz: f32(), water: f32() };
    var lo = f32(), hi = f32(), n = w.n, i, k;
    w.h = new Float32Array(n * n);
    for (i = 0; i < n * n; i++) w.h[i] = lo + (hi - lo) * u16() / 65535;
    w.hi = hi;
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
    var ns = u32();
    w.streams = [];
    for (i = 0; i < ns; i++) {
      var from = u16(), to = u16(), m = u16(), pts = new Float32Array(m * 4);
      for (k = 0; k < m * 4; k++) pts[k] = f32();
      w.streams.push({ from: from, to: to, pts: pts });
    }
    function list(stride) {
      var c = u32(), a = new Float32Array(c * stride);
      for (var j = 0; j < c * stride; j++) a[j] = f32();
      return a;
    }
    w.trees = list(5);
    w.rocks = list(4);
    w.herds = list(4);
    return w;
  }

  /* ---- a little linear algebra ---- */
  function perspective(fovy, aspect, near, far) {
    var f = 1 / Math.tan(fovy / 2), nf = 1 / (near - far);
    return [f / aspect, 0, 0, 0, 0, f, 0, 0, 0, 0, (far + near) * nf, -1, 0, 0, 2 * far * near * nf, 0];
  }
  function lookAt(e, c, up) {
    var zx = e[0] - c[0], zy = e[1] - c[1], zz = e[2] - c[2];
    var l = Math.hypot(zx, zy, zz); zx /= l; zy /= l; zz /= l;
    var xx = up[1] * zz - up[2] * zy, xy = up[2] * zx - up[0] * zz, xz = up[0] * zy - up[1] * zx;
    l = Math.hypot(xx, xy, xz); xx /= l; xy /= l; xz /= l;
    var yx = zy * xz - zz * xy, yy = zz * xx - zx * xz, yz = zx * xy - zy * xx;
    return [xx, yx, zx, 0, xy, yy, zy, 0, xz, yz, zz, 0,
      -(xx * e[0] + xy * e[1] + xz * e[2]), -(yx * e[0] + yy * e[1] + yz * e[2]), -(zx * e[0] + zy * e[1] + zz * e[2]), 1];
  }
  function mul(a, b) {
    var o = new Array(16);
    for (var c = 0; c < 4; c++) for (var r = 0; r < 4; r++) {
      o[c * 4 + r] = a[r] * b[c * 4] + a[4 + r] * b[c * 4 + 1] + a[8 + r] * b[c * 4 + 2] + a[12 + r] * b[c * 4 + 3];
    }
    return o;
  }
  function invert(m) {
    var a = m, inv = new Array(16), det;
    inv[0] = a[5] * a[10] * a[15] - a[5] * a[11] * a[14] - a[9] * a[6] * a[15] + a[9] * a[7] * a[14] + a[13] * a[6] * a[11] - a[13] * a[7] * a[10];
    inv[4] = -a[4] * a[10] * a[15] + a[4] * a[11] * a[14] + a[8] * a[6] * a[15] - a[8] * a[7] * a[14] - a[12] * a[6] * a[11] + a[12] * a[7] * a[10];
    inv[8] = a[4] * a[9] * a[15] - a[4] * a[11] * a[13] - a[8] * a[5] * a[15] + a[8] * a[7] * a[13] + a[12] * a[5] * a[11] - a[12] * a[7] * a[9];
    inv[12] = -a[4] * a[9] * a[14] + a[4] * a[10] * a[13] + a[8] * a[5] * a[14] - a[8] * a[6] * a[13] - a[12] * a[5] * a[10] + a[12] * a[6] * a[9];
    inv[1] = -a[1] * a[10] * a[15] + a[1] * a[11] * a[14] + a[9] * a[2] * a[15] - a[9] * a[3] * a[14] - a[13] * a[2] * a[11] + a[13] * a[3] * a[10];
    inv[5] = a[0] * a[10] * a[15] - a[0] * a[11] * a[14] - a[8] * a[2] * a[15] + a[8] * a[3] * a[14] + a[12] * a[2] * a[11] - a[12] * a[3] * a[10];
    inv[9] = -a[0] * a[9] * a[15] + a[0] * a[11] * a[13] + a[8] * a[1] * a[15] - a[8] * a[3] * a[13] - a[12] * a[1] * a[11] + a[12] * a[3] * a[9];
    inv[13] = a[0] * a[9] * a[14] - a[0] * a[10] * a[13] - a[8] * a[1] * a[14] + a[8] * a[2] * a[13] + a[12] * a[1] * a[10] - a[12] * a[2] * a[9];
    inv[2] = a[1] * a[6] * a[15] - a[1] * a[7] * a[14] - a[5] * a[2] * a[15] + a[5] * a[3] * a[14] + a[13] * a[2] * a[7] - a[13] * a[3] * a[6];
    inv[6] = -a[0] * a[6] * a[15] + a[0] * a[7] * a[14] + a[4] * a[2] * a[15] - a[4] * a[3] * a[14] - a[12] * a[2] * a[7] + a[12] * a[3] * a[6];
    inv[10] = a[0] * a[5] * a[15] - a[0] * a[7] * a[13] - a[4] * a[1] * a[15] + a[4] * a[3] * a[13] + a[12] * a[1] * a[7] - a[12] * a[3] * a[5];
    inv[14] = -a[0] * a[5] * a[14] + a[0] * a[6] * a[13] + a[4] * a[1] * a[14] - a[4] * a[2] * a[13] - a[12] * a[1] * a[6] + a[12] * a[2] * a[5];
    inv[3] = -a[1] * a[6] * a[11] + a[1] * a[7] * a[10] + a[5] * a[2] * a[11] - a[5] * a[3] * a[10] - a[9] * a[2] * a[7] + a[9] * a[3] * a[6];
    inv[7] = a[0] * a[6] * a[11] - a[0] * a[7] * a[10] - a[4] * a[2] * a[11] + a[4] * a[3] * a[10] + a[8] * a[2] * a[7] - a[8] * a[3] * a[6];
    inv[11] = -a[0] * a[5] * a[11] + a[0] * a[7] * a[9] + a[4] * a[1] * a[11] - a[4] * a[3] * a[9] - a[8] * a[1] * a[7] + a[8] * a[3] * a[5];
    inv[15] = a[0] * a[5] * a[10] - a[0] * a[6] * a[9] - a[4] * a[1] * a[10] + a[4] * a[2] * a[9] + a[8] * a[1] * a[6] - a[8] * a[2] * a[5];
    det = a[0] * inv[0] + a[1] * inv[4] + a[2] * inv[8] + a[3] * inv[12];
    det = det ? 1 / det : 0;
    for (var i = 0; i < 16; i++) inv[i] *= det;
    return inv;
  }

  /* ---- shaders ---- */
  var COMMON = '#version 300 es\nprecision highp float;\n';
  var LIGHT = [
    'uniform vec3 uSun; uniform vec3 uCam; uniform vec3 uFog; uniform vec2 uFogRange;',
    'vec3 lit(vec3 col, vec3 n) {',
    '  float d = max(dot(n, uSun), 0.0);',
    '  vec3 sky = mix(vec3(0.20,0.18,0.14), vec3(0.45,0.52,0.60), n.y * 0.5 + 0.5);',
    '  return col * (vec3(1.0,0.90,0.74) * d * 1.15 + sky * 0.6);',
    '}',
    'vec3 fogged(vec3 col, vec3 p) {',
    '  float f = smoothstep(uFogRange.x, uFogRange.y, distance(uCam, p));',
    '  return mix(col, uFog, f);',
    '}',
    'float hash(vec2 p) { return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453); }',
    'float vnoise(vec2 p) { vec2 i = floor(p), f = fract(p); f = f * f * (3.0 - 2.0 * f);',
    '  return mix(mix(hash(i), hash(i + vec2(1,0)), f.x), mix(hash(i + vec2(0,1)), hash(i + vec2(1,1)), f.x), f.y); }'
  ].join('\n') + '\n';
  var SRC = {
    sky: {
      vs: COMMON + 'out vec2 vNdc; void main() { vec2 p = vec2(float((gl_VertexID << 1) & 2), float(gl_VertexID & 2)) * 2.0 - 1.0; vNdc = p; gl_Position = vec4(p, 0.9999, 1.0); }',
      fs: COMMON + 'in vec2 vNdc; uniform mat4 uInv; uniform vec3 uCam; uniform vec3 uSun; uniform vec3 uFog; out vec4 o;\n' +
        'void main() { vec4 q = uInv * vec4(vNdc, 1.0, 1.0); vec3 d = normalize(q.xyz / q.w - uCam);\n' +
        '  vec3 zen = vec3(0.36, 0.52, 0.68), hor = uFog; vec3 c = mix(hor, zen, smoothstep(-0.02, 0.45, d.y));\n' +
        '  float s = max(dot(d, uSun), 0.0); c += vec3(1.0, 0.78, 0.45) * (pow(s, 64.0) * 1.4 + pow(s, 6.0) * 0.22);\n' +
        '  o = vec4(c, 1.0); }'
    },
    ground: {
      vs: COMMON + 'in vec3 aPos; in vec3 aNor; uniform mat4 uVP; out vec3 vPos; out vec3 vNor; void main() { vPos = aPos; vNor = aNor; gl_Position = uVP * vec4(aPos, 1.0); }',
      fs: COMMON + LIGHT + 'in vec3 vPos; in vec3 vNor; uniform float uWater; uniform float uSnow; out vec4 o;\n' +
        'void main() { vec3 n = normalize(vNor); float slope = 1.0 - n.y;\n' +
        '  float g = vnoise(vPos.xz * 0.07) * 0.6 + vnoise(vPos.xz * 0.45) * 0.4;\n' +
        '  vec3 meadow = mix(vec3(0.40, 0.45, 0.19), vec3(0.66, 0.60, 0.28), g);\n' +
        '  vec3 forest = mix(vec3(0.12, 0.20, 0.10), vec3(0.22, 0.30, 0.13), g);\n' +
        '  vec3 rock = mix(vec3(0.40, 0.38, 0.35), vec3(0.62, 0.60, 0.55), g);\n' +
        '  vec3 c = mix(meadow, forest, smoothstep(0.06, 0.22, slope + (g - 0.5) * 0.12));\n' +
        '  c = mix(c, rock, smoothstep(0.30, 0.52, slope));\n' +
        '  c = mix(c, vec3(0.72, 0.66, 0.52), 1.0 - smoothstep(uWater + 0.3, uWater + 1.8, vPos.y));\n' +
        '  c = mix(c, vec3(0.93, 0.95, 0.97), smoothstep(uSnow, uSnow + 14.0, vPos.y + g * 8.0) * (1.0 - smoothstep(0.55, 0.8, slope)));\n' +
        '  o = vec4(fogged(lit(c, n), vPos), 1.0); }'
    },
    water: {
      vs: COMMON + 'in vec3 aPos; in vec2 aUV; uniform mat4 uVP; out vec3 vPos; out vec2 vUV; void main() { vPos = aPos; vUV = aUV; gl_Position = uVP * vec4(aPos, 1.0); }',
      fs: COMMON + LIGHT + 'in vec3 vPos; in vec2 vUV; uniform float uTime; uniform float uLake; out vec4 o;\n' +
        'void main() { vec2 uv = uLake > 0.5 ? vPos.xz * 0.15 : vec2(vUV.x * 1.5, vUV.y * 0.6 - uTime * 0.9);\n' +
        '  float r = vnoise(uv * 3.0 + uTime * 0.15) * 0.5 + vnoise(uv * 7.0 - uTime * 0.2) * 0.5;\n' +
        '  vec3 n = normalize(vec3((r - 0.5) * 0.35, 1.0, (vnoise(uv * 5.0) - 0.5) * 0.35));\n' +
        '  vec3 v = normalize(uCam - vPos); float fr = pow(1.0 - max(dot(n, v), 0.0), 3.0);\n' +
        '  vec3 deep = vec3(0.06, 0.20, 0.22), shine = vec3(0.55, 0.66, 0.66);\n' +
        '  vec3 c = mix(deep, shine, fr * 0.8 + 0.12);\n' +
        '  vec3 h = normalize(uSun + v); c += vec3(1.0, 0.85, 0.6) * pow(max(dot(n, h), 0.0), 90.0) * 1.2;\n' +
        '  float foam = uLake > 0.5 ? 0.0 : smoothstep(0.72, 0.98, abs(vUV.x)) * 0.5 + smoothstep(0.82, 1.0, r) * 0.25;\n' +
        '  c = mix(c, vec3(0.88, 0.92, 0.9), foam);\n' +
        '  o = vec4(fogged(c, vPos), uLake > 0.5 ? 0.88 : 0.92); }'
    },
    solid: {
      vs: COMMON + 'in vec3 aPos; in vec3 aNor; in vec3 aCol; in vec4 aInst; in float aKind;\n' +
        'uniform mat4 uVP; uniform float uTime; uniform float uMode; out vec3 vPos; out vec3 vNor; out vec3 vCol;\n' +
        'void main() { float s = aInst.w; vec3 p = aPos * s; vec3 nn = aNor;\n' +
        '  float yaw = fract(sin(aInst.x * 12.9898 + aInst.z * 78.233) * 43758.5453) * 6.2831;\n' +
        '  vec3 base = aInst.xyz;\n' +
        '  if (uMode > 1.5) { float ph = yaw; float t = uTime * 0.04 + ph; vec2 off = vec2(cos(t), sin(t * 1.3)) * (3.0 + aKind);\n' +
        '    vec2 dir = normalize(vec2(-sin(t), 1.3 * cos(t * 1.3)) + 1e-4); yaw = atan(dir.x, dir.y); base.xz += off; }\n' +
        '  float c = cos(yaw), si = sin(yaw); p = vec3(c * p.x + si * p.z, p.y, -si * p.x + c * p.z); nn = vec3(c * nn.x + si * nn.z, nn.y, -si * nn.x + c * nn.z);\n' +
        '  if (uMode > 0.5 && uMode < 1.5) { p.x += sin(uTime * 1.2 + base.x * 0.07 + base.z * 0.05) * 0.06 * max(aPos.y - 1.0, 0.0) * s; }\n' +
        '  vPos = base + p; vNor = nn; vCol = aCol; gl_Position = uVP * vec4(vPos, 1.0); }',
      fs: COMMON + LIGHT + 'in vec3 vPos; in vec3 vNor; in vec3 vCol; out vec4 o;\n' +
        'void main() { float g = vnoise(vPos.xz * 0.9 + vPos.y) * 0.18 + 0.91; o = vec4(fogged(lit(vCol * g, normalize(vNor)), vPos), 1.0); }'
    },
    marks: {
      vs: COMMON + 'in vec3 aPos; in float aSize; in float aIdx; uniform mat4 uVP; uniform float uFocus; uniform float uScale; out float vFocus;\n' +
        'void main() { vec4 p = uVP * vec4(aPos + vec3(0.0, 0.6, 0.0), 1.0); gl_Position = p; vFocus = abs(aIdx - uFocus) < 0.5 ? 1.0 : 0.0;\n' +
        '  gl_PointSize = clamp(aSize * (1.0 + vFocus) * uScale / p.w, 6.0, 64.0); }',
      fs: COMMON + 'in float vFocus; out vec4 o; void main() { float d = length(gl_PointCoord - 0.5) * 2.0;\n' +
        '  float ring = smoothstep(1.0, 0.86, d) * smoothstep(0.52, 0.66, d); float core = smoothstep(0.42, 0.2, d);\n' +
        '  vec3 c = mix(vec3(1.0, 0.95, 0.82), vec3(1.0, 0.78, 0.32), vFocus); float a = max(ring, core * (0.55 + 0.45 * vFocus));\n' +
        '  if (a < 0.02) discard; o = vec4(c, a); }'
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
    var u = {}, n = gl.getProgramParameter(p, gl.ACTIVE_UNIFORMS);
    for (var i = 0; i < n; i++) {
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
  function attrib(prog, name, buf, size, stride, offset, divisor) {
    var loc = prog.a(name);
    if (loc < 0) return;
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    gl.enableVertexAttribArray(loc);
    gl.vertexAttribPointer(loc, size, gl.FLOAT, false, stride || 0, offset || 0);
    gl.vertexAttribDivisor(loc, divisor || 0);
  }

  /* ---- meshes for the instances: positions, normals, colours ---- */
  function mesh() { return { p: [], n: [], c: [] }; }
  function tri(m, a, b, c, col) {
    var ux = b[0] - a[0], uy = b[1] - a[1], uz = b[2] - a[2], vx = c[0] - a[0], vy = c[1] - a[1], vz = c[2] - a[2];
    var nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx, l = Math.hypot(nx, ny, nz) || 1;
    [a, b, c].forEach(function (v) { m.p.push(v[0], v[1], v[2]); m.n.push(nx / l, ny / l, nz / l); m.c.push(col[0], col[1], col[2]); });
  }
  function cone(m, y0, y1, r, sides, col) {
    for (var i = 0; i < sides; i++) {
      var a0 = i / sides * Math.PI * 2, a1 = (i + 1) / sides * Math.PI * 2;
      var p0 = [Math.cos(a0) * r, y0, Math.sin(a0) * r], p1 = [Math.cos(a1) * r, y0, Math.sin(a1) * r];
      tri(m, p0, [0, y1, 0], p1, col);
      tri(m, p0, p1, [0, y0, 0], col.map(function (v) { return v * 0.7; }));
    }
  }
  function prism(m, y0, y1, r, sides, col) {
    for (var i = 0; i < sides; i++) {
      var a0 = i / sides * Math.PI * 2, a1 = (i + 1) / sides * Math.PI * 2;
      var b0 = [Math.cos(a0) * r, y0, Math.sin(a0) * r], b1 = [Math.cos(a1) * r, y0, Math.sin(a1) * r];
      var t0 = [b0[0], y1, b0[2]], t1 = [b1[0], y1, b1[2]];
      tri(m, b0, t0, b1, col);
      tri(m, b1, t0, t1, col);
    }
  }
  function blob(m, cy, r, col, squash) {
    // An icosahedron: foliage, a boulder, a body.
    var t = (1 + Math.sqrt(5)) / 2;
    var v = [[-1, t, 0], [1, t, 0], [-1, -t, 0], [1, -t, 0], [0, -1, t], [0, 1, t], [0, -1, -t], [0, 1, -t], [t, 0, -1], [t, 0, 1], [-t, 0, -1], [-t, 0, 1]];
    var f = [[0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11], [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8],
      [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9], [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1]];
    var s = r / Math.hypot(1, t);
    var p = v.map(function (q) { return [q[0] * s * (squash ? squash[0] : 1), cy + q[1] * s * (squash ? squash[1] : 1), q[2] * s * (squash ? squash[2] : 1)]; });
    f.forEach(function (q) { tri(m, p[q[0]], p[q[1]], p[q[2]], col); });
  }
  function box(m, x, y, z, w, h, d, col) {
    var X = [x - w / 2, x + w / 2], Y = [y, y + h], Z = [z - d / 2, z + d / 2];
    var c = function (i, j, k) { return [X[i], Y[j], Z[k]]; };
    var quads = [[c(0, 0, 1), c(1, 0, 1), c(1, 1, 1), c(0, 1, 1)], [c(1, 0, 0), c(0, 0, 0), c(0, 1, 0), c(1, 1, 0)],
      [c(0, 0, 0), c(0, 0, 1), c(0, 1, 1), c(0, 1, 0)], [c(1, 0, 1), c(1, 0, 0), c(1, 1, 0), c(1, 1, 1)],
      [c(0, 1, 1), c(1, 1, 1), c(1, 1, 0), c(0, 1, 0)]];
    quads.forEach(function (q) { tri(m, q[0], q[1], q[2], col); tri(m, q[0], q[2], q[3], col); });
  }
  var TRUNK = [0.30, 0.20, 0.12];
  function conifer() {
    var m = mesh();
    prism(m, 0, 1.4, 0.18, 5, TRUNK);
    cone(m, 1.0, 4.2, 1.5, 7, [0.10, 0.25, 0.13]);
    cone(m, 2.6, 6.0, 1.05, 7, [0.13, 0.30, 0.15]);
    return m;
  }
  function broadleaf() {
    var m = mesh();
    prism(m, 0, 1.8, 0.22, 5, TRUNK);
    blob(m, 3.0, 2.0, [0.30, 0.42, 0.14]);
    blob(m, 4.1, 1.3, [0.48, 0.50, 0.16]);
    return m;
  }
  function boulder() {
    var m = mesh();
    blob(m, 0.25, 0.9, [0.52, 0.50, 0.46], [1.2, 0.7, 1.0]);
    return m;
  }
  function beast() {
    // A grazing animal, small and plain: a body, a neck, a head, four legs.
    var m = mesh(), fur = [0.52, 0.38, 0.24], dark = [0.25, 0.18, 0.12];
    box(m, 0, 0.55, 0, 0.42, 0.42, 1.0, fur);
    box(m, 0, 0.8, 0.55, 0.2, 0.32, 0.22, fur);
    box(m, 0, 0.95, 0.72, 0.18, 0.18, 0.3, dark);
    [[-0.15, -0.38], [0.15, -0.38], [-0.15, 0.38], [0.15, 0.38]].forEach(function (l) { box(m, l[0], 0, l[1], 0.09, 0.56, 0.09, dark); });
    return m;
  }

  /* ---- the scene, built once ---- */
  var W, progs = {}, draws = {}, centre = root.getAttribute('data-river-centre');
  var cam = { tx: 0, ty: 0, tz: 0, yaw: Math.PI, pitch: 0.66, dist: 72 };
  var MAX_DIST = 320;
  var fly = null, words = {}, mvp = null, view = null, eye = [0, 0, 0];

  function heightAt(x, z) {
    var n = W.n, fx = Math.max(0, Math.min(n - 1.001, (x - W.x0) / W.sx * (n - 1)));
    var fz = Math.max(0, Math.min(n - 1.001, (z - W.z0) / W.sz * (n - 1)));
    var i = Math.floor(fx), j = Math.floor(fz), tx = fx - i, tz = fz - j, h = W.h;
    var a = h[j * n + i] + (h[j * n + i + 1] - h[j * n + i]) * tx;
    var b = h[(j + 1) * n + i] + (h[(j + 1) * n + i + 1] - h[(j + 1) * n + i]) * tx;
    return a + (b - a) * tz;
  }

  function build() {
    var n = W.n, i, j, k;
    progs.sky = program(SRC.sky);
    progs.ground = program(SRC.ground);
    progs.water = program(SRC.water);
    progs.solid = program(SRC.solid);
    progs.marks = program(SRC.marks);

    // The ground: a grid, its normals from its heights.
    var pos = new Float32Array(n * n * 3), nor = new Float32Array(n * n * 3);
    var dx = W.sx / (n - 1), dz = W.sz / (n - 1);
    for (j = 0; j < n; j++) for (i = 0; i < n; i++) {
      k = j * n + i;
      pos[k * 3] = W.x0 + i * dx; pos[k * 3 + 1] = W.h[k]; pos[k * 3 + 2] = W.z0 + j * dz;
      var hl = W.h[j * n + Math.max(0, i - 1)], hr = W.h[j * n + Math.min(n - 1, i + 1)];
      var hd = W.h[Math.max(0, j - 1) * n + i], hu = W.h[Math.min(n - 1, j + 1) * n + i];
      var nx = (hl - hr) / (2 * dx), nz = (hd - hu) / (2 * dz), l = Math.hypot(nx, 1, nz);
      nor[k * 3] = nx / l; nor[k * 3 + 1] = 1 / l; nor[k * 3 + 2] = nz / l;
    }
    var idx = new Uint32Array((n - 1) * (n - 1) * 6), q = 0;
    for (j = 0; j < n - 1; j++) for (i = 0; i < n - 1; i++) {
      var a = j * n + i, b = a + 1, c = a + n, d = c + 1;
      idx[q++] = a; idx[q++] = c; idx[q++] = b; idx[q++] = b; idx[q++] = c; idx[q++] = d;
    }
    var g = gl.createVertexArray(); gl.bindVertexArray(g);
    attrib(progs.ground, 'aPos', buffer(pos), 3);
    attrib(progs.ground, 'aNor', buffer(nor), 3);
    buffer(idx, gl.ELEMENT_ARRAY_BUFFER);
    draws.ground = { vao: g, count: idx.length };

    // The water: a ribbon along each stream, and the lake.
    var wp = [], wu = [], wi = [];
    W.streams.forEach(function (s) {
      var pts = s.pts, m = pts.length / 4, base = wp.length / 3;
      for (var t = 0; t < m; t++) {
        var x = pts[t * 4], z = pts[t * 4 + 1], y = pts[t * 4 + 2] + 0.15, w = pts[t * 4 + 3];
        var t0 = Math.max(0, t - 1), t1 = Math.min(m - 1, t + 1);
        var ex = pts[t1 * 4] - pts[t0 * 4], ez = pts[t1 * 4 + 1] - pts[t0 * 4 + 1], el = Math.hypot(ex, ez) || 1;
        var px = -ez / el * w, pz = ex / el * w;
        wp.push(x + px, y, z + pz, x - px, y, z - pz);
        wu.push(-1, t * 1.6, 1, t * 1.6);
        if (t < m - 1) { var v = base + t * 2; wi.push(v, v + 1, v + 2, v + 1, v + 3, v + 2); }
      }
    });
    var wv = gl.createVertexArray(); gl.bindVertexArray(wv);
    attrib(progs.water, 'aPos', buffer(new Float32Array(wp)), 3);
    attrib(progs.water, 'aUV', buffer(new Float32Array(wu)), 2);
    buffer(new Uint32Array(wi), gl.ELEMENT_ARRAY_BUFFER);
    draws.streams = { vao: wv, count: wi.length };
    var L = W.water, X0 = W.x0 - 3000, Z0 = W.z0 - 3000, X1 = W.x0 + W.sx + 3000, Z1 = W.z0 + W.sz + 3000;
    var lv = gl.createVertexArray(); gl.bindVertexArray(lv);
    attrib(progs.water, 'aPos', buffer(new Float32Array([X0, L, Z0, X1, L, Z0, X0, L, Z1, X1, L, Z1])), 3);
    attrib(progs.water, 'aUV', buffer(new Float32Array([0, 0, 1, 0, 0, 1, 1, 1])), 2);
    buffer(new Uint32Array([0, 2, 1, 1, 2, 3]), gl.ELEMENT_ARRAY_BUFFER);
    draws.lake = { vao: lv, count: 6 };

    // The instances: trees by kind, rocks, the herds' animals.
    function instanced(m, inst, kind) {
      var vao = gl.createVertexArray(); gl.bindVertexArray(vao);
      attrib(progs.solid, 'aPos', buffer(new Float32Array(m.p)), 3);
      attrib(progs.solid, 'aNor', buffer(new Float32Array(m.n)), 3);
      attrib(progs.solid, 'aCol', buffer(new Float32Array(m.c)), 3);
      attrib(progs.solid, 'aInst', buffer(inst), 4, 0, 0, 1);
      attrib(progs.solid, 'aKind', buffer(kind || new Float32Array(inst.length / 4)), 1, 0, 0, 1);
      return { vao: vao, count: m.p.length / 3, instances: inst.length / 4 };
    }
    var con = [], brd = [];
    for (i = 0; i < W.trees.length; i += 5) {
      (W.trees[i + 4] < 0.5 ? con : brd).push(W.trees[i], W.trees[i + 2] - 0.2, W.trees[i + 1], W.trees[i + 3]);
    }
    draws.conifers = instanced(conifer(), new Float32Array(con));
    draws.broadleaves = instanced(broadleaf(), new Float32Array(brd));
    var rk = [];
    for (i = 0; i < W.rocks.length; i += 4) rk.push(W.rocks[i], W.rocks[i + 2] - 0.3, W.rocks[i + 1], W.rocks[i + 3]);
    draws.rocks = instanced(boulder(), new Float32Array(rk));
    var an = [], ak = [];
    for (i = 0; i < W.herds.length; i += 4) {
      for (k = 0; k < W.herds[i + 3]; k++) {
        var ax = W.herds[i] + (k % 3) * 2.2 - 2.2, az = W.herds[i + 1] + Math.floor(k / 3) * 2.4 - 2.4;
        an.push(ax, heightAt(ax, az), az, 1.6);
        ak.push(k * 0.7);
      }
    }
    draws.beasts = instanced(beast(), new Float32Array(an), new Float32Array(ak));

    // The people's marks.
    var mp = [], ms = [], mi = [];
    W.persons.forEach(function (p) { mp.push(p.x, p.y, p.z); ms.push(30 + p.cls * 7); mi.push(p.index); });
    var mv = gl.createVertexArray(); gl.bindVertexArray(mv);
    attrib(progs.marks, 'aPos', buffer(new Float32Array(mp)), 3);
    attrib(progs.marks, 'aSize', buffer(new Float32Array(ms)), 1);
    attrib(progs.marks, 'aIdx', buffer(new Float32Array(mi)), 1);
    draws.marks = { vao: mv, count: W.persons.length };
    draws.sky = { vao: gl.createVertexArray() };
    gl.bindVertexArray(null);
  }

  /* ---- the camera ---- */
  var size = { w: 1, h: 1, dpr: 1 };
  function resize() {
    var r = stage.getBoundingClientRect();
    size.dpr = Math.min(window.devicePixelRatio || 1, 2);
    size.w = Math.max(1, Math.round(r.width));
    size.h = Math.max(1, Math.round(r.height));
    canvas.width = Math.round(size.w * size.dpr);
    canvas.height = Math.round(size.h * size.dpr);
    dirty = true;
  }
  function matrices() {
    var cp = Math.cos(cam.pitch), sp = Math.sin(cam.pitch);
    eye = [cam.tx + Math.sin(cam.yaw) * cp * cam.dist, cam.ty + sp * cam.dist, cam.tz + Math.cos(cam.yaw) * cp * cam.dist];
    // Never under the ground.
    var floor = heightAt(eye[0], eye[2]) + 3;
    if (eye[1] < floor) eye[1] = floor;
    view = lookAt(eye, [cam.tx, cam.ty, cam.tz], [0, 1, 0]);
    var proj = perspective(0.9, size.w / size.h, 0.5, 4000);
    mvp = mul(proj, view);
  }
  function focusOn(p, ms) {
    var to = { tx: p.x, ty: p.y, tz: p.z, dist: Math.min(Math.max(cam.dist, 45), 120) };
    if (!ms || reduce || cut) { cam.tx = to.tx; cam.ty = to.ty; cam.tz = to.tz; cam.dist = to.dist; dirty = true; return; }
    fly = { from: { tx: cam.tx, ty: cam.ty, tz: cam.tz, dist: cam.dist }, to: to, t0: performance.now(), ms: ms };
    wake();
  }

  /* ---- the names, over the scene ---- */
  var pool = [];
  function project(x, y, z) {
    var m = mvp, cx = m[0] * x + m[4] * y + m[8] * z + m[12], cy = m[1] * x + m[5] * y + m[9] * z + m[13];
    var cw = m[3] * x + m[7] * y + m[11] * z + m[15];
    if (cw <= 0.1) return null;
    return { x: (cx / cw * 0.5 + 0.5) * size.w, y: (1 - (cy / cw * 0.5 + 0.5)) * size.h, w: cw };
  }
  var allNames = false;
  function drawLabels() {
    var near = [];
    W.persons.forEach(function (p) {
      var wd = words[p.id];
      if (!wd) return;
      var s = project(p.x, p.y + 2.5, p.z);
      if (!s || s.x < -40 || s.x > size.w + 40 || s.y < -20 || s.y > size.h + 20 || s.w > (allNames ? 900 : 420)) return;
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
      el.style.opacity = String(Math.max(0.35, 1 - it.s.w / 420));
    });
  }

  /* ---- drawing ---- */
  var dirty = true, raf = null, running = false, onscreen = true, t0 = performance.now(), frozenAt = 0;
  var SUN = (function () { var s = [-0.55, 0.42, -0.72], l = Math.hypot(s[0], s[1], s[2]); return [s[0] / l, s[1] / l, s[2] / l]; })();
  var FOG = [0.80, 0.78, 0.70];
  function still() { return reduce || cut || paused || document.hidden || !onscreen; }
  function frame(now) {
    raf = null;
    if (!W) { running = false; return; }
    if (fly) {
      var t = Math.min(1, (now - fly.t0) / fly.ms), e = t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2;
      ['tx', 'ty', 'tz', 'dist'].forEach(function (k) { cam[k] = fly.from[k] + (fly.to[k] - fly.from[k]) * e; });
      if (t >= 1) fly = null;
      dirty = true;
    }
    var animate = !still();
    if (!dirty && !animate) { running = false; return; }
    var time = animate ? (now - t0) / 1000 : frozenAt;
    if (animate) frozenAt = time;
    matrices();
    gl.viewport(0, 0, canvas.width, canvas.height);
    gl.clearColor(FOG[0], FOG[1], FOG[2], 1);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    gl.enable(gl.DEPTH_TEST);
    gl.enable(gl.CULL_FACE);
    gl.cullFace(gl.BACK);
    // The mist thickens with distance, so that the world's edges are never
    // seen: it is a valley, not a model of one.
    var fogRange = [cam.dist * 1.1 + 70, cam.dist * 2.4 + 380];
    function common(pr) {
      gl.useProgram(pr.p);
      if (pr.u.uVP) gl.uniformMatrix4fv(pr.u.uVP, false, mvp);
      if (pr.u.uSun) gl.uniform3fv(pr.u.uSun, SUN);
      if (pr.u.uCam) gl.uniform3fv(pr.u.uCam, eye);
      if (pr.u.uFog) gl.uniform3fv(pr.u.uFog, FOG);
      if (pr.u.uFogRange) gl.uniform2fv(pr.u.uFogRange, fogRange);
      if (pr.u.uTime) gl.uniform1f(pr.u.uTime, time);
    }
    // The sky, behind everything.
    gl.disable(gl.CULL_FACE);
    gl.depthMask(false);
    common(progs.sky);
    gl.uniformMatrix4fv(progs.sky.u.uInv, false, invert(mvp));
    gl.bindVertexArray(draws.sky.vao);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
    gl.depthMask(true);
    gl.enable(gl.CULL_FACE);
    // The ground.
    common(progs.ground);
    gl.uniform1f(progs.ground.u.uWater, W.water);
    gl.uniform1f(progs.ground.u.uSnow, W.hi - 40);
    gl.bindVertexArray(draws.ground.vao);
    gl.drawElements(gl.TRIANGLES, draws.ground.count, gl.UNSIGNED_INT, 0);
    // Its trees, rocks and herds.
    common(progs.solid);
    [['conifers', 1], ['broadleaves', 1], ['rocks', 0], ['beasts', 2]].forEach(function (d) {
      var it = draws[d[0]];
      if (!it.instances) return;
      gl.uniform1f(progs.solid.u.uMode, d[1]);
      gl.bindVertexArray(it.vao);
      gl.drawArraysInstanced(gl.TRIANGLES, 0, it.count, it.instances);
    });
    // The water, over its beds and the lake.
    gl.disable(gl.CULL_FACE);
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
    common(progs.water);
    gl.uniform1f(progs.water.u.uLake, 1);
    gl.bindVertexArray(draws.lake.vao);
    gl.drawElements(gl.TRIANGLES, draws.lake.count, gl.UNSIGNED_INT, 0);
    gl.uniform1f(progs.water.u.uLake, 0);
    gl.bindVertexArray(draws.streams.vao);
    gl.drawElements(gl.TRIANGLES, draws.streams.count, gl.UNSIGNED_INT, 0);
    // The people's marks, over the ground.
    gl.depthMask(false);
    common(progs.marks);
    var f = W.byId[centre];
    gl.uniform1f(progs.marks.u.uFocus, f ? f.index : -1);
    gl.uniform1f(progs.marks.u.uScale, size.h * size.dpr * 0.35);
    gl.bindVertexArray(draws.marks.vao);
    gl.drawArrays(gl.POINTS, 0, draws.marks.count);
    gl.depthMask(true);
    gl.disable(gl.BLEND);
    gl.bindVertexArray(null);
    drawLabels();
    dirty = false;
    if (fly || animate) wake();
    else running = false;
  }
  function wake() {
    if (!raf) { running = true; raf = requestAnimationFrame(frame); }
  }

  /* ---- moving in it ---- */
  var drag = null;
  canvas.addEventListener('pointerdown', function (e) {
    if (!W) return;
    drag = { x: e.clientX, y: e.clientY, yaw: cam.yaw, pitch: cam.pitch, tx: cam.tx, tz: cam.tz, pan: e.button === 2 || e.shiftKey, moved: 0 };
    canvas.setPointerCapture(e.pointerId);
  });
  canvas.addEventListener('pointermove', function (e) {
    if (!drag) return;
    var dx = e.clientX - drag.x, dy = e.clientY - drag.y;
    drag.moved = Math.max(drag.moved, Math.abs(dx) + Math.abs(dy));
    if (drag.pan) {
      var k = cam.dist / size.h * 1.6, s = Math.sin(cam.yaw), c = Math.cos(cam.yaw);
      cam.tx = drag.tx - (dx * c - dy * s) * k;
      cam.tz = drag.tz + (dx * s + dy * c) * k;
      cam.ty = heightAt(cam.tx, cam.tz);
    } else {
      cam.yaw = drag.yaw - dx * 0.006;
      cam.pitch = Math.max(0.12, Math.min(1.45, drag.pitch + dy * 0.005));
    }
    fly = null;
    dirty = true;
    wake();
  });
  canvas.addEventListener('pointerup', function (e) {
    var d = drag;
    drag = null;
    if (!d || d.moved > 5) return;
    // A tap on a person travels to them, as a tap on the river does.
    var r = canvas.getBoundingClientRect(), x = e.clientX - r.left, y = e.clientY - r.top, best = null, bd = 22;
    W.persons.forEach(function (p) {
      var s = project(p.x, p.y + 0.6, p.z);
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
    cam.dist = Math.max(14, Math.min(MAX_DIST, cam.dist * Math.exp(e.deltaY * 0.0012)));
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
    else if (e.key === '+' || e.key === '=') cam.dist = Math.max(14, cam.dist * 0.85);
    else if (e.key === '-') cam.dist = Math.min(MAX_DIST, cam.dist * 1.18);
    else moved = false;
    if (!moved) return;
    e.preventDefault();
    cam.ty = heightAt(cam.tx, cam.tz);
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
      cam.dist = 140 / Math.max(1, parseFloat(zoom.value) || 1);
      fly = null; dirty = true; wake();
    });
  }
  var namesBtn = root.querySelector('[data-rv-names]');
  if (namesBtn) {
    namesBtn.hidden = false;
    namesBtn.addEventListener('click', function () {
      allNames = namesBtn.getAttribute('aria-pressed') !== 'true';
      namesBtn.setAttribute('aria-pressed', allNames ? 'true' : 'false');
      dirty = true; wake();
    });
  }
  document.addEventListener('visibilitychange', function () { dirty = true; wake(); });
  if (window.IntersectionObserver) {
    new IntersectionObserver(function (es) { onscreen = es[0].isIntersecting; dirty = true; wake(); }).observe(canvas);
  }
  if (window.ResizeObserver) new ResizeObserver(resize).observe(stage);
  else window.addEventListener('resize', resize);

  // The river travels: the camera follows (river.js calls this).
  window.axgfRiverWorld = {
    focus: function (id) {
      centre = id;
      if (!W) return;
      var p = W.byId[id];
      if (p) focusOn(p, 1100);
      else { dirty = true; wake(); }
    }
  };

  /* ---- reading it ---- */
  Promise.all([
    fetch(root.getAttribute('data-world'), { credentials: 'same-origin' }).then(function (r) { if (!r.ok) throw r; return r.arrayBuffer(); }),
    fetch('/canopee/words.json', { credentials: 'same-origin' }).then(function (r) { return r.ok ? r.json() : { people: {} }; }).catch(function () { return { people: {} }; })
  ]).then(function (got) {
    W = parse(got[0]);
    words = got[1].people || {};
    build();
    resize();
    var p = W.byId[centre] || W.persons[0];
    if (p) focusOn(p, 0);
    root.classList.add('rv-world-ready');
    wake();
  }).catch(function (e) {
    if (window.console) console.warn('Canopée world:', e);
    fallBack();
  });
})();
