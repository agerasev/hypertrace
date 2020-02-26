#pragma once

#include <algebra/quaternion.hh>

#include <material.hh>
#include "ray.hh"


typedef enum {
    HOROSPHERE_TILING_NONE = 0,
    HOROSPHERE_TILING_SQUARE,
    HOROSPHERE_TILING_HEXAGONAL,
} HorosphereTiling;

typedef struct {
    HorosphereTiling tiling;
    Material material;
    real size, border;
} Horosphere;

typedef struct {
    quaternion hit_pos;
    quaternion hit_dir;
} HorosphereHit;

#ifdef OPENCL_INTEROP
typedef struct __attribute__((packed)) {
    uint_pk tiling;
    MaterialPk material;
    real_pk size, border;
} HorospherePk;
#endif // OPENCL_INTEROP


bool horosphere_hit(
    const Horosphere *plane, HorosphereHit *cache,
    HyRay ray, quaternion *hit_pos
);

bool horosphere_bounce(
    const Horosphere *plane, const HorosphereHit *cache,
    Rng *rng,
    HyRay *ray,
    float3 *light, float3 *emission
);

#ifdef OPENCL_INTEROP
void pack_horosphere(HorospherePk *dst, const Horosphere *src);
void unpack_horosphere(Horosphere *dst, const HorospherePk *src);
#endif // OPENCL_INTEROP


#ifdef OPENCL
#include "horosphere.cc"
#endif // OPENCL
