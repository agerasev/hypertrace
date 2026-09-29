// Geometry-independent transport distances. A periodic coordinate state never
// replaces the physical distance used for event ordering or optical depth.
struct GeoPath {
    ray: GeoRay,
    // High distance in exact multiples of 1024 world units, with a separate
    // nonnegative remainder. Unit block increments remain exact below 2^34
    // total world units. The ray's own coordinate precision has tighter limits.
    travelled: f32,
    remainder: f32,
}
fn geo_infinity() -> f32 { return bitcast<f32>(0x7f800000u); }
fn geo_path_distance(path: GeoPath) -> f32 {
    // A convenient rounded total; the two stored parts retain more information.
    return path.travelled+path.remainder;
}
fn geo_travel(path: GeoPath, distance: f32, radius: f32) -> GeoPath {
    // Explicit blocks survive shader relaxed arithmetic, unlike a Kahan error
    // term whose subtract/add compensation may be reassociated to zero.
    let blocks = floor(distance/1024)*1024;
    let remainder = path.remainder+(distance-blocks);
    let carry = floor(remainder/1024)*1024;
    return GeoPath(geo_advance(path.ray,distance,radius),
        path.travelled+blocks+carry,remainder-carry);
}
fn geo_free_flight(extinction: f32, uniform: f32) -> f32 {
    if extinction == 0 { return geo_infinity(); }
    return -log(1-uniform)/extinction;
}
fn geo_medium_precedes(distance: f32, surface: GeoHit) -> bool {
    return distance < select(geo_infinity(),surface.distance,surface.valid != 0u);
}
// Analytic segment survival for diagnostics and deterministic transport checks.
// The analog integrator already samples survival through geo_free_flight; it
// must not multiply this weight again after sampling that same interval.
fn geo_transmittance(extinction: f32, distance: f32) -> f32 {
    if extinction == 0 { return 1; }
    return exp(-extinction*distance);
}
