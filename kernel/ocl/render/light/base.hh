#pragma once

#include <algebra/color.hh>

typedef struct LightBase {
    color3 intensity;
    bool diffuse;
} LightBase;

typedef struct LightLocal {
    LightBase base;
    real3 direction;
} LightLocal;
