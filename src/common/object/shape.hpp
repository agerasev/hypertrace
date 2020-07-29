#pragma once

#include <types.h>
#include <geometry/geometry.hpp>
#include <render/light.hpp>

/*
// Base class for all shapes, depends on geometry
template <typename G>
class Shape {
public:
    typedef G Geo;
    // Allow repeated collisions or not
    static const bool repeated = false;

    template <typename Context>
    real detect(
        Context &context,
        typename G::Direction &normal,
        Light<G> &light
    ) const;
};

template <typename G>
class SurfaceShape : public Shape<G> {};
template <typename G>
class VolumeShape : public Shape<G> {
public:
    static const bool repeated = true;
};
*/
