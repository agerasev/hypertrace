struct SurfaceSun {
    radiance: vec4<f32>, // surface RGB, ambient response factor
    ambient: vec4<f32>,  // additive fill RGB, ambient response exponent
}
struct Lighting {
    night: vec4<f32>, // shared night fill RGB, number of Suns
    suns: array<SurfaceSun, MAX_SUNS>,
}
@group(2) @binding(0) var<uniform> lighting: Lighting;
@group(2) @binding(1) var<uniform> surface_lights: array<array<vec4<f32>, MAX_SUNS>, SURFACES_PER_BATCH>;

// Integrate max(0, N·direction)/pi over a circular cap of uniform radiance.
// mu is the cosine to its axis; s and c are sin and cos of its angular radius.
fn diffuse_cap(mu: f32, s: f32, c: f32) -> f32 {
    if mu >= s { return mu*s*s; }
    if mu <= -s { return 0.0; }
    // The cap crosses the receiver's horizon. Its projected area is bounded
    // by arcs of the cap and the horizon, with the following half-angles.
    let q = sqrt(max(0.0, (s-mu)*(s+mu)));
    let cap_arc = atan2(q, -c*mu);
    let horizon_arc = atan2(q, c);
    return max(0.0, (cap_arc*s*s*mu + horizon_arc - c*q)/PI);
}

fn illuminate(input: VaryingData) -> vec3<f32> {
    let k = input.local_klein;
    // The fragment's visible face points toward the eye; screen Y grows down.
    let face = cross(dpdy(k), dpdx(k));
    let rho2 = 1.0 - dot(k,k);
    let unlit = all(input.reflection == vec3<f32>(0.0)) && all(input.transmission == vec3<f32>(0.0));
    if rho2 <= 0.0 || unlit {
        return input.emission;
    }

    // Orient the interpolated vertex normal to the visible side of the triangle.
    var n = face;
    let interpolated = input.smooth_normal;
    if dot(interpolated,interpolated) > 0.0 {
        let alignment = dot(face,interpolated) - dot(face,k)*dot(interpolated,k);
        n = select(-interpolated, interpolated, alignment >= 0.0);
    }
    // (n,n·k) is a Lorentz normal to this geodesic triangle. Normalize it
    // in its object frame, where finite meshes and nearby plane patches live.
    let nw = dot(n,k);
    let normal2 = dot(n,n) - nw*nw;
    if normal2 <= 0.0 { return input.emission; }
    let normal = vec4<f32>(n,nw) / sqrt(normal2);
    let planes = array(input.sun_plane_0, input.sun_plane_1, input.sun_plane_2);
    let rho = sqrt(rho2);
    var front_light = vec3<f32>(0.0);
    var back_light = vec3<f32>(0.0);
    let receives_ambient = input.receives_ambient > 0.5;
    var fill = select(vec3<f32>(0.0), lighting.night.rgb, receives_ambient);
    for (var i = 0u; i < u32(lighting.night.w); i++) {
        let light = lighting.suns[i];
        let sun = planes[i];
        // For P=(k,1)/rho, the signed distance d satisfies sinh(d)=t/rho.
        let t = dot(k,sun.xyz) - sun.w;
        let denom = length(vec2<f32>(rho, t));
        let s = rho/denom; // sin(angular disk radius) = sech(d)
        let c = abs(t)/denom;
        let normal_sun = dot(normal.xyz,sun.xyz) - normal.w*sun.w;
        let mu = clamp(select(1.0, -1.0, t >= 0.0)*normal_sun*s, -1.0, 1.0);
        let front = diffuse_cap(mu,s,c);
        // The signed cosine integral over the whole disk is mu*s².
        let back = max(0.0, front - mu*s*s);
        front_light += light.radiance.rgb * front;
        back_light += light.radiance.rgb * back;
        if receives_ambient {
            let peak = max(light.radiance.r, max(light.radiance.g, light.radiance.b));
            let daylight = min((peak*s*s)*light.radiance.w, 1.0);
            fill += light.ambient.rgb * pow(daylight, light.ambient.w);
        }
    }
    return input.emission + input.reflection * (fill + front_light)
        + input.transmission * (fill + back_light);
}

// Fit overbright highlights into the display gamut while preserving their hue.
fn display_linear(linear: vec3<f32>) -> vec3<f32> {
    return linear / max(1.0, max(linear.r, max(linear.g, linear.b)));
}

fn encode_srgb(linear: vec3<f32>) -> vec3<f32> {
    return select(12.92*linear, 1.055*pow(linear,vec3<f32>(1.0/2.4))-0.055, linear > vec3<f32>(0.0031308));
}
