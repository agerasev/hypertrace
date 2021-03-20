#include "color.hh"


color3 color_new_gamma(float3 c, float g) {
    return pow(c, g);
}

#define color_new(x) color_new_gamma((x), 2.2f)

color3 color_from_uint(uint c) {
    return color_new(MAKE(float3)(
        (float)((c>>16)&0xFF)/0xFF,
        (float)((c>>8)&0xFF)/0xFF,
        (float)((c>>0)&0xFF)/0xFF
    ));
}
