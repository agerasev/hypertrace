#pragma once

#include <traits.hpp>
#include <math.hpp>


// Is int/float
template <typename T>
struct IsInt {
    static const bool value = false;
};
template <typename T>
constexpr bool is_int() {
    return IsInt<T>::value;
}
template <typename T>
struct IsFloat {
    static const bool value = false;
};
template <typename T>
constexpr bool is_float() {
    return IsFloat<T>::value;
}

// Zero
template <typename T>
struct Zero {
    static T zero();
};
template <typename T>
T zero() {
    return Zero<T>::zero();
}

// One
template <typename T>
struct One {
    static T one();
};
template <typename T>
T one() {
    return One<T>::one();
}

// Dim
template <typename T>
struct Dim {};
template <typename T>
constexpr int dim() {
    return Dim<T>::value;
}

// Base type (lowest type for nested structures)
template <typename T>
struct BaseType {};
template <typename T>
using base_type = typename BaseType<T>::type;

// Element type (for containers)
template <typename T>
struct ElementType {};
template <typename T>
using element_type = typename ElementType<T>::type;

// Sequence
template <typename A>
struct Sequence {
    template <typename F>
    static A map(F f, A a);
    template <typename F>
    static base_type<A> reduce(F f, base_type<A> t, A a);
};
template <typename A>
struct SequenceRoutines {
private:
    typedef base_type<A> T;
    typedef Sequence<A> S;
public:
    static A abs(A a) {
        return S::map([](T x) { return math::abs(x); }, a);
    }
    static A sqr(A a) {
        return S::map([](T x) { return x*x; }, a);
    }
};

// Convert
template <typename S, typename T>
struct Convert {
    static S convert(T x) {
        return S(x);
    }
};
template <typename S, typename T>
S convert(T x) {
    return Convert<S, T>::convert(x);
}
template <typename T>
struct Convert<T, T> {
    static T convert(T x) {
        return x;
    }
};

// Norm
template <typename A>
struct Norm {
private:
    typedef base_type<A> T;
    typedef Sequence<A> S;
    typedef SequenceRoutines<A> R;
public:
    static T l1(A a) {
        return S::reduce(
            [](T t, T x) { return t + x; },
            zero<T>(), R::abs(a)
        );
    }
    static enable_if<is_float<T>(), T> l2(A a) {
        return math::sqrt(S::reduce(
            [](T t, T x) { return t + x; },
            zero<T>(), R::sqr(a)
        ));
    }
    static T linf(A a) {
        return S::reduce(
            [](T t, T x) { return math::max(t, x); },
            zero<T>(), R::abs(a)
        );
    }
};
template <typename T>
static base_type<T> norm_l1(T x) {
    return Norm<T>::l1(x);
}
template <typename T>
static enable_if<is_float<base_type<T>>(), base_type<T>> norm_l2(T x) {
    return Norm<T>::l2(x);
}
template <typename T>
static base_type<T> norm_linf(T x) {
    return Norm<T>::linf(x);
}

// Definitions declaration

#define DEFINE_PRIMITIVE_ALGEBRA_TRAITS(T) \
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
    static const int value = 1; \
}; \
template <> \
struct BaseType<T> { \
    typedef T type; \
}; \
template <> \
struct Sequence<T> { \
    template <typename F> \
    static T map(F f, T a) { \
        return f(a); \
    } \
    template <typename F> \
    static T reduce(F f, T t, T a) { \
        return f(t, a); \
    } \
}; \

#define DEFINE_PRIMITIVE_ALGEBRA_TRAITS_UINT(T) \
DEFINE_PRIMITIVE_ALGEBRA_TRAITS(T) \
template <> \
struct IsInt<T> { \
    static const bool value = true; \
}; \

#define DEFINE_PRIMITIVE_ALGEBRA_TRAITS_INT(T) \
DEFINE_PRIMITIVE_ALGEBRA_TRAITS(T) \
template <> \
struct IsInt<T> { \
    static const bool value = true; \
}; \

#define DEFINE_PRIMITIVE_ALGEBRA_TRAITS_FLOAT(T) \
DEFINE_PRIMITIVE_ALGEBRA_TRAITS(T) \
template <> \
struct IsFloat<T> { \
    static const bool value = true; \
}; \

// Actual definition for types

DEFINE_PRIMITIVE_ALGEBRA_TRAITS_UINT(uchar)
DEFINE_PRIMITIVE_ALGEBRA_TRAITS_UINT(ushort)
DEFINE_PRIMITIVE_ALGEBRA_TRAITS_UINT(uint)
DEFINE_PRIMITIVE_ALGEBRA_TRAITS_UINT(ulong)
DEFINE_PRIMITIVE_ALGEBRA_TRAITS_INT(char)
DEFINE_PRIMITIVE_ALGEBRA_TRAITS_INT(short)
DEFINE_PRIMITIVE_ALGEBRA_TRAITS_INT(int)
DEFINE_PRIMITIVE_ALGEBRA_TRAITS_INT(long)

DEFINE_PRIMITIVE_ALGEBRA_TRAITS_FLOAT(float)
#if defined(HOST) || defined(DEVICE_DOUBLE)
DEFINE_PRIMITIVE_ALGEBRA_TRAITS_FLOAT(double)
#endif
