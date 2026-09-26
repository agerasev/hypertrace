// Shared by the baseline oracle and generated scene renderer.
fn uniform_random(state: ptr<function,u32>) -> f32 {
    *state = 1103515245u*(*state)+12345u;
    return f32(*state)*(1.0/4294967296.0);
}
fn object_ray_to_local(object: Object, ray: Ray) -> Ray {
    if params.info.z == 1u {
        return hy_map_ray(hy_inverse(HyMap(object.map0,object.map1)),ray);
    }
    let inverse = qconj(object.map1);
    return Ray(eu_rotate(inverse,ray.position-object.map0.xyz),
        eu_rotate(inverse,ray.direction));
}
fn object_ray_to_world(object: Object, ray: Ray) -> Ray {
    if params.info.z == 1u { return hy_map_ray(HyMap(object.map0,object.map1),ray); }
    return Ray(eu_rotate(object.map1,ray.position)+object.map0.xyz,
        eu_rotate(object.map1,ray.direction));
}
fn positive_remainder(value: i32, modulus: i32) -> i32 {
    return ((value%modulus)+modulus)%modulus;
}
fn tiled_material(object: Object, position: vec3<f32>) -> u32 {
    let kind = object.info.w;
    if kind == 0u { return object.info.y; }
    var index = 0i;
    var border = false;
    if kind == 1u {
        let g = position.xy/object.props.x;
        let whole = floor(g);
        let part = fract(g);
        let width = object.props.y/object.props.x;
        border = any(part<vec2<f32>(width)) || any(part>vec2<f32>(1-width));
        let hx = positive_remainder(i32(whole.x),2);
        let hy = positive_remainder(i32(whole.y),2);
        index = 3*hy-2*hx*hy+hx;
    } else if kind == 2u {
        let bx = vec2<f32>(2/sqrt(3.0),0);
        let by = vec2<f32>(-1/sqrt(3.0),1);
        let size = object.props.x*sqrt(3.0)/2;
        let width = object.props.y/object.props.x;
        var h = vec2<f32>(dot(bx,position.xy),dot(by,position.xy))/size;
        let hx = i32(floor((floor(h.x)-floor(h.y))/3));
        let hy = i32(floor((floor(h.x+h.y)-f32(hx))/2));
        h -= f32(hx)*vec2<f32>(2,-1)+f32(hy)*vec2<f32>(1,1);
        border = abs(h.x-1)>1-width || abs(h.y)>1-width || abs(h.x+h.y-1)>1-width;
        index = 2*hx+hy;
    } else {
        // Same five-step pentagon reduction as the original scene, including
        // its finite tiling depth. A deeper tessellation is a separate change.
        let q0 = sqrt(cos(PI/4+PI/5)/cos(PI/4-PI/5));
        let t0 = sqrt(cos(PI/4+PI/5)*cos(PI/4-PI/5));
        let s0 = (cos(PI/4)-sin(PI/5))/t0;
        let l = t0/cos(PI/4);
        let k = l*(2*cos(PI/5)-1/cos(PI/5));
        let q = log((1+q0)/(1-q0));
        let s = log((1+s0)/(1-s0));
        var p = position;
        var odd = false;
        if p.x < 0 { p.x=-p.x; odd=!odd; }
        if p.y < 0 { p.y=-p.y; odd=!odd; }
        p = hy_apply_pos(hy_chain(hy_xshift(-q),hy_zrotate(-PI/4)),p);
        var edge = false;
        for (var j=0i; j<5; j+=1) {
            var a = array<bool,3>(false,false,false);
            for (var i=0i; i<3-select(0,1,edge); i+=1) {
                let angle = 2*PI*f32(i-1)/5;
                a[i] = dot(vec2<f32>(cos(angle),sin(angle)),p.xy)<l;
            }
            a[2] = a[2] || edge;
            let count = select(0,1,a[0])+select(0,1,a[1])+select(0,1,a[2]);
            if count == 3 { break; }
            if count == 2 {
                // Same missing-side index as (!a[1]) + 2*(!a[2]). Keeping
                // the complement outside the selects avoids an observed
                // optimization-sensitive divergence on Intel/Mesa 23.2.1.
                let i = 3-(select(0,1,a[1])+2*select(0,1,a[2]));
                let angle = 2*PI*f32(i-1)/5;
                p = hy_apply_pos(hy_chain(hy_zrotate(-PI/5),
                    hy_chain(hy_xshift(-2*s),hy_zrotate(-angle))),p);
                edge = true;
                odd = !odd;
            } else {
                let i = select(0,1,a[0]);
                let angle = PI*f32(2*i-1)/5;
                p = hy_apply_pos(hy_chain(hy_xshift(-2*q),hy_zrotate(-angle)),p);
                edge = false;
            }
        }
        let width = object.props.y;
        for (var i=0i; i<5; i+=1) {
            let angle = 2*PI*f32(i)/5;
            let projection = dot(vec2<f32>(cos(angle),sin(angle)),p.xy);
            border = border || projection>l-width*p.z;
            if kind == 4u {
                let ps = k+projection;
                odd = odd != (ps<0);
                border = border || abs(ps)<width*p.z;
            }
        }
        index = select(1,0,odd || object.info.z<2u);
    }
    if border { return object.extra.x; }
    return object.info.y+u32(positive_remainder(index,i32(object.info.z)));
}
fn background(direction: vec3<f32>) -> vec3<f32> {
    if params.options.z == 0u { return params.background0.xyz; }
    let factor = pow(clamp(0.5*(dot(direction,params.background_axis.xyz)+1),0,1),params.misc.z);
    return factor*params.background0.xyz+(1-factor)*params.background1.xyz;
}
fn primary_ray(pixel: vec2<u32>, state: ptr<function,u32>) -> Ray {
    let jitter_x = uniform_random(state);
    let jitter_y = uniform_random(state);
    let screen = vec2<f32>(2*f32(pixel.x)+1-f32(params.info.x),
        f32(params.info.y)-(2*f32(pixel.y)+1))/f32(params.info.y);
    let jitter = 2/f32(params.info.y)*(vec2<f32>(jitter_x,jitter_y)-0.5);
    let direction = normalize(vec3<f32>(screen+jitter,-1/params.misc.x));
    if params.info.z == 1u {
        return hy_map_ray(HyMap(params.camera0,params.camera1),Ray(vec3<f32>(0,0,1),direction));
    }
    return Ray(params.camera0.xyz,eu_rotate(params.camera1,direction));
}
