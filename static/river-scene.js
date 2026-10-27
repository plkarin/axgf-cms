/* The river's living ground (v0.5.0, docs/adr/0004 §11; redrawn 2026-10-24;
 * seen at 60° and alive since 2026-10-27, §17).
 *
 * Under the drawing, a mountain valley seen from the air at golden hour, as
 * the operator's Selva Verde reference: forest, meadows, larches turning
 * gold, granite outcrops and a bed of pale sand, mist in the distance, the
 * low sun behind it on the left (static/img/river-ground.jpg, made in the
 * operator's Canva from a description, not from a photograph). It is drawn
 * in relief: a depth map of it (red in river-ground-maps.jpg) moves its near
 * parts more than its far ones as the pointer leans it and as the river is
 * zoomed. And it lives (§17): the wind runs through its trees (green in the
 * same maps), which bend and show the paler side of their leaves, and over
 * its meadows (blue) in waves; clouds cross the valley, their shadows
 * under them; herds graze and wander on the meadows and the rocks, an eagle
 * circles, birds cross (static/img/river-life.png, made in Canva); the
 * sun's rays turn; the mist drifts.
 *
 * The family's lines are its streams and rivers (§13–§16), on the ground as
 * the photograph sees it, from 60° (§17): each person stays where the
 * drawing puts them, and under them the ground is tilted, so that every
 * stream and river lies on it in perspective — narrower and flatter toward
 * the distance, its banks seen at an angle — with banks of their own,
 * pools, rapids, stones, bars of gravel in its bends and, where two
 * streams meet, a fall and its spray. Over the drawing, each person a ring
 * of light on the ground, and here and there a fish rising.
 *
 * Decoration, and the client's alone. The server's SVG and payload are
 * unchanged (ADR 0001); the water is drawn from the edges river.js already
 * holds, at the places it draws them. Without WebGL, or in print, the
 * photograph (as the stage's background) and the ink stay as they were.
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
    green: colour('--rv-riparian', [0.62, 0.8, 0.6]), sky: colour('--rv-sky-rgb', [0.78, 0.85, 0.86])
  };
  // The family's lines drawn as rivers in the ground (docs/adr/0004 §13),
  // where the theme asks for them: Canopée.
  var RIVERS = getComputedStyle(root).getPropertyValue('--rv-rivers-3d').trim() === '1';
  // The photograph and its maps (red: how near each pixel is; green: its
  // trees; blue: its meadows); how much larger than the stage it is drawn,
  // so that leaning and zooming never show its edge. The ground holds no
  // water of its own: the family's streams and rivers are its only water
  // (docs/adr/0004 §15).
  var PIC = { src: '/static/img/river-ground.jpg?v=7', maps: '/static/img/river-ground-maps.jpg?v=3', aspect: 3840 / 2160, over: 1.1 };

  /* ---- the camera (§17) ----
   * The ground seen as the photograph sees it, from the air, 60° from
   * straight down. The people stay where the drawing puts them; under them
   * the ground is tilted, its horizon above the drawing (HOR times the
   * drawing's height above its first row), so that the ground under the far
   * rows is seen smaller and flatter, as the photograph's distance is. The
   * point of the screen (x, y), in the drawing's units, is the point of the
   * ground (u, w): k = (y - yh) / (y0 - yh) is how large the ground is seen
   * there, u = (x - x0) / k across, w = (y - y0) / (cos 60° · k) toward the
   * viewer. Lengths across the view are seen k times, lengths away from it
   * cos 60° · k² times: a river running across the view is seen flatter
   * than one running away from it, as on the photograph. */
  var COS = Math.cos(Math.PI / 3), HOR = 0.8;
  var CAM = { x0: 0, y0: 0, yh: -1e6, sd: 0 };
  function kAt(y) { return Math.max(0.05, (y - CAM.yh) / (CAM.y0 - CAM.yh)); }
  function toGround(x, y) { var k = kAt(y); return [(x - CAM.x0) / k, (y - CAM.y0) / (COS * k)]; }
  function toScreen(u, w) { var q = Math.max(0.05, 1 - w * CAM.sd); return [CAM.x0 + u / q, CAM.y0 + w * COS / q]; }
  function camera(L) {
    var x0 = Infinity, x1 = -Infinity, y0 = Infinity, y1 = -Infinity;
    function see(x, y) { if (x === undefined || y === undefined) return; x0 = Math.min(x0, x); x1 = Math.max(x1, x); y0 = Math.min(y0, y); y1 = Math.max(y1, y); }
    (L.persons || []).forEach(function (p) { see(p.x, p.y); });
    (L.edges || []).concat(L.tails || []).forEach(function (e) { see(e.x1, e.y1); see(e.x2, e.y2); });
    if (x0 === Infinity) return;
    var span = Math.max(120, y1 - y0), mid = (y0 + y1) / 2, yh = y0 - span * HOR;
    CAM = { x0: (x0 + x1) / 2, y0: mid, yh: yh, sd: COS / (mid - yh) };
  }
  // The same sequence of chances on every visit, so that a family's water
  // and its herds are where they were.
  function seeded(a) {
    return function () {
      a = (a + 0x6d2b79f5) | 0;
      var t = Math.imul(a ^ (a >>> 15), 1 | a);
      t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
      return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
    };
  }

  /* ---- the ground: one triangle and one fragment shader ---- */
  var VERT = 'attribute vec2 a;void main(){gl_Position=vec4(a,0.0,1.0);}';
  var NOISE = [
    'float hash(vec2 p){vec3 q=fract(vec3(p.xyx)*0.1031);q+=dot(q,q.yzx+33.33);return fract((q.x+q.y)*q.z);}',
    'float noise(vec2 p){vec2 i=floor(p),f=fract(p),u=f*f*(3.0-2.0*f);',
    ' return mix(mix(hash(i),hash(i+vec2(1.0,0.0)),u.x),mix(hash(i+vec2(0.0,1.0)),hash(i+vec2(1.0,1.0)),u.x),u.y);}',
    'float fbm(vec2 p){float s=0.0,a=0.5;for(int i=0;i<5;i++){s+=a*noise(p);p=p*2.07+vec2(17.3,9.1);a*=0.5;}return s;}'
  ].join('\n');
  var FRAG = [
    'precision highp float;',
    'uniform vec2 uRes;uniform float uTime;uniform vec4 uRect;uniform vec2 uCam;uniform float uDep;',
    'uniform sampler2D uPic;uniform sampler2D uDepth;uniform vec3 uSun,uMist,uShade,uGreen;',
    'uniform sampler2D uGlow;uniform vec4 uView;uniform vec4 uGlowRect;uniform vec4 uHor;uniform vec3 uMeadow;',
    NOISE,
    // How much of the clearing along the water there is at a point of the
    // screen (device pixels): the near field (green) of the ground's mask.
    'float clearAt(vec2 p){vec2 w=uView.xy+p*uView.zw;float k=(w.y-uHor.z)/(uHor.y-uHor.z);',
    ' return k>0.05?texture2D(uGlow,(vec2((w.x-uHor.x)/k,(w.y-uHor.y)/(uHor.w*k))-uGlowRect.xy)*uGlowRect.zw).g:0.0;}',
    'void main(){',
    ' vec2 px=vec2(gl_FragCoord.x,uRes.y-gl_FragCoord.y);',
    ' vec2 uv=px/uRes;',
    // The photograph, each pixel looked up where the camera's shift, scaled
    // by how near it is, takes it: the spine and the near trees move more
    // than the valleys.
    ' vec2 p=(px-uRect.xy)/uRect.zw,s=uCam/uRect.zw;',
    // Where this pixel lies on the family's tilted ground (§17), and the
    // fields its water made there. Where the water runs the ground holds
    // still, so that the valleys it carved stay under it as the pointer
    // leans the landscape.
    ' vec2 wp=uView.xy+px*uView.zw;',
    ' float kk=(wp.y-uHor.z)/(uHor.y-uHor.z);',
    ' vec2 gq=vec2(-1.0),gp=vec2(0.0);',
    ' if(kk>0.05){gp=vec2((wp.x-uHor.x)/kk,(wp.y-uHor.y)/(uHor.w*kk));gq=(gp-uGlowRect.xy)*uGlowRect.zw;}',
    ' vec3 gt=texture2D(uGlow,gq).rgb;',
    ' s*=1.0-clamp(gt.b*1.6,0.0,0.9);',
    ' vec4 mp=texture2D(uDepth,clamp(p,0.0,1.0));',
    ' float d=uDep>0.5?mp.r:0.5;vec2 q=p-s*(d-0.5);',
    ' if(uDep>0.5){mp=texture2D(uDepth,clamp(q,0.0,1.0));d=mp.r;q=p-s*(d-0.5);}',
    // The wind, from the left with the clouds (§17), on the photograph's own
    // ground (its horizon at its top edge, so that what is far is small):
    // gusts run through the trees, which bend and show the paler side of
    // their leaves, and over the meadows in waves.
    ' float gz=1.0/(clamp(q.y,0.0,1.0)+0.04);vec2 gg=vec2((q.x-0.5)*gz,gz);',
    ' float gust=smoothstep(0.38,0.82,noise(vec2(gg.x*1.4-uTime*0.42,gg.y*1.2)));',
    ' float tr=uDep>0.5?mp.g:0.0,md=uDep>0.5?mp.b:0.0;',
    ' float sway=0.6*sin(uTime*4.7+gg.x*37.0+gg.y*19.0)+0.4*sin(uTime*7.9+gg.x*61.0);',
    ' q+=vec2(1.0,0.12)*tr*gust*(0.55+0.45*sway)*0.0024/gz;',
    ' vec3 col=texture2D(uPic,clamp(q,0.0,1.0)).rgb;',
    ' col+=tr*gust*(0.5+0.5*sway)*vec3(0.045,0.05,0.018);',
    ' float wave=sin(gg.x*24.0-uTime*2.2+noise(gg*2.5)*5.0);',
    ' col*=1.0+md*(0.08*wave*(0.35+0.65*gust)+0.05*(gust-0.4));',
    // The family's water clears its way through the forest (§18): a tree
    // whose foot stands in the clearing along a stream or a river is gone,
    // and the ground there is grass and moss, gravel toward the water, in
    // the shade of the trees beyond; a tree standing outside the clearing
    // still leans over it, and over the water (the rivers' own pass lets
    // it). A tree's foot is under its crown, about a third of its height
    // lower on the photograph and to either side of it, and its height
    // follows the photograph's distance.
    ' float hT=(0.1*q.y+0.01)*uRect.w,foot=0.0;',
    ' if(gt.g>0.08&&tr>0.15&&kk>0.05)foot=clearAt(px+vec2(0.0,0.3*hT));',
    ' if(foot>0.25)foot=min(foot,min(clearAt(px+vec2(-0.18*hT,0.3*hT)),clearAt(px+vec2(0.18*hT,0.3*hT))));',
    ' if(foot>0.25){',
    '  vec2 gn=gp*0.3;',
    '  float clear=smoothstep(0.25,0.6,foot)*smoothstep(0.18,0.45,gt.g+0.25*(noise(gn*0.7)-0.5))*smoothstep(0.15,0.5,tr);',
    '  vec3 grass=mix(uMeadow*0.72,uMeadow*1.15,noise(gn))*(0.86+0.28*noise(gn*4.3+3.1));',
    '  grass=mix(grass,vec3(0.58,0.54,0.45)*(0.78+0.44*noise(gn*6.1)),smoothstep(0.55,0.9,gt.g)*0.55);',
    '  float shd=uDep>0.5?smoothstep(0.45,0.85,texture2D(uDepth,clamp(q-vec2(0.3*hT/uRect.z,0.55*hT/uRect.w),0.0,1.0)).g):0.0;',
    '  col=mix(col,grass*(1.0-0.22*shd),clear);',
    ' }',
    // Mist drifting over the valleys at both edges and into the lower
    // corners, warm toward the sun, a breath of it everywhere.
    ' vec2 w=px/uRes.y;',
    ' float m=fbm(w*2.6+vec2(uTime*0.03,uTime*0.01));',
    ' float side=max(1.0-smoothstep(0.0,0.3,uv.x),smoothstep(0.7,1.0,uv.x))*(0.55+0.45*smoothstep(0.2,1.0,uv.y));',
    ' col=mix(col,mix(uMist,uSun,0.35*(1.0-smoothstep(0.0,0.5,uv.x))*(1.0-uv.y)),clamp(smoothstep(0.38,0.78,m)*side*0.6+0.04*m,0.0,0.6));',
    // The low sun's light from behind the valley, on the left, and its rays
    // across the ground, slowly turning.
    ' float sun=1.0-smoothstep(0.0,0.8,length(uv*vec2(1.0,1.35)));',
    ' col+=uSun*sun*sun*0.1;',
    ' vec2 sd=uv-vec2(-0.04,-0.1);float ang=atan(sd.y,sd.x);',
    ' float ray=smoothstep(0.55,0.95,noise(vec2(ang*11.0,uTime*0.07)))*smoothstep(1.3,0.15,length(sd));',
    ' col+=uSun*ray*0.07;',
    // The ground follows the family's water (docs/adr/0004 §16): each
    // stream and river runs in a valley it has carved, its slopes facing
    // the sun lit and the others in shade, its floor lower and wetter, and
    // along it the vegetation darker and greener, as it grows along water.
    // The valley is a height field from the blurred courses (blue), on the
    // tilted ground, lit by the same sun.
    ' float gw=gt.r;',
    ' vec2 e=uGlowRect.zw*3.0;',
    ' float hx=texture2D(uGlow,gq+vec2(e.x,0.0)).b-texture2D(uGlow,gq-vec2(e.x,0.0)).b;',
    ' float hy=texture2D(uGlow,gq+vec2(0.0,e.y)).b-texture2D(uGlow,gq-vec2(0.0,e.y)).b;',
    ' vec3 vn=normalize(vec3(hx*14.0,hy*14.0,1.0));',
    ' vec3 SUNV=normalize(vec3(-0.55,-0.62,0.56));',
    ' float slope=clamp(1.0+(dot(vn,SUNV)-SUNV.z)*1.8,0.66,1.34);',
    ' col*=mix(1.0,slope,clamp(gt.b*1.6,0.0,0.8));',
    ' col*=1.0-0.16*gt.b*gt.b;',
    ' col=mix(col,col*uGreen,clamp(smoothstep(0.6,0.95,gt.g)*0.45+gw*gw*0.25,0.0,0.6));',
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
  var DERIV = !!gl.getExtension('OES_standard_derivatives');
  function shader(type, src) {
    var s = gl.createShader(type);
    gl.shaderSource(s, src); gl.compileShader(s);
    return gl.getShaderParameter(s, gl.COMPILE_STATUS) ? s : null;
  }
  function program(vsrc, fsrc) {
    var vs = shader(gl.VERTEX_SHADER, vsrc), fs = shader(gl.FRAGMENT_SHADER, fsrc);
    if (!vs || !fs) return null;
    var p = gl.createProgram();
    gl.attachShader(p, vs); gl.attachShader(p, fs); gl.linkProgram(p);
    return gl.getProgramParameter(p, gl.LINK_STATUS) ? p : null;
  }
  var prog = program(VERT, FRAG);
  if (!prog) return;
  gl.useProgram(prog);
  var buf = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, buf);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 3, -1, -1, 3]), gl.STATIC_DRAW);
  var loc = gl.getAttribLocation(prog, 'a');
  gl.enableVertexAttribArray(loc);
  gl.vertexAttribPointer(loc, 2, gl.FLOAT, false, 0, 0);

  /* ---- the rivers: one strip per line, laid on the tilted ground ---- */
  // Each of the family's lines a stream or a river along the river's own
  // curve, laid back on the ground and seen through the camera (§17): its
  // two banks each its own, widening into pools and closing in where it
  // races, a bar of gravel and sand on the inside of its bends; its water a
  // photograph of real water, deeper and greener in the pools, turquoise
  // over the shallows, the sky in it toward the distance and the low sun
  // glittering on it; foam running down it, stones and boulders breaking
  // it, each with its wake; and where two streams meet, the river they
  // make starts in white water under a fall, or in a still pool, or in
  // rapids. An inferred parentage is a stream that runs underground and out
  // again; a speculative one, a dry trail of small pools.
  var RVERT = [
    'attribute vec2 aP;attribute vec2 aSV;attribute vec2 aWB;attribute vec4 aCol;attribute vec4 aMeta;attribute vec2 aN;attribute float aK;',
    'uniform vec2 uRes;uniform vec3 uView;uniform vec4 uCam;',
    'varying vec2 vSV;varying vec2 vWB;varying vec4 vCol;varying vec4 vMeta;varying vec2 vN;varying float vK;varying float vQ;varying vec2 vG;',
    // The ground's point through the camera: its distance in w, so that the
    // water is laid on the ground as the photograph's would be, not
    // stretched across the screen.
    'void main(){float q=1.0-aP.y*uCam.z;',
    ' vec2 px=(uCam.xy*q+vec2(aP.x,aP.y*uCam.w)-uView.xy*q)*uView.z;',
    ' gl_Position=vec4(px.x/uRes.x*2.0-q,q-px.y/uRes.y*2.0,0.0,q);',
    ' vSV=aSV;vWB=aWB;vCol=aCol;vMeta=aMeta;vN=aN;vK=aK;vQ=q;vG=aP;}'
  ].join('\n');
  var RFRAG = [
    DERIV ? '#extension GL_OES_standard_derivatives : enable' : '',
    'precision highp float;',
    'varying vec2 vSV;varying vec2 vWB;varying vec4 vCol;varying vec4 vMeta;varying vec2 vN;varying float vK;varying float vQ;varying vec2 vG;',
    'uniform float uTime,uK;uniform sampler2D uTexS,uTexR;uniform vec3 uSun,uSky;',
    'uniform sampler2D uPic,uMaps,uGlow;uniform vec4 uRect,uGlowRect,uCam;uniform vec2 uPar,uRes;uniform vec3 uView;',
    NOISE,
    // The clearing along the water at a point of the screen (device
    // pixels), as the ground's pass finds it.
    'float clearAt(vec2 p){vec2 w=uView.xy+p/uView.z;float k=1.0+(w.y-uCam.y)*uCam.z/uCam.w;',
    ' return texture2D(uGlow,(vec2((w.x-uCam.x)/k,(w.y-uCam.y)/(uCam.w*k))-uGlowRect.xy)*uGlowRect.zw).g;}',
    // Stones in the bed and boulders breaking the surface, each with its
    // pillow of white water above it and its wake below; along the banks,
    // stones on the shingle. Returns how much of a stone is here.
    'float stones(vec2 sv,float W0,float ph,float sw,float aa,float speed,out vec3 stone,inout float foam){',
    ' float cs=W0*1.15+1.2,cl=W0*2.4+2.4,rock=0.0;stone=vec3(0.0);',
    ' for(int j=0;j<2;j++){',
    '  vec2 cell=vec2(floor(sv.x/cs),floor(sv.y/cl)-float(j));',
    '  float h=hash(cell+ph*17.0);',
    '  if(h>mix(0.55,0.8,sw)){',
    '   vec2 c=(cell+vec2(0.25+0.5*hash(cell+3.1),0.25+0.5*hash(cell+7.7)))*vec2(cs,cl);',
    '   vec2 dd=sv-c;float rr=(0.16+0.24*hash(cell+9.9))*(W0+0.8);',
    '   if(j==0){float b=1.0-smoothstep(rr*0.78,rr+aa,length(dd*vec2(1.0,0.85)));',
    '    vec3 n=normalize(vec3(dd/rr,0.7));float lit=clamp(dot(n,vec3(-0.53,-0.58,0.62)),0.0,1.0);',
    '    if(b>rock){rock=b;stone=mix(vec3(0.2,0.19,0.17),vec3(0.66,0.63,0.57),lit)*(0.8+0.35*hash(cell+1.7));}}',
    '   float wk=exp(-pow(dd.x/(rr*1.2),2.0))*smoothstep(-rr*0.6,rr*0.4,dd.y)*exp(-max(dd.y,0.0)/(rr*4.5+2.0));',
    '   foam+=wk*(0.55+0.45*noise(vec2(dd.x*1.5,(dd.y-uTime*speed)*0.5)));',
    '  }',
    ' }',
    ' return rock;',
    '}',
    'void main(){',
    ' float s=vSV.x,v=vSV.y,len=vMeta.y,W0=vWB.x,B=vWB.y,conf=vCol.a,ph=vMeta.z,sw=vK>0.5?1.0:0.0,bend=vN.x,style=vN.y;',
    // How much ground one pixel covers here: more toward the distance.
    DERIV ? ' float aa=max(max(fwidth(s),fwidth(v))*0.7,0.05);' : ' float aa=vQ*vQ*2.0/uK;',
    ' float fade=vMeta.w>0.5?1.0-smoothstep(0.0,len,v)*(vMeta.w>1.5?1.0:0.7):1.0;',
    ' float side=s<0.0?-1.0:1.0;',
    // Where it runs on the photograph (§18), found as the ground's own pass
    // finds it: the ground under it and round it, blurred (its colours and
    // its light), how near it is, whether it is bare rock, and how tall the
    // trees are there.
    ' vec2 px=vec2(gl_FragCoord.x,uRes.y-gl_FragCoord.y);',
    ' vec3 gt=texture2D(uGlow,(vG-uGlowRect.xy)*uGlowRect.zw).rgb;',
    ' vec2 pp=(px-uRect.xy)/uRect.zw,ps=uPar/uRect.zw*(1.0-clamp(gt.b*1.6,0.0,0.9));',
    ' vec4 mp=texture2D(uMaps,clamp(pp,0.0,1.0));vec2 q=pp-ps*(mp.r-0.5);',
    ' mp=texture2D(uMaps,clamp(q,0.0,1.0));q=clamp(pp-ps*(mp.r-0.5),0.0,1.0);',
    ' vec3 env=texture2D(uPic,q,4.0).rgb;',
    ' float emx=max(env.r,max(env.g,env.b)),emn=min(env.r,min(env.g,env.b));',
    ' float grey=(1.0-smoothstep(0.14,0.3,(emx-emn)/(emx+0.001)))*smoothstep(0.28,0.48,emx);',
    ' float rocky=0.0;',
    ' if(grey>0.05)rocky=grey*smoothstep(0.035,0.09,length(texture2D(uPic,q,1.0).rgb-env));',
    ' float lum=dot(env,vec3(0.3,0.59,0.11)),hT=(0.1*q.y+0.01)*uRect.w;',
    // The trees beyond it shade it, the low sun behind them; far off it
    // fades into the haze, as the photograph's distance does.
    ' float shd=smoothstep(0.45,0.85,texture2D(uMaps,clamp(q-vec2(0.3*hT/uRect.z,0.55*hT/uRect.w),0.0,1.0)).g);',
    ' float far=(1.0-smoothstep(0.12,0.55,mp.r))*0.3;',
    // Its banks, each its own, never a ruled line: they wander, open into
    // pools where the water slows and close in where it races.
    ' float nb=noise(vec2(v*mix(0.07,0.03,sw),ph*13.0+side*5.7));',
    ' float nf=noise(vec2(v*0.24,ph*7.0+side*2.3));',
    ' float fine=noise(vec2(v*0.85+side*7.0,ph*5.0));',
    ' float pool=smoothstep(0.58,0.86,noise(vec2(v*mix(0.022,0.012,sw),ph*3.1+11.0)));',
    // A river that starts in a still pool (style 0) starts wide and deep.
    ' pool=max(pool,sw*(1.0-step(0.25,style))*exp(-max(v,0.0)/(W0*3.0+5.0)));',
    ' float race=smoothstep(0.56,0.84,noise(vec2(v*mix(0.03,0.016,sw),ph*5.3+29.0)))*(1.0-pool);',
    ' float Rb=(W0+B)*(0.8+0.4*nb)*(0.94+0.12*nf)*(0.95+0.1*fine)*(1.0+0.38*pool)*(1.0-0.22*race);',
    // How far from its middle, round at its two ends.
    ' float d=v<0.0?length(vec2(v,s)):(v>len?length(vec2(v-len,s)):abs(s));',
    // Past its bank, only the shadow of its bed in the ground: uneven,
    // mossy rather than black; and past that, nothing (most of the strip).
    ' if(d>Rb+aa){',
    '  float sh=(1.0-smoothstep(Rb,Rb+2.2+Rb*0.3,d))*0.2*(0.55+0.45*nb)*vMeta.x*fade;',
    '  if(sh<0.004)discard;',
    '  gl_FragColor=vec4(vec3(0.04,0.06,0.03)*sh,sh);return;',
    ' }',
    // How sure the parentage is: an inferred one runs underground and out
    // again, leaving a dry bed; a speculative one, a trail of small pools.
    ' float wet=1.0;',
    ' if(conf>0.5&&conf<1.5){float seg=fract((v+ph*31.0)/(10.0+Rb*2.4));wet=smoothstep(0.0,0.05,seg)*(1.0-smoothstep(0.62,0.68,seg));}',
    ' if(conf>1.5){float m=mod(v+ph*17.0,7.0)-3.5;wet=1.0-smoothstep(1.4,1.4+aa*1.5,length(vec2(m,s)));}',
    ' float inB=1.0-smoothstep(Rb-aa,Rb+aa,d);',
    // A tree of the near bank, its foot outside the clearing, leans over
    // the water and hides its edge; the water's middle always shows, so
    // that the family's line is never lost.
    ' float crown=smoothstep(0.4,0.75,mp.g)*smoothstep(0.45,0.8,d/(W0+B));',
    ' if(crown>0.0)crown*=1.0-smoothstep(0.25,0.6,min(clearAt(px+vec2(0.0,0.3*hT)),min(clearAt(px+vec2(-0.18*hT,0.3*hT)),clearAt(px+vec2(0.18*hT,0.3*hT)))));',
    ' float a=inB*mix(conf>1.5?0.0:0.28,1.0,wet)*vMeta.x*fade*(1.0-0.85*crown);',
    // A bar of gravel on one bank here and there, and on the inside of
    // every bend, where the water leaves the bank: gravel and pale sand,
    // wet and darker near the water, and stones on it.
    ' float inner=clamp(bend*side*(W0+B)*5.0,0.0,1.0);',
    ' float bar=max(inner,smoothstep(0.64,0.86,noise(vec2(v*mix(0.05,0.025,sw),ph*9.0+side*3.3))));',
    ' float Rw=Rb*(1.0-0.45*bar*(0.6+0.4*nf));',
    ' float speed=mix(24.0,7.0,sw),foam=0.0;vec3 stone;',
    ' float rock=stones(vec2(s,v),W0,ph,sw,aa,speed,stone,foam);',
    ' float pg=noise(vec2(s,v)*0.8+ph*5.0),pf=hash(floor(vec2(s,v)*1.6+ph*3.0));',
    ' vec3 gravel=mix(vec3(0.42,0.38,0.31),vec3(0.74,0.68,0.56),pg)*(0.82+0.3*pf);',
    ' gravel=mix(gravel*0.62,gravel,smoothstep(Rw,Rw+(Rb-Rw)*0.5+aa,d));',
    ' gravel=mix(gravel,env*(0.85+0.3*pf),grey*0.6);',
    ' gravel=mix(gravel,stone,rock);',
    ' if(d>Rw+aa){',
    '  vec3 g=mix(gravel*0.55+vec3(0.06,0.05,0.03),gravel,wet)*(1.0-0.38*shd);',
    '  g=mix(g,env,far);',
    '  gl_FragColor=vec4(g*a,a);return;',
    ' }',
    // The water itself: a photograph of a stream or of a river laid along
    // the course, repeating down it, each course its own stretch of it,
    // mirrored or not.
    ' float flip=ph>0.5?-1.0:1.0;',
    ' float x=clamp(0.5+flip*s/(2.0*Rb)*0.96,0.0,1.0);',
    ' float tile=mix(6.5,9.0,sw)*(W0+B)*(0.85+0.3*fract(ph*7.13));',
    ' float y=(v+ph*1730.0)/tile;',
    ' float cw=1.0-smoothstep(0.5,0.95,d/Rw);',
    ' vec3 col;',
    ' if(sw>0.5){',
    // A river: its water slides downstream, its banks stay where they are.
    '  vec2 rp=vec2(x*9.0,(v-uTime*7.0)*0.18);',
    '  vec2 off=vec2(noise(rp)-0.5,noise(rp+5.1)-0.5)*0.012*cw;',
    '  col=mix(texture2D(uTexR,vec2(x,y)).rgb,texture2D(uTexR,vec2(x,y-uTime*7.0/tile)+off).rgb,cw);',
    ' }else{',
    // A stream: the water trembles over its stones.
    '  vec2 rp=vec2(x*14.0,(v-uTime*24.0)*0.5);',
    '  vec2 off=vec2(noise(rp)-0.5,noise(rp+3.7)-0.5)*0.016*cw;',
    '  col=texture2D(uTexS,vec2(x,y)+off).rgb;',
    ' }',
    // Deep and green where it pools and down its middle, clear turquoise
    // where it runs shallow and open, the bed seen through it at its edge;
    // its own depth of water, and a little of the line's own colour (the
    // river's ramp of the years).
    ' float deep=(1.0-smoothstep(0.0,0.9,d/Rw))*(0.35+0.65*pool)*mix(0.6,1.0,sw);',
    ' col=mix(col,col*vec3(0.5,0.74,0.76),deep*0.5);',
    ' col=mix(col,col*vec3(0.84,1.05,1.12),0.38*cw*(1.0-pool));',
    ' col*=0.9+0.22*fract(ph*3.71);',
    ' col=mix(vec3(dot(col,vec3(0.299,0.587,0.114))),col,1.18)*1.04;',
    ' col=mix(col,col*0.62+vec3(0.11,0.1,0.05),smoothstep(0.55,1.0,d/Rw)*0.4);',
    ' col=mix(col,col*0.7+vCol.rgb*0.3,0.12);',
    // It takes the colours and the light of what is round it: darker
    // under the forest, golden through the meadows.
    ' col=mix(col,env*1.3+uSky*0.08,0.28);',
    ' col*=0.6+0.52*smoothstep(0.05,0.45,lum);',
    // The sky in it, more as it runs away from the viewer, and the low sun
    // glittering on it.
    ' float fres=0.06+0.5*smoothstep(0.8,2.2,vQ);',
    // Open water always holds a little of the sky down its middle, even
    // under the forest, so that the family's line reads.
    ' col+=uSky*(0.1+0.14*(1.0-smoothstep(0.25,0.5,lum)))*(1.0-smoothstep(0.0,0.55,d/Rw));',
    ' col=mix(col,uSky,fres*cw*0.55);',
    ' float glt=pow(smoothstep(0.62,1.0,noise(vec2(x*28.0,(v-uTime*speed*0.4)*0.9))),4.0);',
    ' col+=uSun*glt*(0.22+0.5*fres)*cw;',
    // White water: streaks running down it, thicker where it races; where
    // two streams fall into the family's pool the river churns white for a
    // while from its start (style 1), or runs down rapids (style 0.5); a
    // stream hurries and breaks as it nears the fall; and down a stream,
    // steps of small falls, each with its pool.
    ' float streak=noise(vec2(s/(W0+0.6)*2.6+2.0*nf,(v-uTime*speed*1.5)*0.22));',
    ' float churn=sw*smoothstep(0.25,0.75,style)*exp(-max(v,0.0)/(W0*(1.2+1.6*style)+4.0));',
    ' float lip=(1.0-sw)*style*smoothstep(len-W0*6.0-8.0,len,v);',
    ' float cv=fract((v+ph*97.0)/(26.0+44.0*ph));',
    ' float boil=noise(vec2(s*0.9+uTime*1.1,v*0.6-uTime*speed*0.12));',
    ' float fall=(1.0-sw)*exp(-pow((cv-0.5)*15.0,2.0))*(0.55+0.45*boil);',
    ' foam+=pow(streak,3.0)*(0.25+race*1.5+churn*2.2+lip*1.4+rocky*2.4)+fall*0.75+churn*0.8*boil*boil+(1.0-sw)*0.12*pow(streak,5.0)+rocky*0.35*boil;',
    ' foam=clamp(foam,0.0,1.0)*cw;',
    ' col=mix(col,vec3(0.93,0.97,0.96)*(0.75+0.3*smoothstep(0.05,0.4,lum)),foam*0.85);',
    ' float inW=1.0-smoothstep(Rw-aa,Rw+aa,d);',
    ' col=mix(gravel,col,inW);',
    ' col=mix(col,stone,rock);',
    ' col*=1.0-0.38*shd;',
    ' col=mix(col,env,far);',
    ' col=mix(col*0.55+vec3(0.06,0.05,0.03),col,wet);',
    ' gl_FragColor=vec4(col*a,a);',
    '}'
  ].join('\n');
  var rprog = null, rbuf = null, RA = {}, RU = {}, rcount = 0, rready = false;
  if (RIVERS) {
    rprog = program(RVERT, RFRAG);
    if (rprog) {
      ['aP', 'aSV', 'aWB', 'aCol', 'aMeta', 'aN', 'aK'].forEach(function (k) { RA[k] = gl.getAttribLocation(rprog, k); });
      ['uRes', 'uView', 'uCam', 'uTime', 'uK', 'uTexS', 'uTexR', 'uSun', 'uSky', 'uPic', 'uMaps', 'uGlow', 'uRect', 'uPar', 'uGlowRect'].forEach(function (k) { RU[k] = gl.getUniformLocation(rprog, k); });
      gl.useProgram(rprog);
      gl.uniform3fv(RU.uSun, P.sun);
      gl.uniform3fv(RU.uSky, P.sky);
      gl.uniform1i(RU.uTexS, 3); gl.uniform1i(RU.uTexR, 4);
      gl.uniform1i(RU.uPic, 0); gl.uniform1i(RU.uMaps, 1); gl.uniform1i(RU.uGlow, 2);
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
  // bends are never tighter than the water and its banks can follow. Under
  // Canopée it winds on the tilted ground (§17): the curve, seen on the
  // screen, is laid back on the ground, the bends are made there, and the
  // course is seen again through the camera.
  var COURSES = {};
  function hsh(k, i) { var h = Math.sin(k * 127.1 + i * 311.7) * 43758.5453; return h - Math.floor(h); }
  function noise1(x) { var i = Math.floor(x), f = x - i, u = f * f * (3 - 2 * f); return hsh(i, 0.37) + (hsh(i + 1, 0.37) - hsh(i, 0.37)) * u; }
  function keyOf(x1, y1, x2, y2) { return [x1, y1, x2, y2].map(function (v) { return Math.round(v * 10); }).join(','); }
  function widthOf(e) {
    var w = WIDTHS[cls(e.d)] * 0.7 + 1.2;
    return e.river ? w * 1.25 + 0.8 : Math.max(1.1, w * 0.55);
  }
  function lay(b) { return RIVERS ? toGround(b[0], b[1]) : b; }
  function course(e) {
    var key = keyOf(e.x1, e.y1, e.x2, e.y2);
    if (COURSES[key]) return COURSES[key];
    var river = !!e.river, k = (Math.abs(Math.sin(e.x1 * 12.9898 + e.y1 * 4.1414 + e.x2 * 78.233 + e.y2 * 37.719)) * 43758.5453) % 1000;
    var g1 = lay([e.x1, e.y1]), g2 = lay([e.x2, e.y2]);
    var est = Math.hypot(g2[0] - g1[0], g2[1] - g1[1]) * 1.4 + 4, n = Math.max(12, Math.min(320, Math.ceil(est / 1.5)));
    var base = [], L = 0, i;
    for (i = 0; i <= n; i++) {
      var b = lay(bez(e, i / n));
      if (i) L += Math.hypot(b[0] - base[i - 1][0], b[1] - base[i - 1][1]);
      base.push(b);
    }
    // The strip round the water: wide enough for its pools, its bars and
    // the shadow of its bed.
    var W = widthOf(e), T = (W * 1.25 + 1.0) * 2.2 + 3.2;
    var waves = river ? 0.8 + 1.2 * hsh(k, 1) : 1.6 + 2.4 * hsh(k, 1) * Math.min(1, L / 90), h2 = river ? 0.2 : 0.3, f2 = 1.7 + 0.8 * hsh(k, 3);
    var kk = 2 * Math.PI * waves / Math.max(L, 1);
    var A = (river ? 0.11 : 0.14) * L * (0.6 + 0.55 * hsh(k, 2));
    A = Math.min(A, river ? 34 : 26, 1 / (T * 0.85 * (1 + h2 * f2 * f2 + (river ? 0 : 0.5)) * kk * kk));
    var ph1 = hsh(k, 4) * 6.283, ph2 = hsh(k, 5) * 6.283, side = hsh(k, 6) < 0.5 ? -1 : 1;
    var pts = [], spts = [], cum = 0;
    for (i = 0; i <= n; i++) {
      var t = i / n, a0 = base[Math.max(0, i - 1)], b0 = base[Math.min(n, i + 1)], dx = b0[0] - a0[0], dy = b0[1] - a0[1], dl = Math.hypot(dx, dy) || 1;
      var a = 6.283 * waves * t + ph1;
      var off = side * A * Math.pow(Math.sin(Math.PI * t), 1.3) * (Math.sin(a) + h2 * Math.sin(a * f2 + ph2) + (river ? 0 : 0.3 * (noise1(k + t * waves * 2.5) - 0.5) * 2));
      var x = base[i][0] - dy / dl * off, y = base[i][1] + dx / dl * off;
      if (i) cum += Math.hypot(x - pts[i - 1][0], y - pts[i - 1][1]);
      pts.push([x, y, cum]);
      spts.push(RIVERS ? toScreen(x, y) : [x, y]);
    }
    return (COURSES[key] = { pts: pts, spts: spts, len: cum, n: n, river: river, W: W, T: T });
  }
  // The course as a path on the screen, for the lit route (river.js).
  function coursePath(x1, y1, x2, y2) {
    var c = COURSES[keyOf(x1, y1, x2, y2)] || course({ x1: x1, y1: y1, x2: x2, y2: y2, d: 1 });
    return 'M' + c.spts.map(function (q) { return (Math.round(q[0] * 10) / 10) + ' ' + (Math.round(q[1] * 10) / 10); }).join('L');
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
  // The strips: along each line's course on the ground, a cross-section
  // every few units, reaching past the water, its bars and its banks, and
  // past its two ends so that they are round. The thinner rivers first, so
  // that a tributary runs into the river it joins. Each cross-section
  // carries how the course bends there (its bars) and how the water meets
  // its family (its fall, its pool, its rapids).
  var FLOAT = 17;
  function rivers(edges, tails) {
    if (!rprog) return;
    // A line the record continues past the window runs on and fades; a line
    // it ends dries up before its last mark.
    var list = edges.slice().concat((tails || []).map(function (t) { return Object.assign({ conf: 'a', tail: t.kind === 'cont' ? 1 : 2 }, t); }));
    list.sort(function (p, q) { return (p.river ? 1 : 0) - (q.river ? 1 : 0) || (p.d || 0) - (q.d || 0); });
    var out = [];
    function push(x, y, s, v, W, B, col, conf, a, len, ph, bend, style) {
      out.push(x, y, s, v, W, B, col[0], col[1], col[2], conf, a, len, ph, fade, bend, style, kind);
    }
    var fade = 0, kind = 0;
    list.forEach(function (e) {
      var c = course(e), W = c.W, B = 1.0 + W * 0.25, T = c.T;
      var conf = e.conf === 'a' ? 0 : e.conf === 'd' ? 1 : 2, col = rgb01(e.c), a = e.a === undefined ? 1 : e.a;
      var pts = c.pts, n = c.n, len = c.len, ph = Math.abs(Math.sin(e.x1 * 12.9898 + e.y2 * 78.233)) % 1;
      if (len < 0.5) return;
      fade = e.tail || 0; kind = c.river ? 1 : 0;
      var style = e.style === undefined ? 0.5 : e.style;
      function tangent(i) {
        var a0 = pts[Math.max(0, i - 1)], b0 = pts[Math.min(n, i + 1)], dx = b0[0] - a0[0], dy = b0[1] - a0[1], dl = Math.hypot(dx, dy) || 1;
        return [dx / dl, dy / dl];
      }
      // How it bends at each point: the turn of its direction per unit of
      // its length, toward its +s side when positive, smoothed.
      var tg = pts.map(function (q, i) { return tangent(i); }), raw = pts.map(function (q, i) {
        var i0 = Math.max(0, i - 2), i1 = Math.min(n, i + 2), dl = (pts[i1][2] - pts[i0][2]) || 1;
        return ((tg[i1][0] - tg[i0][0]) * -tg[i][1] + (tg[i1][1] - tg[i0][1]) * tg[i][0]) / dl;
      });
      var bend = raw.map(function (b, i) { var s = 0, m = 0; for (var j = Math.max(0, i - 3); j <= Math.min(n, i + 3); j++) { s += raw[j]; m++; } return s / m; });
      var rows = [], t0 = tg[0], t1 = tg[n];
      rows.push([pts[0][0] - t0[0] * T, pts[0][1] - t0[1] * T, -T, t0, bend[0]]);
      pts.forEach(function (q, i) { rows.push([q[0], q[1], q[2], tg[i], bend[i]]); });
      rows.push([pts[n][0] + t1[0] * T, pts[n][1] + t1[1] * T, len + T, t1, bend[n]]);
      var first = true;
      rows.forEach(function (r) {
        var nx = -r[3][1], ny = r[3][0];
        if (first) push(r[0] + nx * T, r[1] + ny * T, T, r[2], W, B, col, conf, a, len, ph, r[4], style);
        push(r[0] + nx * T, r[1] + ny * T, T, r[2], W, B, col, conf, a, len, ph, r[4], style);
        push(r[0] - nx * T, r[1] - ny * T, -T, r[2], W, B, col, conf, a, len, ph, r[4], style);
        first = false;
      });
      // A degenerate triangle to the next strip.
      var last = rows[rows.length - 1], lnx = -last[3][1], lny = last[3][0];
      push(last[0] - lnx * T, last[1] - lny * T, -T, last[2], W, B, col, conf, a, len, ph, last[4], style);
    });
    gl.bindBuffer(gl.ARRAY_BUFFER, rbuf);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(out), gl.STATIC_DRAW);
    rcount = out.length / FLOAT;
    rready = rcount > 0;
    stage.classList.toggle('rv-rivers', rready && waterIn >= 2);
  }
  function drawRivers(v, rect, par) {
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
    gl.uniform4f(RU.uCam, CAM.x0, CAM.y0, CAM.sd, COS);
    gl.uniform1f(RU.uTime, clock);
    gl.uniform1f(RU.uK, v.k * dpr);
    gl.uniform4f(RU.uRect, rect[0], rect[1], rect[2], rect[3]);
    gl.uniform2f(RU.uPar, par[0], par[1]);
    gl.uniform4f(RU.uGlowRect, glowRect[0], glowRect[1], glowRect[2], glowRect[3]);
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
    gl.drawArrays(gl.TRIANGLE_STRIP, 0, rcount);
    gl.disable(gl.BLEND);
    ['aP', 'aSV', 'aWB', 'aCol', 'aMeta', 'aN', 'aK'].forEach(function (k) { if (RA[k] >= 0) gl.disableVertexAttribArray(RA[k]); });
  }

  /* ---- life: the herds, the eagle, the birds, the spray of the falls ----
   * Pictures from one atlas (static/img/river-life.png: red deer, chamois
   * and ibex, wild horses and an eagle seen from above, made in the
   * operator's Canva and cut out; the birds' four wingbeats are drawn
   * here), and soft blots: the shade under each animal, away from the sun,
   * and the spray. Each herd keeps to its own ground on the photograph —
   * the deer and the horses to the meadows, the chamois and the ibex to the
   * rocks and the grass round them — grazing, raising its head, walking on
   * a few steps; the herd drifts across its ground over the minutes. Seen
   * as the photograph sees them: smaller toward the distance, and drawn
   * with the relief, so that they stay on their ground as it leans. */
  var LIFE = { src: '/static/img/river-life.png?v=1', w: 1024, h: 512 };
  // In the atlas: x, y, width, height, which way the head is (1 right, -1
  // left), grazing (head down).
  var SPR = {
    deer: [[2, 2, 74, 120, -1, 0], [78, 2, 83, 120, 1, 0], [163, 2, 90, 120, -1, 1], [255, 2, 85, 120, 1, 0], [342, 2, 80, 120, -1, 0], [424, 2, 88, 120, 1, 0], [514, 2, 66, 120, 1, 0], [582, 2, 64, 120, -1, 0]],
    ibex: [[648, 2, 91, 120, 1, 0], [741, 2, 72, 120, 1, 0], [815, 2, 95, 120, -1, 1], [912, 2, 86, 120, 1, 0], [2, 124, 96, 120, 1, 0], [100, 124, 77, 120, 1, 0], [179, 124, 103, 120, 1, 1], [284, 124, 101, 120, 1, 0]],
    horse: [[387, 124, 90, 120, -1, 0], [479, 124, 106, 120, -1, 0], [587, 124, 87, 120, -1, 1], [676, 124, 93, 120, -1, 0], [771, 124, 114, 120, -1, 0], [887, 124, 98, 120, -1, 0]],
    eagle: [[2, 246, 200, 74], [204, 246, 200, 107]],
    bird: [[600, 400, 44, 24], [648, 400, 44, 24], [696, 400, 44, 24], [744, 400, 44, 24]]
  };
  // Each kind: its height against the photograph's (at its foot), its
  // walking pace in its own heights a second, its standing and walking
  // pictures, its grazing ones, its young and its leader.
  var KIND = {
    deer: { h: 0.045, pace: 0.3, walk: [0, 4, 7, 1, 6], graze: [2], young: 5, lead: 3 },
    ibex: { h: 0.04, pace: 0.26, walk: [0, 4, 5, 1], graze: [2, 6], young: 7, lead: 3 },
    horse: { h: 0.054, pace: 0.34, walk: [0, 3, 4, 1], graze: [2], young: 5, lead: -1 }
  };
  // The herds, each on its own ground of the photograph: its centre and
  // reach, in the photograph's own coordinates.
  var HERDS = [
    { kind: 'deer', u: 0.17, v: 0.5, ru: 0.08, rv: 0.1, n: 6 },
    { kind: 'horse', u: 0.75, v: 0.45, ru: 0.06, rv: 0.04, n: 5 },
    { kind: 'ibex', u: 0.36, v: 0.53, ru: 0.03, rv: 0.025, n: 4, rock: true },
    { kind: 'ibex', u: 0.2, v: 0.86, ru: 0.045, rv: 0.04, n: 3, rock: true },
    { kind: 'deer', u: 0.91, v: 0.68, ru: 0.05, rv: 0.05, n: 4 },
    { kind: 'deer', u: 0.44, v: 0.24, ru: 0.06, rv: 0.035, n: 5 },
    { kind: 'horse', u: 0.45, v: 0.6, ru: 0.06, rv: 0.05, n: 4 }
  ];
  var LVERT = [
    'attribute vec2 aP;attribute vec2 aT;attribute vec4 aC;',
    'uniform vec2 uRes;varying vec2 vT;varying vec4 vC;',
    'void main(){gl_Position=vec4(aP.x/uRes.x*2.0-1.0,1.0-aP.y/uRes.y*2.0,0.0,1.0);vT=aT;vC=aC;}'
  ].join('\n');
  // A picture of the atlas (its texture coordinates, its colour multiplied
  // by the light), or a soft blot (negative coordinates: the blot's own,
  // centred on -2).
  var LFRAG = [
    'precision mediump float;',
    'uniform sampler2D uLife;varying vec2 vT;varying vec4 vC;',
    'void main(){',
    ' if(vT.x>=0.0){gl_FragColor=texture2D(uLife,vT)*vec4(vC.rgb,1.0)*vC.a;}',
    ' else{vec2 b=vT+vec2(2.0,0.0);float a=(1.0-smoothstep(0.15,1.0,length(b)))*vC.a;gl_FragColor=vec4(vC.rgb*a,a);}',
    '}'
  ].join('\n');
  var lprog = program(LVERT, LFRAG), lbuf = null, LA = {}, LU = {}, lifeT = null, lifeIn = false;
  var LDATA = new Float32Array(8 * 6 * 240);
  if (lprog) {
    ['aP', 'aT', 'aC'].forEach(function (k) { LA[k] = gl.getAttribLocation(lprog, k); });
    ['uRes', 'uLife'].forEach(function (k) { LU[k] = gl.getUniformLocation(lprog, k); });
    gl.useProgram(lprog);
    gl.uniform1i(LU.uLife, 5);
    gl.useProgram(prog);
    lbuf = gl.createBuffer();
    var lim = new Image();
    lim.onload = function () {
      if (gl.isContextLost()) return;
      var c = document.createElement('canvas');
      c.width = LIFE.w; c.height = LIFE.h;
      var x = c.getContext('2d');
      x.drawImage(lim, 0, 0);
      // A bird far off, its four wingbeats: a body and two wings, raised,
      // level, lowered, level.
      [-0.55, -0.15, 0.4, -0.15].forEach(function (lift, f) {
        var b = SPR.bird[f], cx = b[0] + b[2] / 2, cy = b[1] + b[3] / 2 + 2;
        x.save();
        x.fillStyle = 'rgb(38,34,30)'; x.strokeStyle = 'rgb(38,34,30)'; x.lineCap = 'round'; x.lineWidth = 2.6;
        x.beginPath(); x.ellipse(cx, cy, 6, 2.6, 0, 0, 6.283); x.fill();
        [-1, 1].forEach(function (sd) {
          x.beginPath(); x.moveTo(cx + sd * 3, cy - 1);
          x.quadraticCurveTo(cx + sd * 11, cy - 1 + lift * 16, cx + sd * 20, cy + lift * 10);
          x.stroke();
        });
        x.restore();
      });
      lifeT = gl.createTexture();
      gl.activeTexture(gl.TEXTURE5); gl.bindTexture(gl.TEXTURE_2D, lifeT);
      gl.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL, true);
      gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, c);
      gl.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL, false);
      gl.generateMipmap(gl.TEXTURE_2D);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
      lifeIn = true;
      if (!running) draw(false);
    };
    lim.src = LIFE.src;
  }
  // The photograph's maps, read once, small: where the meadows, the trees
  // and the rocks are, for the herds.
  var MAP = null;
  function mapAt(u, v, c) {
    if (!MAP) return 0;
    var x = Math.max(0, Math.min(MAP.w - 1, Math.floor(u * MAP.w))), y = Math.max(0, Math.min(MAP.h - 1, Math.floor(v * MAP.h)));
    return MAP.d[(y * MAP.w + x) * 4 + c] / 255;
  }
  function allowed(H, u, v) {
    if (u < 0.02 || u > 0.98 || v < 0.14 || v > 0.97) return false;
    var tr = mapAt(u, v, 1), md = mapAt(u, v, 2);
    return H.rock ? tr < 0.3 : md > 0.4 && tr < 0.35;
  }
  var animals = [];
  function herds() {
    var rnd = seeded(20261027);
    animals = [];
    HERDS.forEach(function (H, hi) {
      var K = KIND[H.kind];
      H.cu = H.u; H.cv = H.v; H.move = 25 + rnd() * 30;
      for (var i = 0; i < H.n; i++) {
        var u = H.u, v = H.v;
        for (var t = 0; t < 16; t++) {
          var a = rnd() * 6.283, r = Math.sqrt(rnd());
          u = H.u + Math.cos(a) * H.ru * r; v = H.v + Math.sin(a) * H.rv * r;
          if (allowed(H, u, v)) break;
        }
        var lead = i === 0 && K.lead >= 0, young = i === H.n - 1 && H.n > 3;
        var ws = lead ? K.lead : young ? K.young : K.walk[Math.floor(rnd() * K.walk.length)];
        animals.push({
          herd: H, kind: H.kind, x: u, y: v, tx: u, ty: v, face: rnd() < 0.5 ? -1 : 1, state: 'graze',
          until: rnd() * 8, walk: ws, graze: lead || young ? ws : K.graze[Math.floor(rnd() * K.graze.length)],
          size: young ? 0.72 : lead ? 1.12 : 0.92 + rnd() * 0.16, bob: rnd() * 6.283, rnd: seeded(hi * 131 + i * 17 + 7)
        });
      }
    });
  }
  // What each animal does next: graze a while, raise its head, or walk a
  // few steps to fresh grass near its herd.
  function think(an, t) {
    var H = an.herd, r = an.rnd();
    if (an.state !== 'walk' && r < 0.5) {
      for (var k = 0; k < 8; k++) {
        var a = an.rnd() * 6.283, d = 0.25 + 0.75 * an.rnd();
        var u = H.cu + Math.cos(a) * H.ru * d, v = H.cv + Math.sin(a) * H.rv * d;
        if (allowed(H, u, v)) { an.tx = u; an.ty = v; an.state = 'walk'; an.until = t + 40; return; }
      }
    }
    an.state = r < 0.82 ? 'graze' : 'look';
    an.until = t + (an.state === 'graze' ? 4 + 9 * an.rnd() : 1.5 + 2.5 * an.rnd());
  }
  function live(dt, t) {
    HERDS.forEach(function (H) {
      // The herd drifts across its ground, over the minutes.
      if (t > H.move) {
        var r = seeded(Math.floor(t * 7) + H.n)();
        for (var k = 0; k < 6; k++) {
          var a = r * 6.283 + k, u = H.u + Math.cos(a) * H.ru * 0.6, v = H.v + Math.sin(a) * H.rv * 0.6;
          if (allowed(H, u, v)) { H.cu = u; H.cv = v; break; }
        }
        H.move = t + 30 + r * 40;
      }
    });
    animals.forEach(function (an) {
      if (an.state === 'walk') {
        var K = KIND[an.kind], dx = an.tx - an.x, dy = an.ty - an.y, dist = Math.hypot(dx, dy);
        // Its pace in its own heights, so the far ones walk as slowly as
        // they are small.
        var sp = K.pace * K.h * an.size * an.y * 1.4 * dt;
        if (dist <= sp || t > an.until) { an.x = an.tx; an.y = an.ty; an.state = 'graze'; an.until = t; }
        else { an.x += dx / dist * sp; an.y += dy / dist * sp; if (Math.abs(dx) > dist * 0.2) an.face = dx > 0 ? 1 : -1; }
      }
      if (t >= an.until && an.state !== 'walk') think(an, t);
    });
  }
  // Where a fall's spray rises: each meeting of two streams that falls.
  var FALLS = [];
  var EAGLE = { u: 0.62, v: 0.25, r: 0.14, period: 74 };
  var flock = null, flockAt = 18;
  var lifeClock = -1;
  function drawLife(v, R, moving) {
    if (!lprog || !lifeIn) return;
    if (moving && MAP && animals.length) live(lifeClock < 0 ? 0 : Math.min(0.2, clock - lifeClock), clock);
    lifeClock = clock;
    var D = LDATA, n = 0;
    function quad(Q, T, c, a) {
      var tu = [T[0], T[2], T[2], T[0]], tv = [T[1], T[1], T[3], T[3]], idx = [0, 1, 2, 0, 2, 3];
      if (n + 48 > D.length) return;
      for (var j = 0; j < 6; j++) {
        var q = idx[j];
        D[n++] = Q[q * 2]; D[n++] = Q[q * 2 + 1]; D[n++] = tu[q]; D[n++] = tv[q];
        D[n++] = c[0]; D[n++] = c[1]; D[n++] = c[2]; D[n++] = a;
      }
    }
    function blot(x, y, rx, ry, c, a) { quad([x - rx, y - ry, x + rx, y - ry, x + rx, y + ry, x - rx, y + ry], [-3, -1, -1, 1], c, a); }
    function tex(r) { return [r[0] / LIFE.w, r[1] / LIFE.h, (r[0] + r[2]) / LIFE.w, (r[1] + r[3]) / LIFE.h]; }
    // A picture turned by `ang` and seen at the camera's angle (a flat thing
    // seen from above: the eagle, the birds).
    function turned(r, x, y, w, ang, squash, c, a) {
      var h = w * r[3] / r[2], cs = Math.cos(ang), sn = Math.sin(ang), Q = [];
      [[-w / 2, -h / 2], [w / 2, -h / 2], [w / 2, h / 2], [-w / 2, h / 2]].forEach(function (p) {
        Q.push(x + p[0] * cs - p[1] * sn, y + (p[0] * sn + p[1] * cs) * squash);
      });
      quad(Q, tex(r), c, a);
    }
    var SHADE = [0.02, 0.03, 0.02], SPRAY = [0.94, 0.97, 0.97];
    function at(u, w) {
      var d = mapAt(u, w, 0);
      return [R.x + u * R.w + R.cx * (d - 0.5), R.y + w * R.h + R.cy * (d - 0.5)];
    }
    // The herds, the far ones first; under each its shade, away from the
    // sun; the far ones hazier.
    if (MAP && animals.length) {
      var list = animals.slice().sort(function (a, b) { return a.y - b.y; });
      list.forEach(function (an) {
        var K = KIND[an.kind], p = at(an.x, an.y), h = K.h * an.size * an.y * R.h, r = SPR[an.kind][an.walk];
        var w = h * r[2] / r[3];
        blot(p[0] + w * 0.24, p[1] - h * 0.03, w * 0.56 + h * 0.1, h * 0.12, SHADE, 0.34);
      });
      list.forEach(function (an) {
        var K = KIND[an.kind], p = at(an.x, an.y), h = K.h * an.size * an.y * R.h;
        var r = SPR[an.kind][an.state === 'graze' ? an.graze : an.walk], w = h * r[2] / r[3];
        var bob = an.state === 'walk' ? Math.abs(Math.sin(clock * 6 + an.bob)) * h * 0.035 : 0;
        var T = tex(r);
        if (r[4] !== an.face) { var t0 = T[0]; T[0] = T[2]; T[2] = t0; }
        var y = p[1] - bob, far = Math.max(0, 0.55 - an.y) * 0.9;
        quad([p[0] - w / 2, y - h, p[0] + w / 2, y - h, p[0] + w / 2, y, p[0] - w / 2, y], T,
          [1 - far * 0.25 + far * 0.2, 0.97 - far * 0.24 + far * 0.22, 0.9 - far * 0.22 + far * 0.22], 1);
      });
    }
    // The spray over each fall, rising and drifting with the wind.
    var s = v.k * dpr;
    FALLS.forEach(function (f, i) {
      var X = (f.x - v.x) * s, Y = (f.y - v.y) * s, k = RIVERS ? kAt(f.y) : 1, r0 = (f.w + 2.5) * k * s * 1.6;
      if (X < -r0 * 4 || Y < -r0 * 4 || X > ground.width + r0 * 4 || Y > ground.height + r0 * 4) return;
      for (var j = 0; j < 6; j++) {
        var tt = (clock / 2.8 + j / 6 + f.ph) % 1;
        var dx = Math.sin(j * 2.1 + clock * 0.6 + i) * r0 * 0.5 + tt * r0 * 0.9;
        blot(X + dx, Y - tt * r0 * 2.0, r0 * (0.7 + 1.2 * tt), r0 * (0.55 + 0.9 * tt), SPRAY, 0.34 * Math.sin(Math.PI * tt));
      }
    });
    // An eagle circling over the meadows, its shade far below it.
    var ea = clock * 6.283 / EAGLE.period, eu = EAGLE.u + Math.cos(ea) * EAGLE.r, ev = EAGLE.v + Math.sin(ea) * EAGLE.r * 0.42;
    var hd = Math.atan2(Math.cos(ea) * EAGLE.r * 0.42, -Math.sin(ea) * EAGLE.r), span = R.h * 0.075 * (0.8 + 0.4 * ev);
    var ex = R.x + eu * R.w, ey = R.y + ev * R.h;
    blot(ex + R.h * 0.05, ey + R.h * 0.1, span * 0.42, span * 0.12, SHADE, 0.16);
    var flap = (clock % 9) < 0.9 && Math.floor(clock * 5) % 2 ? 1 : 0;
    turned(SPR.eagle[flap], ex, ey, span, hd + Math.PI / 2, 0.72, [1.22, 1.12, 0.98], 1);
    // Now and then a flock of small birds crossing the valley with the wind.
    if (moving) {
      if (!flock && clock > flockAt) {
        var fr = seeded(Math.floor(clock))();
        flock = { t0: clock, v: 0.12 + fr * 0.3, n: 6 + Math.floor(fr * 6), sp: R.w * (0.055 + 0.02 * fr) };
      }
      if (flock) {
        var lead = -R.w * 0.05 + (clock - flock.t0) * flock.sp, done = true;
        for (var b = 0; b < flock.n; b++) {
          var row = Math.ceil(b / 2), sd = b % 2 ? 1 : -1;
          var bx = R.x + lead - row * R.h * 0.022, by = R.y + flock.v * R.h + sd * row * R.h * 0.014 + Math.sin(clock * 1.7 + b) * R.h * 0.004;
          if (bx < R.x + R.w * 1.05) done = false;
          var fw = R.h * 0.016 * (0.85 + 0.3 * ((b * 37) % 10) / 10);
          turned(SPR.bird[Math.floor(clock * 9 + b * 1.7) % 4], bx, by, fw, 0, 1, [1, 1, 1], 0.85);
        }
        if (done) { flock = null; flockAt = clock + 35 + seeded(Math.floor(clock * 3))() * 45; }
      }
    }
    if (!n) return;
    gl.useProgram(lprog);
    gl.activeTexture(gl.TEXTURE5); gl.bindTexture(gl.TEXTURE_2D, lifeT);
    gl.bindBuffer(gl.ARRAY_BUFFER, lbuf);
    gl.bufferData(gl.ARRAY_BUFFER, D.subarray(0, n), gl.DYNAMIC_DRAW);
    [['aP', 2, 0], ['aT', 2, 2], ['aC', 4, 4]].forEach(function (q) {
      if (LA[q[0]] < 0) return;
      gl.enableVertexAttribArray(LA[q[0]]);
      gl.vertexAttribPointer(LA[q[0]], q[1], gl.FLOAT, false, 32, q[2] * 4);
    });
    gl.uniform2f(LU.uRes, ground.width, ground.height);
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
    gl.drawArrays(gl.TRIANGLES, 0, n / 8);
    gl.disable(gl.BLEND);
    ['aP', 'aT', 'aC'].forEach(function (k) { if (LA[k] >= 0) gl.disableVertexAttribArray(LA[k]); });
  }

  /* ---- the clouds crossing the valley, and their shadows ---- */
  // Over everything on the ground — the forest, the family's water, the
  // herds: low clouds drifting across with the wind, more toward the
  // distance and the edges than over the drawing, and the shadow of each a
  // little lower and to the right of it, away from the low sun.
  var CFRAG = [
    'precision highp float;',
    'uniform vec2 uRes;uniform float uTime;uniform vec4 uRect;uniform vec3 uMist,uSun;',
    NOISE,
    'float fbm3(vec2 p){float s=0.0,a=0.5;for(int i=0;i<3;i++){s+=a*noise(p);p=p*2.07+vec2(17.3,9.1);a*=0.5;}return s/0.875;}',
    'void main(){',
    ' vec2 px=vec2(gl_FragCoord.x,uRes.y-gl_FragCoord.y),uv=px/uRes;',
    ' vec2 p=(px-uRect.xy)/uRect.zw;',
    ' float gz=1.0/(clamp(p.y,0.0,1.0)+0.25);vec2 g=vec2((p.x-0.5)*gz*2.4,gz*1.6);',
    ' vec2 drift=vec2(-uTime*0.05,uTime*0.006);',
    ' float c=fbm3(g+drift),cs=fbm3(g+drift+vec2(-0.5,-0.42));',
    ' float far=1.0-smoothstep(0.08,0.75,p.y);',
    ' float edge=max(1.0-smoothstep(0.0,0.32,uv.x),smoothstep(0.68,1.0,uv.x));',
    ' float a=smoothstep(0.58,0.84,c)*(0.22+0.4*far+0.3*edge)*0.8;',
    ' float sh=smoothstep(0.5,0.74,cs)*0.3*(1.0-a);',
    ' vec3 cc=mix(uMist,vec3(1.0),0.35)+uSun*0.12*(1.0-uv.x);',
    ' gl_FragColor=vec4(cc*a,a+sh);',
    '}'
  ].join('\n');
  var cprog = program(VERT, CFRAG), CU = {}, cloc = -1;
  if (cprog) {
    cloc = gl.getAttribLocation(cprog, 'a');
    ['uRes', 'uTime', 'uRect', 'uMist', 'uSun'].forEach(function (k) { CU[k] = gl.getUniformLocation(cprog, k); });
    gl.useProgram(cprog);
    gl.uniform3fv(CU.uMist, P.mist); gl.uniform3fv(CU.uSun, P.sun);
    gl.useProgram(prog);
  }
  function drawClouds(rect) {
    if (!cprog) return;
    gl.useProgram(cprog);
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    gl.enableVertexAttribArray(cloc);
    gl.vertexAttribPointer(cloc, 2, gl.FLOAT, false, 0, 0);
    gl.uniform2f(CU.uRes, ground.width, ground.height);
    gl.uniform1f(CU.uTime, clock);
    gl.uniform4f(CU.uRect, rect[0], rect[1], rect[2], rect[3]);
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
    gl.disable(gl.BLEND);
    gl.disableVertexAttribArray(cloc);
  }

  var U = {};
  ['uRes', 'uTime', 'uRect', 'uCam', 'uDep', 'uPic', 'uDepth', 'uSun', 'uMist', 'uShade', 'uGreen', 'uGlow', 'uView', 'uGlowRect', 'uHor', 'uMeadow']
    .forEach(function (k) { U[k] = gl.getUniformLocation(prog, k); });
  gl.useProgram(prog);
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
  // The grass of a clearing (§18) is the photograph's own meadows' tone: their
  // mean colour, once the picture and its maps are both here.
  var picSmall = null;
  gl.uniform3fv(U.uMeadow, [0.56, 0.52, 0.27]);
  function meadowTone() {
    if (!picSmall || !MAP) return;
    var t = [0, 0, 0], n = 0;
    for (var i = 0; i < MAP.w * MAP.h; i++) {
      if (MAP.d[i * 4 + 2] < 170 || MAP.d[i * 4 + 1] > 60) continue;
      t[0] += picSmall[i * 4]; t[1] += picSmall[i * 4 + 1]; t[2] += picSmall[i * 4 + 2]; n++;
    }
    if (n < 50) return;
    gl.useProgram(prog);
    gl.uniform3fv(U.uMeadow, t.map(function (c) { return c / n / 255; }));
  }
  function small(im) {
    var c = document.createElement('canvas');
    c.width = 256; c.height = 144;
    var x = c.getContext('2d', { willReadFrequently: true });
    x.drawImage(im, 0, 0, c.width, c.height);
    return x.getImageData(0, 0, c.width, c.height).data;
  }
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
    try { picSmall = small(pic); meadowTone(); } catch (e) { /* the default tone */ }
    stage.classList.add('rv-alive');
    if (!running) draw(false);
  };
  dep.onload = function () {
    if (gl.isContextLost()) return;
    gl.activeTexture(gl.TEXTURE1); gl.bindTexture(gl.TEXTURE_2D, depT);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGB, gl.RGB, gl.UNSIGNED_BYTE, dep);
    hasDep = true;
    try {
      MAP = { w: 256, h: 144, d: small(dep) };
      herds();
      meadowTone();
    } catch (e) { /* no herds without the maps */ }
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
    if (RIVERS) camera(L);
    var rects = blockers(), dots = L.persons || [];
    flows = [];
    rings = dots.filter(function (d) { return d.x !== undefined; }).map(function (d, i) { return { x: d.x, y: d.y, c: d.role === 'centre', ph: Math.abs(Math.sin(d.x * 3.17 + d.y * 7.31 + i)) % 1 }; });
    COURSES = {};
    // Where the water of a family meets (§17): the point where the streams
    // from its parents end and its rivers to its children begin. Where two
    // streams meet, the river falls (and its spray rises), or gathers in a
    // still pool, or runs down rapids: the same at every visit.
    var at = {};
    function meet(x, y) { var k = Math.round(x * 10) + ',' + Math.round(y * 10); return (at[k] = at[k] || { x: x, y: y, ins: 0, w: 0 }); }
    L.edges.forEach(function (e) { var m; if (e.river) { m = meet(e.x1, e.y1); m.w = Math.max(m.w, widthOf(e)); } else meet(e.x2, e.y2).ins++; });
    FALLS = [];
    Object.keys(at).forEach(function (k) {
      var m = at[k], h = Math.abs(Math.sin(m.x * 3.731 + m.y * 1.913) * 43758.5453) % 1;
      m.style = m.ins >= 2 ? (h < 0.6 ? 1 : h < 0.85 ? 0.5 : 0) : 0.5;
      m.ph = h;
      if (m.style === 1 && m.w) FALLS.push(m);
    });
    var lines = L.edges.map(function (e) {
      var m = e.river ? at[Math.round(e.x1 * 10) + ',' + Math.round(e.y1 * 10)] : at[Math.round(e.x2 * 10) + ',' + Math.round(e.y2 * 10)];
      return Object.assign({ style: m ? m.style : 0.5 }, e);
    });
    glow(lines);
    rivers(lines, L.tails);
    lines.forEach(function (e) {
      if (e.conf !== 'a') return;
      var C = RIVERS ? course(e) : null, n = C ? C.n : Math.max(6, Math.ceil((Math.hypot(e.x2 - e.x1, e.y2 - e.y1) * 1.25 + 4) / STEP));
      var xs = new Float32Array(n + 1), ys = new Float32Array(n + 1), cum = new Float32Array(n + 1), ok = new Uint8Array(n + 1);
      for (var i = 0; i <= n; i++) {
        var p = C ? C.spts[i] : bez(e, i / n);
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
      flows.push({ x: xs, y: ys, cum: cum, ok: ok, n: n, len: len, w: WIDTHS[cls(e.d)], river: !!e.river,
        k: Math.max(1, Math.round(len / 70)), phase: Math.abs(Math.sin(e.x1 * 12.9898 + e.y2 * 78.233)) % 1 });
    });
    request();
  }
  // Where the family's rivers shape the ground: every course drawn wide and
  // soft into a mask, on the tilted ground (§17), the attested ones
  // brighter, at half a pixel per unit at most.
  var rings = [];
  function glow(edges) {
    var x0 = Infinity, y0 = Infinity, x1 = -Infinity, y1 = -Infinity;
    edges.forEach(function (e) { course(e).pts.forEach(function (q) { x0 = Math.min(x0, q[0]); x1 = Math.max(x1, q[0]); y0 = Math.min(y0, q[1]); y1 = Math.max(y1, q[1]); }); });
    if (x0 === Infinity) return;
    x0 -= 90; y0 -= 90; x1 += 90; y1 += 90;
    var S = Math.min(0.5, 2048 / (x1 - x0), 2048 / (y1 - y0)), W = Math.max(2, Math.ceil((x1 - x0) * S)), H = Math.max(2, Math.ceil((y1 - y0) * S));
    var c = document.createElement('canvas');
    c.width = W; c.height = H;
    var m = c.getContext('2d');
    m.fillStyle = 'black'; m.fillRect(0, 0, W, H);
    m.setTransform(S, 0, 0, S, -x0 * S, -y0 * S);
    m.lineCap = 'round'; m.strokeStyle = 'white';
    // Three fields along the courses: a wide one (red) and a near one
    // (green) for the wetter ground, and the valleys the water carved
    // (blue), wide and soft, as a height field.
    m.globalCompositeOperation = 'lighter';
    [[34, 0.22, 'rgb(255,0,0)', 6], [16, 0.45, 'rgb(255,0,0)', 6], [22, 0.9, 'rgb(0,255,0)', 5], [60, 0.35, 'rgb(0,0,255)', 12], [24, 0.4, 'rgb(0,0,255)', 7]].forEach(function (pass) {
      if ('filter' in m) m.filter = 'blur(' + (pass[3] * S / 0.5).toFixed(2) + 'px)';
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
    var camx = (lean[0] * 14 + sway * 5) * dpr, camy = (lean[1] * 9) * dpr;
    gl.viewport(0, 0, W, H);
    gl.useProgram(prog);
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    gl.enableVertexAttribArray(loc);
    gl.vertexAttribPointer(loc, 2, gl.FLOAT, false, 0, 0);
    gl.uniform2f(U.uRes, W, H);
    gl.uniform1f(U.uTime, clock);
    gl.uniform4f(U.uRect, px, py, pw, ph);
    gl.uniform2f(U.uCam, camx, camy);
    gl.uniform1f(U.uDep, hasDep ? 1 : 0);
    gl.uniform4f(U.uView, v.x, v.y, 1 / (v.k * dpr), 1 / (v.k * dpr));
    gl.uniform4f(U.uGlowRect, glowRect[0], glowRect[1], glowRect[2], glowRect[3]);
    gl.uniform4f(U.uHor, CAM.x0, CAM.y0, CAM.yh, RIVERS ? COS : 1);
    gl.activeTexture(gl.TEXTURE0); gl.bindTexture(gl.TEXTURE_2D, picT);
    gl.activeTexture(gl.TEXTURE1); gl.bindTexture(gl.TEXTURE_2D, depT);
    gl.activeTexture(gl.TEXTURE2); gl.bindTexture(gl.TEXTURE_2D, glowT);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
    gl.disableVertexAttribArray(loc);
    drawRivers(v, [px, py, pw, ph], [camx, camy]);
    drawLife(v, { x: px, y: py, w: pw, h: ph, cx: camx, cy: camy }, moving);
    drawClouds([px, py, pw, ph]);
    wctx.setTransform(1, 0, 0, 1, 0, 0);
    wctx.clearRect(0, 0, water.width, water.height);
    if (!travelling) glints(v, moving);
  }
  var SPEED = 22, rises = [], riseAt = 4;
  function glints(v, moving) {
    if (!moving) return;
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
    // point's wider; under Canopée it lies on the ground, seen at the
    // camera's angle (§17).
    wctx.setTransform(1, 0, 0, 1, 0, 0);
    wctx.strokeStyle = coreRgb;
    for (var q = 0; q < rings.length; q++) {
      var rg = rings[q], tt = (clock / 3.2 + rg.ph) % 1, rad = (rg.c ? 11 : 6) + tt * (rg.c ? 14 : 8);
      var kx = RIVERS ? Math.min(1.5, kAt(rg.y)) : 1, ky = RIVERS ? kx * Math.max(0.36, COS * kx) : 1;
      wctx.globalAlpha = (rg.c ? 0.55 : 0.4) * (1 - tt) * Math.min(1, tt * 6);
      wctx.lineWidth = (rg.c ? 1.6 : 1.1) * dpr;
      wctx.beginPath(); wctx.ellipse(rg.x * s + ox, rg.y * s + oy, rad * s * kx, rad * s * ky, 0, 0, 6.283); wctx.stroke();
    }
    // Here and there on the rivers a fish rises: a ring spreading on the
    // water, seen at the camera's angle.
    if (rready && clock > riseAt) {
      var pick = seeded(Math.floor(clock * 13)), rv = flows.filter(function (fl) { return fl.river; });
      if (rv.length) {
        var fl = rv[Math.floor(pick() * rv.length)], at = Math.floor((0.2 + 0.6 * pick()) * fl.n);
        rises.push({ x: fl.x[at], y: fl.y[at], t0: clock });
      }
      riseAt = clock + 2.5 + pick() * 4;
    }
    wctx.strokeStyle = 'rgb(240,248,246)';
    rises = rises.filter(function (r) { return clock - r.t0 < 1.6; });
    rises.forEach(function (r) {
      var age = (clock - r.t0) / 1.6, k = RIVERS ? kAt(r.y) : 1, rx = (1.5 + age * 9) * k * s;
      wctx.globalAlpha = 0.55 * (1 - age);
      wctx.lineWidth = 0.9 * dpr;
      wctx.beginPath(); wctx.ellipse(r.x * s + ox, r.y * s + oy, rx, rx * (RIVERS ? Math.max(0.36, COS * k) : 1), 0, 0, 6.283); wctx.stroke();
    });
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
