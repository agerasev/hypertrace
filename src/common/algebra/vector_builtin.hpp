#pragma once

#include <types.h>
#include <builtins/vector.h>


template <typename T>
struct IsBuiltinVector {
    static const bool value = false;
};
template <typename T>
constexpr bool is_builtin_vector() {
    return IsBuiltinVector<T>::value;
}

#define DEFINE_VECTOR_BUILTIN(T, N) \
template <> \
class vector<T, N> : public vector_base<T, N, N, sizeof(T)*(N + (N == 3))> { \
private: \
    typedef vector_base<T, N, N, sizeof(T)*(N + (N == 3))> base; \
public: \
    template <typename ...Args> \
    inline explicit vector(Args ...args) : base(args...) {} \
    inline static vector load(const T *data) { \
        vector v; \
        xv_load_##T##N(v.s, 0, data); \
        return v; \
    } \
    inline void store(T *data) const { \
        xv_store_##T##N(s, 0, data); \
    } \
    inline static vector<T, N> vload(__local const T *p, size_t i=0) { \
        vector v; \
        xv_load_##T##N##_local(v.s, i, p); \
        return v; \
    } \
    inline static vector<T, N> vload(__global const T *p, size_t i=0) { \
        vector v; \
        xv_load_##T##N##_global(v.s, i, p); \
        return v; \
    } \
    inline static vector<T, N> vload(__constant const T *p, size_t i=0) { \
        vector v; \
        xv_load_##T##N##_constant(v.s, i, p); \
        return v; \
    } \
    inline void vstore(__local T *p, size_t i=0) const { \
        xv_store_##T##N##_local(s, i, p); \
    } \
    inline void vstore(__global T *p, size_t i=0) const { \
        xv_store_##T##N##_global(s, i, p); \
    } \
    template <typename F, typename ...Args> \
    inline static vector<T, N> map(F f, Args ...args) { \
        return base::map(f, args...); \
    } \
    inline friend vector operator+(vector a) { \
        return a; \
    } \
    inline friend vector operator-(vector a) { \
        vector o; \
        xv_neg_##T##N(o.s, a.s); \
        return o; \
    } \
    inline friend vector operator+(vector a, vector b) { \
        vector o; \
        xv_add_##T##N##_##T##N(o.s, a.s, b.s); \
        return o; \
    } \
    inline friend vector operator+(vector a, T b) { \
        vector o; \
        xv_add_##T##N##_##T(o.s, a.s, b); \
        return o; \
    } \
    inline friend vector operator+(T a, vector b) { \
        vector o; \
        xv_add_##T##_##T##N(o.s, a, b.s); \
        return o; \
    } \
    inline friend vector operator-(vector a, vector b) { \
        vector o; \
        xv_sub_##T##N##_##T##N(o.s, a.s, b.s); \
        return o; \
    } \
    inline friend vector operator-(vector a, T b) { \
        vector o; \
        xv_sub_##T##N##_##T(o.s, a.s, b); \
        return o; \
    } \
    inline friend vector operator-(T a, vector b) { \
        vector o; \
        xv_sub_##T##_##T##N(o.s, a, b.s); \
        return o; \
    } \
    inline friend vector operator*(vector a, vector b) { \
        vector o; \
        xv_mul_##T##N##_##T##N(o.s, a.s, b.s); \
        return o; \
    } \
    inline friend vector operator*(vector a, T b) { \
        vector o; \
        xv_mul_##T##N##_##T(o.s, a.s, b); \
        return o; \
    } \
    inline friend vector operator*(T a, vector b) { \
        vector o; \
        xv_mul_##T##_##T##N(o.s, a, b.s); \
        return o; \
    } \
    inline friend vector operator/(vector a, vector b) { \
        vector o; \
        xv_div_##T##N##_##T##N(o.s, a.s, b.s); \
        return o; \
    } \
    inline friend vector operator/(vector a, T b) { \
        vector o; \
        xv_div_##T##N##_##T(o.s, a.s, b); \
        return o; \
    } \
    inline friend vector operator/(T a, vector b) { \
        vector o; \
        xv_div_##T##_##T##N(o.s, a, b.s); \
        return o; \
    } \
    inline vector &operator+=(vector a) { \
        xv_addassign_##T##N##_##T##N(s, a.s); \
        return *this; \
    } \
    inline vector &operator+=(T a) { \
        xv_addassign_##T##N##_##T(s, a); \
        return *this; \
    } \
    inline vector &operator-=(vector a) { \
        xv_subassign_##T##N##_##T##N(s, a.s); \
        return *this; \
    } \
    inline vector &operator-=(T a) { \
        xv_subassign_##T##N##_##T(s, a); \
        return *this; \
    } \
    inline vector &operator*=(vector a) { \
        xv_mulassign_##T##N##_##T##N(s, a.s); \
        return *this; \
    } \
    inline vector &operator*=(T a) { \
        xv_mulassign_##T##N##_##T(s, a); \
        return *this; \
    } \
    inline vector &operator/=(vector a) { \
        xv_divassign_##T##N##_##T##N(s, a.s); \
        return *this; \
    } \
    inline vector &operator/=(T a) { \
        xv_divassign_##T##N##_##T(s, a); \
        return *this; \
    } \
}; \
template <> \
struct IsBuiltinVector<vector<T, N>> { \
    static const bool value = true; \
}; \

#define DEFINE_VECTOR_BUILTIN_TRAITS(T, N) \
template <> \
struct SequenceRoutines<vector<T, N>> { \
    inline static vector<T, N> abs(vector<T, N> a) { \
        return math::abs(a); \
    } \
    inline static vector<T, N> sqr(vector<T, N> a) { \
        return a*a; \
    } \
}; \

#define DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, F, XF) \
inline vector<T, N> F(vector<T, N> a) { \
    vector<T, N> o; \
    xv_##XF##_##T##N(o.data(), a.data()); \
    return o; \
} \

#define DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_AB(T, N, F, XF) \
inline vector<T, N> F(vector<T, N> a, vector<T, N> b) { \
    vector<T, N> o; \
    xv_##XF##_##T##N(o.data(), a.data(), b.data()); \
    return o; \
} \

#define DEFINE_VECTOR_BUILTIN_MATH_COMMON(T, N) \
inline vector<T, N> clamp(vector<T, N> a, vector<T, N> b, vector<T, N> c) { \
    vector<T, N> o; \
    xv_clamp_##T##N(o.data(), a.data(), b.data(), c.data()); \
    return o; \
} \

#define DEFINE_VECTOR_BUILTIN_MATH_FLOAT(T, N) \
DEFINE_VECTOR_BUILTIN_MATH_COMMON(T, N) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, acos, acos) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, acosh, acosh) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, asin, asin) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, asinh, asinh) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, atan, atan) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_AB(T, N, atan2, atan2) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, atanh, atanh) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, ceil, ceil) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, cos, cos) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, cosh, cosh) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, erfc, erfc) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, erf, erf) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, exp, exp) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, abs, fabs) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, floor, floor) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_AB(T, N, max, fmax) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_AB(T, N, min, fmin) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_AB(T, N, mod, fmod) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, log, log) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_AB(T, N, pow, pow) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, round, round) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, sqrt, sqrt) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, sin, sin) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, sinh, sinh) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, tan, tan) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, tanh, tanh) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, tgamma, tgamma) \
inline pair<vector<T, N>, vector<T, N>> fract(vector<T, N> a) { \
    pair<vector<T, N>, vector<T, N>> p; \
    xv_fract_##T##N(p.first.data(), a.data(), p.second.data()); \
    return p; \
} \

#define DEFINE_VECTOR_BUILTIN_MATH_INT(T, N) \
DEFINE_VECTOR_BUILTIN_MATH_COMMON(T, N) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, abs, abs) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_AB(T, N, max, max) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_AB(T, N, min, min) \

#define DEFINE_VECTOR_BUILTIN_MATH_UINT(T, N) \
DEFINE_VECTOR_BUILTIN_MATH_COMMON(T, N) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_A(T, N, abs, abs) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_AB(T, N, max, max) \
DEFINE_VECTOR_BUILTIN_MATH_FUNCTION_AB(T, N, min, min) \

#define DEFINE_VECTOR_BUILTIN_UINT(T, N) \
DEFINE_VECTOR_BUILTIN(T, N) \
namespace math { \
DEFINE_VECTOR_BUILTIN_MATH_UINT(T, N) \
} \
DEFINE_VECTOR_BUILTIN_TRAITS(T, N) \

#define DEFINE_VECTOR_BUILTIN_INT(T, N) \
DEFINE_VECTOR_BUILTIN(T, N) \
namespace math { \
DEFINE_VECTOR_BUILTIN_MATH_INT(T, N) \
} \
DEFINE_VECTOR_BUILTIN_TRAITS(T, N) \

#define DEFINE_VECTOR_BUILTIN_FLOAT(T, N) \
DEFINE_VECTOR_BUILTIN(T, N) \
namespace math { \
DEFINE_VECTOR_BUILTIN_MATH_FLOAT(T, N) \
} \
DEFINE_VECTOR_BUILTIN_TRAITS(T, N) \


#define DEFINE_VECTOR_BUILTIN_GEOMETRY(T, N) \
template <> \
inline T dot(vector<T, N> a, vector<T, N> b) { \
    return xv_dot_##T##N(a.data(), b.data()); \
} \
template <> \
inline T length(vector<T, N> a) { \
    return xv_length_##T##N(a.data()); \
} \
template <> \
inline vector<T, N> normalize(vector<T, N> a) { \
    vector<T, N> o; \
    xv_normalize_##T##N(o.data(), a.data()); \
    return o; \
} \

#define DEFINE_VECTOR_BUILTIN_GEOMETRY_CROSS(T, N) \
template <> \
inline vector<T, N> cross<T>(vector<T, N> a, vector<T, N> b) { \
    vector<T, N> o; \
    xv_cross_##T##N(o.data(), a.data(), b.data()); \
    return o; \
} \

#define DEFINE_VECTOR_BUILTIN_CONVERT(S, T, N) \
template <> \
struct Convert<vector<S, N>, vector<T, N>> { \
    static vector<S, N> convert(vector<T, N> v) { \
        vector<S, N> o; \
        xv_convert_##S##N##_##T##N(o.data(), v.data()); \
        return o; \
    } \
}; \

#define DEFINE_VECTOR_BUILTIN_UINT_ALL(T) \
DEFINE_VECTOR_BUILTIN_UINT(T, 2) \
DEFINE_VECTOR_BUILTIN_UINT(T, 3) \
DEFINE_VECTOR_BUILTIN_UINT(T, 4) \
DEFINE_VECTOR_BUILTIN_UINT(T, 8) \
DEFINE_VECTOR_BUILTIN_UINT(T, 16) \

#define DEFINE_VECTOR_BUILTIN_INT_ALL(T) \
DEFINE_VECTOR_BUILTIN_INT(T, 2) \
DEFINE_VECTOR_BUILTIN_INT(T, 3) \
DEFINE_VECTOR_BUILTIN_INT(T, 4) \
DEFINE_VECTOR_BUILTIN_INT(T, 8) \
DEFINE_VECTOR_BUILTIN_INT(T, 16) \

#define DEFINE_VECTOR_BUILTIN_FLOAT_ALL(T) \
DEFINE_VECTOR_BUILTIN_FLOAT(T, 2) \
DEFINE_VECTOR_BUILTIN_FLOAT(T, 3) \
DEFINE_VECTOR_BUILTIN_FLOAT(T, 4) \
DEFINE_VECTOR_BUILTIN_FLOAT(T, 8) \
DEFINE_VECTOR_BUILTIN_FLOAT(T, 16) \
DEFINE_VECTOR_BUILTIN_GEOMETRY(T, 2) \
DEFINE_VECTOR_BUILTIN_GEOMETRY(T, 3) \
DEFINE_VECTOR_BUILTIN_GEOMETRY(T, 4) \
DEFINE_VECTOR_BUILTIN_GEOMETRY_CROSS(T, 3) \
DEFINE_VECTOR_BUILTIN_GEOMETRY_CROSS(T, 4) \

#ifdef DEVICE_DOUBLE
#define DEFINE_VECTOR_BUILTIN_IF_DOUBLE_ENABLED(DEF) DEF
#else
#define DEFINE_VECTOR_BUILTIN_IF_DOUBLE_ENABLED(DEF)
#endif

DEFINE_VECTOR_BUILTIN_UINT_ALL(uchar)
DEFINE_VECTOR_BUILTIN_UINT_ALL(ushort)
DEFINE_VECTOR_BUILTIN_UINT_ALL(uint)
DEFINE_VECTOR_BUILTIN_UINT_ALL(ulong)

DEFINE_VECTOR_BUILTIN_INT_ALL(char)
DEFINE_VECTOR_BUILTIN_INT_ALL(short)
DEFINE_VECTOR_BUILTIN_INT_ALL(int)
DEFINE_VECTOR_BUILTIN_INT_ALL(long)

DEFINE_VECTOR_BUILTIN_FLOAT_ALL(float)
DEFINE_VECTOR_BUILTIN_IF_DOUBLE_ENABLED(
    DEFINE_VECTOR_BUILTIN_FLOAT_ALL(double)
)

#define DEFINE_VECTOR_BUILTIN_CONVERT_DST(T, N) \
DEFINE_VECTOR_BUILTIN_CONVERT(uchar, T, N) \
DEFINE_VECTOR_BUILTIN_CONVERT(ushort, T, N) \
DEFINE_VECTOR_BUILTIN_CONVERT(uint, T, N) \
DEFINE_VECTOR_BUILTIN_CONVERT(ulong, T, N) \
DEFINE_VECTOR_BUILTIN_CONVERT(char, T, N) \
DEFINE_VECTOR_BUILTIN_CONVERT(short, T, N) \
DEFINE_VECTOR_BUILTIN_CONVERT(int, T, N) \
DEFINE_VECTOR_BUILTIN_CONVERT(long, T, N) \
DEFINE_VECTOR_BUILTIN_CONVERT(float, T, N) \
DEFINE_VECTOR_BUILTIN_IF_DOUBLE_ENABLED( \
    DEFINE_VECTOR_BUILTIN_CONVERT(double, T, N) \
) \

#define DEFINE_VECTOR_BUILTIN_CONVERT_ALL(N) \
DEFINE_VECTOR_BUILTIN_CONVERT_DST(uchar, N) \
DEFINE_VECTOR_BUILTIN_CONVERT_DST(ushort, N) \
DEFINE_VECTOR_BUILTIN_CONVERT_DST(uint, N) \
DEFINE_VECTOR_BUILTIN_CONVERT_DST(ulong, N) \
DEFINE_VECTOR_BUILTIN_CONVERT_DST(char, N) \
DEFINE_VECTOR_BUILTIN_CONVERT_DST(short, N) \
DEFINE_VECTOR_BUILTIN_CONVERT_DST(int, N) \
DEFINE_VECTOR_BUILTIN_CONVERT_DST(long, N) \
DEFINE_VECTOR_BUILTIN_CONVERT_DST(float, N) \
DEFINE_VECTOR_BUILTIN_IF_DOUBLE_ENABLED( \
    DEFINE_VECTOR_BUILTIN_CONVERT_DST(double, N) \
) \

DEFINE_VECTOR_BUILTIN_CONVERT_ALL(2)
DEFINE_VECTOR_BUILTIN_CONVERT_ALL(3)
DEFINE_VECTOR_BUILTIN_CONVERT_ALL(4)
DEFINE_VECTOR_BUILTIN_CONVERT_ALL(8)
DEFINE_VECTOR_BUILTIN_CONVERT_ALL(16)
