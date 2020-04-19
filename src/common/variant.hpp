#pragma once

#include <types.h>
#include <traits.hpp>


template <typename ...Elems>
class Union {};

template <int P, typename ...Elems>
struct Accessor {
    static const nth_arg<P, Elems...> &elem(const Union<Elems...> &u) {
        return u.inner.template elem<P - 1>();
    }
    static nth_arg<P, Elems...> &elem(Union<Elems...> &u) {
        return u.inner.template elem<P - 1>();
    }
};
template <typename ...Elems>
struct Accessor<0, Elems...> {
    static const nth_arg<0, Elems...> &elem(const Union<Elems...> &u) {
        return u.value;
    }
    static nth_arg<0, Elems...> &elem(Union<Elems...> &u) {
        return u.value;
    }
};

template <typename ...Elems>
struct Caller {};

template <typename T, typename ...Elems>
struct Caller<T, Elems...> {
    template <template <typename, int> typename F, int P, typename ...Args>
    static decltype(auto) call(const Union<T, Elems...> &u, int i, Args &&...args) {
        if (i == P) {
            return F<T, P>::call(u.value, forward<Args>(args)...);
        } else {
            return u.inner.template call<F, P + 1>(i, forward<Args>(args)...);
        }
    }
    template <template <typename, int> typename F, int P, typename ...Args>
    static decltype(auto) call(Union<T, Elems...> &u, int i, Args &&...args) {
        if (i == P) {
            return F<T, P>::call(u.value, forward<Args>(args)...);
        } else {
            return u.inner.template call<F, P + 1>(i, forward<Args>(args)...);
        }
    }
};
template <typename T>
struct Caller<T> {
    template <template <typename, int> typename F, int P, typename ...Args>
    static decltype(auto) call(const Union<T> &u, int i, Args &&...args) {
        return F<T, P>::call(u.value, forward<Args>(args)...);
    }
    template <template <typename, int> typename F, int P, typename ...Args>
    static decltype(auto) call(Union<T> &u, int i, Args &&...args) {
        return F<T, P>::call(u.value, forward<Args>(args)...);
    }
};

template <typename T, typename ...Elems>
class Union<T, Elems...> {
private:
    union {
        T value;
        Union<Elems...> inner;
    };
    template <int P, typename ...Args>
    friend struct Accessor;
    template <typename ...Args>
    friend struct Caller;

public:
    template <int P>
    const nth_arg<P, T, Elems...> &elem() const {
        return Accessor<P, T, Elems...>::elem(*this);
    }
    template <int P>
    nth_arg<P, T, Elems...> &elem() {
        return Accessor<P, T, Elems...>::elem(*this);
    }

    template <template <typename, int> typename F, int P, typename ...Args>
    decltype(auto) call(int i, Args &&...args) const {
        return Caller<T, Elems...>::template call<F, P>(*this, i, forward<Args>(args)...);
    }
    template <template <typename, int> typename F, int P, typename ...Args>
    decltype(auto) call(int i, Args &&...args) {
        return Caller<T, Elems...>::template call<F, P>(*this, i, forward<Args>(args)...);
    }
};


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

#define DERIVE_VARIANT_METHOD(name) \
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
        return as_union().call<name##Caller, 0>(id(), forward<Args>(args)...); \
    } \

#define DERIVE_VARIANT_METHOD_CONST(name) \
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
        return as_union().call<name##Caller, 0>(id(), forward<Args>(args)...); \
    } \

/*
template <typename ...Elems>
class RandomChoice {

};
*/
