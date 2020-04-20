#pragma once

#include <traits.hpp>
#include "object.hpp"


template <typename Obj>
class Mapped : public Object<typename Obj::Geo> {
public:
    typedef typename Obj::Cache Cache;

    Obj inner;
    typename Obj::Geo::Map map;

    Mapped() = default;
    Mapped(const Obj &obj, typename Obj::Geo::Map map) :
        inner(obj), map(map)
    {}

    template <typename Context>
    real detect(
        Context &context, Cache &cache,
        Light<typename Obj::Geo> &light
    ) const {
        light.ray = light.ray.map(map);
        real dist = inner.detect(context, cache, light);
        if (dist > 0_r) {
            light.ray = light.ray.map(inverse(map));
        }
        return dist;
    }

    template <typename Context>
    bool interact(
        Context &context, const Cache &cache,
        Light<typename Obj::Geo> &light, float3 &luminance
    ) const {
        light.ray = light.ray.map(map);
        bool prod = inner.interact(context, cache, light, luminance);
        if (prod) {
            light.ray = light.ray.map(inverse(map));
        }
        return prod;
    }
};

#ifdef HOST

template <typename Obj>
struct ToDevice<Mapped<Obj>> {
    struct type {
        device_type<Obj> inner;
        device_type<typename Obj::Geo::Map> map;
    };
    static type to_device(const Mapped<Obj> &mobj) {
        return type{
            .inner = ::to_device(mobj.inner),
            .map = ::to_device(mobj.map)
        };
    }
};

#endif
