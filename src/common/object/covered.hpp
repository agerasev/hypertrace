#pragma once

#include "shape.hpp"
#include "object.hpp"


template <typename Shp>
class Covered : public Object<typename Shp::Geo> {
public:
    typedef typename Shp::Geo Geo;
    typedef Object<Hy>::Cache Cache;
    static const bool repeated = Shp::repeated;

public:
    Shp shape;
    float3 color;

public:
    Covered() = default;
    Covered(const Shp &shp, float3 col) :
        shape(shp), color(col)
    {}

    template <typename Context>
    real detect(Context &context, Cache &, Light<Hy> &light) const {
        return shape.detect(context, light);
    }

    template <typename Context>
    bool interact(
        Context &context, const Cache &,
        Light<Hy> &light, float3 &luminance
    ) const {
        luminance += color*light.intensity;
        return false;
    }
};

#ifdef HOST

template <typename Shp>
struct ToDevice<Covered<Shp>> {
    struct type {
        device_type<Shp> shape;
        device_type<float3> color;
    };
    static type to_device(const Covered<Shp> &cov) {
        return type {
            .shape = ::to_device(cov.shape),
            .color = ::to_device(cov.color),
        };
    }
};

#endif
