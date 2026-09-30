fn {{self}}_remainder(value:i32,modulus:i32)->i32 {return ((value%modulus)+modulus)%modulus;}
fn {{self}}(position:vec4<f32>,cell:f32,width:f32,count:u32)->u32 {
    var index=0i;
    var border=false;
        let g = position.xy/cell;
        // Beyond this range f32 cannot resolve adjacent integer cells. Stop
        // the sample instead of saturating a float-to-integer conversion.
        if !all(abs(g)<vec2<f32>(16777216.0)) {return count+1u;}
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
