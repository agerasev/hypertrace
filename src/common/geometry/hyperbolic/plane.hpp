#pragma once

#include <algebra/complex.hpp>

#include <object.hpp>
#include <material.hpp>
#include "ray.hpp"


#define HYPLANE_TILING_NONE       0
#define HYPLANE_TILING_PENTAGONAL 1
#define HYPLANE_TILING_PENTASTAR  2


bool hyplane_hit(
    const Object *plane, ObjectHit *cache,
    PathInfo *path, HyRay ray
);

void hyplane_bounce(
    const Object *plane, const ObjectHit *cache,
    quaternion *hit_dir, quaternion *normal,
    Material *material
);
