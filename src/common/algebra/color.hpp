#pragma once

#include "vector.hpp"


typedef float3 color3;

color3 make_color(color3 c, float g=2.2f) {
    return color3::map([g](float x) { return math::pow(x, g); }, c);
}

color3 make_color(uint c, float g=2.2f) {
    return make_color(color3(
        float((c>>16)&0xFF)/0xFF,
        float((c>>8)&0xFF)/0xFF,
        float((c>>0)&0xFF)/0xFF
    ), g);
}
