#pragma once

#include <traits.hpp>
#include <container/variant.hpp>

#include "shape.hpp"
#include "object.hpp"


template <typename Shp>
constexpr bool _shp_repeated() {
    return Shp::repeated;
}

template <typename ...Shps>
class VariantShape {
// : public Shape<typename nth_arg<0, Shps...>::Geo>
public:
    typedef typename nth_arg<0, Shps...>::Geo Geo;
    static_assert(all<is_same<typename Shps::Geo, Geo>()...>());
    static const bool repeated = any<_shp_repeated<Shps>()...>();
    
private:
    Variant<Shps...> variant;

public:
    VariantShape() = default;
    VariantShape(Variant<Shps...> v) : variant(v) {}
    template <int P, typename E>
    static VariantShape init(const E &e) {
        return VariantShape(Variant<Shps...>::template init<P>(e));
    }
    Variant<Shps...> &as_variant() {
        return variant;
    }
    const Variant<Shps...> &as_variant() const {
        return variant;
    }

private:
    template <typename E, int P>
    struct DetectCaller {
        template <typename Context>
        static real call(
            const E &e, 
            Context &context, typename Geo::Direction &normal, Light<Hy> &light
        ) {
            if (!context.repeat || E::repeated) {
                return e.detect(context, normal, light);
            } else {
                return -1_r;
            }
        }
    };
public:
    template <typename Context>
    real detect(
        Context &context, typename Geo::Direction &normal, Light<Hy> &light
    ) const {
        return variant.as_union().template call<DetectCaller, 0>(
            variant.id(), context, normal, light
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
class VariantObject {
// : public Object<typename nth_arg<0, Objs...>::Geo>
public:
    typedef typename nth_arg<0, Objs...>::Geo Geo;
    static_assert(all<is_same<typename Objs::Geo, Geo>()...>());

    typedef Union<_obj_cache_type<Objs>...> Cache;

    static const bool repeated = any<_obj_repeated<Objs>()...>();

    
private:
    Variant<Objs...> variant;

public:
    VariantObject() = default;
    VariantObject(Variant<Objs...> v) : variant(v) {}
    template <int P, typename E>
    static VariantObject init(const E &e) {
        return VariantObject(Variant<Objs...>::template init<P>(e));
    }
    Variant<Objs...> &as_variant() {
        return variant;
    }
    const Variant<Objs...> &as_variant() const {
        return variant;
    }

private:
    template <typename E, int P>
    struct DetectCaller {
        template <typename Context>
        static real call(const E &e, Context &context, Cache &cache, Light<Hy> &light) {
            if (!context.repeat || E::repeated) {
                return e.detect(context, cache.template elem<P>(), light);
            } else {
                return -1_r;
            }
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
        return variant.as_union().template call<DetectCaller, 0>(
            variant.id(), context, cache, light
        );
    }
    template <typename Context>
    bool interact(
        Context &context, const Cache &cache,
        Light<Hy> &light, float3 &luminance
    ) const {
        return variant.as_union().template call<InteractCaller, 0>(
            variant.id(), context, cache, light, luminance
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
        return shp.as_variant().as_union().template call<PackCaller, 0>(
            shp.as_variant().id()
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
        return obj.as_variant().as_union().template call<PackCaller, 0>(
            obj.as_variant().id()
        );
    }
};

#endif
