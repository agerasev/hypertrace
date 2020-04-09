#include <types.hpp>

#include <cmath>
#include <algorithm>


#define DEFINE_PRIMITIVE_MATH(T) \
T min_##T(T x, T y) { return std::min(x, y); } \
T max_##T(T x, T y) { return std::max(x, y); } \

#define DEFINE_PRIMITIVE_MATH_UINT(T) \
T abs_##T(T x) { return x; } \
DEFINE_PRIMITIVE_MATH(T) \

#define DEFINE_PRIMITIVE_MATH_INT(T) \
T abs_##T(T x) { return std::abs(x); } \
DEFINE_PRIMITIVE_MATH(T) \

#define DEFINE_PRIMITIVE_MATH_FLOAT(T) \
T abs_##T(T x) { return std::abs(x); } \
DEFINE_PRIMITIVE_MATH(T) \
T sqrt_##T(T x) { return std::sqrt(x); } \

extern "C" {

DEFINE_PRIMITIVE_MATH_UINT(uchar)
DEFINE_PRIMITIVE_MATH_UINT(ushort)
DEFINE_PRIMITIVE_MATH_UINT(uint)
DEFINE_PRIMITIVE_MATH_UINT(ulong)

DEFINE_PRIMITIVE_MATH_INT(char)
DEFINE_PRIMITIVE_MATH_INT(short)
DEFINE_PRIMITIVE_MATH_INT(int)
DEFINE_PRIMITIVE_MATH_INT(long)

DEFINE_PRIMITIVE_MATH_FLOAT(float)
DEFINE_PRIMITIVE_MATH_FLOAT(double)

};
