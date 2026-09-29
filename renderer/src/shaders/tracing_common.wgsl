// Shared random sampling and background evaluation.
fn uniform_random(state: ptr<function,u32>) -> f32 {
    *state = 1103515245u*(*state)+12345u;
    return f32(*state)*(1.0/4294967296.0);
}
fn background(direction: vec3<f32>) -> vec3<f32> {
    if params.options.z == 0u { return params.background0.xyz; }
    let factor = pow(clamp(0.5*(dot(direction,params.background_axis.xyz)+1),0,1),params.misc.z);
    return factor*params.background0.xyz+(1-factor)*params.background1.xyz;
}
