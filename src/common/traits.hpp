#pragma once

// Enable if
template<bool B, typename T = void>
struct EnableIf {};
template<typename T>
struct EnableIf<true, T> {
    typedef T type;
};
template <bool B, typename T = void>
using enable_if = typename EnableIf<B, T>::type;

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
