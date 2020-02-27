#pragma once

#include <algebra/quaternion.hh>
#include <algebra/moebius.hh>


typedef struct HyRay {
    quaternion start;
    quaternion direction;
} HyRay;


HyRay hyray_init();
HyRay hyray_map(Moebius map, HyRay src);
