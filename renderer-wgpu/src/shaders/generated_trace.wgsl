// Shared runtime for structurally generated shapes and compositional materials.
struct Params {
    camera0: vec4<f32>, camera1: vec4<f32>,
    background0: vec4<f32>, background1: vec4<f32>, background_axis: vec4<f32>,
    info: vec4<u32>, options: vec4<u32>, misc: vec4<f32>, medium: vec4<f32>,
}
struct Object {
    map0: vec4<f32>, map1: vec4<f32>,
    info: vec4<u32>, extra: vec4<u32>, props: vec4<f32>,
}
struct MaterialRecord { data: vec4<u32> }
// Explicit v1 chart contracts, retained for existing custom shader leaves.
struct TaggedHit { hit: Hit, identity: u32 }
struct MaterialContext { position: vec3<f32>, normal: vec3<f32> }
// Embedded v2 contracts. Normals/directions passed to materials use the local
// orthonormal frame at the object's hit position.
struct GeoTaggedHit { hit: GeoHit, identity: u32 }
struct GeoSceneHit { hit: GeoHit, object_index: u32, identity: u32 }
struct GeoMaterialContext { position: vec4<f32>, normal: vec3<f32> }
struct MaterialSample {
    direction: vec3<f32>, attenuation: vec3<f32>, emission: vec3<f32>, alive: u32,
}

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage,read> objects: array<Object>;
@group(0) @binding(2) var<storage,read> materials: array<MaterialRecord>;
@group(0) @binding(3) var<storage,read_write> accumulation: array<vec4<f32>>;
@group(0) @binding(4) var<storage,read_write> seeds: array<u32>;
@group(0) @binding(5) var<storage,read> scene_words: array<u32>;

fn load_u32(base: u32) -> u32 { return scene_words[base]; }
fn load_f32(base: u32) -> f32 { return bitcast<f32>(scene_words[base]); }
fn load_vec3(base: u32) -> vec3<f32> {
    return vec3<f32>(load_f32(base),load_f32(base+1u),load_f32(base+2u));
}
fn load_vec4(base: u32) -> vec4<f32> {
    return vec4<f32>(load_vec3(base),load_f32(base+3u));
}
fn geo_legacy_ray(ray: GeoRay) -> Ray {
    return Ray(geo_to_chart_pos(ray.position),geo_to_chart_dir(ray.position,ray.tangent));
}
fn geo_from_legacy_hit(hit: Hit) -> GeoHit {
    if hit.valid == 0u { return geo_miss(); }
    let p = geo_from_chart_pos(hit.position);
    return GeoHit(hit.valid,hit.distance*select(1.0,params.misc.y,GEO_K != 0),p,
        geo_from_chart_dir(p,hit.direction),geo_from_chart_dir(p,hit.normal));
}
fn geo_map_hit(map: GeoMap, hit: GeoHit) -> GeoHit {
    if hit.valid == 0u { return hit; }
    let state = geo_map_ray(map,GeoRay(hit.position,hit.tangent));
    if hit.valid == 2u || !geo_ray_supported(state) { return geo_failure(); }
    return GeoHit(hit.valid,hit.distance,state.position,state.tangent,geo_map_apply(map,hit.normal));
}
fn geo_scene_hit(ray: GeoRay, previous: u32, previous_identity: u32) -> GeoSceneHit {
    var result = GeoSceneHit(geo_miss(),0xffffffffu,0xffffffffu);
    for (var i=0u; i<params.info.w; i+=1u) {
        let object = objects[i];
        let identity = select(0xffffffffu,previous_identity,i==previous);
        let local = geo_map_ray(geo_inverse(GeoMap(object.map0,object.map1)),ray);
        if !geo_ray_supported(local) { return GeoSceneHit(geo_failure(),i,identity); }
        let candidate = ht_shape_dispatch(object.info.x,object.extra.y,local,identity);
        if candidate.hit.valid == 2u { return GeoSceneHit(candidate.hit,i,candidate.identity); }
        if candidate.hit.valid != 0u {
            if result.hit.valid == 0u || candidate.hit.distance < result.hit.distance {
                result = GeoSceneHit(candidate.hit,i,candidate.identity);
            }
        }
    }
    return result;
}
fn geo_primary_ray(pixel: vec2<u32>, state: ptr<function,u32>) -> GeoRay {
    let jitter_x = uniform_random(state);
    let jitter_y = uniform_random(state);
    let screen = vec2<f32>(2*f32(pixel.x)+1-f32(params.info.x),
        f32(params.info.y)-(2*f32(pixel.y)+1))/f32(params.info.y);
    let jitter = 2/f32(params.info.y)*(vec2<f32>(jitter_x,jitter_y)-0.5);
    let direction = normalize(vec3<f32>(screen+jitter,-1/params.misc.x));
    return GeoRay(vec4<f32>(1,0,0,0),vec4<f32>(0,direction));
}
fn geo_uniform_open(state: ptr<function,u32>) -> f32 {
    *state = 1103515245u*(*state)+12345u;
    // 23 bits plus a half-bin offset: both endpoints stay excluded in f32.
    return (f32(*state >> 9u)+0.5)*(1.0/8388608.0);
}
fn geo_background(ray: GeoRay) -> vec3<f32> {
    // Euclidean directional environments retain their world-space orientation.
    // Curved environments are evaluated in the camera-relative tangent frame.
    var direction = normalize(geo_to_local(ray.position,ray.tangent));
    if GEO_K == 0 { direction = eu_rotate(params.camera0,direction); }
    return background(direction);
}
fn sample_path(pixel: vec2<u32>, state: ptr<function,u32>) -> vec3<f32> {
    var path = GeoPath(geo_primary_ray(pixel,state),0,0);
    var sample = MaterialSample(path.ray.tangent.yzw,vec3<f32>(1),vec3<f32>(0),1u);
    var previous = 0xffffffffu;
    var previous_identity = 0xffffffffu;
    for (var bounce=0u; bounce<params.options.y; bounce+=1u) {
        // Numerical failure is an explicit termination, never a physical miss.
        // Retain prior emission without adding any background contribution.
        if !geo_ray_supported(path.ray) { break; }
        let selected = geo_scene_hit(path.ray,previous,previous_identity);
        if selected.hit.valid == 2u { break; }
        if params.medium.w > 0 {
            let distance = geo_free_flight(params.medium.w,geo_uniform_open(state));
            if geo_medium_precedes(distance,selected.hit) {
                if !geo_advance_supported(path.ray,distance,params.misc.y) { break; }
                path = geo_travel(path,distance,params.misc.y);
                // Analog free-flight already accounts for survival. Weight only
                // by scattering albedo, never a second exponential attenuation.
                sample.attenuation *= params.medium.xyz;
                if all(sample.attenuation == vec3<f32>(0)) { break; }
                let z = 2*geo_uniform_open(state)-1;
                let phi = 2*PI*geo_uniform_open(state);
                let xy = sqrt(max(0.0,1-z*z));
                path.ray.tangent = geo_from_local(path.ray.position,
                    vec3<f32>(xy*cos(phi),xy*sin(phi),z));
                previous = 0xffffffffu;
                previous_identity = 0xffffffffu;
                continue;
            }
        }
        if selected.hit.valid == 0u {
            sample.emission += sample.attenuation*geo_background(path.ray);
            break;
        }
        if !geo_advance_supported(path.ray,selected.hit.distance,params.misc.y) { break; }
        path = geo_travel(path,selected.hit.distance,params.misc.y);
        let object = objects[selected.object_index];
        let hit = selected.hit;
        let material = materials[tiled_material(object,geo_to_chart_pos(hit.position))];
        sample.direction = normalize(geo_to_local(hit.position,hit.tangent));
        let normal = normalize(geo_to_local(hit.position,hit.normal));
        ht_material_dispatch(material.data.x,material.data.y,
            GeoMaterialContext(hit.position,normal),&sample,state);
        if sample.alive == 0u { break; }
        let local = GeoRay(hit.position,geo_from_local(hit.position,normalize(sample.direction)));
        path.ray = geo_map_ray(GeoMap(object.map0,object.map1),local);
        previous = selected.object_index;
        previous_identity = selected.identity;
    }
    return sample.emission;
}

@compute @workgroup_size(8,8,1)
fn render(@builtin(global_invocation_id) global: vec3<u32>) {
    if global.x>=params.info.x || global.y>=params.info.y { return; }
    let index = global.x+params.info.x*global.y;
    var state = seeds[index];
    var sum = vec3<f32>(0);
    for (var sample_index=0u; sample_index<params.options.x; sample_index+=1u) {
        sum += sample_path(global.xy,&state);
    }
    accumulation[index] += vec4<f32>(sum,f32(params.options.x));
    seeds[index] = state;
}
