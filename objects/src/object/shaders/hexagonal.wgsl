fn {{self}}_remainder(value:i32,modulus:i32)->i32 {return ((value%modulus)+modulus)%modulus;}
fn {{self}}(embedded:vec4<f32>,cell:f32,width:f32,count:u32)->u32 {
    let position = geo_to_half_space_pos(embedded);
    var index=0i;
    var border=false;
        let bx = vec2<f32>(2/sqrt(3.0),0);
        let by = vec2<f32>(-1/sqrt(3.0),1);
        let size = cell*sqrt(3.0)/2;
        let border_ratio = width/cell;
        var h = vec2<f32>(dot(bx,position.xy),dot(by,position.xy))/size;
        let hx = i32(floor((floor(h.x)-floor(h.y))/3));
        let hy = i32(floor((floor(h.x+h.y)-f32(hx))/2));
        h -= f32(hx)*vec2<f32>(2,-1)+f32(hy)*vec2<f32>(1,1);
        border = abs(h.x-1)>1-border_ratio || abs(h.y)>1-border_ratio || abs(h.x+h.y-1)>1-border_ratio;
        index = 2*hx+hy;
    if border {return count;}
    return u32({{self}}_remainder(index,i32(count)));
}
