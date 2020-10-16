#pragma once

#include <rstd/prelude.hpp>
#include <common/geometry/geometry.hpp>
#include "type.hpp"


// Base class for all shapes
template <typename G>
class Shape : public virtual Type {
public:
    typename G Geo;

    // Whether to allow repeated collisions or not.
    virtual bool repeat_allowed() = 0;
};

template <typename G>
class SurfaceShape : public virtual Shape<G> {};

template <typename G>
class VolumeShape : public virtual Shape<G> {
public:
    virtual bool repeat_allowed() override {
        return true;
    }
};
