// Shared runtime for structurally generated shapes and compositional materials.
// Shared geometry contracts. Normals/directions passed to materials use the local
// orthonormal frame at the object's hit position.
struct GeoTaggedHit { hit: GeoHit, identity: u32 }
struct GeoSceneHit { hit: GeoHit, object_index: u32, identity: u32 }
struct GeoMaterialContext { position: vec4<f32>, normal: vec3<f32> }
struct MaterialSample {
    direction: vec3<f32>, attenuation: vec3<f32>, emission: vec3<f32>, alive: u32,
    // Delta events have no density with respect to solid angle.
    delta: u32,
}
// value is the BSDF times the absolute receiving cosine; pdf is per steradian.
struct MaterialEvaluation { value: vec3<f32>, pdf: f32, valid: u32 }
struct MaterialEmission { value: vec3<f32>, valid: u32 }
struct LightSample { tangent: vec4<f32>, pdf: f32, valid: u32 }
struct LightPdf { value: f32, valid: u32 }

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
        let candidate = ht_shape_dispatch(object.info.x,object.info.z,local,identity);
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
    if GEO_K == 0 { direction = rotate_vector(params.camera0,direction); }
    return background(direction);
}
fn sample_path(pixel: vec2<u32>, state: ptr<function,u32>) -> vec3<f32> {
    var path = GeoPath(geo_primary_ray(pixel,state),0,0);
    var throughput=vec3<f32>(1);
    var radiance=vec3<f32>(0);
    var mis_ray=path.ray;
    var mis_pdf=0.0;
    var mis_delta=true;
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
                throughput *= params.medium.xyz;
                if all(throughput == vec3<f32>(0)) { break; }
                // A connection ends at another surface event. Do not extend the
                // finite interaction budget beyond that of ordinary sampling.
                if bounce+1u<params.options.y && params.options.z>0u {
                    let direct=geo_direct_volume(path.ray.position,state);
                    if direct.valid==0u {break;}
                    radiance+=throughput*direct.value;
                }
                let z = 2*geo_uniform_open(state)-1;
                let phi = 2*PI*geo_uniform_open(state);
                let xy = sqrt(max(0.0,1-z*z));
                path.ray.tangent = geo_from_local(path.ray.position,
                    vec3<f32>(xy*cos(phi),xy*sin(phi),z));
                mis_ray=path.ray;
                mis_pdf=1/(4*PI);
                mis_delta=false;
                previous = 0xffffffffu;
                previous_identity = 0xffffffffu;
                continue;
            }
        }
        if selected.hit.valid == 0u {
            radiance += throughput*geo_background(path.ray);
            break;
        }
        if !geo_advance_supported(path.ray,selected.hit.distance,params.misc.y) { break; }
        path = geo_travel(path,selected.hit.distance,params.misc.y);
        let object = objects[selected.object_index];
        let hit = selected.hit;
        let material = materials[object.info.y];
        let incoming=normalize(geo_to_local(hit.position,hit.tangent));
        let normal=normalize(geo_to_local(hit.position,hit.normal));
        let context=GeoMaterialContext(hit.position,normal);
        var emission_weight=1.0;
        if !mis_delta && params.options.z>0u {
            let light_pdf=geo_light_pdf(selected.object_index,mis_ray);
            if light_pdf.valid==0u {break;}
            emission_weight=geo_power_weight(mis_pdf,light_pdf.value);
        }
        // The event carries local emission and throughput changes. Weight only
        // its emission, never the scattering continuation from the same object.
        var sample=MaterialSample(incoming,vec3<f32>(1),vec3<f32>(0),1u,1u);
        ht_material_dispatch(material.data.x,material.data.y,context,&sample,state);
        radiance+=throughput*sample.emission*emission_weight;
        let map=GeoMap(object.map0,object.map1);
        if bounce+1u<params.options.y && params.options.z>0u {
            let direct=geo_direct_surface(geo_map_apply(map,hit.position),selected.object_index,
                selected.identity,context,incoming,state);
            if direct.valid==0u {break;}
            radiance+=throughput*direct.value;
        }
        if sample.alive == 0u { break; }
        throughput*=sample.attenuation;
        let outgoing=normalize(sample.direction);
        mis_delta=sample.delta!=0u;
        mis_pdf=0;
        if !mis_delta && params.options.z>0u {
            let evaluation=ht_material_evaluate(material.data.x,material.data.y,context,incoming,outgoing);
            if evaluation.valid==0u || !(evaluation.pdf>0) || !geo_density_supported(evaluation.pdf) {break;}
            mis_pdf=evaluation.pdf;
        }
        let local=GeoRay(hit.position,geo_from_local(hit.position,outgoing));
        path.ray=geo_map_ray(map,local);
        mis_ray=path.ray;
        previous = selected.object_index;
        previous_identity = selected.identity;
    }
    return radiance;
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
