fn {{self}}_intersect(ray: GeoRay, minimum: f32, maximum: f32, radius: f32) -> GeoHit {
    if GEO_K >= 0 { return geo_miss(); }
    let distance = geo_section_root(ray.position.x-ray.position.w,
        ray.tangent.x-ray.tangent.w,1,minimum,maximum,radius);
    if distance < 0 { return geo_miss(); }
    let state = geo_advance(ray,distance,radius);
    let normal = geo_from_chart_dir(state.position,vec3<f32>(0,0,-1));
    return GeoHit(1u,distance,state.position,state.tangent,normal);
}

fn {{self}}(base:u32,ray:GeoRay,previous_identity:u32)->GeoTaggedHit {
    return GeoTaggedHit({{self}}_intersect(ray,select(0.0,8.0*EPS*params.misc.y,base==previous_identity),geo_infinity(),params.misc.y),base);
}
