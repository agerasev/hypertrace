// Shared runtime for structurally generated shapes and compositional materials.
struct Params {
    camera0: vec4<f32>, camera1: vec4<f32>,
    background0: vec4<f32>, background1: vec4<f32>, background_axis: vec4<f32>,
    info: vec4<u32>, options: vec4<u32>, misc: vec4<f32>,
}
struct Object {
    map0: vec4<f32>, map1: vec4<f32>,
    info: vec4<u32>, extra: vec4<u32>, props: vec4<f32>,
}
// The generated functions use word offsets into scene_words. A leaf's base
// also identifies that shape when excluding the immediately previous hit.
struct MaterialRecord { data: vec4<u32> }
struct TaggedHit { hit: Hit, identity: u32 }
struct SceneHit { hit: Hit, object_index: u32, identity: u32 }
struct MaterialContext { position: vec3<f32>, normal: vec3<f32> }
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
fn shape_ray_to_local(map0: vec4<f32>, map1: vec4<f32>, hyperbolic: bool, ray: Ray) -> Ray {
    if hyperbolic { return hy_map_ray(hy_inverse(HyMap(map0,map1)),ray); }
    let inverse = qconj(map1);
    return Ray(eu_rotate(inverse,ray.position-map0.xyz),eu_rotate(inverse,ray.direction));
}
fn shape_hit_to_parent(map0: vec4<f32>, map1: vec4<f32>, hyperbolic: bool, hit: Hit) -> Hit {
    if hit.valid == 0u { return hit; }
    if hyperbolic {
        let map = HyMap(map0,map1);
        return Hit(hit.valid,hit.distance,hy_apply_pos(map,hit.position),
            hy_apply_dir(map,hit.position,hit.direction),hy_apply_dir(map,hit.position,hit.normal));
    }
    return Hit(hit.valid,hit.distance,eu_rotate(map1,hit.position)+map0.xyz,
        eu_rotate(map1,hit.direction),eu_rotate(map1,hit.normal));
}

fn scene_hit(ray: Ray, previous: u32, previous_identity: u32) -> SceneHit {
    var result = SceneHit(miss(),0xffffffffu,0xffffffffu);
    for (var i=0u; i<params.info.w; i+=1u) {
        let object = objects[i];
        let identity = select(0xffffffffu,previous_identity,i==previous);
        let candidate = ht_shape_dispatch(object.info.x,object.extra.y,
            object_ray_to_local(object,ray),identity);
        if candidate.hit.valid != 0u {
            if result.hit.valid == 0u || candidate.hit.distance < result.hit.distance {
                result = SceneHit(candidate.hit,i,candidate.identity);
            }
        }
    }
    return result;
}
fn sample_path(pixel: vec2<u32>, state: ptr<function,u32>) -> vec3<f32> {
    var ray = primary_ray(pixel,state);
    var sample = MaterialSample(ray.direction,vec3<f32>(1),vec3<f32>(0),1u);
    var previous = 0xffffffffu;
    var previous_identity = 0xffffffffu;
    for (var bounce=0u; bounce<params.options.y; bounce+=1u) {
        let selected = scene_hit(ray,previous,previous_identity);
        if selected.hit.valid == 0u {
            sample.emission += sample.attenuation*background(ray.direction);
            break;
        }
        let object = objects[selected.object_index];
        let hit = selected.hit;
        let material = materials[tiled_material(object,hit.position)];
        sample.direction = hit.direction;
        ht_material_dispatch(material.data.x,material.data.y,
            MaterialContext(hit.position,hit.normal),&sample,state);
        if sample.alive == 0u { break; }
        ray = object_ray_to_world(object,Ray(hit.position,normalize(sample.direction)));
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
