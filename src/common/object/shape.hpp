#pragma once

#include <rstd/prelude.hpp>
#include <common/geometry/geometry.hpp>
#include <host/dyntype/type.hpp>


// Base class for all shapes
template <typename G>
class Shape : public dyn::Type {
public:
    typedef G Geo;

    // Whether to allow repeated collisions or not.
    virtual bool repeat_allowed() const = 0;

    virtual std::string prefix() const = 0;
};

template <typename G>
class SurfaceShape : public Shape<G> {};

template <typename G>
class VolumeShape : public Shape<G> {
public:
    virtual bool repeat_allowed() const override {
        return true;
    }
};
