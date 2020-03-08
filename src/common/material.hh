#pragma once

#include <algebra/real.hh>
#include <random.hh>
#include <path.hh>

typedef struct {
    float3 diffuse_color;
    float gloss;
    float transparency;
    float3 glow;
} Material;

void material_bounce(
    const Material *material,
    Rng *rng, PathInfo *path,
    real3 hit_dir, real3 normal, real3 *bounce_dir,
    float3 *light, float3 *emission
);

void specular_bounce(
    real3 hit_dir, real3 normal, real3 *bounce_dir
);

void lambert_bounce(
    float3 color,
    Rng *rng,
    real3 hit_dir, real3 normal, real3 *bounce_dir,
    float3 *light
);

void material_interpolate(
    Material *o,
    const Material *a, const Material *b,
    real t
);

#ifdef OPENCL_INTEROP

typedef struct _PACKED_STRUCT_ATTRIBUTE_ {
    float3_pk diffuse_color;
    float_pk gloss;
    float_pk transparency;
    float3_pk glow;
} MaterialPk;

void pack_material(MaterialPk *dst, const Material *src);
void unpack_material(Material *dst, const MaterialPk *src);

#define material_pack pack_material
#define material_unpack unpack_material

#endif // OPENCL_INTEROP
