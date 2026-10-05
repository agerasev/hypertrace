fn {{self}}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {
var normal=ctx.normal;
if dot((*sample).direction,normal)>0 {normal=-normal;}
let phi=2*PI*uniform_random(rng);
let square_cosine=uniform_random(rng);
let sine=sqrt(max(0,1-square_cosine));
let local=vec3<f32>(cos(phi)*sine,sin(phi)*sine,sqrt(square_cosine));
(*sample).direction=rotate_vector(rotation_look(-normal),local);
(*sample).delta=0u;
}
fn {{self}}_evaluate(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>,outgoing:vec3<f32>)->MaterialEvaluation {
    let normal=select(ctx.normal,-ctx.normal,dot(incoming,ctx.normal)>0);
    let pdf=max(0.0,dot(normal,outgoing))/PI;
    return MaterialEvaluation(vec3<f32>(pdf),pdf,1u);
}
fn {{self}}_emission(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>)->MaterialEmission {
    return MaterialEmission(vec3<f32>(0),1u);
}
