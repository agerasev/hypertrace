#pragma once

#include <algebra/color.hh>
#include <geometry/ray.hh>


typedef struct LightBase {
    color3 intensity;
    bool diffuse;
} LightBase;

typedef struct LightLocal {
    LightBase base;
    real3 direction;
} LightLocal;


typedef struct LightEu {
    LightBase base;
    RayEu ray;
} LightEu;

LightLocal light_eu_to_local(LightEu light);

void light_eu_update_local(LightEu *light, LightLocal ll);


typedef struct LightHy {
    LightBase base;
    RayHy ray;
} LightHy;

LightLocal light_hy_to_local(LightHy light);

void light_hy_update_local(LightHy *light, LightLocal ll);


#ifndef HOST
#include "light.cc"
#endif // !HOST
