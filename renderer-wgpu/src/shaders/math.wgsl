// Hyperbolic geometry uses the upper half-space (x,y,z), z > 0.
// Complex 2x2 matrices remain the stored isometries. Quaternions below are
// temporary values, with real-first components, q = x + yi + zj + wk.
const PI: f32 = 3.14159265358979323846;
const EPS: f32 = 1e-6;

struct HyMap { ab: vec4<f32>, cd: vec4<f32> }
struct Ray { position: vec3<f32>, direction: vec3<f32> }
struct Hit {
    valid: u32,
    distance: f32,
    position: vec3<f32>,
    direction: vec3<f32>,
    normal: vec3<f32>,
}

fn cmul(a: vec2<f32>, b: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(a.x*b.x-a.y*b.y, a.x*b.y+a.y*b.x);
}
fn qconj(a: vec4<f32>) -> vec4<f32> { return vec4<f32>(a.x, -a.yzw); }
fn qmul(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(a.x*b.x-dot(a.yzw,b.yzw),
        a.x*b.yzw+b.x*a.yzw+cross(a.yzw,b.yzw));
}
fn cqmul(a: vec2<f32>, b: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(a.x*b.x-a.y*b.y, a.x*b.y+a.y*b.x,
        a.x*b.z-a.y*b.w, a.x*b.w+a.y*b.z);
}
fn qinverse(a: vec4<f32>) -> vec4<f32> { return qconj(a)/dot(a,a); }
fn hy_identity() -> HyMap {
    return HyMap(vec4<f32>(1,0,0,0), vec4<f32>(0,0,1,0));
}
fn hy_inverse(m: HyMap) -> HyMap {
    // All stored transforms have determinant one, by the transform representation’s contract.
    return HyMap(vec4<f32>(m.cd.zw,-m.ab.zw),vec4<f32>(-m.cd.xy,m.ab.xy));
}
fn hy_chain(a: HyMap, b: HyMap) -> HyMap {
    return HyMap(vec4<f32>(cmul(a.ab.xy,b.ab.xy)+cmul(a.ab.zw,b.cd.xy),
        cmul(a.ab.xy,b.ab.zw)+cmul(a.ab.zw,b.cd.zw)),
        vec4<f32>(cmul(a.cd.xy,b.ab.xy)+cmul(a.cd.zw,b.cd.xy),
        cmul(a.cd.xy,b.ab.zw)+cmul(a.cd.zw,b.cd.zw)));
}
fn hy_apply_pos(m: HyMap, p: vec3<f32>) -> vec3<f32> {
    let q = vec4<f32>(p,0);
    let numerator = cqmul(m.ab.xy,q)+vec4<f32>(m.ab.zw,0,0);
    let denominator = cqmul(m.cd.xy,q)+vec4<f32>(m.cd.zw,0,0);
    return qmul(numerator,qinverse(denominator)).xyz;
}
fn hy_apply_dir(m: HyMap, p: vec3<f32>, direction: vec3<f32>) -> vec3<f32> {
    let q = vec4<f32>(p,0);
    let v = vec4<f32>(direction,0);
    let u = cqmul(m.ab.xy,q)+vec4<f32>(m.ab.zw,0,0);
    let d = cqmul(m.cd.xy,q)+vec4<f32>(m.cd.zw,0,0);
    let d2 = dot(d,d);
    let dv = cqmul(m.cd.xy,v);
    let g1 = qmul(cqmul(m.ab.xy,v),qinverse(d));
    let g2 = qmul(u,(qconj(dv)-(2*dot(d,dv)/d2)*qconj(d))/d2);
    return normalize((g1+g2).xyz);
}
fn hy_map_ray(m: HyMap, ray: Ray) -> Ray {
    return Ray(hy_apply_pos(m,ray.position),hy_apply_dir(m,ray.position,ray.direction));
}
fn hy_xshift(distance: f32) -> HyMap {
    let c = cosh(distance/2);
    let s = sinh(distance/2);
    return HyMap(vec4<f32>(c,0,s,0),vec4<f32>(s,0,c,0));
}
fn hy_zrotate(angle: f32) -> HyMap {
    let c = cos(angle/2);
    let s = sin(angle/2);
    return HyMap(vec4<f32>(c,s,0,0),vec4<f32>(0,0,c,-s));
}
fn norm3(v: vec3<f32>) -> f32 {
    let scale = max(max(abs(v.x),abs(v.y)),abs(v.z));
    if scale == 0 { return 0; }
    return scale*length(v/scale);
}
fn hy_distance(a: vec3<f32>, b: vec3<f32>) -> f32 {
    let x = (norm3(a-b)/sqrt(a.z))/sqrt(b.z)*0.5;
    // Avoid relying on a backend's log(1+x) implementation for close points.
    if x < 0.001 { return 2*x*(1-x*x/6); }
    if x > 1e10 { return 2*(log(x)+log(2.0)); }
    return 2*asinh(x);
}
fn hy_dir_at(p: vec3<f32>, d: vec3<f32>, h: vec3<f32>) -> vec3<f32> {
    return normalize(vec3<f32>(h.z/p.z*d.xy,
        d.z-length(p.xy-h.xy)/p.z*length(d.xy)));
}
fn eu_rotate(q: vec4<f32>, v: vec3<f32>) -> vec3<f32> {
    return qmul(qmul(q,vec4<f32>(0,v)),qconj(q)).yzw;
}
fn rotation_look_cont(d: vec3<f32>) -> vec4<f32> {
    let c = sqrt((1-d.z)*0.5);
    return vec4<f32>(c,cross(vec3<f32>(0,0,-1),d)/(2*c));
}
fn rotation_look(d: vec3<f32>) -> vec4<f32> {
    if d.z < 0 { return rotation_look_cont(d); }
    return qmul(rotation_look_cont(-d),vec4<f32>(0,1,0,0));
}
fn miss() -> Hit {
    return Hit(0u,0,vec3<f32>(0),vec3<f32>(0),vec3<f32>(0));
}
fn hy_plane(ray: Ray, repeated: bool) -> Hit {
    if repeated { return miss(); }
    let p = ray.position;
    let d = ray.direction;
    let pd = dot(p,d);
    // pd scales with the half-space height: even a vertical ray has tiny pd
    // close to the absolute. Only an exactly zero denominator is parallel.
    if pd == 0 { return miss(); }
    let t = (1-dot(p,p))/(2*pd);
    if t < -EPS { return miss(); }
    let xy = p.xy+d.xy*t;
    let z2 = 1-dot(xy,xy);
    if z2 <= 0 { return miss(); }
    let h = vec3<f32>(xy,sqrt(z2));
    return Hit(1u,hy_distance(p,h),h,hy_dir_at(p,d,h),-h);
}
fn hy_horosphere(ray: Ray, repeated: bool) -> Hit {
    let p = ray.position;
    let d = ray.direction;
    let a = dot(d.xy,d.xy);
    let b = p.z*d.z;
    let c = (1-p.z)*(1+p.z);
    let threshold = select(-EPS,EPS,repeated);
    var t: f32;
    if a == 0 {
        // Linear limit of a*t*t - 2*b*t + c = 0.
        if b == 0 { return miss(); }
        t = c/(2*b);
        if t < threshold { return miss(); }
    } else {
        let discriminant = p.z*p.z-a;
        if discriminant < 0 { return miss(); }
        let root = sqrt(discriminant);
        let q = b+select(-root,root,b >= 0);
        if q == 0 {
            t = 0;
            if t < threshold { return miss(); }
        } else {
            let t0 = q/a;
            let t1 = c/q;
            t = min(t0,t1);
            if t < threshold { t = max(t0,t1); }
            if t < threshold { return miss(); }
        }
    }
    let h = vec3<f32>(p.xy+d.xy*t,1);
    return Hit(1u,hy_distance(p,h),h,hy_dir_at(p,d,h),vec3<f32>(0,0,-1));
}
fn eu_plane(ray: Ray, repeated: bool) -> Hit {
    if repeated || ray.direction.z == 0 { return miss(); }
    let t = -ray.position.z/ray.direction.z;
    if t < -EPS { return miss(); }
    return Hit(1u,max(t,0),vec3<f32>(ray.position.xy+ray.direction.xy*t,0),
        ray.direction,vec3<f32>(0,0,-1));
}
fn eu_sphere(ray: Ray, repeated: bool) -> Hit {
    let b = -dot(ray.direction,ray.position);
    let g = ray.position+ray.direction*b;
    let discriminant = 1-dot(g,g);
    if discriminant < 0 { return miss(); }
    let root = sqrt(discriminant);
    let threshold = select(-EPS,EPS,repeated);
    var t = b-root;
    if t < threshold { t = b+root; }
    if t < threshold { return miss(); }
    let h = normalize(ray.position+ray.direction*t);
    return Hit(1u,max(t,0),h,ray.direction,h);
}
fn eu_cube(ray: Ray, repeated: bool) -> Hit {
    var near = -1e30;
    var far = 1e30;
    var normal_near = vec3<f32>(0);
    var normal_far = vec3<f32>(0);
    for (var axis = 0u; axis < 3u; axis += 1u) {
        let p = ray.position[axis];
        let d = ray.direction[axis];
        if d == 0 {
            if abs(p) > 1 { return miss(); }
        } else {
            let t0 = (-1-p)/d;
            let t1 = (1-p)/d;
            let lo = min(t0,t1);
            let hi = max(t0,t1);
            if lo >= near {
                near = lo;
                normal_near = vec3<f32>(0);
                normal_near[axis] = -sign(d);
            }
            if hi <= far {
                far = hi;
                normal_far = vec3<f32>(0);
                normal_far[axis] = sign(d);
            }
        }
    }
    if near > far { return miss(); }
    let threshold = select(-2*EPS,2*EPS,repeated);
    var t = near;
    var normal = normal_near;
    if t < threshold { t = far; normal = normal_far; }
    if t < threshold { return miss(); }
    return Hit(1u,max(t,0),ray.position+ray.direction*t,ray.direction,normal);
}
