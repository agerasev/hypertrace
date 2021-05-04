#pragma once

#include <geometry/ray_eu.hh>
#include "base.hh"

typedef struct LightEu {
    LightBase base;
    RayEu ray;
} LightEu;

LightLocal light_eu_to_local(LightEu light);
void light_eu_update_local(LightEu *light, LightLocal ll);

#ifndef HOST
#include "eu.cc"
#endif // !HOST
