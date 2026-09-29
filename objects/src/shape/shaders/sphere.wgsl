fn {{self}}_intersect(ray: GeoRay, minimum: f32, maximum: f32,
    radius: f32, sphere_radius: f32) -> GeoHit {
    if sphere_radius <= 0 || minimum < 0 || minimum >= maximum { return geo_miss(); }
    var distance: f32;
    var normal: vec4<f32>;
    if GEO_K == 0 {
        let center = -dot(ray.position.yzw,ray.tangent.yzw);
        let closest = ray.position.yzw+center*ray.tangent.yzw;
        let discriminant = sphere_radius*sphere_radius-dot(closest,closest);
        if discriminant < 0 { return geo_miss(); }
        let delta = sqrt(discriminant);
        distance = geo_first(center-delta,center+delta,minimum,maximum);
    } else {
        if GEO_K > 0 && sphere_radius >= PI*radius { return geo_miss(); }
        let cs = geo_cs(sphere_radius/radius);
        distance = geo_section_root(ray.position.x,ray.tangent.x,cs.x,
            minimum,maximum,radius);
    }
    if distance < 0 { return geo_miss(); }
    let state = geo_advance(ray,distance,radius);
    if GEO_K == 0 {
        normal = vec4<f32>(0,state.position.yzw/sphere_radius);
    } else {
        let cs = geo_cs(sphere_radius/radius);
        normal = (cs.x*state.position-vec4<f32>(1,0,0,0))/cs.y;
    }
    return GeoHit(1u,distance,state.position,state.tangent,normal);
}
