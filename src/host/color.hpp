#pragma once

#include <cstdint>
#include <vectype.hpp>


typedef vectype<float, 3> color3;

color3 make_color(color3 c, float g=2.2f) {
    return c.map([g](float x) { return pow(x, g); });
}

color3 make_color(uint32_t c, float g=2.2f) {
    return make_color(color3(
        float((c>>16)&0xFF)/0xFF,
        float((c>>8)&0xFF)/0xFF,
        float((c>>0)&0xFF)/0xFF
    ), g);
}
