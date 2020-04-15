#pragma once

#include <types.h>
#include <array.hpp>
#include <random.hpp>
#include <path.hpp>
#include <material.hpp>

#include <algebra/moebius.hpp>
#include <geometry/hyperbolic.hpp>

#include <geometry/hyperbolic/ray.hpp>

using namespace random;
using namespace hyperbolic;

enum ObjectType : uint {
    PLANE = 1,
    HOROSPHERE,
};

typedef uint TilingType;

class Tiling {
public:
    TilingType type;
    real cell_size;
    real border_width;
    Material border_material;
    /*
    static Tiling interpolate(
        const Tiling &a, const Tiling &b,
        real t
    );
    */
};

class ObjectHit {
public:
    quat pos;
    quat dir;
};

class Object {
public:
    ObjectType type;
    Moebius map;
    array<Material, 4> materials;
    Tiling tiling;

    template <typename R>
    real hit(
        ObjectHit &cache,
        R &rng, PathInfo *path,
        Ray ray
    ) const {
        Ray r = ray.map(!map);

        bool h = false;
        if (type == ObjectType::PLANE) {
            h = hyplane_hit(
                object, cache,
                path, r
            );
        } else if (type == ObjectType::HOROSPHERE) {
            h = horosphere_hit(
                object, cache,
                path, r
            );
        }

        if (h) {
            return hy_distance(cache->pos, r.start);
        } else {
            return (real)(-1);
        }
    }

    template <typename R>
    bool bounce(
        const ObjectHit &cache,
        R &rng, PathInfo &path,
        Ray *ray,
        float3 *light, float3 *emission
    ) {

    }

    /*
    static Object interpolate(
        const Object *a, const Object *b,
        real t
    );
    */
};
