fn {{self}}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {
(*sample).alive=0u;
}
fn {{self}}_evaluate(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>,outgoing:vec3<f32>)->MaterialEvaluation {
    return MaterialEvaluation(vec3<f32>(0),0,1u);
}
fn {{self}}_emission(base:u32,ctx:GeoMaterialContext,incoming:vec3<f32>)->MaterialEmission {
    return MaterialEmission(vec3<f32>(0),1u);
}
