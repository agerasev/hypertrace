#pragma once

#include <common/types.hh>

#include "helper.hpp"


template <typename ...Elems>
class Union {};

template <typename T, typename ...Elems>
class Union<T, Elems...> {
private:
    union {
        T value;
        Union<Elems...> inner;
    };
    template <template <typename...> typename A, int P, typename ...Args>
    friend struct helper::Accessor;
    template <template <typename...> typename A, typename ...Args>
    friend struct helper::Caller;

public:
    Union() {}

    static constexpr int size() {
        return sizeof...(Elems) + 1;
    }
    template <int P>
    const helper::nth_arg<P, T, Elems...> &elem() const {
        return helper::Accessor<Union, P, T, Elems...>::elem(*this);
    }
    template <int P>
    helper::nth_arg<P, T, Elems...> &elem() {
        return helper::Accessor<Union, P, T, Elems...>::elem(*this);
    }

    template <template <typename, int> typename F, int P, typename ...Args>
    decltype(auto) call(int i, Args &&...args) const {
        return helper::Caller<Union, T, Elems...>::template call<F, P>(
            *this, i, std::forward<Args>(args)...
        );
    }
    template <template <typename, int> typename F, int P, typename ...Args>
    decltype(auto) call(int i, Args &&...args) {
        return helper::Caller<Union, T, Elems...>::template call<F, P>(
            *this, i, std::forward<Args>(args)...
        );
    }
};
