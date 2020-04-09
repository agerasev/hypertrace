#pragma once

#include <types.hpp>


#define DECLARE_PRIMITIVE_MATH(T) \
T abs_##T(T x); \
T min_##T(T x, T y); \
T max_##T(T x, T y); \

#define DECLARE_PRIMITIVE_MATH_FLOAT(T) \
DECLARE_PRIMITIVE_MATH(T) \
T sqrt_##T(T x); \

#define DEFINE_PRIMITIVE_MATH_OVERLOADED(T) \
inline T abs(T x) { return abs_##T(x); } \
inline T min(T x, T y) { return min_##T(x, y); } \
inline T max(T x, T y) { return max_##T(x, y); } \

#define DEFINE_PRIMITIVE_MATH_OVERLOADED_FLOAT(T) \
DEFINE_PRIMITIVE_MATH_OVERLOADED(T) \
inline T sqrt(T x) { return sqrt_##T(x); } \

extern "C" {

DECLARE_PRIMITIVE_MATH(uchar)
DECLARE_PRIMITIVE_MATH(ushort)
DECLARE_PRIMITIVE_MATH(uint)
DECLARE_PRIMITIVE_MATH(ulong)

DECLARE_PRIMITIVE_MATH(char)
DECLARE_PRIMITIVE_MATH(short)
DECLARE_PRIMITIVE_MATH(int)
DECLARE_PRIMITIVE_MATH(long)

DECLARE_PRIMITIVE_MATH_FLOAT(float)
#if defined(HOST) || defined(DEVICE_DOUBLE)
DECLARE_PRIMITIVE_MATH_FLOAT(double)
#endif

};

namespace math {

DEFINE_PRIMITIVE_MATH_OVERLOADED(uchar)
DEFINE_PRIMITIVE_MATH_OVERLOADED(ushort)
DEFINE_PRIMITIVE_MATH_OVERLOADED(uint)
DEFINE_PRIMITIVE_MATH_OVERLOADED(ulong)

DEFINE_PRIMITIVE_MATH_OVERLOADED(char)
DEFINE_PRIMITIVE_MATH_OVERLOADED(short)
DEFINE_PRIMITIVE_MATH_OVERLOADED(int)
DEFINE_PRIMITIVE_MATH_OVERLOADED(long)

DEFINE_PRIMITIVE_MATH_OVERLOADED_FLOAT(float)
#if defined(HOST) || defined(DEVICE_DOUBLE)
DEFINE_PRIMITIVE_MATH_OVERLOADED_FLOAT(double)
#endif

}