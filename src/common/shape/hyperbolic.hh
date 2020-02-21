#pragma once

#include <algebra/quaternion.hh>


bool plane_hit(
    quaternion src_pos, quaternion src_dir,
    quaternion *hit_pos, quaternion *hit_norm
);
bool horosphere_hit(
    quaternion src_pos, quaternion src_dir,
    quaternion *hit_pos, quaternion *hit_norm
);


#ifdef OPENCL
#include "hyperbolic.cc"
#endif // OPENCL
