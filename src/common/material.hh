#pragma once

#include <algebra/real.hh>
#include <random.hh>


typedef struct Material {
    float3 diffuse_color;
    float gloss;
} Material;

bool material_bounce(
    Material *material,
    Rng *rng,
    real3 hit_dir, real3 normal, real3 *bounce_dir,
    float3 *light, float3 *emission
);

void specular_bounce(
    float3 color,
    real3 hit_dir, real3 normal, real3 *bounce_dir,
    float3 *light
);

void lambert_bounce(
    float3 color,
    Rng *rng,
    real3 hit_dir, real3 normal, real3 *bounce_dir,
    float3 *light
);

#ifdef OPENCL_INTEROP
typedef struct __attribute__ ((packed)) MaterialPk {
    float3_pk diffuse_color;
    float_pk gloss;
} MaterialPk;

void pack_material(MaterialPk *dst, const Material *src);
void unpack_material(Material *dst, const MaterialPk *src);
#endif // OPENCL_INTEROP
