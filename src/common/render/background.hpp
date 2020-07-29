#pragma once

#include <geometry/euclidean.hpp>
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

class GradientBackground {
// : public Background<Eu>
public:
    real3 dir;
    float3 up, down;

    GradientBackground() = default;
    GradientBackground(real3 dir, float3 u, float3 d) : dir(dir), up(u), down(d) {}
    template <typename Context>
    void interact(Context &, Light<Eu> &light, float3 &emission) const {
        real t = 0.5_r*(dot(light.ray.direction, dir) + 1_r);
        float3 color = t*up + (1 - t)*down;
        emission += color*light.intensity;
    }
};

class TestBackground {
// : public Background<Eu>
public:
    TestBackground() = default;
    template <typename Context>
    void interact(Context &, Light<Eu> &light, float3 &emission) const {
        emission += 0.5*(light.ray.direction + 1)*light.intensity;
    }
};

