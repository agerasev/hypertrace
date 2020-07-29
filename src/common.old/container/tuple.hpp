#pragma once

#include <types.h>
#include <traits.hpp>

#include "helper.hpp"


template <typename ...Elems>
class Tuple {};

template <typename T, typename ...Elems>
class Tuple<T, Elems...> {
private:
    T value;
    Tuple<Elems...> inner;

    template <template <typename...> typename A, int P, typename ...Args>
    friend struct helper::Accessor;
    template <template <typename...> typename A, typename ...Args>
    friend struct helper::Caller;

public:
    static constexpr int size() {
        return sizeof...(Elems) + 1;
    }
    template <int P>
    const nth_arg<P, T, Elems...> &elem() const {
        return helper::Accessor<Tuple, P, T, Elems...>::elem(*this);
    }
    template <int P>
    nth_arg<P, T, Elems...> &elem() {
        return helper::Accessor<Tuple, P, T, Elems...>::elem(*this);
    }

    template <template <typename, int> typename F, int P, typename ...Args>
    decltype(auto) call(int i, Args &&...args) const {
        return helper::Caller<Tuple, T, Elems...>::template call<F, P>(
            *this, i, forward<Args>(args)...
        );
    }
    template <template <typename, int> typename F, int P, typename ...Args>
    decltype(auto) call(int i, Args &&...args) {
        return helper::Caller<Tuple, T, Elems...>::template call<F, P>(
            *this, i, forward<Args>(args)...
        );
    }
};
