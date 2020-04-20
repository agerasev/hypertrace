#pragma once

#include <types.h>
#include <geometry/geometry.hpp>
#include <render/light.hpp>


// Base class for all shapes, depends on geometry
template <typename G>
class Shape {
public:
    typedef G Geo;
    // Allow repeated collisions or not
    static const bool repeated = false;
};
