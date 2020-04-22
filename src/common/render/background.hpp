#pragma once

#include "light.hpp"


/*
template <typename G>
class Background {
public:
    template <typename Context>
    void interact(Context &context, LocalLight &light, float3 &emission) const;
};
*/

template <typename G>
class ConstantBackground {
// : public Background
public:
    float3 color;

    ConstantBackground() = default;
    explicit ConstantBackground(float3 c) : color(c) {}
    template <typename Context>
    void interact(Context &, Light<G> &light, float3 &emission) const {
        emission += color*light.intensity;
    }
};
