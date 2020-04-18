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


/*
typedef uint TilingType;

class Tiling {
public:
    TilingType type;
    real cell_size;
    real border_width;
    Material border_material;
    static Tiling interpolate(
        const Tiling &a, const Tiling &b,
        real t
    );
};
*/

class ObjectHit {
public:
    quat pos;
    quat dir;
};

class Object {
public:
    //class Cache {};
    template <typename R>
    real hit(
        Cache &cache,
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
    ) const {
        Material material;
        quaternion hit_dir, normal;
        
        if (object->type == OBJECT_HYPLANE) {
            hyplane_bounce(
                object, cache,
                &hit_dir, &normal,
                &material
            );
        } else if (object->type == OBJECT_HOROSPHERE) {
            horosphere_bounce(
                object, cache,
                &hit_dir, &normal,
                &material
            );
        }

        real3 bounce_dir;
        material_bounce(
            &material,
            rng, path,
            hit_dir.xyz, normal.xyz, &bounce_dir,
            light, emission
        );

        ray->start = cache->pos;
        ray->direction = q_new(bounce_dir, (real)0);

        *ray = hyray_map(object->map, *ray);

        return true;
    }
};
