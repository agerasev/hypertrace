#pragma once

#include <rstd.hpp>
#include <geometry/geometry.hpp>
#include <host/dyntype/type.hpp>


// Base class for all shapes
template <typename G>
class Shape : public dyn::Type {
public:
    typedef G Geo;

    class Instance : public dyn::Type::Instance {
    public:
        virtual Shape *_type() const override = 0;
        rs::Box<Shape> type() const { return rs::Box<Shape>::_from_raw(_type()); }
    };

    virtual Shape *_clone() const override = 0;
    rs::Box<Shape> clone() const { return rs::Box<Shape>::_from_raw(_clone()); }

    virtual Instance *_load(const uchar *src) const override = 0;
    rs::Box<Instance> load(const uchar *src) const { return rs::Box<Instance>::_from_raw(_load(src)); }


    // Whether to allow repeated collisions or not.
    virtual bool repeat_allowed() const = 0;
    std::string prefix() const { return rs::to_lower(name()); }
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
