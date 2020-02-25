#pragma once

#include <algebra/quaternion.hh>

#include <material.hh>
#include "ray.hh"


typedef enum {
    HYPLANE_TILING_NONE = 0,
    HYPLANE_TILING_PENTAGONAL,
} HyPlaneTiling;

typedef struct {
    HyPlaneTiling tiling;
    Material material;
} HyPlane;

typedef struct {
    quaternion hit_pos;
    quaternion hit_dir;
} HyPlaneHit;

#ifdef OPENCL_INTEROP
typedef struct __attribute__((packed)) {
    uint tiling;
    MaterialPk material;
} HyPlanePk;
#endif // OPENCL_INTEROP


bool hyplane_hit(
    const HyPlane *plane, HyPlaneHit *cache,
    HyRay ray, quaternion *hit_pos
);

bool hyplane_bounce(
    const HyPlane *plane, const HyPlaneHit *cache,
    Rng *rng,
    HyRay *ray,
    float3 *light, float3 *emission
);

#ifdef OPENCL_INTEROP
void pack_hyplane(HyPlanePk *dst, const HyPlane *src);
void unpack_hyplane(HyPlane *dst, const HyPlanePk *src);
#endif // OPENCL_INTEROP


#ifdef OPENCL
#include "plane.cc"
#endif // OPENCL
