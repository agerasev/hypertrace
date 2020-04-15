#pragma once

#include <algebra/complex.hpp>

#include <object.hpp>
#include <path.hpp>
#include <material.hpp>
#include "ray.hpp"


#define HOROSPHERE_TILING_NONE 0
#define HOROSPHERE_TILING_SQUARE 1
#define HOROSPHERE_TILING_HEXAGONAL 2


bool horosphere_hit(
    const Object *horosphere, ObjectHit *cache,
    PathInfo *path, HyRay ray
);

void horosphere_bounce(
    const Object *horosphere, const ObjectHit *cache,
    quaternion *hit_dir, quaternion *normal,
    Material *material
);
