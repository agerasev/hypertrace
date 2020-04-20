#pragma once

#include <types.h>
#include <geometry/geometry.hpp>
#include <render/light.hpp>


// Base class for all objects, depends on geometry
template <typename G>
class Object {
public:
    typedef G Geo;
    // Storage for state between detection and interaction
    struct Cache {};
    // Allow repeated collisions or not
    static const bool repeated = false;
};
