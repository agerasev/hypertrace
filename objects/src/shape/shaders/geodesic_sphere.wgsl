fn {{self}}(base:u32,ray:GeoRay,previous_identity:u32)->GeoTaggedHit {
    return GeoTaggedHit({{dep0}}_intersect(ray,select(0.0,8.0*EPS*params.misc.y,base==previous_identity),geo_infinity(),params.misc.y,load_f32(base)),base);
}
