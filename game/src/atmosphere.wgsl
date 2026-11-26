// Additive single scattering by the full solar disk in a layer of constant density.
// Plane.w is its signed distance coefficient at the camera (-normal.w).
struct AirFrame {
    earth: vec4<f32>,
    sun: vec4<f32>,
    scattering: vec4<f32>, // solar radiance times RGB scattering coefficients, sinh(layer height)
};
@group(3) @binding(0) var<uniform> air: AirFrame;

// Along a unit eye ray, z=exp(-distance/R) goes from 1 at the eye to 0 at infinity.
// The signed sinh-distance to a plane is (a+b)/(2z) + (a-b)z/2.
// In this variable both boundaries of the layer are quadratics, even on grazing rays.
fn air_interval(direction: vec3<f32>, end: f32) -> vec2<f32> {
    let a = air.earth.w;
    let b = dot(air.earth.xyz, direction);
    let p = 0.5 * (a+b);
    let q = 0.5 * (a-b);
    var far_z = end;
    var near_z = 1.0;
    // Clip to the half-space above the ground: p + q*z² >= 0.
    if p < 0.0 {
        if q <= 0.0 { return vec2<f32>(0.0); }
        far_z = max(far_z, sqrt(-p/q));
    } else if q < 0.0 {
        near_z = min(near_z, sqrt(-p/q));
    }
    // Clip below the layer top: p - h*z + q*z² <= 0.
    let h = air.scattering.w;
    let disc = h*h - 4.0*p*q;
    if disc <= 0.0 { return vec2<f32>(0.0); }
    let denominator = h + sqrt(disc);
    far_z = max(far_z, 2.0*p/denominator);
    if q > 0.0 { near_z = min(near_z, denominator/(2.0*q)); }
    return vec2<f32>(near_z, far_z);
}

// First component equals z*exp(d/R), with d the unsigned Sun-plane distance.
// The second is d'(s)*sech²(d/R), used by the Rayleigh angular phase integral.
fn sun_endpoint(z: f32, p: f32, q: f32, scale: f32) -> vec2<f32> {
    let w = z*scale;
    let n = p + q*z*z;
    let c = length(vec2<f32>(w, n));
    if c == 0.0 { return vec2<f32>(0.0); }
    return vec2<f32>(abs(n)+c, sign(n)*((p-q*z*z)/c)*(w/c)*(w/c));
}

fn air_glow(ray: vec3<f32>, end: f32) -> vec3<f32> {
    if air.scattering.w == 0.0 { return vec3<f32>(0.0); }
    let direction = normalize(ray);
    let z = air_interval(direction, end);
    if z.x <= z.y { return vec3<f32>(0.0); }
    let a = air.sun.w;
    let b = dot(air.sun.xyz, direction);
    let scale = 1.0 / max(1.0, max(abs(a), abs(b)));
    let p = 0.5*(a*scale + b*scale);
    let q = 0.5*(a*scale - b*scale);
    let entry = sun_endpoint(z.x, p, q, scale);
    let exit = sun_endpoint(z.y, p, q, scale);
    // Integral of the solar sky fraction (1-tanh(d/R))/2. Cancel the exp(length)
    // factors algebraically so even z=0 at the ideal horizon has a finite limit.
    let denominator = entry.x*exit.x + (scale*z.y)*(scale*z.y);
    if denominator <= 0.0 { return vec3<f32>(0.0); }
    let ratio = ((z.x-z.y)*scale)*((z.x+z.y)*scale)/denominator;
    let logarithm = select(log(1.0+ratio), ratio*(1.0-0.5*ratio), ratio < 0.001);
    // Integrating 3(1+cos²(theta))/(16*pi) over the Sun disk and along the
    // ray adds this boundary term to the isotropic result.
    let integral = max(0.0, 0.5*logarithm + (entry.y-exit.y)/16.0);
    return air.scattering.rgb * integral;
}

fn air_to_surface(point: vec4<f32>) -> vec3<f32> {
    let radius = length(point.xyz);
    let end = sqrt(max(0.0, (point.w-radius)/(point.w+radius)));
    return air_glow(point.xyz, end);
}
