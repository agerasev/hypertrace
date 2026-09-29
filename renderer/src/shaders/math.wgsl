// Shared quaternion algebra, tangent rotations, and half-space chart helpers.
const PI: f32 = 3.14159265358979323846;
const EPS: f32 = 1e-6;

struct HyMap { ab: vec4<f32>, cd: vec4<f32> }
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
