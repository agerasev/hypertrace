#pragma once


#include <types.h>

// Enable if
template<bool B, typename T = void>
struct EnableIf {};
template<typename T>
struct EnableIf<true, T> {
    typedef T type;
};
template <bool B, typename T = void>
using enable_if = typename EnableIf<B, T>::type;

// Is same
template<class T, class U>
struct IsSame {
    static const bool value = false;
};
template<class T>
struct IsSame<T, T> {
    static const bool value = true;
};
template <typename T, typename U>
constexpr bool is_same() {
    return IsSame<T, U>::value;
}

// Nth argument
template <int N, typename ...Args>
struct NthArg {};
template <int N, typename T, typename ...Args>
struct NthArg<N, T, Args...> {
    typedef typename NthArg<N - 1, Args...>::type type;
};
template <typename T, typename ...Args>
struct NthArg<0, T, Args...> {
    typedef T type;
};
template <int N, typename ...Args>
using nth_arg = typename NthArg<N, Args...>::type;

// Forward
template <typename T> struct remove_reference       { typedef T type; };
template <typename T> struct remove_reference<T &>  { typedef T type; };
template <typename T> struct remove_reference<T &&> { typedef T type; };
template <typename T>
inline T &&forward(typename remove_reference<T>::type  &t) {
    return static_cast<T &&>(t);
}
template <typename T>
inline T &&forward(typename remove_reference<T>::type &&t) {
    return static_cast<T &&>(t);
}

// Any / all
template<bool ...B>
struct BoolPack {};
template<bool ...B> 
constexpr bool all() {
    return is_same<BoolPack<B..., true>, BoolPack<true, B...>>();
}
template<bool ...B> 
constexpr bool any() {
    return !is_same<BoolPack<B..., false>, BoolPack<false, B...>>();
}

// Is primitive
template <typename T>
struct IsPrimitive {
    static const bool value = false;
};
template <typename T>
constexpr bool is_prim() {
    return IsPrimitive<T>::value;
}
// Type name
template <typename T>
struct TypeName {};
template <typename T>
const char *type_name() {
    return TypeName<T>::get_name();
}

#define DEFINE_PRIMITIVE_TRAITS(T) \
template <> \
struct IsPrimitive<T> { \
    static const bool value = true; \
}; \
template <> \
struct TypeName<T> { \
    static const char *get_name() { \
        return #T; \
    } \
}; \

DEFINE_PRIMITIVE_TRAITS(uchar)
DEFINE_PRIMITIVE_TRAITS(ushort)
DEFINE_PRIMITIVE_TRAITS(uint)
DEFINE_PRIMITIVE_TRAITS(ulong)
DEFINE_PRIMITIVE_TRAITS(char)
DEFINE_PRIMITIVE_TRAITS(short)
DEFINE_PRIMITIVE_TRAITS(int)
DEFINE_PRIMITIVE_TRAITS(long)
DEFINE_PRIMITIVE_TRAITS(float)
#if defined(HOST) || defined(DEVICE_DOUBLE)
DEFINE_PRIMITIVE_TRAITS(double)
#endif

#ifdef HOST

// Device data
template <typename T>
struct ToDevice {
    //typedef T type;
    //static type to_device(T x);
};
template <typename T>
using device_type = typename ToDevice<T>::type;
template <typename T>
device_type<T> to_device(const T &x) {
    return ToDevice<T>::to_device(x);
}

#define DEFINE_PRIMITIVE_TRAITS_HOST(T) \
template <> \
struct ToDevice<T> { \
    typedef T type; \
    static T to_device(T x) { \
        return x; \
    } \
}; \

#define DEFINE_PRIMITIVE_TRAITS_HOST_CAST(S, T) \
template <> \
struct ToDevice<T> { \
    typedef S type; \
    static S to_device(T x) { \
        return S(x); \
    } \
}; \


DEFINE_PRIMITIVE_TRAITS_HOST(uchar)
DEFINE_PRIMITIVE_TRAITS_HOST(ushort)
DEFINE_PRIMITIVE_TRAITS_HOST(uint)
DEFINE_PRIMITIVE_TRAITS_HOST(ulong)
DEFINE_PRIMITIVE_TRAITS_HOST(char)
DEFINE_PRIMITIVE_TRAITS_HOST(short)
DEFINE_PRIMITIVE_TRAITS_HOST(int)
DEFINE_PRIMITIVE_TRAITS_HOST(long)
DEFINE_PRIMITIVE_TRAITS_HOST(float)
#ifdef DEVICE_DOUBLE
DEFINE_PRIMITIVE_TRAITS_HOST(double)
#else
DEFINE_PRIMITIVE_TRAITS_HOST_CAST(float, double)
#endif

#endif
