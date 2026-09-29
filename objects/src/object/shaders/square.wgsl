fn {{self}}_remainder(value:i32,modulus:i32)->i32 {return ((value%modulus)+modulus)%modulus;}
fn {{self}}(embedded:vec4<f32>,cell:f32,width:f32,count:u32)->u32 {
    let position = geo_to_half_space_pos(embedded);
    var index=0i;
    var border=false;
        let g = position.xy/cell;
        let whole = floor(g);
        let part = fract(g);
        let border_ratio = width/cell;
        border = any(part<vec2<f32>(border_ratio)) || any(part>vec2<f32>(1-border_ratio));
        let hx = {{self}}_remainder(i32(whole.x),2);
        let hy = {{self}}_remainder(i32(whole.y),2);
        index = 3*hy-2*hx*hy+hx;
    if border {return count;}
    return u32({{self}}_remainder(index,i32(count)));
}
