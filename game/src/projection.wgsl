// Quaternion components are scalar first. Vertex coordinates are (X,Y,Z,W).
fn quat_mul(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(a.x*b.x - dot(a.yzw, b.yzw),
        a.x*b.yzw + b.x*a.yzw + cross(a.yzw, b.yzw));
}

fn place_vertex(real: vec4<f32>, imag: vec4<f32>, vertex: vec4<f32>) -> vec4<f32> {
    // For e² = -1, sandwich W + e*(0,X,Y,Z) by the camera-relative pair.
    let vector = vec4<f32>(0.0, vertex.xyz);
    let a = vertex.w * real - quat_mul(imag, vector);
    let b = quat_mul(real, vector) + vertex.w * imag;
    let real_conj = vec4<f32>(real.x, -real.yzw);
    let imag_conj = vec4<f32>(imag.x, -imag.yzw);
    let xyz = (quat_mul(b, real_conj) - quat_mul(a, imag_conj)).yzw;
    let w = (quat_mul(a, real_conj) + quat_mul(b, imag_conj)).x;
    return vec4<f32>(xyz, w);
}
