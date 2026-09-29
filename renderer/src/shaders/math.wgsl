// Shared quaternion algebra and tangent rotations.
const PI: f32 = 3.14159265358979323846;
const EPS: f32 = 1e-6;

fn qconj(a: vec4<f32>) -> vec4<f32> { return vec4<f32>(a.x, -a.yzw); }
fn qmul(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(a.x*b.x-dot(a.yzw,b.yzw),
        a.x*b.yzw+b.x*a.yzw+cross(a.yzw,b.yzw));
}
fn rotate_vector(q: vec4<f32>, v: vec3<f32>) -> vec3<f32> {
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
