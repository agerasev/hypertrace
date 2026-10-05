// Solid angle is measured on the local unit tangent sphere in every curvature.
struct {{self}}_Cone { axis:vec3<f32>, width:f32, count:u32, valid:u32 }
fn {{self}}_cone(base:u32,position:vec4<f32>)->{{self}}_Cone {
    let radial=length(position.yzw);
    let radius=load_f32(base);
    var extent=radius;
    var count=1u;
    if GEO_K>0 {
        let angle=radius/params.misc.y;
        // Large bounds and points inside either the bound or its antipodal
        // ball need the full sphere. No axis is defined at either pole.
        if angle>=PI/2 {return {{self}}_Cone(vec3<f32>(0,0,1),2,1u,1u);}
        extent=sin(angle);
        count=2u;
    } else if GEO_K<0 {extent=sinh(radius/params.misc.y);}
    if !(radial>=0 && radial<=bitcast<f32>(0x7f7fffffu)) {
        return {{self}}_Cone(vec3<f32>(0),0,0u,0u);
    }
    if radial<=extent {return {{self}}_Cone(vec3<f32>(0,0,1),2,1u,1u);}
    let ratio=extent/radial;
    let sine_squared=min(1.0,ratio*ratio);
    // Stable 1-cos(alpha); a conservative minimum width keeps f32 directions
    // resolvable for extremely small or distant lights. The PDF uses this same
    // widened cone, so broadening changes efficiency, not expected radiance.
    let width=max(1e-6,sine_squared/(1+sqrt(max(0.0,1-sine_squared))));
    return {{self}}_Cone(-position.yzw/radial,width,count,1u);
}
fn {{self}}_sample(base:u32,position:vec4<f32>,rng:ptr<function,u32>)->LightSample {
    let cone={{self}}_cone(base,position);
    if cone.valid==0u {return LightSample(vec4<f32>(0),0,0u);}
    var axis=cone.axis;
    // Both orientations of a spherical geodesic can reach the same emitter.
    if cone.count==2u && geo_uniform_open(rng)<0.5 {axis=-axis;}
    let z=1-geo_uniform_open(rng)*cone.width;
    let phi=2*PI*geo_uniform_open(rng);
    let sine=sqrt(max(0.0,(1-z)*(1+z)));
    let direction=rotate_vector(rotation_look(-axis),vec3<f32>(sine*cos(phi),sine*sin(phi),z));
    return LightSample(geo_from_local(position,direction),1/(2*PI*cone.width*f32(cone.count)),1u);
}
fn {{self}}_pdf(base:u32,ray:GeoRay)->LightPdf {
    let cone={{self}}_cone(base,ray.position);
    if cone.valid==0u {return LightPdf(0,0u);}
    let cosine=dot(cone.axis,normalize(geo_to_local(ray.position,ray.tangent)));
    let inside=select(cosine,abs(cosine),cone.count==2u)>=1-cone.width;
    return LightPdf(select(0.0,1/(2*PI*cone.width*f32(cone.count)),inside),1u);
}
