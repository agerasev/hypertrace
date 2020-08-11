#pragma once

#include <common/algebra/color.hh>
#include <common/geometry/ray.hh>


struct LightBase {
    color3 intensity;
    bool diffuse;
};

struct LightLocal {
    real3 direction;
    LightBase base;
};

struct LightEu {
    RayEu ray;
    LightBase base;
};

struct LightHy {
    RayHy ray;
    LightBase base;
};
