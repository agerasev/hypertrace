#pragma once

#include <types.h>


template<bool B, typename T = void>
struct enable_if {};
template<typename T>
struct enable_if<true, T> {
    typedef T type;
};

template <typename T>
struct Zero {
    static T zero();
};
template <typename T>
T zero() {
    return Zero<T>::zero();
}

template <typename T>
struct One {
    static T one();
};
template <typename T>
T one() {
    return One<T>::one();
}

template <typename T>
struct Dim {};
template <typename T>
constexpr int dim() {
    return Dim<T>::N;
}

template <typename T>
struct base_type {};

template <typename T>
struct Conj {
    static T conj(T x);
};
template <typename T>
static T conj(T x) {
    return Conj<T>::conj(x);
}

#define DEFINE_PRIMITIVE_TRAITS(T) \
template <> \
struct Zero<T> { \
    static T zero() { \
        return T(0); \
    } \
}; \
template <> \
struct One<T> { \
    static T one() { \
        return T(1); \
    } \
}; \
template <> \
struct Dim<T> { \
    static const int N = 1; \
}; \
template <> \
struct base_type<T> { \
    typedef T type; \
}; \
template <> \
struct Conj<T> { \
    static T conj(T x) { \
        return x; \
    } \
}; \

DEFINE_PRIMITIVE_TRAITS(uchar);
DEFINE_PRIMITIVE_TRAITS(char);
DEFINE_PRIMITIVE_TRAITS(ushort);
DEFINE_PRIMITIVE_TRAITS(short);
DEFINE_PRIMITIVE_TRAITS(uint);
DEFINE_PRIMITIVE_TRAITS(int);
DEFINE_PRIMITIVE_TRAITS(ulong);
DEFINE_PRIMITIVE_TRAITS(long);

DEFINE_PRIMITIVE_TRAITS(float);
#if defined(HOST) || defined(DEVICE_DOUBLE)
DEFINE_PRIMITIVE_TRAITS(double);
#endif
