#include "material.hh"

#include <algebra/rotation.hh>


bool material_bounce(
    Material *material,
    Rng *rng,
    real3 hit_dir, real3 normal, real3 *bounce_dir,
    float3 *light, float3 *emission
) {
    if (rand_uniform(rng) < material->gloss) {
        specular_bounce(
            make_float3(0.0f),
            hit_dir, normal, bounce_dir,
            light
        );
    } else {
        lambert_bounce(
            material->diffuse_color,
            rng,
            hit_dir, normal, bounce_dir,
            light
        );
    }
    return true;
}

void specular_bounce(
    float3 color,
    real3 hit_dir, real3 normal, real3 *bounce_dir,
    float3 *light
) {
    *bounce_dir = hit_dir - (2*dot(hit_dir, normal))*normal;
}

void lambert_bounce(
    float3 color,
    Rng *rng,
    real3 hit_dir, real3 normal, real3 *bounce_dir,
    float3 *light
) {
    if (dot(normal, hit_dir) > 0) {
        normal = -normal;
    }
    real3 rand = rand_hemisphere_cosine(rng);
    rotation3 rot = rot3_look_at(normal);
    *bounce_dir = rot3_apply(rot, rand);
    
    *light *= color;
}

#ifdef OPENCL_INTEROP
void pack_material(MaterialPk *dst, const Material *src) {
    dst->diffuse_color = pack_float3(src->diffuse_color);
    dst->gloss = src->gloss;
}
void unpack_material(Material *dst, const MaterialPk *src) {
    dst->diffuse_color = unpack_float3(src->diffuse_color);
    dst->gloss = src->gloss;
}
#endif // OPENCL_INTEROP