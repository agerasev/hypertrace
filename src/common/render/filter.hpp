#pragma once

#include <types.h>
#include <math.hpp>
#include <algebra/color.hpp>

/*
class Filter {
public:
    color3 apply(color3 in) const;
};
*/

class GammaFilter {
// : public Filter
public:
    float factor = 1/2.2f;
    GammaFilter() = default;
    GammaFilter(float f) : factor(f) {}
    color3 apply(color3 in) const {
        return math::pow(in, factor);
    }
};
