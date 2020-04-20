#pragma once

#include <traits.hpp>
#include <variant.hpp>
#include "shape.hpp"
#include "object.hpp"


template <typename Shp>
constexpr bool _shp_repeated() {
    return Shp::repeated;
}

template <typename ...Shps>
class VariantShape :
    public Shape<typename nth_arg<0, Shps...>::Geo>,
    public Variant<Shps...>
{
public:
    typedef typename nth_arg<0, Shps...>::Geo Geo;
    static_assert(all<is_same<typename Shps::Geo, Geo>()...>());
    static const bool repeated = any<_shp_repeated<Shps>()...>();

    VariantShape() = default;
    VariantShape(Variant<Shps...> v) : Variant<Shps...>(v) {}
    template <int P, typename E>
    static VariantShape init(const E &e) {
        return VariantShape(Variant<Shps...>::template init<P>(e));
    }

private:
    template <typename E, int P>
    struct DetectCaller {
        template <typename Context>
        static real call(
            const E &e, 
            Context &context, typename Geo::Direction &normal, Light<Hy> &light
        ) {
            return e.detect(context, normal, light);
        }
    };
public:
    template <typename Context>
    real detect(
        Context &context, typename Geo::Direction &normal, Light<Hy> &light
    ) const {
        return Variant<Shps...>::as_union().template call<DetectCaller, 0>(
            Variant<Shps...>::id(), context, normal, light
        );
    }
};


template <typename Obj>
using _obj_cache_type = typename Obj::Cache;
template <typename Obj>
constexpr bool _obj_repeated() {
    return Obj::repeated;
}

template <typename ...Objs>
class VariantObject :
    public Object<typename nth_arg<0, Objs...>::Geo>,
    public Variant<Objs...>
{
public:
    typedef typename nth_arg<0, Objs...>::Geo Geo;
    static_assert(all<is_same<typename Objs::Geo, Geo>()...>());

    typedef Union<_obj_cache_type<Objs>...> Cache;

    static const bool repeated = any<_obj_repeated<Objs>()...>();

    VariantObject() = default;
    VariantObject(Variant<Objs...> v) : Variant<Objs...>(v) {}
    template <int P, typename E>
    static VariantObject init(const E &e) {
        return VariantObject(Variant<Objs...>::template init<P>(e));
    }

private:
    template <typename E, int P>
    struct DetectCaller {
        template <typename Context>
        static real call(const E &e, Context &context, Cache &cache, Light<Hy> &light) {
            return e.detect(context, cache.template elem<P>(), light);
        }
    };
    template <typename E, int P>
    struct InteractCaller {
        template <typename Context>
        static real call(
            const E &e,
            Context &context, const Cache &cache,
            Light<Hy> &light, float3 &luminance
        ) {
            return e.interact(context, cache.template elem<P>(), light, luminance);
        }
    };
public:
    template <typename Context>
    real detect(Context &context, Cache &cache, Light<Hy> &light) const {
        return Variant<Objs...>::as_union().template call<DetectCaller, 0>(
            Variant<Objs...>::id(), context, cache, light
        );
    }
    template <typename Context>
    bool interact(
        Context &context, const Cache &cache,
        Light<Hy> &light, float3 &luminance
    ) const {
        return Variant<Objs...>::as_union().template call<InteractCaller, 0>(
            Variant<Objs...>::id(), context, cache, light, luminance
        );
    }
};


#ifdef HOST

template <typename ...Shps>
struct ToDevice<VariantShape<Shps...>> {
public:
    typedef Variant<device_type<Shps>...> type;
private:
    template <typename E, int P>
    struct PackCaller {
        static type call(const E &e) {
            return type::template init<P>(::to_device(e));
        }
    };
public:
    static type to_device(const VariantShape<Shps...> &shp) {
        return shp.Variant<Shps...>::as_union().template call<PackCaller, 0>(
            shp.Variant<Shps...>::id()
        );
    }
};

template <typename ...Objs>
struct ToDevice<VariantObject<Objs...>> {
public:
    typedef Variant<device_type<Objs>...> type;
private:
    template <typename E, int P>
    struct PackCaller {
        static type call(const E &e) {
            return type::template init<P>(::to_device(e));
        }
    };
public:
    static type to_device(const VariantObject<Objs...> &obj) {
        return obj.Variant<Objs...>::as_union().template call<PackCaller, 0>(
            obj.Variant<Objs...>::id()
        );
    }
};

#endif
