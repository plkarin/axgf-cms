/* The river's living ground (v0.5.0, docs/adr/0004 §11).
 *
 * Under the drawing, a forest seen from above, computed for every pixel of
 * the screen it is shown on: never a picture stretched to fit, so never soft
 * and never in steps. Crowns lit by a low sun, shadows between them, gusts
 * crossing the canopy and leaning the trees as they pass, mist drifting in
 * two corners, and banks where the water runs. Over the drawing, the water's
 * light: glints moving down every attested line from the sources toward the
 * living, as Selva Verde's path is lit.
 *
 * Decoration, and the client's alone. The server's SVG and payload are
 * unchanged (ADR 0001); everything here is drawn from the edges river.js
 * already holds, at the places it draws them. Without WebGL — or under
 * Gravure, or in print — the photograph and the ink stay as they were.
 * prefers-reduced-motion, "Cut" in the settings, or the river's own pause
 * button draw the ground once and hold it still; nothing moves while the
 * river is off screen or the tab is hidden.
 */
(function () {
  'use strict';
  var root = document.querySelector('.river');
  var stage = document.getElementById('rv-canvas');
  if (!root || !stage || !window.WebGLRenderingContext) return;
  if (document.documentElement.getAttribute('data-theme') === 'high-contrast') return;

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
  var GREY = [0.1, 0.12, 0.11];
  var P = {
    deep: colour('--rv-canopy-deep', GREY), mid: colour('--rv-canopy-mid', GREY),
    lit: colour('--rv-canopy-lit', GREY), warm: colour('--rv-canopy-warm', GREY),
    sun: colour('--rv-sun', [1, 1, 1]), bank: colour('--rv-bank', GREY),
    mist: colour('--rv-mist-rgb', [0.85, 0.88, 0.86]), shade: colour('--rv-shade-rgb', [0.04, 0.07, 0.05]),
    core: colour('--rv-core', [1, 0.95, 0.85])
  };

  /* ---- the ground: one triangle and one fragment shader ---- */
  var VERT = 'attribute vec2 a;void main(){gl_Position=vec4(a,0.0,1.0);}';
  var FRAG = [
    'precision highp float;',
    'uniform vec2 uRes;uniform vec4 uView;uniform float uTime;uniform float uWind;',
    'uniform sampler2D uMask;uniform vec4 uMaskRect;',
    'uniform vec3 uDeep,uMid,uLit,uWarm,uSun,uMist,uBank,uShade;',
    // A tree every CELL units; the wind blows toward the upper right; the
    // sun is low in the upper right too, as over Selva Verde.
    'const float CELL=11.0;const vec2 WIND=vec2(0.86,-0.51);',
    'const vec3 SUN=vec3(0.55,-0.62,0.56);',
    'float hash(vec2 p){return fract(sin(dot(p,vec2(127.1,311.7)))*43758.5453);}',
    'vec2 hash2(vec2 p){return vec2(hash(p),hash(p+19.19));}',
    'float noise(vec2 p){vec2 i=floor(p),f=fract(p),u=f*f*(3.0-2.0*f);',
    ' return mix(mix(hash(i),hash(i+vec2(1.0,0.0)),u.x),mix(hash(i+vec2(0.0,1.0)),hash(i+vec2(1.0,1.0)),u.x),u.y);}',
    // Value noise with its derivatives, for the hills' slopes.
    'vec3 noised(vec2 p){vec2 i=floor(p),f=fract(p),u=f*f*(3.0-2.0*f),du=6.0*f*(1.0-f);',
    ' float a=hash(i),b=hash(i+vec2(1.0,0.0)),c=hash(i+vec2(0.0,1.0)),d=hash(i+vec2(1.0,1.0));',
    ' return vec3(a+(b-a)*u.x+(c-a)*u.y+(a-b-c+d)*u.x*u.y,du*(vec2(b-a,c-a)+(a-b-c+d)*u.yx));}',
    'vec3 hills(vec2 p){float s=0.0,a=0.5,k=1.0;vec2 d=vec2(0.0);',
    ' for(int i=0;i<4;i++){vec3 n=noised(p);s+=a*n.x;d+=a*k*n.yz;p=p*2.03+vec2(13.1,7.7);k*=2.03;a*=0.5;}',
    ' return vec3(s,d);}',
    'float fbm(vec2 p){float s=0.0,a=0.5;for(int i=0;i<4;i++){s+=a*noise(p);p=p*2.07+vec2(17.3,9.1);a*=0.5;}return s;}',
    // How near the water a point is (0 far, 1 on it): the mask river-scene.js
    // draws from the edges.
    'float wet(vec2 w){return texture2D(uMask,(w-uMaskRect.xy)*uMaskRect.zw).r;}',
    // A gust: a band of wind travelling across the forest; and each tree's
    // own sway, small, at its own rhythm.
    'vec2 sway(vec2 c,out float g){',
    ' g=smoothstep(0.42,0.92,noise(c*0.08-WIND*uTime*0.45));',
    ' float own=sin(uTime*(1.1+0.7*hash(c+2.3))+6.2831*hash(c+8.7));',
    ' return WIND*uWind*(0.035*own*(0.4+g)+0.12*g);}',
    // One tree: where its crown is, how wide, how tall, and whether it stands
    // at all (not in the water, thinning on a bank).
    'float tree(vec2 c,vec2 g,vec2 fp,out vec2 d,out float rad,out float tall,out vec2 sw,out float gu){',
    ' float room=1.0-smoothstep(0.1,0.5,wet((c+0.5)*CELL));',
    ' sw=sway(c,gu);',
    ' d=fp-(g+0.2+0.6*hash2(c)+sw);',
    ' rad=(0.46+0.34*hash(c+4.1))*mix(0.8,1.22,noise(c*0.06))*mix(0.6,1.0,room);',
    ' tall=(0.55+0.45*hash(c+7.7))*room;',
    // A conifer seen from above: a crown with points, not a disc.
    ' float an=atan(d.y,d.x)+6.2831*hash(c+3.3);',
    ' float sp=1.0+0.09*sin(an*9.0)+0.07*sin(an*4.0+2.0*hash(c+5.9))+0.05*sin(an*13.0);',
    ' return room<0.06?2.0:dot(d,d)/(rad*rad*sp*sp);}',
    // The canopy over p (in cells): the height of the tallest crown there,
    // its normal, a per-tree tone, and the gust it is in.
    'float canopy(vec2 p,out vec3 n,out float tone,out float gst){',
    ' vec2 ip=floor(p),fp=fract(p);float best=0.0;n=vec3(0.0,0.0,1.0);tone=0.0;gst=0.0;',
    ' for(int j=-1;j<=1;j++)for(int i=-1;i<=1;i++){',
    '  vec2 g=vec2(float(i),float(j)),c=ip+g,d,sw;float rad,tall,gu;',
    '  float q=tree(c,g,fp,d,rad,tall,sw,gu);',
    '  if(q>=1.0)continue;',
    '  float s=sqrt(1.0-q),z=tall*(0.3+0.7*s);',
    '  if(z>best){best=z;n=normalize(vec3(tall*0.7*d/(rad*rad*max(s,0.12))+sw*2.6,1.0));tone=hash(c+1.3);gst=gu;}',
    ' }',
    ' return best;}',
    // The height alone, for the shadow a crown casts on its neighbours.
    'float height(vec2 p){',
    ' vec2 ip=floor(p),fp=fract(p);float best=0.0;',
    ' for(int j=-1;j<=1;j++)for(int i=-1;i<=1;i++){',
    '  vec2 g=vec2(float(i),float(j)),c=ip+g,d,sw;float rad,tall,gu;',
    '  float q=tree(c,g,fp,d,rad,tall,sw,gu);',
    '  if(q<1.0)best=max(best,tall*(0.3+0.7*sqrt(1.0-q)));',
    ' }',
    ' return best;}',
    'void main(){',
    ' vec2 px=vec2(gl_FragCoord.x,uRes.y-gl_FragCoord.y);',
    ' vec2 w=uView.xy+px*uView.zw;',
    ' vec2 uv=px/uRes;',
    ' vec3 L=normalize(SUN);',
    // The hills under the forest: their slopes, lit by the same sun.
    ' vec3 hl=hills(w*0.0032);',
    ' vec3 hn=normalize(vec3(-hl.yz*0.0032*270.0,1.0));',
    ' float hs=clamp(dot(hn,L)*1.25,0.0,1.4);',
    ' vec3 n;float tone,gst;',
    ' float h=canopy(w/CELL,n,tone,gst);',
    // Needles: a fine bump on every crown.
    ' float l0=noise(w*0.5),lx=noise(w*0.5+vec2(0.6,0.0)),ly=noise(w*0.5+vec2(0.0,0.6));',
    ' n=normalize(n+vec3((lx-l0)*1.3,(ly-l0)*1.3,0.0));',
    ' float diff=max(dot(n,L),0.0);',
    ' vec3 alb=mix(uMid,uLit,smoothstep(0.3,1.0,tone)*0.85);',
    ' alb=mix(alb,uWarm,smoothstep(0.955,0.99,fract(tone*7.31))*0.75);',
    // Valleys are greener and deeper, ridges catch the light.
    ' alb*=mix(0.72,1.12,smoothstep(0.25,0.75,hl.x));',
    ' alb*=1.0+0.24*gst*uWind;',
    ' float ao=smoothstep(0.0,0.85,h);',
    ' vec3 col=alb*(0.17+0.46*ao)+alb*uSun*diff*0.86;',
    ' col+=uSun*pow(max(dot(reflect(-L,n),vec3(0.0,0.0,1.0)),0.0),14.0)*0.045*ao;',
    // A crown upsun of this point, taller than it, shades it.
    ' float sh=height(w/CELL+vec2(0.34,-0.38));',
    ' col*=1.0-0.5*smoothstep(0.06,0.3,sh-h);',
    // The forest floor, and the wet ground of a bank.
    ' float mk=wet(w);',
    ' vec3 floorc=mix(uDeep,uBank,smoothstep(0.08,0.55,mk))*(0.8+0.35*noise(w*0.3));',
    ' col=h>0.0?col:floorc;',
    ' col*=mix(0.48,1.34,hs/1.4);',
    // Mist, drifting, thickest in two corners and warm where the sun is; a
    // breath of it everywhere.
    ' float m=fbm(w*0.004+vec2(uTime*0.011,uTime*0.004));',
    ' float bl=1.0-smoothstep(0.0,0.7,length((uv-vec2(0.0,1.0))*vec2(1.0,1.4)));',
    ' float tr=1.0-smoothstep(0.0,0.62,length((uv-vec2(1.0,0.0))*vec2(1.1,1.5)));',
    ' col=mix(col,uMist,clamp(smoothstep(0.3,0.74,m)*bl*0.85+0.05*m,0.0,0.8));',
    ' col=mix(col,mix(uMist,uSun,0.45),clamp(smoothstep(0.28,0.74,m)*tr*0.75+tr*0.12,0.0,0.75));',
    // The shade every ink above was measured on, deeper at the edges.
    ' float v=smoothstep(0.42,1.05,length((uv-0.5)*vec2(1.0,1.15)));',
    ' col=mix(col,uShade,0.12+0.48*v);',
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
  ['uRes', 'uView', 'uTime', 'uWind', 'uMask', 'uMaskRect', 'uDeep', 'uMid', 'uLit', 'uWarm', 'uSun', 'uMist', 'uBank', 'uShade']
    .forEach(function (k) { U[k] = gl.getUniformLocation(prog, k); });
  [['uDeep', 'deep'], ['uMid', 'mid'], ['uLit', 'lit'], ['uWarm', 'warm'], ['uSun', 'sun'], ['uMist', 'mist'], ['uBank', 'bank'], ['uShade', 'shade']]
    .forEach(function (p) { gl.uniform3fv(U[p[0]], P[p[1]]); });
  var tex = gl.createTexture();
  gl.bindTexture(gl.TEXTURE_2D, tex);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
  gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 1, 1, 0, gl.RGBA, gl.UNSIGNED_BYTE, new Uint8Array([0, 0, 0, 255]));
  gl.uniform1i(U.uMask, 0);
  var maskRect = [0, 0, 1, 1];

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
  stage.classList.add('rv-alive');

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
    var x0 = Infinity, y0 = Infinity, x1 = -Infinity, y1 = -Infinity;
    flows = [];
    L.edges.forEach(function (e) {
      x0 = Math.min(x0, e.x1, e.x2); x1 = Math.max(x1, e.x1, e.x2);
      y0 = Math.min(y0, e.y1, e.y2); y1 = Math.max(y1, e.y1, e.y2);
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
    if (x0 === Infinity) { x0 = 0; y0 = 0; x1 = 1; y1 = 1; }
    mask(L.edges, x0 - 90, y0 - 90, x1 + 90, y1 + 90);
    request();
  }
  // How near the water every point is, drawn at half a pixel per unit and
  // softened: the trees thin out on a bank, and the floor there is wet.
  function mask(edges, x0, y0, x1, y1) {
    var S = 0.5, W = Math.max(2, Math.ceil((x1 - x0) * S)), H = Math.max(2, Math.ceil((y1 - y0) * S));
    var c = document.createElement('canvas');
    c.width = W; c.height = H;
    var m = c.getContext('2d');
    m.fillStyle = 'black'; m.fillRect(0, 0, W, H);
    if ('filter' in m) m.filter = 'blur(3px)';
    m.setTransform(S, 0, 0, S, -x0 * S, -y0 * S);
    m.lineCap = 'round'; m.strokeStyle = 'white';
    [[30, 0.16], [18, 0.3], [9, 0.55], [3, 1]].forEach(function (pass) {
      m.globalAlpha = pass[1];
      edges.forEach(function (e) {
        var mid = (e.y1 + e.y2) / 2;
        m.lineWidth = WIDTHS[cls(e.d)] + pass[0];
        m.beginPath(); m.moveTo(e.x1, e.y1); m.bezierCurveTo(e.x1, mid, e.x2, mid, e.x2, e.y2); m.stroke();
      });
    });
    gl.bindTexture(gl.TEXTURE_2D, tex);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, c);
    maskRect = [x0, y0, 1 / (x1 - x0), 1 / (y1 - y0)];
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
  function draw(moving) {
    size();
    var v = view();
    if (!v) return;
    gl.viewport(0, 0, ground.width, ground.height);
    gl.uniform2f(U.uRes, ground.width, ground.height);
    gl.uniform4f(U.uView, v.x, v.y, 1 / (v.k * dpr), 1 / (v.k * dpr));
    gl.uniform1f(U.uTime, clock);
    gl.uniform1f(U.uWind, moving ? 1 : 0);
    gl.uniform4f(U.uMaskRect, maskRect[0], maskRect[1], maskRect[2], maskRect[3]);
    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, tex);
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
