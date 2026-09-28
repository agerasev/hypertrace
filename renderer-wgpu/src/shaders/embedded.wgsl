// Shared scalar-first constant-curvature kernel. Shader assembly supplies GEO_K
// as -1, 0, or +1. math.wgsl supplies quaternion arithmetic and PI.
// Positions/tangents are normalized ambient coordinates; distances and query
// intervals are physical. Surface queries include minimum and exclude maximum.
struct GeoRay { position: vec4<f32>, tangent: vec4<f32> }
struct GeoHit {
    valid: u32,
    distance: f32,
    position: vec4<f32>,
    tangent: vec4<f32>,
    normal: vec4<f32>,
}
struct GeoMap { a: vec4<f32>, b: vec4<f32> }

fn geo_metric(p: vec4<f32>, q: vec4<f32>) -> f32 {
    return p.x*q.x + GEO_K*dot(p.yzw,q.yzw);
}
fn geo_miss() -> GeoHit {
    return GeoHit(0u,0,vec4<f32>(0),vec4<f32>(0),vec4<f32>(0));
}
fn geo_identity() -> GeoMap {
    return GeoMap(vec4<f32>(1,0,0,0),vec4<f32>(0));
}
fn geo_chain(outer: GeoMap, inner: GeoMap) -> GeoMap {
    return GeoMap(qmul(outer.a,inner.a)+GEO_K*qmul(outer.b,inner.b),
        qmul(outer.a,inner.b)+qmul(outer.b,inner.a));
}
fn geo_inverse(map: GeoMap) -> GeoMap {
    return GeoMap(qconj(map.a),qconj(map.b));
}
fn geo_map_apply(map: GeoMap, p: vec4<f32>) -> vec4<f32> {
    // g*(w + e*(0,xyz))*combined_conjugate(g), valid also for tangents.
    let v = vec4<f32>(0,p.yzw);
    let a = p.x*map.a + GEO_K*qmul(map.b,v);
    let b = qmul(map.a,v) + p.x*map.b;
    let real = qmul(a,qconj(map.a))-GEO_K*qmul(b,qconj(map.b));
    let imag = qmul(b,qconj(map.a))-qmul(a,qconj(map.b));
    return vec4<f32>(real.x,imag.yzw);
}
fn geo_map_ray(map: GeoMap, ray: GeoRay) -> GeoRay {
    return GeoRay(geo_map_apply(map,ray.position),geo_map_apply(map,ray.tangent));
}
fn geo_cs(t: f32) -> vec2<f32> {
    if GEO_K < 0 { return vec2<f32>(cosh(t),sinh(t)); }
    if GEO_K > 0 {
        // Reduce only the coordinate-evaluation phase, never event distance.
        let phase = t - floor((t+PI)/(2*PI))*(2*PI);
        return vec2<f32>(cos(phase),sin(phase));
    }
    return vec2<f32>(1,t);
}
fn geo_advance(ray: GeoRay, distance: f32, radius: f32) -> GeoRay {
    let t = distance/select(1.0,radius,GEO_K != 0);
    let cs = geo_cs(t);
    return GeoRay(cs.x*ray.position+cs.y*ray.tangent,
        -GEO_K*cs.y*ray.position+cs.x*ray.tangent);
}
fn geo_from_local(p: vec4<f32>, direction: vec3<f32>) -> vec4<f32> {
    if GEO_K > 0 { return qmul(p,vec4<f32>(0,direction)); }
    if GEO_K < 0 {
        let scalar = dot(p.yzw,direction);
        return vec4<f32>(scalar,direction+p.yzw*(scalar/(p.x+1)));
    }
    return vec4<f32>(0,direction);
}
fn geo_to_local(p: vec4<f32>, tangent: vec4<f32>) -> vec3<f32> {
    if GEO_K > 0 { return qmul(qconj(p),tangent).yzw; }
    if GEO_K < 0 { return tangent.yzw-p.yzw*(tangent.x/(p.x+1)); }
    return tangent.yzw;
}
fn geo_from_chart_pos(p: vec3<f32>) -> vec4<f32> {
    if GEO_K < 0 {
        let square = dot(p,p);
        return vec4<f32>((square+1)/(2*p.z),p.xy/p.z,(square-1)/(2*p.z));
    }
    // Legacy charts are only defined for Euclidean and hyperbolic geometries.
    // Scene validation rejects spherical legacy custom leaves.
    return vec4<f32>(1,p);
}
fn geo_to_chart_pos(p: vec4<f32>) -> vec3<f32> {
    if GEO_K < 0 { return vec3<f32>(p.yz,1)/(p.x-p.w); }
    return p.yzw;
}
fn geo_from_chart_dir(p: vec4<f32>, direction: vec3<f32>) -> vec4<f32> {
    if GEO_K < 0 {
        let chart = geo_to_chart_pos(p);
        let horizontal = dot(chart.xy,direction.xy);
        return vec4<f32>(horizontal+(chart.z-p.x)*direction.z,
            direction.xy-p.yz*direction.z,
            horizontal+(chart.z-p.w)*direction.z);
    }
    return vec4<f32>(0,direction);
}
fn geo_to_chart_dir(p: vec4<f32>, tangent: vec4<f32>) -> vec3<f32> {
    if GEO_K < 0 {
        let chart = geo_to_chart_pos(p);
        let vertical = tangent.x-tangent.w;
        return vec3<f32>(tangent.yz-chart.xy*vertical,-chart.z*vertical);
    }
    return tangent.yzw;
}

fn geo_in_interval(distance: f32, minimum: f32, maximum: f32) -> bool {
    return distance >= minimum && distance < maximum;
}
fn geo_first(a: f32, b: f32, minimum: f32, maximum: f32) -> f32 {
    var first = -1.0;
    if geo_in_interval(a,minimum,maximum) { first = a; }
    if geo_in_interval(b,minimum,maximum) && (first < 0 || b < first) { first = b; }
    return first;
}
fn geo_periodic_root(root: f32, minimum: f32, maximum: f32, radius: f32) -> f32 {
    let period = 2*PI*radius;
    let base = root*radius;
    var lifted = base+ceil((minimum-base)/period)*period;
    // The quotient and subsequent multiply may round on opposite sides of an
    // interval endpoint. Correct one period in either direction before testing
    // the half-open interval; this also makes a returned hit reusable as min.
    if lifted < minimum { lifted += period; }
    if lifted-period >= minimum { lifted -= period; }
    if geo_in_interval(lifted,minimum,maximum) { return lifted; }
    return -1;
}
fn geo_refine_periodic(root: f32, a: f32, b: f32, c: f32) -> f32 {
    let cs = geo_cs(root);
    let derivative = -a*cs.y+b*cs.x;
    // Near a double root, Newton division magnifies f32 residual roundoff;
    // retain the phase solution there instead of perturbing a tangent contact.
    if abs(derivative) > 0.001*length(vec2<f32>(a,b)) {
        return root-(a*cs.x+b*cs.y-c)/derivative;
    }
    return root;
}
// First root of a*C_K(t)+b*S_K(t)=c, with physical interval endpoints.
fn geo_section_root(a: f32, b: f32, c: f32,
    minimum: f32, maximum: f32, radius: f32) -> f32 {
    if minimum < 0 || minimum >= maximum { return -1; }
    if GEO_K == 0 {
        if b == 0 { return -1; }
        return geo_first((c-a)/b,-1,minimum,maximum);
    }
    if GEO_K > 0 {
        let amplitude = length(vec2<f32>(a,b));
        if amplitude == 0 || abs(c) > amplitude { return -1; }
        let phase = atan2(b,a);
        // atan2 avoids the relatively loose native acos approximation on some
        // adapters, while retaining the correct branch for radii above pi/2.
        let delta = atan2(sqrt(max(0.0,(amplitude-c)*(amplitude+c))),c);
        let first = geo_periodic_root(geo_refine_periodic(phase-delta,a,b,c),
            minimum,maximum,radius);
        let second = geo_periodic_root(geo_refine_periodic(phase+delta,a,b,c),
            minimum,maximum,radius);
        return geo_first(first,second,minimum,maximum);
    }
    // u=exp(t): (a+b)*u^2 - 2*c*u + (a-b) = 0.
    // The q form retains the smaller root when subtraction would cancel it.
    let leading = a+b;
    let constant = a-b;
    if leading == 0 {
        if c == 0 { return -1; }
        let u = constant/(2*c);
        if u <= 0 { return -1; }
        return geo_first(log(u)*radius,-1,minimum,maximum);
    }
    let discriminant = c*c-leading*constant;
    if discriminant < 0 { return -1; }
    let q = c+select(-sqrt(discriminant),sqrt(discriminant),c >= 0);
    var first = -1.0;
    var second = -1.0;
    if q != 0 {
        let u0 = q/leading;
        let u1 = constant/q;
        if u0 > 0 { first = log(u0)*radius; }
        if u1 > 0 { second = log(u1)*radius; }
    } else {
        let u = c/leading;
        if u > 0 { first = log(u)*radius; }
    }
    return geo_first(first,second,minimum,maximum);
}
fn geo_plane(ray: GeoRay, minimum: f32, maximum: f32, radius: f32) -> GeoHit {
    let distance = geo_section_root(ray.position.w,ray.tangent.w,0,
        minimum,maximum,radius);
    if distance < 0 { return geo_miss(); }
    let state = geo_advance(ray,distance,radius);
    return GeoHit(1u,distance,state.position,state.tangent,vec4<f32>(0,0,0,-1));
}
fn geo_sphere(ray: GeoRay, minimum: f32, maximum: f32,
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
fn geo_cube(ray: GeoRay, minimum: f32, maximum: f32, radius: f32) -> GeoHit {
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
fn geo_horosphere(ray: GeoRay, minimum: f32, maximum: f32, radius: f32) -> GeoHit {
    if GEO_K >= 0 { return geo_miss(); }
    let distance = geo_section_root(ray.position.x-ray.position.w,
        ray.tangent.x-ray.tangent.w,1,minimum,maximum,radius);
    if distance < 0 { return geo_miss(); }
    let state = geo_advance(ray,distance,radius);
    let normal = geo_from_chart_dir(state.position,vec3<f32>(0,0,-1));
    return GeoHit(1u,distance,state.position,state.tangent,normal);
}
