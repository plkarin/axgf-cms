/* The river's living ground (v0.5.0, docs/adr/0004 §11; redrawn 2026-10-24).
 *
 * Under the drawing, the ground of the operator's Selva Verde reference: a
 * forested mountain valley seen from the air at golden hour, terraces and a
 * golden meadow on its left, turquoise streams and ponds on its right, a
 * rocky outcrop at its foot, mist at its edges, the low sun in the upper
 * right (static/img/river-ground.jpg, made in the operator's Canva from a
 * description of the reference, not from it). It is drawn in relief: a
 * depth map of it (red in river-ground-maps.jpg) moves its near parts more
 * than its far ones as the pointer leans it and as the river is zoomed.
 * And it moves: ripples run down its water (green in the same maps) and the
 * sun glints on it; the family's rivers light the forest around them in
 * slow waves; the sun's rays turn; the mist drifts; the shade every ink
 * above was measured on deepens toward the edges. Over the drawing, the
 * water's light: glints moving down every attested line from the sources
 * toward the living, as Selva Verde's path is lit, and each person a marker
 * of light whose ring opens every few seconds.
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
    shade: colour('--rv-shade-rgb', [0.04, 0.07, 0.05]), core: colour('--rv-core', [1, 0.95, 0.85]),
    green: colour('--rv-riparian', [0.62, 0.8, 0.6])
  };
  // The family's lines drawn as rivers in the ground (docs/adr/0004 §13),
  // where the theme asks for them: Canopée.
  var RIVERS = getComputedStyle(root).getPropertyValue('--rv-rivers-3d').trim() === '1';
  // The photograph and its map (red: how near each pixel is); how much
  // larger than the stage it is drawn, so that leaning and zooming never
  // show its edge. The ground holds no water of its own: the family's
  // streams and rivers are its only water (docs/adr/0004 §15).
  var PIC = { src: '/static/img/river-ground.jpg?v=6', maps: '/static/img/river-ground-maps.jpg?v=2', aspect: 1680 / 944, over: 1.1 };

  /* ---- the ground: one triangle and one fragment shader ---- */
  var VERT = 'attribute vec2 a;void main(){gl_Position=vec4(a,0.0,1.0);}';
  var FRAG = [
    'precision highp float;',
    'uniform vec2 uRes;uniform float uTime;uniform vec4 uRect;uniform vec2 uCam;uniform float uDep;',
    'uniform sampler2D uPic;uniform sampler2D uDepth;uniform vec3 uSun,uMist,uShade,uGreen;',
    'uniform sampler2D uGlow;uniform vec4 uView;uniform vec4 uGlowRect;',
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
    // Where the family's water runs the ground holds still, so that the
    // valleys it carved stay under it as the pointer leans the landscape.
    ' vec2 wp=uView.xy+px*uView.zw;vec2 gq=(wp-uGlowRect.xy)*uGlowRect.zw;',
    ' vec3 gt=texture2D(uGlow,gq).rgb;',
    ' s*=1.0-clamp(gt.b*1.6,0.0,0.9);',
    ' float d=uDep>0.5?texture2D(uDepth,clamp(p,0.0,1.0)).r:0.5;vec2 q=p-s*(d-0.5);',
    ' if(uDep>0.5){d=texture2D(uDepth,clamp(q,0.0,1.0)).r;q=p-s*(d-0.5);}',
    ' vec3 col=texture2D(uPic,clamp(q,0.0,1.0)).rgb;',

    // Mist drifting over the valleys at both edges and into the lower
    // corners, warm where the sun is, a breath of it everywhere.
    ' vec2 w=px/uRes.y;',
    ' float m=fbm(w*2.6+vec2(uTime*0.03,uTime*0.01));',
    ' float side=max(1.0-smoothstep(0.0,0.3,uv.x),smoothstep(0.7,1.0,uv.x))*(0.55+0.45*smoothstep(0.2,1.0,uv.y));',
    ' col=mix(col,mix(uMist,uSun,0.35*smoothstep(0.5,1.0,uv.x)*(1.0-uv.y)),clamp(smoothstep(0.38,0.78,m)*side*0.6+0.04*m,0.0,0.6));',
    // The low sun's light from the upper right, and its rays across the
    // ground, slowly turning.
    ' float sun=1.0-smoothstep(0.0,0.8,length((uv-vec2(1.0,0.0))*vec2(1.0,1.35)));',
    ' col+=uSun*sun*sun*0.1;',
    ' vec2 sd=uv-vec2(1.04,-0.1);float ang=atan(sd.y,sd.x);',
    ' float ray=smoothstep(0.55,0.95,noise(vec2(ang*11.0,uTime*0.07)))*smoothstep(1.3,0.15,length(sd));',
    ' col+=uSun*ray*0.07;',
    // The ground follows the family's water (docs/adr/0004 §16): each
    // stream and river runs in a valley it has carved, its slopes facing
    // the sun lit and the others in shade, its floor lower and wetter, and
    // along it the vegetation darker and greener, as it grows along water.
    // The valley is a height field from the blurred courses (blue), lit as
    // the hills were.
    ' float gw=gt.r;',
    ' vec2 e=vec2(uGlowRect.z*3.0,uGlowRect.w*3.0);',
    ' float hx=texture2D(uGlow,gq+vec2(e.x,0.0)).b-texture2D(uGlow,gq-vec2(e.x,0.0)).b;',
    ' float hy=texture2D(uGlow,gq+vec2(0.0,e.y)).b-texture2D(uGlow,gq-vec2(0.0,e.y)).b;',
    ' vec3 vn=normalize(vec3(hx*14.0,hy*14.0,1.0));',
    ' vec3 SUNV=normalize(vec3(0.55,-0.62,0.56));',
    ' float slope=clamp(1.0+(dot(vn,SUNV)-SUNV.z)*2.6,0.5,1.5);',
    ' col*=mix(1.0,slope,clamp(gt.b*2.2,0.0,1.0));',
    ' col*=1.0-0.3*gt.b*gt.b;',
    ' col=mix(col,col*uGreen,clamp(gt.g*0.6+gw*gw*0.3,0.0,0.65));',
    // The shade every ink above was measured on, deeper at the edges.
    ' float v=smoothstep(0.38,1.05,length((uv-0.5)*vec2(1.0,1.15)));',
    ' col=mix(col,uShade,0.14+0.38*v);',
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
  /* ---- the rivers: one strip per line, carved into the ground ---- */
  // Each of the family's lines a river along the river's own curve: its
  // water, deepest down the middle, rippling as it runs from the sources
  // toward the living; the light down its middle where the parentage is
  // attested, as Selva Verde's path is lit; its banks of wet earth, the
  // slope facing the sun lit, the other in shade; and the glow it casts on
  // the forest. An inferred parentage is a stream that runs underground
  // and out again; a speculative one, a dry trail of small pools.
  var RVERT = [
    'attribute vec2 aP;attribute vec2 aSV;attribute vec2 aWB;attribute vec4 aCol;attribute vec4 aMeta;attribute vec2 aN;attribute float aK;',
    'uniform vec2 uRes;uniform vec3 uView;',
    'varying vec2 vSV;varying vec2 vWB;varying vec4 vCol;varying vec4 vMeta;varying vec2 vN;varying float vK;',
    'void main(){vec2 px=(aP-uView.xy)*uView.z;gl_Position=vec4(px.x/uRes.x*2.0-1.0,1.0-px.y/uRes.y*2.0,0.0,1.0);',
    ' vSV=aSV;vWB=aWB;vCol=aCol;vMeta=aMeta;vN=aN;vK=aK;}'
  ].join('\n');
  var RFRAG = [
    'precision highp float;',
    'varying vec2 vSV;varying vec2 vWB;varying vec4 vCol;varying vec4 vMeta;varying vec2 vN;varying float vK;',
    'uniform float uTime,uK;uniform sampler2D uTexS,uTexR;uniform vec3 uSun;',
    'float hash(vec2 p){return fract(sin(dot(p,vec2(127.1,311.7)))*43758.5453);}',
    'float noise(vec2 p){vec2 i=floor(p),f=fract(p),u=f*f*(3.0-2.0*f);',
    ' return mix(mix(hash(i),hash(i+vec2(1.0,0.0)),u.x),mix(hash(i+vec2(0.0,1.0)),hash(i+vec2(1.0,1.0)),u.x),u.y);}',
    'void main(){',
    ' float s=vSV.x,v=vSV.y,len=vMeta.y,W0=vWB.x,B=vWB.y,aa=1.0/uK,conf=vCol.a,ph=vMeta.z,riv=vK;',
    ' float fade=vMeta.w>0.5?1.0-smoothstep(0.0,len,v)*(vMeta.w>1.5?1.0:0.7):1.0;',
    // How far from its middle, round at its two ends.
    ' float d=v<0.0?length(vec2(v,s)):(v>len?length(vec2(v-len,s)):abs(s));',
    // Its edge is the ground's, never a ruled line: it breathes along the
    // course and frays where the water meets earth and stone.
    ' float R=(W0+B)*(0.82+0.36*noise(vec2(v*(riv>0.5?0.025:0.06),ph*13.0)))*(0.96+0.08*noise(vec2(v*0.18,ph*7.0)));',
    // The water itself: a photograph of a stream or of a river laid along
    // the course, its banks at the edges of the picture, repeating down it.
    // Each is its own: a stretch of the photograph of its own, mirrored or
    // not, its water a little deeper or lighter.
    ' float flip=ph>0.5?-1.0:1.0;',
    ' float x=clamp(0.5+flip*s/(2.0*R)*0.96,0.0,1.0);',
    // The picture runs a little long down the course, as moving water reads.
    ' float tile=(riv>0.5?9.0:6.5)*R*(0.85+0.3*fract(ph*7.13));',
    ' float y=(v+ph*1730.0)/tile;',
    ' vec3 col;',
    ' if(riv>0.5){',
    // A river: its water slides downstream, its banks stay where they are.
    '  float cw=1.0-smoothstep(0.24,0.36,abs(x-0.5));',
    '  vec2 rp=vec2(x*9.0,(v-uTime*7.0)*0.18);',
    '  vec2 off=vec2(noise(rp)-0.5,noise(rp+5.1)-0.5)*0.012*cw;',
    '  vec3 still=texture2D(uTexR,vec2(x,y)).rgb;',
    '  vec3 run=texture2D(uTexR,vec2(x,y-uTime*7.0/tile)+off).rgb;',
    '  col=mix(still,run,cw);',
    '  col+=uSun*pow(smoothstep(0.62,1.0,noise(vec2(x*28.0,(v-uTime*9.0)*0.9))),4.0)*0.35*cw;',
    // Clear mountain water: toward turquoise where it runs open.
    '  col=mix(col,col*vec3(0.82,1.04,1.16),0.6*cw);',
    ' }else{',
    // A stream: the water trembles over its stones, foam runs down it, and
    // here and there it falls a step.
    '  float cw=1.0-smoothstep(0.2,0.34,abs(x-0.5));',
    '  vec2 rp=vec2(x*14.0,(v-uTime*24.0)*0.5);',
    '  vec2 off=vec2(noise(rp)-0.5,noise(rp+3.7)-0.5)*0.016*cw;',
    '  col=texture2D(uTexS,vec2(x,y)+off).rgb;',
    '  float foam=pow(noise(vec2(x*22.0,(v-uTime*38.0)*0.8)),5.0)*cw;',
    '  float cv=fract((v+ph*97.0)/(34.0+46.0*ph));float fall=exp(-pow((cv-0.5)*16.0,2.0))*(0.55+0.45*noise(vec2(x*18.0,uTime*6.0)))*cw;',
    '  col=mix(col,vec3(0.95,0.99,0.98),clamp(foam*1.3+fall*0.7,0.0,0.85));',
    '  col+=uSun*pow(smoothstep(0.6,1.0,noise(vec2(x*30.0,(v-uTime*30.0)*1.2))),4.0)*0.3*cw;',
    ' }',
    // Its own depth of water, and a little of the line's own colour (the
    // river's ramp of the years).
    ' col*=0.9+0.22*fract(ph*3.71);',
    // Clear water in the low sun: a little more colour and light than the
    // photographs carry, so that it shines in the valley.
    ' col=mix(vec3(dot(col,vec3(0.299,0.587,0.114))),col,1.3)*1.12;',
    ' col=mix(col,col*0.7+vCol.rgb*0.3,0.12);',
    // The sun's sheen on its open water, brighter toward the light.
    ' col+=uSun*0.06*(1.0-smoothstep(0.0,1.0,abs(s)/R));',
    // How sure the parentage is: an inferred one runs underground and out
    // again, leaving a dry bed; a speculative one, a trail of small pools.
    ' float wet=1.0;',
    ' if(conf>0.5&&conf<1.5){float seg=fract((v+ph*31.0)/(10.0+R*2.4));wet=smoothstep(0.0,0.05,seg)*(1.0-smoothstep(0.62,0.68,seg));}',
    ' if(conf>1.5){float m=mod(v+ph*17.0,7.0)-3.5;wet=1.0-smoothstep(1.4,1.4+aa*1.5,length(vec2(m,s)));}',
    ' float inside=1.0-smoothstep(R-aa*1.5-R*0.08,R+aa*1.5,d);',
    ' float a=inside*mix(conf>1.5?0.0:0.28,1.0,wet)*vMeta.x*fade;',
    ' col=mix(col*0.55+vec3(0.06,0.05,0.03),col,wet);',
    // Its bed lies in the ground: a shadow of wet earth just outside it.
    ' float sh=(1.0-smoothstep(R,R+2.2+R*0.25,d))*(1.0-inside)*0.38*vMeta.x*fade;',
    ' gl_FragColor=vec4(col*a,max(a,sh));',
    '}'
  ].join('\n');
  var rprog = null, rbuf = null, RA = {}, RU = {}, rcount = 0, rready = false;
  if (RIVERS) {
    var rvs = shader(gl.VERTEX_SHADER, RVERT), rfs = shader(gl.FRAGMENT_SHADER, RFRAG);
    if (rvs && rfs) {
      rprog = gl.createProgram();
      gl.attachShader(rprog, rvs); gl.attachShader(rprog, rfs); gl.linkProgram(rprog);
      if (!gl.getProgramParameter(rprog, gl.LINK_STATUS)) rprog = null;
    }
    if (rprog) {
      ['aP', 'aSV', 'aWB', 'aCol', 'aMeta', 'aN', 'aK'].forEach(function (k) { RA[k] = gl.getAttribLocation(rprog, k); });
      ['uRes', 'uView', 'uTime', 'uK', 'uTexS', 'uTexR', 'uSun'].forEach(function (k) { RU[k] = gl.getUniformLocation(rprog, k); });
      gl.useProgram(rprog);
      gl.uniform3fv(RU.uSun, P.sun);
      gl.uniform1i(RU.uTexS, 3); gl.uniform1i(RU.uTexR, 4);
      rbuf = gl.createBuffer();
      gl.useProgram(prog);
    }
  }
  // Each line's course (docs/adr/0004 §14): the river's own curve, bent into
  // a watercourse of its own. A stream — a parent's line to their family —
  // winds in tight, uneven bends, as water finds its way down a steep slope;
  // a river — a family's line to a child, where both parents' streams meet —
  // in broad, calm ones. No two alike: the bends come from the line's own
  // ends. The course leaves and reaches each end as the curve does, and its
  // bends are never tighter than the water and its banks can follow.
  var COURSES = {};
  function hsh(k, i) { var h = Math.sin(k * 127.1 + i * 311.7) * 43758.5453; return h - Math.floor(h); }
  function noise1(x) { var i = Math.floor(x), f = x - i, u = f * f * (3 - 2 * f); return hsh(i, 0.37) + (hsh(i + 1, 0.37) - hsh(i, 0.37)) * u; }
  function keyOf(x1, y1, x2, y2) { return [x1, y1, x2, y2].map(function (v) { return Math.round(v * 10); }).join(','); }
  function widthOf(e) {
    var w = WIDTHS[cls(e.d)] * 0.7 + 1.2;
    return e.river ? w * 1.25 + 0.8 : Math.max(1.1, w * 0.55);
  }
  function course(e) {
    var key = keyOf(e.x1, e.y1, e.x2, e.y2);
    if (COURSES[key]) return COURSES[key];
    var river = !!e.river, k = (Math.abs(Math.sin(e.x1 * 12.9898 + e.y1 * 4.1414 + e.x2 * 78.233 + e.y2 * 37.719)) * 43758.5453) % 1000;
    var est = Math.hypot(e.x2 - e.x1, e.y2 - e.y1) * 1.3 + 4, n = Math.max(12, Math.min(240, Math.ceil(est / 1.5)));
    var base = [], L = 0, i;
    for (i = 0; i <= n; i++) {
      var b = bez(e, i / n);
      if (i) L += Math.hypot(b[0] - base[i - 1][0], b[1] - base[i - 1][1]);
      base.push(b);
    }
    // The strip round the water: a river's reaches out to its halo; a
    // stream's, narrow, hugs it, so that it can wind tightly.
    var W = widthOf(e), T = (W + 1.0 + W * 0.25) * 1.25 + 3.2;
    var waves = river ? 0.8 + 1.2 * hsh(k, 1) : 1.6 + 2.4 * hsh(k, 1) * Math.min(1, L / 90), h2 = river ? 0.2 : 0.3, f2 = 1.7 + 0.8 * hsh(k, 3);
    var kk = 2 * Math.PI * waves / Math.max(L, 1);
    var A = (river ? 0.11 : 0.14) * L * (0.6 + 0.55 * hsh(k, 2));
    A = Math.min(A, river ? 26 : 20, 1 / (T * 0.85 * (1 + h2 * f2 * f2 + (river ? 0 : 0.5)) * kk * kk));
    var ph1 = hsh(k, 4) * 6.283, ph2 = hsh(k, 5) * 6.283, side = hsh(k, 6) < 0.5 ? -1 : 1;
    var pts = [], cum = 0;
    for (i = 0; i <= n; i++) {
      var t = i / n, a0 = base[Math.max(0, i - 1)], b0 = base[Math.min(n, i + 1)], dx = b0[0] - a0[0], dy = b0[1] - a0[1], dl = Math.hypot(dx, dy) || 1;
      var a = 6.283 * waves * t + ph1;
      var off = side * A * Math.pow(Math.sin(Math.PI * t), 1.3) * (Math.sin(a) + h2 * Math.sin(a * f2 + ph2) + (river ? 0 : 0.3 * (noise1(k + t * waves * 2.5) - 0.5) * 2));
      var x = base[i][0] - dy / dl * off, y = base[i][1] + dx / dl * off;
      if (i) cum += Math.hypot(x - pts[i - 1][0], y - pts[i - 1][1]);
      pts.push([x, y, cum]);
    }
    return (COURSES[key] = { pts: pts, len: cum, n: n, river: river, W: W, T: T });
  }
  // The course as a path, for the lit route (river.js).
  function coursePath(x1, y1, x2, y2) {
    var c = COURSES[keyOf(x1, y1, x2, y2)] || course({ x1: x1, y1: y1, x2: x2, y2: y2, d: 1 });
    return 'M' + c.pts.map(function (q) { return (Math.round(q[0] * 10) / 10) + ' ' + (Math.round(q[1] * 10) / 10); }).join('L');
  }
  // The water of the streams and of the rivers: two photographs, seen from
  // above, that repeat down a watercourse without a seam
  // (tools/pictures/tileable.py). Until they are here, no river is drawn.
  var WATER = { stream: '/static/img/river-water-stream.jpg?v=1', river: '/static/img/river-water-river.jpg?v=1' };
  var texS = null, texR = null, waterIn = 0;
  function waterTex(src, unit) {
    var t = gl.createTexture(), im = new Image();
    gl.activeTexture(unit); gl.bindTexture(gl.TEXTURE_2D, t);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 1, 1, 0, gl.RGBA, gl.UNSIGNED_BYTE, new Uint8Array([40, 110, 100, 255]));
    im.onload = function () {
      if (gl.isContextLost()) return;
      gl.activeTexture(unit); gl.bindTexture(gl.TEXTURE_2D, t);
      gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGB, gl.RGB, gl.UNSIGNED_BYTE, im);
      gl.generateMipmap(gl.TEXTURE_2D);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.REPEAT);
      waterIn++;
      if (waterIn >= 2 && rready && !travelling) stage.classList.add('rv-rivers');
      if (!running) draw(false);
    };
    im.src = src;
    return t;
  }
  if (rprog) { texS = waterTex(WATER.stream, gl.TEXTURE3); texR = waterTex(WATER.river, gl.TEXTURE4); }
  // A line's colour, as river.js gives it (#rrggbb, or rgb()).
  function rgb01(c) {
    if (typeof c !== 'string') return [0.53, 0.69, 0.56];
    if (c.charAt(0) === '#') return [1, 3, 5].map(function (i) { return parseInt(c.slice(i, i + 2), 16) / 255; });
    var m = c.match(/[\d.]+/g);
    return m && m.length >= 3 ? m.slice(0, 3).map(function (n) { return parseFloat(n) / 255; }) : [0.53, 0.69, 0.56];
  }
  // The strips: along each line's curve (river.js's own, so the lit route
  // and the glints follow the same water), a cross-section every few
  // units, reaching past the water and its banks to the end of its glow,
  // and past its two ends so that they are round. The thinner rivers
  // first, so that a tributary runs into the river it joins.
  var FLOAT = 17;
  function rivers(edges, tails) {
    if (!rprog) return;
    // A line the record continues past the window runs on and fades; a line
    // it ends dries up before its last mark.
    var list = edges.slice().concat((tails || []).map(function (t) { return Object.assign({ conf: 'a', tail: t.kind === 'cont' ? 1 : 2 }, t); }));
    list.sort(function (p, q) { return (p.river ? 1 : 0) - (q.river ? 1 : 0) || (p.d || 0) - (q.d || 0); });
    var out = [];
    function push(x, y, s, v, W, B, col, conf, a, len, ph, nx, ny) {
      out.push(x, y, s, v, W, B, col[0], col[1], col[2], conf, a, len, ph, fade, nx, ny, kind);
    }
    var fade = 0, kind = 0;
    list.forEach(function (e) {
      var c = course(e), W = c.W, B = 1.0 + W * 0.25, T = c.T;
      var conf = e.conf === 'a' ? 0 : e.conf === 'd' ? 1 : 2, col = rgb01(e.c), a = e.a === undefined ? 1 : e.a;
      var pts = c.pts, n = c.n, len = c.len, ph = Math.abs(Math.sin(e.x1 * 12.9898 + e.y2 * 78.233)) % 1;
      if (len < 0.5) return;
      fade = e.tail || 0; kind = c.river ? 1 : 0;
      function tangent(i) {
        var a0 = pts[Math.max(0, i - 1)], b0 = pts[Math.min(n, i + 1)], dx = b0[0] - a0[0], dy = b0[1] - a0[1], dl = Math.hypot(dx, dy) || 1;
        return [dx / dl, dy / dl];
      }
      var rows = [], t0 = tangent(0), t1 = tangent(n);
      rows.push([pts[0][0] - t0[0] * T, pts[0][1] - t0[1] * T, -T, t0]);
      pts.forEach(function (q, i) { rows.push([q[0], q[1], q[2], tangent(i)]); });
      rows.push([pts[n][0] + t1[0] * T, pts[n][1] + t1[1] * T, len + T, t1]);
      var first = true;
      rows.forEach(function (r) {
        var nx = -r[3][1], ny = r[3][0];
        if (first) push(r[0] + nx * T, r[1] + ny * T, T, r[2], W, B, col, conf, a, len, ph, nx, ny);
        push(r[0] + nx * T, r[1] + ny * T, T, r[2], W, B, col, conf, a, len, ph, nx, ny);
        push(r[0] - nx * T, r[1] - ny * T, -T, r[2], W, B, col, conf, a, len, ph, nx, ny);
        first = false;
      });
      // A degenerate triangle to the next strip.
      var last = rows[rows.length - 1], lnx = -last[3][1], lny = last[3][0];
      push(last[0] - lnx * T, last[1] - lny * T, -T, last[2], W, B, col, conf, a, len, ph, lnx, lny);
    });
    gl.bindBuffer(gl.ARRAY_BUFFER, rbuf);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(out), gl.STATIC_DRAW);
    rcount = out.length / FLOAT;
    rready = rcount > 0;
    stage.classList.toggle('rv-rivers', rready && waterIn >= 2);
  }
  function drawRivers(v) {
    if (!rprog || !rready || travelling || waterIn < 2) return;
    gl.useProgram(rprog);
    gl.activeTexture(gl.TEXTURE3); gl.bindTexture(gl.TEXTURE_2D, texS);
    gl.activeTexture(gl.TEXTURE4); gl.bindTexture(gl.TEXTURE_2D, texR);
    gl.bindBuffer(gl.ARRAY_BUFFER, rbuf);
    var stride = FLOAT * 4;
    [['aP', 2, 0], ['aSV', 2, 2], ['aWB', 2, 4], ['aCol', 4, 6], ['aMeta', 4, 10], ['aN', 2, 14], ['aK', 1, 16]].forEach(function (q) {
      if (RA[q[0]] < 0) return;
      gl.enableVertexAttribArray(RA[q[0]]);
      gl.vertexAttribPointer(RA[q[0]], q[1], gl.FLOAT, false, stride, q[2] * 4);
    });
    gl.uniform2f(RU.uRes, ground.width, ground.height);
    gl.uniform3f(RU.uView, v.x, v.y, v.k * dpr);
    gl.uniform1f(RU.uTime, clock);
    gl.uniform1f(RU.uK, v.k * dpr);
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
    gl.drawArrays(gl.TRIANGLE_STRIP, 0, rcount);
    gl.disable(gl.BLEND);
    ['aP', 'aSV', 'aWB', 'aCol', 'aMeta', 'aN', 'aK'].forEach(function (k) { if (RA[k] >= 0) gl.disableVertexAttribArray(RA[k]); });
    gl.useProgram(prog);
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    gl.enableVertexAttribArray(loc);
    gl.vertexAttribPointer(loc, 2, gl.FLOAT, false, 0, 0);
  }

  var U = {};
  ['uRes', 'uTime', 'uRect', 'uCam', 'uDep', 'uPic', 'uDepth', 'uSun', 'uMist', 'uShade', 'uGreen', 'uGlow', 'uView', 'uGlowRect']
    .forEach(function (k) { U[k] = gl.getUniformLocation(prog, k); });
  [['uSun', 'sun'], ['uMist', 'mist'], ['uShade', 'shade'], ['uGreen', 'green']].forEach(function (p) { gl.uniform3fv(U[p[0]], P[p[1]]); });
  gl.uniform1i(U.uPic, 0); gl.uniform1i(U.uDepth, 1); gl.uniform1i(U.uGlow, 2);
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
  var picT = texture(gl.TEXTURE0), depT = texture(gl.TEXTURE1), glowT = texture(gl.TEXTURE2), hasPic = false, hasDep = false;
  gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 1, 1, 0, gl.RGBA, gl.UNSIGNED_BYTE, new Uint8Array([0, 0, 0, 255]));
  var glowRect = [0, 0, 1, 1];
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
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGB, gl.RGB, gl.UNSIGNED_BYTE, dep);
    hasDep = true;
    if (!running) draw(false);
  };
  pic.src = PIC.src;
  dep.src = PIC.maps;

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
  var coreRgb = 'rgb(' + P.core.map(function (v) { return Math.round(v * 255); }).join(',') + ')';

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
    rings = dots.filter(function (d) { return d.x !== undefined; }).map(function (d, i) { return { x: d.x, y: d.y, c: d.role === 'centre', ph: Math.abs(Math.sin(d.x * 3.17 + d.y * 7.31 + i)) % 1 }; });
    COURSES = {};
    glow(L.edges);
    rivers(L.edges, L.tails);
    L.edges.forEach(function (e) {
      if (e.conf !== 'a') return;
      var C = RIVERS ? course(e) : null, n = C ? C.n : Math.max(6, Math.ceil((Math.hypot(e.x2 - e.x1, e.y2 - e.y1) * 1.25 + 4) / STEP));
      var xs = new Float32Array(n + 1), ys = new Float32Array(n + 1), cum = new Float32Array(n + 1), ok = new Uint8Array(n + 1);
      for (var i = 0; i <= n; i++) {
        var p = C ? C.pts[i] : bez(e, i / n);
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
  // Where the family's rivers light the ground: every line drawn wide and
  // soft into a mask, the attested ones brighter, at half a pixel per unit.
  var rings = [];
  function glow(edges) {
    var x0 = Infinity, y0 = Infinity, x1 = -Infinity, y1 = -Infinity;
    edges.forEach(function (e) { x0 = Math.min(x0, e.x1, e.x2); x1 = Math.max(x1, e.x1, e.x2); y0 = Math.min(y0, e.y1, e.y2); y1 = Math.max(y1, e.y1, e.y2); });
    if (x0 === Infinity) return;
    x0 -= 80; y0 -= 80; x1 += 80; y1 += 80;
    var S = 0.5, W = Math.max(2, Math.ceil((x1 - x0) * S)), H = Math.max(2, Math.ceil((y1 - y0) * S));
    var c = document.createElement('canvas');
    c.width = W; c.height = H;
    var m = c.getContext('2d');
    m.fillStyle = 'black'; m.fillRect(0, 0, W, H);
    if ('filter' in m) m.filter = 'blur(6px)';
    m.setTransform(S, 0, 0, S, -x0 * S, -y0 * S);
    m.lineCap = 'round'; m.strokeStyle = 'white';
    // Three fields along the courses: a wide one (red) and a near one
    // (green) for the wetter ground, and the valleys the water carved
    // (blue), wide and soft, as a height field.
    m.globalCompositeOperation = 'lighter';
    [[34, 0.22, 'rgb(255,0,0)', 6], [16, 0.45, 'rgb(255,0,0)', 6], [6, 0.8, 'rgb(0,255,0)', 3], [60, 0.35, 'rgb(0,0,255)', 12], [24, 0.4, 'rgb(0,0,255)', 7]].forEach(function (pass) {
      if ('filter' in m) m.filter = 'blur(' + pass[3] + 'px)';
      m.strokeStyle = pass[2];
      edges.forEach(function (e) {
        var c = course(e);
        var valley = pass[2] === 'rgb(0,0,255)';
        // A valley is carved however sure the parentage; a river's is wider.
        m.globalAlpha = pass[1] * (valley ? 1 : e.conf === 'a' ? 1 : 0.35) * (c.river ? 1 : 0.7);
        m.lineWidth = c.W * 2 + pass[0] * (valley && c.river ? 1.25 : 1);
        m.beginPath();
        c.pts.forEach(function (q, i) { if (i) m.lineTo(q[0], q[1]); else m.moveTo(q[0], q[1]); });
        m.stroke();
      });
    });
    gl.activeTexture(gl.TEXTURE2); gl.bindTexture(gl.TEXTURE_2D, glowT);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, c);
    glowRect = [x0, y0, 1 / (x1 - x0), 1 / (y1 - y0)];
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
    gl.uniform4f(U.uView, v.x, v.y, 1 / (v.k * dpr), 1 / (v.k * dpr));
    gl.uniform4f(U.uGlowRect, glowRect[0], glowRect[1], glowRect[2], glowRect[3]);
    gl.activeTexture(gl.TEXTURE0); gl.bindTexture(gl.TEXTURE_2D, picT);
    gl.activeTexture(gl.TEXTURE1); gl.bindTexture(gl.TEXTURE_2D, depT);
    gl.activeTexture(gl.TEXTURE2); gl.bindTexture(gl.TEXTURE_2D, glowT);
    gl.useProgram(prog);
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    gl.enableVertexAttribArray(loc);
    gl.vertexAttribPointer(loc, 2, gl.FLOAT, false, 0, 0);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
    drawRivers(v);
    wctx.setTransform(1, 0, 0, 1, 0, 0);
    wctx.clearRect(0, 0, water.width, water.height);
    if (moving && !travelling) glints(v);
  }
  var SPEED = 22;
  function glints(v) {
    var s = v.k * dpr, ox = -v.x * s, oy = -v.y * s;
    wctx.globalCompositeOperation = 'lighter';
    for (var f = 0; f < (rready ? 0 : flows.length); f++) {
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
    // Each person a marker of light, as the reference's stations are: a
    // ring opening from it and fading, every few seconds, the fixed
    // point's wider.
    wctx.setTransform(1, 0, 0, 1, 0, 0);
    wctx.strokeStyle = coreRgb;
    for (var q = 0; q < rings.length; q++) {
      var rg = rings[q], tt = (clock / 3.2 + rg.ph) % 1, rad = (rg.c ? 11 : 6) + tt * (rg.c ? 14 : 8);
      wctx.globalAlpha = (rg.c ? 0.55 : 0.4) * (1 - tt) * Math.min(1, tt * 6);
      wctx.lineWidth = (rg.c ? 1.6 : 1.1) * dpr;
      wctx.beginPath(); wctx.arc(rg.x * s + ox, rg.y * s + oy, rad * s, 0, 6.283); wctx.stroke();
    }
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
    layout: function (L) { travelling = false; layout(L); if (!running) draw(false); },
    // A travel begins: the glints wait for the river to come to rest.
    travel: function () { travelling = true; stage.classList.remove('rv-rivers'); if (!running) draw(false); },
    // A line's course, as the scene draws it: the lit route follows it.
    edge: RIVERS ? coursePath : null,
    // The window moved (zoom, pan, resize): draw it now if nothing runs.
    view: function () { if (!running) draw(false); }
  };
  size();
  if (window.axgfRiver && window.axgfRiver.layout) layout(window.axgfRiver.layout());
  // A first frame now, so the stage is never black before the first tick.
  draw(false);
  request();
})();
