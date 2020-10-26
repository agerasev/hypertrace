#pragma once

#include "shape.hpp"
#include "object.hpp"
#include "material.hpp"


template <typename Shp, typename Mat>
class Covered {
// : public Object<typename Shp::Geo> 
public:
    typedef typename Shp::Geo Geo;
    struct Cache {
    // : public Object<Hy>::Cache
        typename Geo::Direction normal;
    };
    static const bool repeated = Shp::repeated;

public:
    Shp shape;
    Mat material;

public:
    Covered() = default;
    Covered(const Shp &shp, const Mat &mat) :
        shape(shp), material(mat)
    {}

    template <typename Context>
    real detect(Context &context, Cache &cache, Light<Geo> &light) const {
        return shape.detect(context, cache.normal, light);
    }

    template <typename Context>
    bool interact(
        Context &context, const Cache &cache,
        Light<Geo> &light, float3 &luminance
    ) const {
        real3 ln = Geo::dir_to_local(light.ray.start, cache.normal);
        LocalLight ll = light.get_local();
        bool bounce = material.interact(context, ln, ll, luminance);
        if (bounce) {
            light.set_local(ll);
        }
        return bounce;
    }
};

#ifdef HOST

template <typename Shp, typename Mat>
struct ToDevice<Covered<Shp, Mat>> {
    struct type {
        device_type<Shp> shape;
        device_type<Mat> material;
    };
    static type to_device(const Covered<Shp, Mat> &cov) {
        return type {
            .shape = ::to_device(cov.shape),
            .material = ::to_device(cov.material),
        };
    }
};

#endif
