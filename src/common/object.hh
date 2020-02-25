#pragma once

#include <types.hh>
#include <ray.hh>
#include <random.hh>

#include <algebra/moebius.hh>
#include <geometry/hyperbolic.hh>

#include <geometry/hyperbolic/plane.hh>
#include <geometry/hyperbolic/horosphere.hh>
#include <material.hh>


typedef struct Object Object;
typedef struct HitInfo HitInfo;

#ifdef OPENCL_INTEROP
typedef struct ObjectPk ObjectPk;
#endif // OPENCL_INTEROP


real object_intersect(
    Rng *rng,
    const Object *obj, HitInfo *info,
    const Ray *ray
);
bool object_interact(
    Rng *rng,
    const Object *obj, const HitInfo *info,
    const Ray *ray, Ray *new_ray
);

#ifdef OPENCL_INTEROP
void pack_object(ObjectPk *dst, const Object *src);
void unpack_object(Object *dst, const ObjectPk *src);
ObjectPk pack_copy_object(Object src);
Object unpack_copy_object(ObjectPk src);
#endif // OPENCL_INTEROP


typedef enum ObjectType {
    OBJECT_NONE = 0,
    OBJECT_PLANE,
    OBJECT_HOROSPHERE
} ObjectType;


struct Object {
    ObjectType type;
    Moebius map;
    float3 color;
    float gloss;
};

struct HitInfo {
    quaternion pos;
    quaternion norm;
    quaternion local_pos;
};

#ifdef OPENCL_INTEROP

struct __attribute__ ((packed)) ObjectPk {
    uint_pk type;
    MoebiusPk map;
    float3_pk color;
    float_pk gloss;
};

#endif // OPENCL_INTEROP


#ifdef OPENCL
#include "object.cc"
#endif // OPENCL
