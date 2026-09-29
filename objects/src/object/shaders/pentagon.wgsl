fn {{self}}_remainder(value:i32,modulus:i32)->i32 {return ((value%modulus)+modulus)%modulus;}
fn {{self}}(position:vec4<f32>,width:f32,count:u32,star:bool)->u32 {
    var index=0i;
    var border=false;
    // Reduce through five adjacent pentagons. Positions and isometries stay
    // embedded; half-space coordinates only classify edges and borders.
    let q0 = sqrt(cos(PI/4+PI/5)/cos(PI/4-PI/5));
    let t0 = sqrt(cos(PI/4+PI/5)*cos(PI/4-PI/5));
    let s0 = (cos(PI/4)-sin(PI/5))/t0;
    let l = t0/cos(PI/4);
    let k = l*(2*cos(PI/5)-1/cos(PI/5));
    let q = log((1+q0)/(1-q0));
    let s = log((1+s0)/(1-s0));
    var p = position;
    var odd = false;
    if p.y < 0 { p.y=-p.y; odd=!odd; }
    if p.z < 0 { p.z=-p.z; odd=!odd; }
    p = geo_map_apply(geo_chain(geo_translation(vec3<f32>(1,0,0),-q),geo_rotation(vec3<f32>(0,0,1),-PI/4)),p);
    var edge = false;
    for (var j=0i; j<5; j+=1) {
        let chart = geo_to_half_space_pos(p);
        var a = array<bool,3>(false,false,false);
        for (var i=0i; i<3-select(0,1,edge); i+=1) {
            let angle = 2*PI*f32(i-1)/5;
            a[i] = dot(vec2<f32>(cos(angle),sin(angle)),chart.xy)<l;
        }
        a[2] = a[2] || edge;
        let adjacent_count = select(0,1,a[0])+select(0,1,a[1])+select(0,1,a[2]);
        if adjacent_count == 3 { break; }
        if adjacent_count == 2 {
            // Same missing-side index as (!a[1]) + 2*(!a[2]). Keeping
            // the complement outside the selects avoids an observed
            // optimization-sensitive divergence on Intel/Mesa 23.2.1.
            let i = 3-(select(0,1,a[1])+2*select(0,1,a[2]));
            let angle = 2*PI*f32(i-1)/5;
            p = geo_map_apply(geo_chain(geo_rotation(vec3<f32>(0,0,1),-PI/5),
                geo_chain(geo_translation(vec3<f32>(1,0,0),-2*s),geo_rotation(vec3<f32>(0,0,1),-angle))),p);
            edge = true;
            odd = !odd;
        } else {
            let i = select(0,1,a[0]);
            let angle = PI*f32(2*i-1)/5;
            p = geo_map_apply(geo_chain(geo_translation(vec3<f32>(1,0,0),-2*q),geo_rotation(vec3<f32>(0,0,1),-angle)),p);
            edge = false;
        }
    }
    let chart = geo_to_half_space_pos(p);
    for (var i=0i; i<5; i+=1) {
        let angle = 2*PI*f32(i)/5;
        let projection = dot(vec2<f32>(cos(angle),sin(angle)),chart.xy);
        border = border || projection>l-width*chart.z;
        if star {
            let ps = k+projection;
            odd = odd != (ps<0);
            border = border || abs(ps)<width*chart.z;
        }
    }
    index = select(1,0,odd || count<2u);
    if border {return count;}
    return u32({{self}}_remainder(index,i32(count)));
}
