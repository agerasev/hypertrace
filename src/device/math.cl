
#define DEFINE_PRIMITIVE_MATH_UINT(T) \
T abs_##T(T x) { return x; } \
T min_##T(T x, T y) { return min(x, y); } \
T max_##T(T x, T y) { return max(x, y); } \

#define DEFINE_PRIMITIVE_MATH_INT(T) \
T abs_##T(T x) { return abs(x); } \
T min_##T(T x, T y) { return min(x, y); } \
T max_##T(T x, T y) { return max(x, y); } \

#define DEFINE_PRIMITIVE_MATH_FLOAT(T) \
T abs_##T(T x) { return fabs(x); } \
T min_##T(T x, T y) { return fmin(x, y); } \
T max_##T(T x, T y) { return fmax(x, y); } \
T sqrt_##T(T x) { return sqrt(x); } \


DEFINE_PRIMITIVE_MATH_UINT(uchar)
DEFINE_PRIMITIVE_MATH_UINT(ushort)
DEFINE_PRIMITIVE_MATH_UINT(uint)
DEFINE_PRIMITIVE_MATH_UINT(ulong)

DEFINE_PRIMITIVE_MATH_INT(char)
DEFINE_PRIMITIVE_MATH_INT(short)
DEFINE_PRIMITIVE_MATH_INT(int)
DEFINE_PRIMITIVE_MATH_INT(long)

DEFINE_PRIMITIVE_MATH_FLOAT(float)
#if defined(HOST) || defined(DEVICE_DOUBLE)
DEFINE_PRIMITIVE_MATH_FLOAT(double)
#endif
