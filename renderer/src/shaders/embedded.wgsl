// Shared scalar-first constant-curvature kernel. Shader assembly supplies GEO_K
// as -1, 0, or +1. math.wgsl supplies quaternion arithmetic and PI.
// Positions/tangents are normalized ambient coordinates; distances and query
// intervals are physical. Surface queries include minimum and exclude maximum.
struct GeoRay { position: vec4<f32>, tangent: vec4<f32> }
// Custom leaves return valid=0 (miss) or 1 (surface). Value 2 is reserved
// internally for numerical failure and must propagate without becoming a miss.
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
fn geo_failure() -> GeoHit {
    return GeoHit(2u,0,vec4<f32>(0),vec4<f32>(0),vec4<f32>(0));
}
// Emergency corruption guards, not a precision certification. The tested local
// hyperbolic range is <=3 radii, while valid rays outside it may continue. A
// failed guard terminates transport without clamping distance or adding a miss
// background. Later per-path recentering can extend the useful numerical range.
fn geo_ray_supported(ray: GeoRay) -> bool {
    let largest = bitcast<f32>(0x7f7fffffu);
    if !all(abs(ray.position) <= vec4<f32>(largest)) ||
        !all(abs(ray.tangent) <= vec4<f32>(largest)) { return false; }
    let tolerance = 0.01;
    if GEO_K == 0 {
        return abs(ray.position.x-1) <= tolerance &&
            abs(ray.tangent.x) <= tolerance &&
            abs(dot(ray.tangent.yzw,ray.tangent.yzw)-1) <= tolerance;
    }
    if GEO_K < 0 && ray.position.x <= 0 { return false; }
    return abs(geo_metric(ray.position,ray.position)-1) <= tolerance &&
        abs(GEO_K*geo_metric(ray.tangent,ray.tangent)-1) <= tolerance &&
        abs(geo_metric(ray.position,ray.tangent)) <= tolerance;
}
fn geo_advance_supported(ray: GeoRay, distance: f32, radius: f32) -> bool {
    let largest = bitcast<f32>(0x7f7fffffu);
    if !(distance >= 0 && distance <= largest && radius > 0 && radius <= largest) ||
        !geo_ray_supported(ray) { return false; }
    let phase = distance/select(1.0,radius,GEO_K != 0);
    if !(phase <= largest) { return false; }
    // Avoid exponential overflow before evaluating a hyperbolic free flight.
    // This is an emergency arithmetic bound, never a substitute surface hit.
    if GEO_K < 0 && phase > 40 { return false; }
    return geo_ray_supported(geo_advance(ray,distance,radius));
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
// A section discriminant subtracts squared f32 coefficients. Within a few
// coefficient ULPs its sign cannot reliably distinguish crossing, tangency,
// and a miss (native trig can move a tangent coefficient by an ULP too).
// The first-order perturbation of x*x is 2*x*dx. Taking |dx|<=4*eps*|x|
// gives 8*eps*(a*a+b*b+c*c) as the corresponding backward-error scale.
// Only this scale-relative uncertainty band is represented by a double root;
// clear negative discriminants remain misses. No distance tolerance is added.
fn geo_classify_section_discriminant(value: f32, a: f32, b: f32, c: f32) -> f32 {
    let coefficient_error = 8.0*1.1920928955078125e-7*(a*a+b*b+c*c);
    if abs(value) <= coefficient_error { return 0; }
    return value;
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
        if amplitude == 0 { return -1; }
        let discriminant = geo_classify_section_discriminant(
            (amplitude-c)*(amplitude+c),a,b,c);
        if discriminant < 0 { return -1; }
        let phase = atan2(b,a);
        // atan2 avoids the relatively loose native acos approximation on some
        // adapters, while retaining the correct branch for radii above pi/2.
        let delta = atan2(sqrt(discriminant),c);
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
    let discriminant = geo_classify_section_discriminant(c*c-leading*constant,a,b,c);
    if discriminant < 0 { return -1; }
    if discriminant == 0 {
        let repeated = c/leading;
        if repeated <= 0 { return -1; }
        return geo_first(log(repeated)*radius,-1,minimum,maximum);
    }
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
