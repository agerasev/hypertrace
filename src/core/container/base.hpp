#pragma once

#include <cstdlib>
#include <utility>

namespace container {

constexpr size_t max(const size_t a, const size_t b) {
    if (a > b) {
        return a;
    } else {
        return b;
    }
}

constexpr size_t gcd(const size_t a, const size_t b) {
    if (a == 0) {
        return b;
    }
    if (b == 0 || a == b) {
        return a;
    }

    if (a == 1 || b == 1) {
        return 1;
    }

    bool ao = a & 1, bo = b & 1;
    if (!ao && !bo) {
        return 2 * gcd(a >> 1, b >> 1);
    }
    if (ao && !bo) {
        return gcd(a, b >> 1);
    }
    if (!ao && bo) {
        return gcd(a >> 1, b);
    }
    
    if (a > b) {
        return gcd((a - b) >> 1, b);
    } else {
        return gcd(a, (b - a) >> 1);
    }
}

constexpr size_t lcm(size_t a, size_t b) {
    return (a*b)/gcd(a, b);
}

template <typename ...Types>
struct CommonSize {
    static const size_t value = 0;
};
template <typename T, typename ...Types>
struct CommonSize<T, Types...> {
    static const size_t value = max(sizeof(T), CommonSize<Types...>::value);
};
template <typename ...Types>
struct CommonSize<void, Types...> {
    static const size_t value = max(0, CommonSize<Types...>::value);
};
template <typename ...Types>
constexpr size_t common_size() {
    return CommonSize<Types...>::value;
}


template <typename ...Types>
struct CommonAlign {
    static const size_t value = 1;
};
template <typename T, typename ...Types>
struct CommonAlign<T, Types...> {
    static const size_t value = lcm(alignof(T), CommonAlign<Types...>::value);
};
template <typename ...Types>
struct CommonAlign<void, Types...> {
    static const size_t value = lcm(1, CommonAlign<Types...>::value);
};
template <typename ...Types>
constexpr size_t common_align() {
    return CommonAlign<Types...>::value;
}

template <template <size_t> typename F, size_t S, size_t Q = S - 1>
struct Dispatcher {
    static const size_t P = S - Q - 1;
    template <typename ...Args>
    static decltype(auto) dispatch(size_t i, Args &&...args) {
        if (i == P) {
            return F<P>::call(std::forward<Args>(args)...);
        } else {
            return Dispatcher<F, S, Q - 1>::dispatch(i, std::forward<Args>(args)...);
        }
    }
};
template <template <size_t> typename F, size_t S>
struct Dispatcher<F, S, 0> {
    template <typename ...Args>
    static decltype(auto) dispatch(size_t, Args &&...args) {
        return F<S - 1>::call(std::forward<Args>(args)...);
    }
};

} // namespace container
