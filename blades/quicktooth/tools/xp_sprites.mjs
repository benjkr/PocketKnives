// Generates core/src/popup/sprites.rs: the pixel art of the popup header (the approved "B1"
// design: XP shell-icon look, coloured outlines, 1.5 art pixels per unit, drawn at 2x).
//
//     node tools/xp_sprites.mjs
//
// The earbud is the right bud of airpods_pro_shapes.json (the tray glyph's source too), rastered
// cell by cell and shaded from the top left with an ordered dither; the case is built from rounded
// boxes on a 34 x 34 unit grid. Each sprite is written as runs of palette entries.
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const here = dirname(fileURLToPath(import.meta.url));
const SHAPES = JSON.parse(readFileSync(join(here, "airpods_pro_shapes.json"), "utf8"));

const hex2=n=>Math.round(n).toString(16).padStart(2,"0").toUpperCase();
const PAL={w:"#FFFFFF",p:"#F2F4F8",e:"#E2E7EF",u:"#CDD5E0",v:"#B6C0CF",        // pearl shell, light to underside
  i:"#EEF3F9",j:"#D5DEEA",y:"#BECADA",q:"#A7B4C7",7:"#8793A7",                  // ear tip, translucent blue-grey; 7 is also the seam
  g:"#E2DACA",h:"#B5AA93",H:"#8F8571",                                          // warm silver stem cap
  1:"#6C7688",4:"#586378",2:"#343B48",3:"#1C2028",                              // outline: lit top, lit left, mid, underside
  m:"#9AA4B2",s:"#727D8E",c:"#39414F",z:"#1B1F27"};                             // lid wells, case mouth, mesh
const PAL_OFF={};                                  // not connected: every colour goes to the same grey ramp
for(const key in PAL){
  const v=parseInt(PAL[key].slice(1),16),y=.299*(v>>16)+.587*(v>>8&255)+.114*(v&255),g=hex2(94+(y-32)*.641);
  PAL_OFF[key]="#"+g+g+g;
}
/* k: pixels per unit; css: screen pixels per pixel; line: outline weight; steps: tones per material;
   bayer, dw: the ordered-dither matrix and how far it reaches across a tone step; bg: backdrop dither cell;
   detail: 0 none, 1 thumb groove, 2 also mesh, sensor line, cap ring and lit lips */
const B1={k:1.5,css:2,line:1,steps:3,bayer:4,dw:.1,bg:1,detail:0};

/* the earbud's regions: the right bud of blades/quicktooth/tools/airpods_pro_shapes.json, x y pairs */
const SH=Object.fromEntries(["tip","noz","body","vent","stem"].map(k=>[k,SHAPES[k].flat(3)]));
function inPoly(p,x,y){
  let c=false;
  for(let i=0,j=p.length-2;i<p.length;j=i,i+=2){
    const xi=p[i],yi=p[i+1],xj=p[j],yj=p[j+1];
    if((yi>y)!==(yj>y)&&x<(xj-xi)*(y-yi)/(yj-yi)+xi)c=!c;
  }
  return c;
}
/* An earbud in materials, drawn as the right one (ear tip to the left), h pixels tall:
   S head shell, P stem, T ear tip, x the seam where the tip meets the shell, N nozzle, V vent, O sensor line, C charging cap */
function budMap(h,detail){
  const X0=66.96,Y0=36.79,s=h/74.42,w=Math.ceil(53.16*s),g=[];
  for(let y=0;y<h;y++){
    const r=[];
    for(let x=0;x<w;x++){
      const X=X0+(x+.5)/s,Y=Y0+(y+.5)/s;
      r.push(inPoly(SH.body,X,Y)?(inPoly(SH.vent,X,Y)?"V":"S")
        :inPoly(SH.stem,X,Y)||inPoly(SH.stem,X,Y+1.4)?(Y>103.7?"C":"P")      // the stem reaches up into the head: no gap where the drawing's stroke was
        :inPoly(SH.tip,X,Y)?(inPoly(SH.noz,X,Y)?"N":"T"):".");
    }
    g.push(r);
  }
  const at=(x,y)=>x<0||y<0||x>=w||y>=h?".":g[y][x];
  const near=(x,y,set)=>set.includes(at(x-1,y))||set.includes(at(x+1,y))||set.includes(at(x,y-1))||set.includes(at(x,y+1));
  const mark=(test,c)=>{const hit=[];for(let y=0;y<h;y++)for(let x=0;x<w;x++)if(test(x,y))hit.push([x,y]);for(const [x,y] of hit)g[y][x]=c;};
  mark((x,y)=>at(x,y)==="."&&near(x,y,"TN")&&near(x,y,"SP"),"x");          // the stroke between tip and shell
  mark((x,y)=>at(x,y)==="T"&&near(x,y,"SPV"),"x");                         // and where they touch
  mark((x,y)=>at(x,y)!=="."&&!near(x,y,"SPTxNVC"),".");                    // no stray single pixels
  if(detail>1){
    const cx=Math.floor((102.6-X0)*s);
    for(let y=0;y<h;y++){const Y=Y0+(y+.5)/s;if(Y>85&&Y<98&&g[y][cx]==="P")g[y][cx]="O";}
  }
  return g.map(r=>r.join(""));
}

/* per solid cell: distance to the nearest empty cell and the direction the surface faces there,
   averaged over the empty cells up to `spread` further away, so that a stepped edge faces one way */
const RMAX=14;
const OFF=(()=>{
  const o=[];
  for(let dy=-RMAX;dy<=RMAX;dy++)for(let dx=-RMAX;dx<=RMAX;dx++){const d=Math.hypot(dx,dy);if(d&&d<=RMAX)o.push([d,dx,dy]);}
  o.sort((a,b)=>a[0]-b[0]);
  return o;
})();
const FIELDS=new Map();
function field(rows,spread){
  const key=spread+rows.join("\n");
  if(FIELDS.has(key))return FIELDS.get(key);
  const h=rows.length,w=rows[0].length,W2=w+2*RMAX,m=new Uint8Array(W2*(h+2*RMAX));
  for(let y=0;y<h;y++)for(let x=0;x<w;x++)if(rows[y][x]!==".")m[(y+RMAX)*W2+x+RMAX]=1;
  const d=new Float32Array(w*h),nx=new Float32Array(w*h),ny=new Float32Array(w*h);
  for(let y=0;y<h;y++)for(let x=0;x<w;x++){
    const base=(y+RMAX)*W2+x+RMAX;
    if(!m[base])continue;
    let dm=0,sx=0,sy=0;
    for(const o of OFF){
      if(dm&&o[0]>dm+spread)break;
      if(!m[base+o[2]*W2+o[1]]){if(!dm)dm=o[0];sx+=o[1]/o[0];sy+=o[2]/o[0];}
    }
    const l=Math.hypot(sx,sy),i=y*w+x;
    d[i]=dm||99;nx[i]=l?sx/l:0;ny[i]=l?sy/l:0;
  }
  const f={d,nx,ny};
  FIELDS.set(key,f);
  return f;
}

/* Materials to colours, lit from the top left. The outline takes the light: slate on top and on the left,
   near black only underneath. Inside, each material steps from light to shade with how squarely the nearest
   edge faces away from the light and how close it is; an ordered dither spreads each step. */
const SX=.64,SY=.77;                                // the direction the shade falls in
const BAYER4=[[0,8,2,10],[12,4,14,6],[3,11,1,9],[15,7,13,5]],BAYER2=[[0,2],[3,1]];
const RAMPS={                                       // [shade above this, colour], first match
  3:{shell:[[.36,"u"],[-.3,"p"],[-9,"w"]],tip:[[.32,"q"],[-.28,"j"],[-9,"i"]],inner:[[.36,"q"],[-.3,"u"],[-9,"p"]],cap:[[.2,"h"],[-9,"g"]]},
  5:{shell:[[.62,"v"],[.38,"u"],[.17,"e"],[-.3,"p"],[-9,"w"]],tip:[[.6,"7"],[.36,"q"],[.15,"y"],[-.28,"j"],[-9,"i"]],inner:[[.5,"m"],[.3,"q"],[-.3,"u"],[-9,"p"]],cap:[[.2,"h"],[-9,"g"]]}};
const RAMP_OF={S:"shell",P:"shell",W:"shell",R:"shell",T:"tip",I:"inner",C:"cap"};
const FLAT={x:"7",L:"m",O:"u",a:"a",b:"b",Z:"z",M:"c",G:"c",J:"s",E:"m"};
const rimKey=(nx,ny)=>ny>.45?"3":nx>.3||ny>.12?"2":ny<-.6?"1":"4";
function shade(rows,T,R){
  const h=rows.length,w=rows[0].length,at=(x,y)=>x<0||y<0||x>=w||y>=h?".":rows[y][x];
  const spread=1.5+.7*T.k,sil=field(rows,spread),wal=field(rows.map(r=>r.replace(/x/g,".")),spread);      // the tip's seam is an edge for the shading
  const ramp=RAMPS[T.steps],B=T.bayer===2?BAYER2:BAYER4,bn=T.bayer,fine=T.detail>1,dn=Math.round(T.k*.67);
  const out=[];
  for(let y=0;y<h;y++){
    let s="";
    for(let x=0;x<w;x++){
      const c=rows[y][x],i=y*w+x;
      if(c==="."){s+=".";continue;}
      if(sil.d[i]<=T.line+.01){s+=rimKey(sil.nx[i],sil.ny[i]);continue;}
      const up=at(x,y-1),left=at(x-1,y);
      if(FLAT[c]){s+=FLAT[c];continue;}
      if(c==="V"||c==="N"){                         // a hole: its top left edge is in shade
        const deep=up===c&&left===c;
        s+=fine&&c==="V"?(deep&&(x+y)&1?"c":"z"):deep?"c":"z";continue;
      }
      if(c==="D"){                                  // a well in the lid
        let n=1;while(n<=2*dn&&at(x-n,y)==="D"&&at(x,y-n)==="D")n++;
        s+=n<=dn?"s":fine&&n<=2*dn?"7":"m";continue;
      }
      if(fine){                                     // the far edge of a recess catches the light
        if(c!=="C"&&(up==="V"||left==="V"||up==="L")){s+="w";continue;}
        if(c==="I"&&(up==="D"||left==="D")){s+="p";continue;}
        if(c==="C"&&up==="P"){s+="H";continue;}
      }
      const f=Math.max(0,1-(wal.d[i]-1)/R),t=(wal.nx[i]*SX+wal.ny[i]*SY)*f+((B[y%bn][x%bn]+.5)/(bn*bn)-.5)*T.dw;
      s+=ramp[RAMP_OF[c]].find(b=>t>b[0])[1];
    }
    out.push(s);
  }
  return out;
}

function sprite(rows){
  const runs=[];
  rows.forEach((r,y)=>{for(let x=0;x<r.length;){const c=r[x];let n=1;while(x+n<r.length&&r[x+n]===c)n++;if(c!==".")runs.push([x,y,n,c]);x+=n;}});
  return {w:rows[0].length,h:rows.length,rows,runs};
}
/* ax, ay: the column and row that sit on the earbud's position; sh: the columns its shadow covers.
   The left earbud is the map mirrored and then lit, so both are lit from the top left like the case. */
function budSprite(T,h,R){
  const map=budMap(h,T.detail),w=map[0].length;
  return {w,h,ax:Math.round(w/2),ay:Math.round(h*.46),sh:[Math.round(w*.3),Math.round(w*.74)],
    R:sprite(shade(map,T,R)),L:sprite(shade(map.map(r=>[...r].reverse().join("")),T,R))};
}

/* the case, in units: 34 wide, row PRIM is the body's top edge. W shell, L lid seam, O thumb groove, a b light,
   G gap under a lifted lid, M mouth, Z bud wells, R I D the open lid's rim, inner face and wells, E J a tilted lid's edge and underside */
const CW=34,CH=34,PRIM=16;
function rr(x,y,x0,y0,x1,y1,rt,rb){                 // a box with rounded top and bottom corners, [rx, ry] each
  if(x<x0||x>x1||y<y0||y>y1)return false;
  const r=rt&&y<y0+rt[1]?[rt[0],rt[1],y0+rt[1]-y]:rb&&y>y1-rb[1]?[rb[0],rb[1],y-(y1-rb[1])]:0;
  if(!r)return true;
  const dx=Math.max(0,x0+r[0]-x,x-(x1-r[0]));
  return (dx/r[0])**2.2+(r[2]/r[1])**2.2<=1;
}
function raster(wu,hu,k,fn){
  const w=Math.floor(wu*k),h=Math.floor(hu*k),rows=[];
  for(let y=0;y<h;y++){let s="";for(let x=0;x<w;x++)s+=fn((x+.5)/k,(y+.5)/k)||".";rows.push(s);}
  return rows;
}
function compose(list,w,h){
  const g=Array.from({length:h},()=>Array(w).fill("."));
  for(const rows of list)rows.forEach((r,y)=>{for(let x=0;x<w;x++)if(r[x]!==".")g[y][x]=r[x];});
  return g.map(r=>r.join(""));
}
function caseSet(T){
  const k=T.k,px=1/k,R=4*k,r7=[7,7],r45=[4.5,4.5];
  const led=(x,y)=>Math.hypot(x-17,y-PRIM-5)<1.08?(y<PRIM+5.2?"a":"b"):"";
  const groove=(x,y)=>T.detail&&y>=PRIM+px&&y<PRIM+3*px&&Math.abs(x-17)<3.3-(y-PRIM-px)*k*.9?"O":"";
  const face=(x,y)=>led(x,y)||groove(x,y)||"W";
  const side=x=>Math.abs(x-17)-8.5;               // distance from the nearer bud's middle
  const body=(x,y)=>rr(x,y,0,PRIM,CW,CH,0,r7)?face(x,y):"";
  const mouth=(x,y)=>rr(x,y,0,PRIM-3,CW,PRIM+1,[5,2.2],0)?((side(x)/3.6)**4+((y-PRIM+.3)/1.1)**4<=1?"Z":"M"):"";
  const closed=(x,y)=>rr(x,y,0,PRIM-8,CW,CH,r7,r7)?(y>=PRIM&&y<PRIM+px?"L":face(x,y)):"";
  const cracked=(x,y)=>rr(x,y,0,PRIM-9,CW,PRIM,r7,0)||rr(x,y,0,PRIM,CW,CH,0,r7)?(y>=PRIM-1&&y<PRIM&&x>1&&x<CW-1?"G":face(x,y)):"";
  const tilted=(x,y)=>rr(x,y,0,PRIM-12,CW,PRIM-3,[5,5],[3,3])?(y<PRIM-7?"W":y<PRIM-6?"E":"J"):"";
  const lid=(y0,rim,cy,ry)=>(x,y)=>rr(x,y,0,y0,CW,PRIM-2,r45,r45)?(y<y0+rim?"R":(side(x)/5.5)**2+((y-cy)/ry)**2<=1?"D":"I"):"";
  const mk=fn=>shade(raster(CW,CH,k,fn),T,R),BODY=mk(body),MOUTH=mk(mouth),w=CW*k,h=CH*k;
  return [                                          // 0 closed, 1 cracked open, 2 tilted and still in front of the buds, 3 nearly up, 4 open
    {back:[],front:[mk(closed)]},
    {back:[MOUTH],front:[mk(cracked)]},
    {back:[MOUTH],front:[BODY,mk(tilted)]},
    {back:[mk(lid(PRIM-14,3,PRIM-7,3)),MOUTH],front:[BODY]},
    {back:[mk(lid(PRIM-15,2,PRIM-8.5,3.5)),MOUTH],front:[BODY]}
  ].map(f=>({back:sprite(compose(f.back,w,h)),front:sprite(compose(f.front,w,h))}));
}
/* the closed case drawn small, for when both earbuds are in the ears */
const miniMap=k=>raster(20,15,k,(x,y)=>rr(x,y,0,0,20,15,[4,4],[4,4])?(y>=5&&y<5+1/k?"L":y>=8&&y<9&&x>=9&&x<11?"a":"W"):"");
function build(T){
  const k=T.k;
  return {out:budSprite(T,24*k,2.4*k),seat:budSprite(T,16*k,1.7*k),frames:caseSet(T),mini:sprite(shade(miniMap(k),T,2.4*k))};
}

// ---------------------------------------------------------------- output
const A = build(B1);
const all = [A.out.L, A.out.R, A.seat.L, A.seat.R, A.frames[0].front, A.frames[4].back, A.frames[4].front, A.mini];
const keys = [...new Set(all.flatMap((s) => s.runs.map((r) => r[3])))].sort();
const idx = (c) => keys.indexOf(c) + 1;
const rgb = (pal, c) => (c === "a" || c === "b" ? "0x000000" : "0x" + pal[c].slice(1).toUpperCase());
const runs = (s) => s.runs.map((r) => `${r[0]},${r[1]},${r[2]},${idx(r[3])}`).join(",");
const emit = (name, s, extra) => {
  if (s.runs.some((r) => r[0] > 255 || r[1] > 255 || r[2] > 255)) throw new Error(`${name}: too big for u8 runs`);
  return `pub const ${name}: Sprite = Sprite { w: ${s.w}, h: ${s.h}${extra}, runs: &[${runs(s)}] };\n`;
};
const bud = (name, b, side) => emit(name, b[side], `, ax: ${b.ax}, ay: ${b.ay}, shadow: [${b.sh[0]}, ${b.sh[1]}]`);
const none = ", ax: 0, ay: 0, shadow: [0, 0]";
let out = `// Generated by tools/xp_sprites.mjs from tools/airpods_pro_shapes.json. Don't edit.
use super::Sprite;

/// Palette entries 1.. (0 is transparent): the keys, their colours when connected and when not.
/// \`a\` and \`b\` are the case's status light, coloured when drawn.
pub const KEYS: &[u8] = b"${keys.join("")}";
pub const ON: &[u32] = &[${keys.map((c) => rgb(PAL, c)).join(", ")}];
pub const OFF: &[u32] = &[${keys.map((c) => rgb(PAL_OFF, c)).join(", ")}];

`;
out += bud("OUT_LEFT", A.out, "L") + bud("OUT_RIGHT", A.out, "R") + bud("SEAT_LEFT", A.seat, "L") + bud("SEAT_RIGHT", A.seat, "R");
out += emit("CASE_CLOSED", A.frames[0].front, none) + emit("CASE_OPEN_BACK", A.frames[4].back, none);
out += emit("CASE_OPEN_FRONT", A.frames[4].front, none) + emit("CASE_MINI", A.mini, none);
writeFileSync(join(here, "../core/src/popup/sprites.rs"), out);
console.log(`sprites.rs: ${keys.length} colours, ${out.length} bytes`);
