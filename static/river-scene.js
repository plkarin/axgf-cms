/* The river's living ground (v0.5.0, docs/adr/0004 §11; redrawn 2026-10-24).
 *
 * Under the drawing, the ground of the operator's Selva Verde reference: a
 * forested mountainside seen from the air at golden hour, its spine of
 * clearings down the middle, mist over the valleys at both edges, the low
 * sun in the upper right (static/img/river-ground.jpg, made in the
 * operator's Canva from a description of the reference, not from it). It is
 * drawn in relief: a depth map of it (river-ground-depth.jpg) moves its
 * near parts more than its far ones as the pointer leans it and as the
 * river is zoomed, the mist drifts over it, and the shade every ink above
 * was measured on deepens toward the edges. Over the drawing, the water's
 * light: glints moving down every attested line from the sources toward the
 * living, as Selva Verde's path is lit.
 *
 * Decoration, and the client's alone. The server's SVG and payload are
 * unchanged (ADR 0001); the glints are drawn from the edges river.js
 * already holds, at the places it draws them. Without WebGL, or in print,
 * the photograph (as the stage's background) and the ink stay as they were.
 * prefers-reduced-motion, "Cut" in the settings, or the river's own pause
 * button draw the ground once and hold it still; nothing moves while the
 * river is off screen or the tab is hidden.
 */
(function () {
  'use strict';
  var root = document.querySelector('.river');
  var stage = document.getElementById('rv-canvas');
  if (!root || !stage || !window.WebGLRenderingContext) return;

  var reduce = window.matchMedia && matchMedia('(prefers-reduced-motion: reduce)').matches;
  var cut = root.dataset.riverMotion === 'cut';
  var STORE = 'axgf-river-still';
  var paused = false;
  try { paused = localStorage.getItem(STORE) === '1'; } catch (e) { /* private window */ }

  /* ---- the palette, from tokens.css ---- */
  var css = getComputedStyle(root);
  function colour(name, fallback) {
    var v = css.getPropertyValue(name).trim(), out;
    if (v.charAt(0) === '#') {
      var h = v.slice(1);
      if (h.length === 3) h = h.charAt(0) + h.charAt(0) + h.charAt(1) + h.charAt(1) + h.charAt(2) + h.charAt(2);
      out = [0, 2, 4].map(function (i) { return parseInt(h.slice(i, i + 2), 16) / 255; });
    } else {
      out = v.split(/[\s,]+/).filter(Boolean).slice(0, 3).map(function (n) { return parseFloat(n) / 255; });
    }
    return out.length === 3 && out.every(function (n) { return n >= 0 && n <= 1; }) ? out : fallback;
  }
  var P = {
    sun: colour('--rv-sun', [1, 1, 1]), mist: colour('--rv-mist-rgb', [0.85, 0.88, 0.86]),
    shade: colour('--rv-shade-rgb', [0.04, 0.07, 0.05]), core: colour('--rv-core', [1, 0.95, 0.85])
  };
  // The photograph and its depth map; how much larger than the stage it is
  // drawn, so that leaning and zooming never show its edge.
  var PIC = { src: '/static/img/river-ground.jpg?v=4', depth: '/static/img/river-ground-depth.jpg?v=1', aspect: 1680 / 944, over: 1.1 };

  /* ---- the ground: one triangle and one fragment shader ---- */
  var VERT = 'attribute vec2 a;void main(){gl_Position=vec4(a,0.0,1.0);}';
  var FRAG = [
    'precision highp float;',
    'uniform vec2 uRes;uniform float uTime;uniform vec4 uRect;uniform vec2 uCam;uniform float uDep;',
    'uniform sampler2D uPic;uniform sampler2D uDepth;uniform vec3 uSun,uMist,uShade;',
    'float hash(vec2 p){return fract(sin(dot(p,vec2(127.1,311.7)))*43758.5453);}',
    'float noise(vec2 p){vec2 i=floor(p),f=fract(p),u=f*f*(3.0-2.0*f);',
    ' return mix(mix(hash(i),hash(i+vec2(1.0,0.0)),u.x),mix(hash(i+vec2(0.0,1.0)),hash(i+vec2(1.0,1.0)),u.x),u.y);}',
    'float fbm(vec2 p){float s=0.0,a=0.5;for(int i=0;i<5;i++){s+=a*noise(p);p=p*2.07+vec2(17.3,9.1);a*=0.5;}return s;}',
    'void main(){',
    ' vec2 px=vec2(gl_FragCoord.x,uRes.y-gl_FragCoord.y);',
    ' vec2 uv=px/uRes;',
    // The photograph, each pixel looked up where the camera's shift, scaled
    // by how near it is, takes it: the spine and the near trees move more
    // than the valleys.
    ' vec2 p=(px-uRect.xy)/uRect.zw,s=uCam/uRect.zw;',
    ' float d=uDep>0.5?texture2D(uDepth,clamp(p,0.0,1.0)).r:0.5;vec2 q=p-s*(d-0.5);',
    ' if(uDep>0.5){d=texture2D(uDepth,clamp(q,0.0,1.0)).r;q=p-s*(d-0.5);}',
    ' vec3 col=texture2D(uPic,clamp(q,0.0,1.0)).rgb;',
    // Mist drifting over the valleys at both edges and into the lower
    // corners, warm where the sun is, a breath of it everywhere.
    ' vec2 w=px/uRes.y;',
    ' float m=fbm(w*2.6+vec2(uTime*0.018,uTime*0.006));',
    ' float side=max(1.0-smoothstep(0.0,0.3,uv.x),smoothstep(0.7,1.0,uv.x))*(0.55+0.45*smoothstep(0.2,1.0,uv.y));',
    ' col=mix(col,mix(uMist,uSun,0.35*smoothstep(0.5,1.0,uv.x)*(1.0-uv.y)),clamp(smoothstep(0.38,0.78,m)*side*0.6+0.04*m,0.0,0.6));',
    // The low sun's light from the upper right.
    ' float sun=1.0-smoothstep(0.0,0.8,length((uv-vec2(1.0,0.0))*vec2(1.0,1.35)));',
    ' col+=uSun*sun*sun*0.1;',
    // The shade every ink above was measured on, deeper at the edges.
    ' float v=smoothstep(0.38,1.05,length((uv-0.5)*vec2(1.0,1.15)));',
    ' col=mix(col,uShade,0.2+0.42*v);',
    // Dither, so a dark gradient never shows its steps.
    ' col+=(hash(gl_FragCoord.xy+fract(uTime)*91.7)-0.5)/255.0;',
    ' gl_FragColor=vec4(col,1.0);',
    '}'
  ].join('\n');

  var ground = document.createElement('canvas');
  ground.className = 'rv-ground';
  ground.setAttribute('aria-hidden', 'true');
  var gl = ground.getContext('webgl', { alpha: false, antialias: false, depth: false, stencil: false, premultipliedAlpha: false, powerPreference: 'low-power' });
  if (!gl) return;
  function shader(type, src) {
    var s = gl.createShader(type);
    gl.shaderSource(s, src); gl.compileShader(s);
    return gl.getShaderParameter(s, gl.COMPILE_STATUS) ? s : null;
  }
  var vs = shader(gl.VERTEX_SHADER, VERT), fs = shader(gl.FRAGMENT_SHADER, FRAG);
  if (!vs || !fs) return;
  var prog = gl.createProgram();
  gl.attachShader(prog, vs); gl.attachShader(prog, fs); gl.linkProgram(prog);
  if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) return;
  gl.useProgram(prog);
  var buf = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, buf);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 3, -1, -1, 3]), gl.STATIC_DRAW);
  var loc = gl.getAttribLocation(prog, 'a');
  gl.enableVertexAttribArray(loc);
  gl.vertexAttribPointer(loc, 2, gl.FLOAT, false, 0, 0);
  var U = {};
  ['uRes', 'uTime', 'uRect', 'uCam', 'uDep', 'uPic', 'uDepth', 'uSun', 'uMist', 'uShade']
    .forEach(function (k) { U[k] = gl.getUniformLocation(prog, k); });
  [['uSun', 'sun'], ['uMist', 'mist'], ['uShade', 'shade']].forEach(function (p) { gl.uniform3fv(U[p[0]], P[p[1]]); });
  gl.uniform1i(U.uPic, 0); gl.uniform1i(U.uDepth, 1);
  // The photograph arrives once. For the graphics card it is redrawn at a
  // power of two each way, so that it can be filtered as it shrinks (its
  // aspect is the stage's rectangle's, not the texture's). Until it is
  // here, the stage's own background shows: the ground is not drawn.
  function texture(unit) {
    var t = gl.createTexture();
    gl.activeTexture(unit); gl.bindTexture(gl.TEXTURE_2D, t);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    return t;
  }
  var picT = texture(gl.TEXTURE0), depT = texture(gl.TEXTURE1), hasPic = false, hasDep = false;
  var pic = new Image(), dep = new Image();
  pic.onload = function () {
    if (gl.isContextLost()) return;
    var big = Math.max(stage.clientWidth, 800) * Math.min(window.devicePixelRatio || 1, 2) > 1700;
    var c = document.createElement('canvas');
    c.width = big ? 4096 : 2048; c.height = big ? 2048 : 1024;
    var cx = c.getContext('2d');
    cx.imageSmoothingQuality = 'high';
    cx.drawImage(pic, 0, 0, c.width, c.height);
    gl.activeTexture(gl.TEXTURE0); gl.bindTexture(gl.TEXTURE_2D, picT);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGB, gl.RGB, gl.UNSIGNED_BYTE, c);
    gl.generateMipmap(gl.TEXTURE_2D);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR);
    hasPic = true;
    stage.classList.add('rv-alive');
    if (!running) draw(false);
  };
  dep.onload = function () {
    if (gl.isContextLost()) return;
    gl.activeTexture(gl.TEXTURE1); gl.bindTexture(gl.TEXTURE_2D, depT);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.LUMINANCE, gl.LUMINANCE, gl.UNSIGNED_BYTE, dep);
    hasDep = true;
    if (!running) draw(false);
  };
  pic.src = PIC.src;
  dep.src = PIC.depth;

  /* ---- the water's light: a 2D canvas over the drawing ---- */
  var water = document.createElement('canvas');
  water.className = 'rv-water';
  water.setAttribute('aria-hidden', 'true');
  var wctx = water.getContext('2d');
  // One pulse of light, drawn once: a warm halo wider than the water's
  // core, which is what shows it moving on the teal around it, and a bright
  // spindle down its middle.
  function spindle(rgb, inner, outer) {
    var c = document.createElement('canvas');
    c.width = 64; c.height = 32;
    var x = c.getContext('2d'), g = x.createRadialGradient(32, 16, 0, 32, 16, 32);
    var t = rgb.map(function (v) { return Math.round(v * 255); }).join(',');
    g.addColorStop(0, 'rgba(' + t + ',' + inner + ')');
    g.addColorStop(0.4, 'rgba(' + t + ',' + outer + ')');
    g.addColorStop(1, 'rgba(' + t + ',0)');
    x.setTransform(1, 0, 0, 0.5, 0, 8);
    x.fillStyle = g; x.fillRect(0, -16, 64, 64);
    return c;
  }
  var halo = spindle(colour('--rv-lit-halo', P.core), 0.55, 0.22), spark = spindle(P.core, 1, 0.5);

  stage.insertBefore(ground, stage.firstChild);
  stage.appendChild(water);

  /* ---- the layout river.js holds ---- */
  var WIDTHS = [0, 1.6, 3, 5, 8, 12];
  function cls(d) { return d <= 1 ? 1 : d <= 5 ? 2 : d <= 20 ? 3 : d <= 80 ? 4 : 5; }
  var flows = [], STEP = 2.5;
  function bez(e, t) {
    var m = (e.y1 + e.y2) / 2, u = 1 - t;
    return [u * u * u * e.x1 + 3 * u * u * t * e.x1 + 3 * u * t * t * e.x2 + t * t * t * e.x2,
            u * u * u * e.y1 + 3 * u * u * t * m + 3 * u * t * t * m + t * t * t * e.y2];
  }
  // Where a glint may not shine: on a person, and over the words beside them.
  function blockers() {
    var svg = stage.querySelector('svg.rv-svg'), out = [];
    if (!svg) return out;
    svg.querySelectorAll('text').forEach(function (t) {
      try { var b = t.getBBox(); if (b.width) out.push([b.x - 2, b.y - 2, b.x + b.width + 2, b.y + b.height + 2]); } catch (e) { /* not rendered */ }
    });
    return out;
  }
  function layout(L) {
    if (!L || !L.edges) return;
    var rects = blockers(), dots = L.persons || [];
    flows = [];
    L.edges.forEach(function (e) {
      if (e.conf !== 'a') return;
      var est = Math.hypot(e.x2 - e.x1, e.y2 - e.y1) * 1.25 + 4, n = Math.max(6, Math.ceil(est / STEP));
      var xs = new Float32Array(n + 1), ys = new Float32Array(n + 1), cum = new Float32Array(n + 1), ok = new Uint8Array(n + 1);
      for (var i = 0; i <= n; i++) {
        var p = bez(e, i / n);
        xs[i] = p[0]; ys[i] = p[1];
        if (i) cum[i] = cum[i - 1] + Math.hypot(xs[i] - xs[i - 1], ys[i] - ys[i - 1]);
        var free = true;
        for (var k = 0; free && k < dots.length; k++) {
          var r = dots[k].role === 'centre' ? 16 : 9;
          if (Math.abs(dots[k].x - p[0]) < r && Math.abs(dots[k].y - p[1]) < r) free = false;
        }
        for (k = 0; free && k < rects.length; k++) {
          var b = rects[k];
          if (p[0] > b[0] && p[0] < b[2] && p[1] > b[1] && p[1] < b[3]) free = false;
        }
        ok[i] = free ? 1 : 0;
      }
      var len = cum[n];
      if (len < 12) return;
      flows.push({ x: xs, y: ys, cum: cum, ok: ok, n: n, len: len, w: WIDTHS[cls(e.d)],
        k: Math.max(1, Math.round(len / 70)), phase: Math.abs(Math.sin(e.x1 * 12.9898 + e.y2 * 78.233)) % 1 });
    });
    request();
  }
  /* ---- drawing ---- */
  var dpr = 1, cw = 0, ch = 0, clock = 0, lastNow = 0, cap = 2;
  function size() {
    var w = stage.clientWidth, h = stage.clientHeight;
    var r = Math.min(window.devicePixelRatio || 1, cap);
    if (w === cw && h === ch && r === dpr) return false;
    cw = w; ch = h; dpr = r;
    ground.width = water.width = Math.max(1, Math.round(w * dpr));
    ground.height = water.height = Math.max(1, Math.round(h * dpr));
    return true;
  }
  function view() {
    var svg = stage.querySelector('svg.rv-svg'), vb = svg && svg.viewBox && svg.viewBox.baseVal;
    if (!vb || !vb.width || !cw) return null;
    // The SVG keeps its aspect (xMidYMid meet) and fills the stage, so one
    // scale serves both axes.
    var k = Math.min(cw / vb.width, ch / vb.height);
    return { x: vb.x - (cw / k - vb.width) / 2, y: vb.y - (ch / k - vb.height) / 2, k: k };
  }
  // The photograph covers the stage, a little larger. Zoomed, it grows and
  // follows the camera a third as much as the drawing does, as distant
  // ground would; the pointer leans it, and while the scene lives it
  // sways a little.
  var lean = [0, 0], leanTo = [0, 0], rest = null;
  if (!reduce && !cut) {
    stage.addEventListener('pointermove', function (ev) {
      var r = stage.getBoundingClientRect();
      leanTo = [Math.max(-1, Math.min(1, (ev.clientX - r.left) / r.width * 2 - 1)), Math.max(-1, Math.min(1, (ev.clientY - r.top) / r.height * 2 - 1))];
      if (!running) draw(false);
    }, { passive: true });
    stage.addEventListener('pointerleave', function () { leanTo = [0, 0]; if (!running) draw(false); });
  }
  function draw(moving) {
    size();
    var v = view();
    if (!v || !hasPic) return;
    var svg = stage.querySelector('svg.rv-svg'), zoomed = svg && svg.hasAttribute('data-zoom');
    var cx = v.x + cw / (2 * v.k), cy = v.y + ch / (2 * v.k);
    if (!zoomed || !rest) rest = { x: cx, y: cy, k: v.k };
    var z = Math.max(1, v.k / rest.k), grow = 1 + (z - 1) / 3;
    var W = ground.width, H = ground.height, sc = Math.max(W / PIC.aspect, H) * PIC.over * grow;
    var pw = sc * PIC.aspect, ph = sc;
    var px = (W - pw) / 2 - (cx - rest.x) * rest.k * dpr * (z - 1) / 3, py = (H - ph) / 2 - (cy - rest.y) * rest.k * dpr * (z - 1) / 3;
    lean[0] += (leanTo[0] - lean[0]) * (moving ? 0.08 : 1); lean[1] += (leanTo[1] - lean[1]) * (moving ? 0.08 : 1);
    var sway = moving ? Math.sin(clock * 0.27) : 0;
    gl.viewport(0, 0, W, H);
    gl.uniform2f(U.uRes, W, H);
    gl.uniform1f(U.uTime, clock);
    gl.uniform4f(U.uRect, px, py, pw, ph);
    gl.uniform2f(U.uCam, (lean[0] * 14 + sway * 5) * dpr, (lean[1] * 9) * dpr);
    gl.uniform1f(U.uDep, hasDep ? 1 : 0);
    gl.activeTexture(gl.TEXTURE0); gl.bindTexture(gl.TEXTURE_2D, picT);
    gl.activeTexture(gl.TEXTURE1); gl.bindTexture(gl.TEXTURE_2D, depT);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
    wctx.setTransform(1, 0, 0, 1, 0, 0);
    wctx.clearRect(0, 0, water.width, water.height);
    if (moving && !travelling) glints(v);
  }
  var SPEED = 22;
  function glints(v) {
    var s = v.k * dpr, ox = -v.x * s, oy = -v.y * s;
    wctx.globalCompositeOperation = 'lighter';
    for (var f = 0; f < flows.length; f++) {
      var e = flows[f];
      for (var j = 0; j < e.k; j++) {
        var pos = ((e.phase + j / e.k) * e.len + clock * SPEED) % e.len;
        // The sample at that length along the line (the samples are close
        // to evenly spaced, so a guess and a short walk find it).
        var i = Math.min(e.n - 1, Math.floor(pos / e.len * e.n));
        while (i > 0 && e.cum[i] > pos) i--;
        while (i < e.n - 1 && e.cum[i + 1] < pos) i++;
        if (!e.ok[i] || !e.ok[i + 1]) continue;
        var a = Math.min(1, pos / 16, (e.len - pos) / 16) * (0.75 + 0.25 * Math.sin(clock * 2.3 + j * 1.7 + e.phase * 20));
        if (a <= 0.02) continue;
        var dx = e.x[i + 1] - e.x[i], dy = e.y[i + 1] - e.y[i], d = Math.hypot(dx, dy) || 1, c = dx / d, sn = dy / d;
        var x = e.x[i] * s + ox, y = e.y[i] * s + oy;
        wctx.globalAlpha = a;
        // The halo: long and wide, onto the water around the core.
        var len = (22 + e.w * 1.6) * s, thick = (e.w * 1.5 + 6) * s;
        wctx.setTransform(c * len / 64, sn * len / 64, -sn * thick / 32, c * thick / 32, x, y);
        wctx.drawImage(halo, -32, -16);
        // The spark: as wide as the core, brighter than it.
        len = (10 + e.w * 0.8) * s; thick = Math.max(1.6, e.w * 0.5) * s;
        wctx.setTransform(c * len / 64, sn * len / 64, -sn * thick / 32, c * thick / 32, x, y);
        wctx.drawImage(spark, -32, -16);
      }
    }
    wctx.setTransform(1, 0, 0, 1, 0, 0);
    wctx.globalAlpha = 1;
    wctx.globalCompositeOperation = 'source-over';
  }

  /* ---- when it moves ---- */
  var raf = 0, running = false, onscreen = true, travelling = false, FRAME = 1000 / 30, last = 0, gaps = [], tired = false;
  function still() { return reduce || cut || paused || tired || document.hidden || !onscreen; }
  function loop(now) {
    raf = 0;
    if (!running) return;
    if (now - last >= FRAME - 2) {
      // A machine that cannot keep up — no graphics card, a busy one — gets
      // fewer pixels (never fewer than the screen's own), then fewer frames,
      // then a ground that holds still: never a stutter.
      if (last) { gaps.push(now - last); if (gaps.length > 12) gaps.shift(); }
      if (gaps.length === 12 && gaps.reduce(function (a, b) { return a + b; }) / 12 > 2.5 * FRAME) {
        gaps = [];
        if (dpr > 1) { cap = 1; size(); }
        else if (FRAME < 60) FRAME = 1000 / 15;
        else { tired = true; request(); return; }
      }
      // The clock runs only while the river moves, so a pause resumes where
      // it stopped instead of jumping.
      clock += Math.min(0.1, (now - (lastNow || now)) / 1000);
      lastNow = now; last = now;
      draw(true);
    } else lastNow = lastNow || now;
    raf = requestAnimationFrame(loop);
  }
  function request() {
    var go = !still();
    if (go && !running) { running = true; lastNow = 0; last = 0; gaps = []; raf = requestAnimationFrame(loop); }
    else if (!go) {
      running = false;
      if (raf) cancelAnimationFrame(raf);
      raf = 0;
      draw(false);
    }
  }
  if ('IntersectionObserver' in window) {
    new IntersectionObserver(function (es) { onscreen = es[0].isIntersecting; request(); }).observe(stage);
  }
  document.addEventListener('visibilitychange', request);
  if ('ResizeObserver' in window) new ResizeObserver(function () { if (size()) request(); if (!running) draw(false); }).observe(stage);
  ground.addEventListener('webglcontextlost', function (e) {
    // Back to the photograph rather than a black ground.
    e.preventDefault(); running = false; stage.classList.remove('rv-alive');
    ground.remove(); water.remove();
  });

  // The river's pause (WCAG 2.2.2): shown only where something moves.
  var btn = root.querySelector('[data-rv-still]');
  if (btn && !reduce && !cut) {
    btn.hidden = false;
    btn.setAttribute('aria-pressed', paused ? 'true' : 'false');
    btn.addEventListener('click', function () {
      paused = !paused;
      // Asked to move again, a machine that was too slow gets another try.
      if (!paused) tired = false;
      btn.setAttribute('aria-pressed', paused ? 'true' : 'false');
      try { localStorage.setItem(STORE, paused ? '1' : '0'); } catch (e) { /* private window */ }
      request();
    });
  }

  /* ---- what river.js tells it ---- */
  window.axgfRiverScene = {
    // A new layout at rest: the banks and the glints follow it.
    layout: function (L) { travelling = false; layout(L); },
    // A travel begins: the glints wait for the river to come to rest.
    travel: function () { travelling = true; },
    // The window moved (zoom, pan, resize): draw it now if nothing runs.
    view: function () { if (!running) draw(false); }
  };
  size();
  if (window.axgfRiver && window.axgfRiver.layout) layout(window.axgfRiver.layout());
  // A first frame now, so the stage is never black before the first tick.
  draw(false);
  request();
})();
