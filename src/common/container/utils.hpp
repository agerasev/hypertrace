#pragma once

#include <cstdlib>
#include <algorithm>

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

template <size_t P, typename ...Types>
struct NthType {};
template <size_t P, typename T, typename ...Types>
struct NthType<P, T, Types...> {
    typedef typename NthType<P - 1, Types...>::type type;
};
template <typename T, typename ...Types>
struct NthType<0, T, Types...> {
    typedef T type;
};
template <size_t P, typename ...Types>
using nth_type = typename NthType<P, Types...>::type;

} // namespace container
