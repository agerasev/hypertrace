#pragma once

typedef unsigned long ulong;
typedef unsigned int uint;
typedef unsigned short ushort;
typedef unsigned char uchar;

#ifdef __cplusplus

template <typename T>
class Zero {
public:
    static T zero();
};
template <typename T>
class One {
public:
    static T one();
};

template <typename T>
T zero() {
    return Zero<T>::zero();
}
template <typename T>
T one() {
    return One<T>::one();
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

#endif // __cplusplus
