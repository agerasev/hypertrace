fn {{self}}_intersect(ray: GeoRay, minimum: f32, maximum: f32, radius: f32) -> GeoHit {
    let distance = geo_section_root(ray.position.w,ray.tangent.w,0,
        minimum,maximum,radius);
    if distance < 0 { return geo_miss(); }
    let state = geo_advance(ray,distance,radius);
    return GeoHit(1u,distance,state.position,state.tangent,vec4<f32>(0,0,0,-1));
}

fn {{self}}(base:u32,ray:GeoRay,previous_identity:u32)->GeoTaggedHit {
    return GeoTaggedHit({{self}}_intersect(ray,select(0.0,8.0*EPS*params.misc.y,base==previous_identity),geo_infinity(),params.misc.y),base);
}
