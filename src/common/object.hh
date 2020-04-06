#pragma once

#include <types.hh>
#include <random.hh>
#include <path.hh>
#include <material.hh>

#include <algebra/moebius.hh>
#include <geometry/hyperbolic.hh>

#include <geometry/hyperbolic/ray.hh>


typedef uchar    ObjectType;

#define OBJECT_NONE       0
#define OBJECT_HYPLANE    1
#define OBJECT_HOROSPHERE 2

typedef uchar    TilingType;

typedef struct {
    TilingType type;
    real cell_size;
    real border_width;
    Material border_material;
} Tiling;

#define MATERIAL_COUNT_MAX 4

typedef struct {
    ObjectType type;
    Moebius map;
    Material materials[MATERIAL_COUNT_MAX];
    int material_count;
    Tiling tiling;
} Object;

typedef struct {
    quaternion pos;
    quaternion dir;
} ObjectHit;


#ifdef OPENCL_INTEROP

typedef uchar_pk ObjectTypePk;
typedef uchar_pk TilingTypePk;

typedef struct _PACKED_STRUCT_ATTRIBUTE_ {
    TilingTypePk type _PACKED_FIELD_ATTRIBUTE_;
    real_pk cell_size _PACKED_FIELD_ATTRIBUTE_;
    real_pk border_width _PACKED_FIELD_ATTRIBUTE_;
    MaterialPk border_material _PACKED_FIELD_ATTRIBUTE_;
} TilingPk;

// FIXME: Use explicit alignment instead of `packed` attribute
// because it suppresses referencing of field of such structure.
typedef struct _PACKED_STRUCT_ATTRIBUTE_ {
    uint_pk type _PACKED_FIELD_ATTRIBUTE_;
    MoebiusPk map _PACKED_FIELD_ATTRIBUTE_;
    MaterialPk materials[MATERIAL_COUNT_MAX] _PACKED_FIELD_ATTRIBUTE_;
    int_pk material_count _PACKED_FIELD_ATTRIBUTE_;
    TilingPk tiling _PACKED_FIELD_ATTRIBUTE_;
} ObjectPk;

#endif // OPENCL_INTEROP


real object_hit(
    const Object *object, ObjectHit *cache,
    Rng *rng, PathInfo *path,
    HyRay ray
);

bool object_bounce(
    const Object *object, const ObjectHit *cache,
    Rng *rng, PathInfo *path,
    HyRay *ray,
    float3 *light, float3 *emission
);

void object_interpolate(
    Object *o,
    const Object *a, const Object *b,
    real t
);
void tiling_interpolate(
    Tiling *o,
    const Tiling *a, const Tiling *b,
    real t
);

#ifdef OPENCL_INTEROP
void pack_object(ObjectPk *dst, const Object *src);
void unpack_object(Object *dst, const ObjectPk *src);
void pack_tiling(TilingPk *dst, const Tiling *src);
void unpack_tiling(Tiling *dst, const TilingPk *src);
#define object_pack pack_object
#define object_unpack unpack_object
#define tiling_pack pack_tiling
#define tiling_unpack unpack_tiling
#endif // OPENCL_INTEROP
