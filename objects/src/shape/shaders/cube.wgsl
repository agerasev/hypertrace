fn {{self}}_intersect(ray: GeoRay, minimum: f32, maximum: f32, radius: f32) -> GeoHit {
    if GEO_K != 0 || minimum < 0 || minimum >= maximum { return geo_miss(); }
    var near = -1e30;
    var far = 1e30;
    var normal_near = vec4<f32>(0);
    var normal_far = vec4<f32>(0);
    for (var axis = 1u; axis < 4u; axis += 1u) {
        let p = ray.position[axis];
        let d = ray.tangent[axis];
        if d == 0 {
            if abs(p) > 1 { return geo_miss(); }
        } else {
            let first = (-1-p)/d;
            let second = (1-p)/d;
            let lo = min(first,second);
            let hi = max(first,second);
            if lo >= near {
                near = lo;
                normal_near = vec4<f32>(0);
                normal_near[axis] = -sign(d);
            }
            if hi <= far {
                far = hi;
                normal_far = vec4<f32>(0);
                normal_far[axis] = sign(d);
            }
        }
    }
    if near > far { return geo_miss(); }
    let distance = geo_first(near,far,minimum,maximum);
    if distance < 0 { return geo_miss(); }
    let state = geo_advance(ray,distance,radius);
    let normal = select(normal_far,normal_near,distance == near);
    return GeoHit(1u,distance,state.position,state.tangent,normal);
}

fn {{self}}(base:u32,ray:GeoRay,previous_identity:u32)->GeoTaggedHit {
    return GeoTaggedHit({{self}}_intersect(ray,select(0.0,8.0*EPS*params.misc.y,base==previous_identity),geo_infinity(),params.misc.y),base);
}
