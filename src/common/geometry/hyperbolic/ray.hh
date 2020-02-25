#pragma once

#include <algebra/quaternion.hh>
#include <algebra/moebius.hh>


typedef struct HyRay {
    quaternion start;
    quaternion direction;
} HyRay;


HyRay hyray_map(Moebius map, HyRay src);


#ifdef OPENCL
#include "ray.cc"
#endif // OPENCL
