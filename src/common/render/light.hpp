#pragma once

#include <algebra/vector.hpp>
#include <geometry/ray.hpp>


template <typename G>
struct Light {
    Ray<G> ray;
    float3 color;
    bool diffuse = false;
};
