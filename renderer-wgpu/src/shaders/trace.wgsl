// Initial, bounded scene lowering for the built-in eu and hy examples.
// Shapes/materials are explicit records, independent of the OpenCL Entity ABI.
// The math module and hit interface can be reused by generated scene dispatch.
struct Params {
    camera0: vec4<f32>, camera1: vec4<f32>,
    background0: vec4<f32>, background1: vec4<f32>, background_axis: vec4<f32>,
    info: vec4<u32>, options: vec4<u32>, misc: vec4<f32>,
}
struct Object {
    map0: vec4<f32>, map1: vec4<f32>,
    info: vec4<u32>, extra: vec4<u32>, props: vec4<f32>,
}
struct Material {
    diffuse: vec4<f32>, emission: vec4<f32>,
    transmission: vec4<f32>, properties: vec4<f32>,
}
struct SceneHit { hit: Hit, object_index: u32 }

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage,read> objects: array<Object>;
@group(0) @binding(2) var<storage,read> materials: array<Material>;
@group(0) @binding(3) var<storage,read_write> accumulation: array<vec4<f32>>;
@group(0) @binding(4) var<storage,read_write> seeds: array<u32>;

fn object_hit(object: Object, ray: Ray, repeated: bool) -> Hit {
    if params.info.z == 1u {
        if object.info.x == 0u { return hy_plane(ray,repeated); }
        if object.info.x == 3u { return hy_horosphere(ray,repeated); }
    } else {
        switch object.info.x {
            case 0u: { return eu_plane(ray,repeated); }
            case 1u: { return eu_sphere(ray,repeated); }
            case 2u: { return eu_cube(ray,repeated); }
            default: {}
        }
    }
    return miss();
}
fn scene_hit(ray: Ray, previous: u32) -> SceneHit {
    var result = SceneHit(miss(),0xffffffffu);
    for (var i=0u; i<params.info.w; i+=1u) {
        let candidate = object_hit(objects[i],object_ray_to_local(objects[i],ray),i==previous);
        if candidate.valid != 0u {
            if result.hit.valid == 0u || candidate.distance < result.hit.distance {
                result = SceneHit(candidate,i);
            }
        }
    }
    return result;
}
fn sample_path(pixel: vec2<u32>, state: ptr<function,u32>) -> vec3<f32> {
    var ray = primary_ray(pixel,state);
    var throughput = vec3<f32>(1);
    var emission = vec3<f32>(0);
    var previous = 0xffffffffu;
    for (var bounce=0u; bounce<params.options.y; bounce+=1u) {
        let selected = scene_hit(ray,previous);
        if selected.hit.valid == 0u {
            emission += throughput*background(ray.direction);
            break;
        }
        let object = objects[selected.object_index];
        let hit = selected.hit;
        let material = materials[tiled_material(object,hit.position)];
        emission += throughput*material.emission.xyz;
        var direction = hit.direction;
        var normal = hit.normal;
        var choice = uniform_random(state);
        choice -= material.diffuse.w;
        if choice < 0 {
            throughput *= material.diffuse.xyz;
            if dot(direction,normal)>0 { normal=-normal; }
            let phi = 2*PI*uniform_random(state);
            let square_cosine = uniform_random(state);
            let sine = sqrt(max(0,1-square_cosine));
            let local = vec3<f32>(cos(phi)*sine,sin(phi)*sine,sqrt(square_cosine));
            direction = eu_rotate(rotation_look(-normal),local);
        } else {
            choice -= material.emission.w;
            if choice < 0 {
                direction -= 2*dot(direction,normal)*normal;
            } else {
                choice -= material.transmission.w;
                if choice < 0 {
                    // Transparent is uncolored in the original hy mixture.
                } else {
                    choice -= material.properties.x;
                    if choice >= 0 { break; }
                    throughput *= material.transmission.xyz;
                    var ratio = material.properties.y;
                    let a = dot(direction,normal);
                    if a < -EPS { ratio=1/ratio; }
                    let x = (a*a-1)*ratio*ratio+1;
                    if x > EPS {
                        direction = direction*ratio+(sign(a)*sqrt(x)-a*ratio)*normal;
                    } else {
                        direction -= 2*a*normal;
                    }
                }
            }
        }
        ray = object_ray_to_world(object,Ray(hit.position,normalize(direction)));
        previous = selected.object_index;
    }
    return emission;
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
