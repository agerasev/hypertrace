#pragma once

#include <geometry/ray_hy.hh>
#include "base.hh"

typedef struct LightHy {
    LightBase base;
    RayHy ray;
} LightHy;

LightLocal light_hy_to_local(LightHy light);
void light_hy_update_local(LightHy *light, LightLocal ll);


#ifndef HOST
#include "hy.cc"
#endif // !HOST
