#pragma once

#include <algebra/quaternion.hh>


bool horosphere_hit(
    quaternion src_pos, quaternion src_dir,
    quaternion *hit_pos, quaternion *hit_norm
);


#ifdef OPENCL
#include "horosphere.cc"
#endif // OPENCL
