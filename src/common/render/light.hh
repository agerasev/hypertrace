#pragma once

#include <common/algebra/color.hh>
#include <common/geometry/ray.hh>


typedef struct LightBase {
    color3 intensity;
    bool diffuse;
} LightBase;

typedef struct LightLocal {
    real3 direction;
    LightBase base;
} LightLocal;


typedef struct LightEu {
    RayEu ray;
    LightBase base;
} LightEu;

LightLocal light_eu_to_local(LightEu light);

void light_eu_update_local(LightEu *light, LightLocal ll);


typedef struct LightHy {
    RayHy ray;
    LightBase base;
} LightHy;

LightLocal light_hy_to_local(LightHy light);

void light_hy_update_local(LightHy *light, LightLocal ll);


#ifndef HOST
#include "light.cc"
#endif // !HOST
