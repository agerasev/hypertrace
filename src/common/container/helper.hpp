#pragma once

#include <types.h>
#include <traits.hpp>


namespace helper {

template <template <typename...> typename A, int P, typename ...Elems>
struct Accessor {
    static const nth_arg<P, Elems...> &elem(const A<Elems...> &u) {
        return u.inner.template elem<P - 1>();
    }
    static nth_arg<P, Elems...> &elem(A<Elems...> &u) {
        return u.inner.template elem<P - 1>();
    }
};
template <template <typename...> typename A, typename ...Elems>
struct Accessor<A, 0, Elems...> {
    static const nth_arg<0, Elems...> &elem(const A<Elems...> &u) {
        return u.value;
    }
    static nth_arg<0, Elems...> &elem(A<Elems...> &u) {
        return u.value;
    }
};

template <template <typename...> typename A, typename ...Elems>
struct Caller {};

template <template <typename...> typename A, typename T, typename ...Elems>
struct Caller<A, T, Elems...> {
    template <template <typename, int> typename F, int P, typename ...Args>
    static decltype(auto) call(const A<T, Elems...> &u, int i, Args &&...args) {
        if (i == P) {
            return F<T, P>::call(u.value, forward<Args>(args)...);
        } else {
            return u.inner.template call<F, P + 1>(i, forward<Args>(args)...);
        }
    }
    template <template <typename, int> typename F, int P, typename ...Args>
    static decltype(auto) call(A<T, Elems...> &u, int i, Args &&...args) {
        if (i == P) {
            return F<T, P>::call(u.value, forward<Args>(args)...);
        } else {
            return u.inner.template call<F, P + 1>(i, forward<Args>(args)...);
        }
    }
};
template <template <typename...> typename A, typename T>
struct Caller<A, T> {
    template <template <typename, int> typename F, int P, typename ...Args>
    static decltype(auto) call(const A<T> &u, int i, Args &&...args) {
        return F<T, P>::call(u.value, forward<Args>(args)...);
    }
    template <template <typename, int> typename F, int P, typename ...Args>
    static decltype(auto) call(A<T> &u, int i, Args &&...args) {
        return F<T, P>::call(u.value, forward<Args>(args)...);
    }
};

}
