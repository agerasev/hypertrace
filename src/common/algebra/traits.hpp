#pragma once

#include <types.h>


template <typename T>
class Zero {
public:
    static T zero();
};
template <typename T>
T zero() {
    return Zero<T>::zero();
}

template <typename T>
class One {
public:
    static T one();
};
template <typename T>
T one() {
    return One<T>::one();
}

template <typename T>
class Dim {};
template <typename T>
constexpr int dim() {
    return Dim<T>::N;
}

template <typename T>
class BaseType {};

template <typename T>
class Conj {
public:
    static T conj(T x);
};
template <typename T>
static T conj(T x) {
    return Conj<T>::conj(x);
}

#define DEFINE_PRIMITIVE_TRAITS(T) \
template <> \
class Zero<T> { \
public: \
    static T zero() { \
        return T(0); \
    } \
}; \
template <> \
class One<T> { \
public: \
    static T one() { \
        return T(1); \
    } \
}; \
template <> \
class Dim<T> { \
public: \
    static const int N = 1; \
}; \
template <> \
class BaseType<T> { \
public: \
    typedef T type; \
}; \
template <> \
class Conj<T> { \
public: \
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
#ifdef DEVICE_DOUBLE
DEFINE_PRIMITIVE_TRAITS(double);
#endif // DEVICE_DOUBLE
