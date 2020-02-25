#pragma once

#include <types.hh>
#include <random.hh>

#include <algebra/moebius.hh>
#include <geometry/hyperbolic.hh>

#include <geometry/hyperbolic/ray.hh>
#include <geometry/hyperbolic/plane.hh>
#include <geometry/hyperbolic/horosphere.hh>


typedef enum {
    OBJECT_NONE = 0,
    OBJECT_PLANE,
    OBJECT_HOROSPHERE
} ObjectType;

typedef struct {
    ObjectType type;
    Moebius map;
    union {
        HyPlane plane;
        Horosphere horosphere;
    };
} Object;

typedef union {
    HyPlaneHit plane;
    HorosphereHit horosphere;
} ObjectHit;

#ifdef OPENCL_INTEROP

typedef struct __attribute__ ((packed)) {
    uint_pk type;
    MoebiusPk map;
    union __attribute__ ((packed)) {
        HyPlanePk plane;
        HorospherePk horosphere;
    };
} ObjectPk;

#endif // OPENCL_INTEROP


real object_hit(
    const Object *object, ObjectHit *cache,
    Rng *rng,
    HyRay ray
);

bool object_bounce(
    const Object *object, const ObjectHit *cache,
    Rng *rng,
    HyRay *ray,
    float3 *light, float3 *emission
);

#ifdef OPENCL_INTEROP
void pack_object(ObjectPk *dst, const Object *src);
void unpack_object(Object *dst, const ObjectPk *src);
ObjectPk pack_copy_object(Object src);
Object unpack_copy_object(ObjectPk src);
#endif // OPENCL_INTEROP


#ifdef OPENCL
#include "object.cc"
#endif // OPENCL
