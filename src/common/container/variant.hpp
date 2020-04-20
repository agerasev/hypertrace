#pragma once

#include <types.h>
#include <traits.hpp>
#include "union.hpp"


template <typename ...Elems>
class Variant {
private:
    int id_ = -1;
    Union<Elems...> union_;

public:
    Variant() = default;
    template <int P>
    void set(const nth_arg<P, Elems...> &e) {
        static_assert(P >= 0 && P < sizeof...(Elems), "Index is out of bounds");
        as_union().template elem<P>() = e;
        id_ = P;
    }
    template <int P>
    static Variant init(const nth_arg<P, Elems...> &e) {
        Variant v;
        v.set<P>(e);
        return v;
    }
    static constexpr int size() {
        return sizeof...(Elems);
    }
    int id() const {
        return id_;
    }
    const Union<Elems...> &as_union() const {
        return union_;
    }
    Union<Elems...> &as_union() {
        return union_;
    }
};

#define DERIVE_VARIANT_METHOD(field, name) \
private: \
    template <typename E, int P> \
    struct name##Caller { \
        template <typename ...Args> \
        static decltype(auto) call(E &e, Args &&...args) { \
            return e.name(forward<Args>(args)...); \
        } \
    }; \
public: \
    template <typename ...Args> \
    decltype(auto) name(Args &&...args) { \
        return field.as_union().call<name##Caller, 0>( \
            field.id(), forward<Args>(args)... \
        ); \
    } \

#define DERIVE_VARIANT_METHOD_CONST(field, name) \
private: \
    template <typename E, int P> \
    struct name##Caller { \
        template <typename ...Args> \
        static decltype(auto) call(const E &e, Args &&...args) { \
            return e.name(forward<Args>(args)...); \
        } \
    }; \
public: \
    template <typename ...Args> \
    decltype(auto) name(Args &&...args) const { \
        return field.as_union().call<name##Caller, 0>( \
            field.id(), forward<Args>(args)... \
        ); \
    } \

