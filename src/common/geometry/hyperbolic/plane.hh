#pragma once

#include <algebra/quaternion.hh>

typedef struct {
    quaternion pos;
} PlaneHit;

bool plane_intersect(
    quaternion src_pos, quaternion src_dir,
    quaternion *hit_pos, quaternion *hit_norm
);


#ifdef OPENCL
#include "plane.cc"
#endif // OPENCL
