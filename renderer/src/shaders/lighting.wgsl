// One uniformly selected emitter per eligible vertex. These helpers know only
// compiled sampler IDs; shape and sampler implementations belong to components.
struct GeoLightConnection { ray:GeoRay, selected:GeoSceneHit, pdf:f32, valid:u32 }
fn geo_power_weight(a:f32,b:f32)->f32 {
    let scale=max(a,b);
    if scale==0 {return 0;}
    let x=a/scale;
    let y=b/scale;
    return x*x/(x*x+y*y);
}
fn geo_nonnegative(value:vec3<f32>)->bool {
    return all(value>=vec3<f32>(0)) && all(value<=vec3<f32>(bitcast<f32>(0x7f7fffffu)));
}
fn geo_density_supported(value:f32)->bool {
    return value>=0 && value<=bitcast<f32>(0x7f7fffffu);
}
fn geo_light_pdf(index:u32,ray:GeoRay)->LightPdf {
    let object=objects[index];
    if object.sampling.x==0xffffffffu || params.options.z==0u {return LightPdf(0,1u);}
    let local=geo_map_ray(geo_inverse(GeoMap(object.sampling_map0,object.sampling_map1)),ray);
    if !geo_ray_supported(local) {return LightPdf(0,0u);}
    let density=ht_light_pdf(object.sampling.x,object.sampling.y,local);
    if density.valid==0u || !geo_density_supported(density.value) {return LightPdf(0,0u);}
    return LightPdf(density.value/f32(params.options.z),1u);
}
fn geo_light_connection(position:vec4<f32>,previous:u32,identity:u32,rng:ptr<function,u32>)->GeoLightConnection {
    var result=GeoLightConnection(GeoRay(position,vec4<f32>(0)),
        GeoSceneHit(geo_miss(),0xffffffffu,0xffffffffu),0,0u);
    if params.options.z==0u {return result;}
    var choice=min(u32(geo_uniform_open(rng)*f32(params.options.z)),params.options.z-1u);
    var index=0xffffffffu;
    for(var i=0u;i<params.info.w;i+=1u) {
        if objects[i].sampling.x!=0xffffffffu {
            if choice==0u {index=i;break;}
            choice-=1u;
        }
    }
    if index==0xffffffffu {result.valid=2u;return result;}
    let object=objects[index];
    let map=GeoMap(object.sampling_map0,object.sampling_map1);
    let local_position=geo_map_apply(geo_inverse(map),position);
    let proposal=ht_light_sample(object.sampling.x,object.sampling.y,local_position,rng);
    if proposal.valid==0u || !(proposal.pdf>0) || !geo_density_supported(proposal.pdf) {
        result.valid=2u;return result;
    }
    let local=GeoRay(local_position,proposal.tangent);
    result.ray=geo_map_ray(map,local);
    if !geo_ray_supported(local) || !geo_ray_supported(result.ray) {result.valid=2u;return result;}
    result.pdf=proposal.pdf/f32(params.options.z);
    result.selected=geo_scene_hit(result.ray,previous,identity);
    if result.selected.hit.valid==2u {result.valid=2u;return result;}
    // The bound is only a proposal. A miss, an occluder, or a different emitter
    // contributes zero for this selected emitter, without rejection resampling.
    if result.selected.hit.valid==0u || result.selected.object_index!=index {return result;}
    if !geo_advance_supported(result.ray,result.selected.hit.distance,params.misc.y) {
        result.valid=2u;return result;
    }
    result.valid=1u;
    return result;
}
fn geo_connection_emission(connection:GeoLightConnection)->MaterialEmission {
    let hit=connection.selected.hit;
    let material=materials[objects[connection.selected.object_index].info.y];
    let context=GeoMaterialContext(hit.position,normalize(geo_to_local(hit.position,hit.normal)));
    let emission=ht_material_emission(material.data.x,material.data.y,context,
        normalize(geo_to_local(hit.position,hit.tangent)));
    if emission.valid==0u || !geo_nonnegative(emission.value) {return MaterialEmission(vec3<f32>(0),0u);}
    // Unlike the continuing analog path, this deterministic visibility ray has
    // not sampled survival. Account for extinction once over its full distance.
    return MaterialEmission(emission.value*geo_transmittance(params.medium.w,hit.distance),1u);
}
fn geo_direct_surface(position:vec4<f32>,index:u32,identity:u32,ctx:GeoMaterialContext,
    incoming:vec3<f32>,rng:ptr<function,u32>)->MaterialEmission {
    let connection=geo_light_connection(position,index,identity,rng);
    if connection.valid==0u {return MaterialEmission(vec3<f32>(0),1u);}
    if connection.valid==2u {return MaterialEmission(vec3<f32>(0),0u);}
    let object=objects[index];
    let material=materials[object.info.y];
    let local=geo_map_ray(geo_inverse(GeoMap(object.map0,object.map1)),connection.ray);
    if !geo_ray_supported(local) {return MaterialEmission(vec3<f32>(0),0u);}
    let outgoing=normalize(geo_to_local(local.position,local.tangent));
    let evaluation=ht_material_evaluate(material.data.x,material.data.y,ctx,incoming,outgoing);
    if evaluation.valid==0u || !geo_nonnegative(evaluation.value) || !geo_density_supported(evaluation.pdf) {
        return MaterialEmission(vec3<f32>(0),0u);
    }
    let emission=geo_connection_emission(connection);
    let weight=geo_power_weight(connection.pdf,evaluation.pdf)/connection.pdf;
    return MaterialEmission(evaluation.value*emission.value*weight,emission.valid);
}
fn geo_direct_volume(position:vec4<f32>,rng:ptr<function,u32>)->MaterialEmission {
    let connection=geo_light_connection(position,0xffffffffu,0xffffffffu,rng);
    if connection.valid==0u {return MaterialEmission(vec3<f32>(0),1u);}
    if connection.valid==2u {return MaterialEmission(vec3<f32>(0),0u);}
    let emission=geo_connection_emission(connection);
    let phase=1/(4*PI);
    let weight=geo_power_weight(connection.pdf,phase)*phase/connection.pdf;
    return MaterialEmission(emission.value*weight,emission.valid);
}
