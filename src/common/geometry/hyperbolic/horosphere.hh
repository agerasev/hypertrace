#pragma once

#include <algebra/quaternion.hh>

#include <path.hh>
#include "ray.hh"
#include <material.hh>


#define HOROSPHERE_MATERIAL_COUNT_MAX 4

typedef enum {
    HOROSPHERE_TILING_NONE = 0,
    HOROSPHERE_TILING_SQUARE,
    HOROSPHERE_TILING_HEXAGONAL,
} HorosphereTilingType;


typedef struct {
    real width;
    Material material;
} HorosphereTilingBorder;

typedef struct {
    HorosphereTilingType type;
    real cell_size;
    HorosphereTilingBorder border;
} HorosphereTiling;

typedef struct {
    Material materials[HOROSPHERE_MATERIAL_COUNT_MAX];
    int material_count;
    HorosphereTiling tiling;
} Horosphere;

typedef struct {
    quaternion hit_pos;
    quaternion hit_dir;
} HorosphereHit;

#ifdef OPENCL_INTEROP
typedef struct __attribute__((packed)) {
    MaterialPk materials[HOROSPHERE_MATERIAL_COUNT_MAX];
    int_pk material_count;
    uint_pk tiling_type;
    real_pk tiling_cell_size;
    real_pk tiling_border_width;
    MaterialPk tiling_border_material;
} HorospherePk;
#endif // OPENCL_INTEROP


bool horosphere_hit(
    const Horosphere *plane, HorosphereHit *cache,
    PathInfo *path, HyRay ray, quaternion *hit_pos
);

void horosphere_bounce(
    const Horosphere *plane, const HorosphereHit *cache,
    quaternion *hit_dir, quaternion *normal,
    Material *material
);

#ifdef OPENCL_INTEROP
void pack_horosphere(HorospherePk *dst, const Horosphere *src);
void unpack_horosphere(Horosphere *dst, const HorospherePk *src);
#endif // OPENCL_INTEROP
