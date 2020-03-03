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

// FIXME:
// Bug in union copying on Nvidia GPUs.
// So don't copy `Object` using `=` or passing by value or returning it.
// A special function that checks `type` and copies appropriate union member
// should be implemented instead.
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
// FIXME: Use explicit alignment instead of `packed` attribute
// because it suppresses referencing of field of such structure.
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
#endif // OPENCL_INTEROP
