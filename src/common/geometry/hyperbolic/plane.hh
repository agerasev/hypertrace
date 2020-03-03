#pragma once

#include <algebra/quaternion.hh>

#include <material.hh>
#include "ray.hh"


#define HYPLANE_MATERIAL_COUNT_MAX 2

typedef enum {
    HYPLANE_TILING_NONE = 0,
    HYPLANE_TILING_PENTAGONAL,
    HYPLANE_TILING_PENTASTAR,
} HyPlaneTilingType;

typedef struct {
    real width;
    Material material;
} HyPlaneTilingBorder;

typedef struct {
    HyPlaneTilingType type;
    HyPlaneTilingBorder border;
} HyPlaneTiling;


typedef struct {
    Material materials[HYPLANE_MATERIAL_COUNT_MAX];
    int material_count;
    HyPlaneTiling tiling;
} HyPlane;

typedef struct {
    quaternion hit_pos;
    quaternion hit_dir;
} HyPlaneHit;

#ifdef OPENCL_INTEROP
typedef struct __attribute__((packed)) {
    MaterialPk materials[HYPLANE_MATERIAL_COUNT_MAX];
    int_pk material_count;
    uint_pk tiling_type;
    real_pk tiling_border_width;
    MaterialPk tiling_border_material;
} HyPlanePk;
#endif // OPENCL_INTEROP


bool hyplane_hit(
    const HyPlane *plane, HyPlaneHit *cache,
    PathInfo *path, HyRay ray, quaternion *hit_pos
);

void hyplane_bounce(
    const HyPlane *plane, const HyPlaneHit *cache,
    quaternion *hit_dir, quaternion *normal,
    Material *material
);

#ifdef OPENCL_INTEROP
void pack_hyplane(HyPlanePk *dst, const HyPlane *src);
void unpack_hyplane(HyPlane *dst, const HyPlanePk *src);
#endif // OPENCL_INTEROP
