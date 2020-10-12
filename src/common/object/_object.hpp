#pragma once

#include <common/types.hh>
#include <common/geometry/geometry.hpp>
#include <render/light.hpp>

/*
// Base class for all objects, depends on geometry
template <typename G>
class Object {
public:
    typedef G Geo;
    // Storage for state between detection and interaction
    struct Cache {};
    // Allow repeated collisions or not
    static const bool repeated = false;

    template <typename Context>
    real detect(
        Context &context,
        Cache &cache,
        Light<G> &light
    ) const;

    template <typename Context>
    bool interact(
        Context &context,
        const Cache &cache,
        Light<G> &light,
        float3 &luminance
    ) const;
};
*/
