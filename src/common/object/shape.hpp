#pragma once

#include <rstd/prelude.hpp>
#include <common/geometry/geometry.hpp>
#include <host/dyntype/type.hpp>


// Base class for all shapes
template <typename G>
class Shape : public dyn::Type {
public:
    typedef G Geo;

    class Instance : public dyn::Type::Instance {
    public:
        virtual Shape *_type() const override = 0;
        rstd::Box<Shape> type() const { return rstd::Box<Shape>::_from_raw(_type()); }
    };

    virtual Shape *_clone() const override = 0;
    rstd::Box<Shape> clone() const { return rstd::Box<Shape>::_from_raw(_clone()); }

    virtual Instance *_load(const uchar *src) const override = 0;
    rstd::Box<Instance> load(const uchar *src) const { return rstd::Box<Instance>::_from_raw(_load(src)); }


    // Whether to allow repeated collisions or not.
    virtual bool repeat_allowed() const = 0;
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
