fn {{self}}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {
let direction=(*sample).direction;
var ratio=load_f32(base);
let a=dot(direction,ctx.normal);
if a < -EPS {ratio=1/ratio;}
let x=(a*a-1)*(ratio*ratio)+1;
if x>EPS {(*sample).direction=direction*ratio+(sign(a)*sqrt(x)-a*ratio)*ctx.normal;}
else {(*sample).direction=direction-2*a*ctx.normal;}
}
