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
    template <template <typename> typename F, typename ...Args>
    static decltype(auto) call(const Union<T, Elems...> &u, int i, Args &&...args) {
        if (i == 0) {
            return F<T>::call(u.value, forward<Args>(args)...);
        } else {
            return u.inner.template call<F>(i - 1, forward<Args>(args)...);
        }
    }
    template <template <typename> typename F, typename ...Args>
    static decltype(auto) call(Union<T, Elems...> &u, int i, Args &&...args) {
        if (i == 0) {
            return F<T>::call(u.value, forward<Args>(args)...);
        } else {
            return u.inner.template call<F>(i - 1, forward<Args>(args)...);
        }
    }
};
template <typename T>
struct Caller<T> {
    template <template <typename> typename F, typename ...Args>
    static decltype(auto) call(const Union<T> &u, int i, Args &&...args) {
        return F<T>::call(u.value, forward<Args>(args)...);
    }
    template <template <typename> typename F, typename ...Args>
    static decltype(auto) call(Union<T> &u, int i, Args &&...args) {
        return F<T>::call(u.value, forward<Args>(args)...);
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

    template <template <typename> typename F, typename ...Args>
    decltype(auto) call(int i, Args &&...args) const {
        return Caller<T, Elems...>::template call<F>(*this, i, forward<Args>(args)...);
    }
    template <template <typename> typename F, typename ...Args>
    decltype(auto) call(int i, Args &&...args) {
        return Caller<T, Elems...>::template call<F>(*this, i, forward<Args>(args)...);
    }
};


template <typename ...Elems>
class Variant : public Union<Elems...>{
private:
    int id_ = -1;

public:
    Variant() = default;
    template <int P>
    void set(const nth_arg<P, Elems...> &e) {
        static_assert(P >= 0 && P < sizeof...(Elems), "Index is out of bounds");
        Union<Elems...>::template elem<P>() = e;
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
};

#define DERIVE_VARIANT_METHOD(name) \
private: \
    template <typename E> \
    struct name##Caller { \
        template <typename ...Args> \
        static decltype(auto) call(E &e, Args &&...args) { \
            return e.name(forward<Args>(args)...); \
        } \
    }; \
public: \
    template <typename ...Args> \
    decltype(auto) name(Args &&...args) { \
        return call<name##Caller>(id(), forward<Args>(args)...); \
    } \

#define DERIVE_VARIANT_METHOD_CONST(name) \
private: \
    template <typename E> \
    struct name##Caller { \
        template <typename ...Args> \
        static decltype(auto) call(const E &e, Args &&...args) { \
            return e.name(forward<Args>(args)...); \
        } \
    }; \
public: \
    template <typename ...Args> \
    decltype(auto) name(Args &&...args) const { \
        return call<name##Caller>(id(), forward<Args>(args)...); \
    } \

/*
template <typename ...Elems>
class RandomChoice {

};
*/
