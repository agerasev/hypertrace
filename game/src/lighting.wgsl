struct Lighting {
    sun: vec4<f32>,
    night: vec4<f32>, // RGB radiance, ambient response factor
    day: vec4<f32>,   // RGB radiance, ambient response exponent
}
@group(2) @binding(0) var<uniform> lighting: Lighting;

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

fn illuminate(input: VaryingData, albedo: vec3<f32>) -> vec3<f32> {
    let k = input.local_klein;
    // The fragment's visible face points toward the eye; screen Y grows down.
    let face = cross(dpdy(k), dpdx(k));
    let rho2 = 1.0 - dot(k,k);
    if input.material.x > 0.5 { return albedo; }
    if rho2 <= 0.0 { return vec3<f32>(0.0); }

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
    if normal2 <= 0.0 { return vec3<f32>(0.0); }
    let normal = vec4<f32>(n,nw) / sqrt(normal2);
    let sun = input.sun_plane;

    // P=(k,1)/rho is the unit hyperboloid point. Its signed distance to
    // the Sun satisfies sinh(d)=g(P,sun)=t/rho, in curvature-radius units.
    let t = dot(k,sun.xyz) - sun.w;
    let denom = sqrt(rho2 + t*t);
    let s = sqrt(rho2)/denom; // sech(d), sine of the apparent angular radius
    let c = abs(t)/denom;
    let normal_sun = dot(normal.xyz,sun.xyz) - normal.w*sun.w;
    let mu = clamp(select(1.0, -1.0, t >= 0.0)*normal_sun*s, -1.0, 1.0);
    let front = diffuse_cap(mu,s,c);
    // The signed cosine integral is mu*s². Subtracting it from the
    // front integral gives the light incident on the back of a thin leaf.
    let back = max(0.0, front - mu*s*s);
    // The projected solar disk area (in units of pi) drives the local diffuse fill.
    let sun_peak = max(lighting.sun.r, max(lighting.sun.g, lighting.sun.b));
    let daylight = min((sun_peak * s*s) * lighting.night.w, 1.0);
    let fill = mix(lighting.night.rgb, lighting.day.rgb, pow(daylight, lighting.day.w));
    return albedo * (fill + lighting.sun.rgb * mix(front, back, input.material.y));
}

// Fit overbright highlights into the display gamut while preserving their hue.
fn display_linear(linear: vec3<f32>) -> vec3<f32> {
    return linear / max(1.0, max(linear.r, max(linear.g, linear.b)));
}

fn encode_srgb(linear: vec3<f32>) -> vec3<f32> {
    return select(12.92*linear, 1.055*pow(linear,vec3<f32>(1.0/2.4))-0.055, linear > vec3<f32>(0.0031308));
}
