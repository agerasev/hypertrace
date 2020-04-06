#pragma once

#include <builtin.h>


#define DEFINE_VECTOR_BUILTIN(T, N, L, A) \
template <> \
class __attribute__((aligned(A))) vector<T, N> { \
    public: \
    T s[L]; \
    T &operator[](int i) { \
        return s[i]; \
    } \
    const T &operator[](int i) const { \
        return s[i]; \
    } \
    static vector load(const T *data) { \
        vector v; \
        ocl_load_##T##N(v.s, data); \
        return v; \
    } \
    void store(T *data) const { \
        ocl_store_##T##N(s, data); \
    } \
    friend vector operator+(vector a) { \
        return a; \
    } \
    friend vector operator-(vector a) { \
        vector o; \
        ocl_neg_##T##N(o.s, a.s); \
        return o; \
    } \
    friend vector operator+(vector a, vector b) { \
        vector o; \
        ocl_add_##T##N##_##T##N(o.s, a.s, b.s); \
        return o; \
    } \
    friend vector operator+(vector a, T b) { \
        vector o; \
        ocl_add_##T##N##_##T(o.s, a.s, b); \
        return o; \
    } \
    friend vector operator+(T a, vector b) { \
        vector o; \
        ocl_add_##T##_##T##N(o.s, a, b.s); \
        return o; \
    } \
    friend vector operator-(vector a, vector b) { \
        vector o; \
        ocl_sub_##T##N##_##T##N(o.s, a.s, b.s); \
        return o; \
    } \
    friend vector operator-(vector a, T b) { \
        vector o; \
        ocl_sub_##T##N##_##T(o.s, a.s, b); \
        return o; \
    } \
    friend vector operator-(T a, vector b) { \
        vector o; \
        ocl_sub_##T##_##T##N(o.s, a, b.s); \
        return o; \
    } \
    friend vector operator*(vector a, vector b) { \
        vector o; \
        ocl_mul_##T##N##_##T##N(o.s, a.s, b.s); \
        return o; \
    } \
    friend vector operator*(vector a, T b) { \
        vector o; \
        ocl_mul_##T##N##_##T(o.s, a.s, b); \
        return o; \
    } \
    friend vector operator*(T a, vector b) { \
        vector o; \
        ocl_mul_##T##_##T##N(o.s, a, b.s); \
        return o; \
    } \
    friend vector operator/(vector a, vector b) { \
        vector o; \
        ocl_div_##T##N##_##T##N(o.s, a.s, b.s); \
        return o; \
    } \
    friend vector operator/(vector a, T b) { \
        vector o; \
        ocl_div_##T##N##_##T(o.s, a.s, b); \
        return o; \
    } \
    friend vector operator/(T a, vector b) { \
        vector o; \
        ocl_div_##T##_##T##N(o.s, a, b.s); \
        return o; \
    } \
    vector &operator+=(vector a) { \
        ocl_addassign_##T##N##_##T##N(s, a.s); \
        return *this; \
    } \
    vector &operator+=(T a) { \
        ocl_addassign_##T##N##_##T(s, a); \
        return *this; \
    } \
    vector &operator-=(vector a) { \
        ocl_subassign_##T##N##_##T##N(s, a.s); \
        return *this; \
    } \
    vector &operator-=(T a) { \
        ocl_subassign_##T##N##_##T(s, a); \
        return *this; \
    } \
    vector &operator*=(vector a) { \
        ocl_mulassign_##T##N##_##T##N(s, a.s); \
        return *this; \
    } \
    vector &operator*=(T a) { \
        ocl_mulassign_##T##N##_##T(s, a); \
        return *this; \
    } \
    vector &operator/=(vector a) { \
        ocl_divassign_##T##N##_##T##N(s, a.s); \
        return *this; \
    } \
    vector &operator/=(T a) { \
        ocl_divassign_##T##N##_##T(s, a); \
        return *this; \
    } \
}; \


#define DEFINE_VECTOR_BUILTIN_GEOMETRY(T, N, L, A) \
template <> \
T dot(vector<T, N> a, vector<T, N> b) { \
    return ocl_dot_##T##N(a.s, b.s); \
} \
template <> \
T length(vector<T, N> a) { \
    return ocl_length_##T##N(a.s); \
} \
template <> \
vector<T, N> normalize(vector<T, N> a) { \
    vector<T, N> o; \
    ocl_normalize_##T##N(o.s, a.s); \
    return o; \
} \


#define DEFINE_VECTOR_BUILTIN_GEOMETRY_CROSS(T, N) \
template <> \
vector<T, N> cross<T>(vector<T, N> a, vector<T, N> b) { \
    vector<T, N> o; \
    ocl_cross_##T##N(o.s, a.s, b.s); \
    return o; \
} \


DEFINE_VECTOR_BUILTIN(uchar, 2, 2, 2)
DEFINE_VECTOR_BUILTIN(uchar, 3, 4, 4)
DEFINE_VECTOR_BUILTIN(uchar, 4, 4, 4)
DEFINE_VECTOR_BUILTIN(uchar, 8, 8, 8)
DEFINE_VECTOR_BUILTIN(uchar, 16, 16, 16)
DEFINE_VECTOR_BUILTIN(char, 2, 2, 2)
DEFINE_VECTOR_BUILTIN(char, 3, 4, 4)
DEFINE_VECTOR_BUILTIN(char, 4, 4, 4)
DEFINE_VECTOR_BUILTIN(char, 8, 8, 8)
DEFINE_VECTOR_BUILTIN(char, 16, 16, 16)

DEFINE_VECTOR_BUILTIN(ushort, 2, 2, 4)
DEFINE_VECTOR_BUILTIN(ushort, 3, 4, 8)
DEFINE_VECTOR_BUILTIN(ushort, 4, 4, 8)
DEFINE_VECTOR_BUILTIN(ushort, 8, 8, 16)
DEFINE_VECTOR_BUILTIN(ushort, 16, 16, 32)
DEFINE_VECTOR_BUILTIN(short, 2, 2, 4)
DEFINE_VECTOR_BUILTIN(short, 3, 4, 8)
DEFINE_VECTOR_BUILTIN(short, 4, 4, 8)
DEFINE_VECTOR_BUILTIN(short, 8, 8, 16)
DEFINE_VECTOR_BUILTIN(short, 16, 16, 32)

DEFINE_VECTOR_BUILTIN(uint, 2, 2, 8)
DEFINE_VECTOR_BUILTIN(uint, 3, 4, 16)
DEFINE_VECTOR_BUILTIN(uint, 4, 4, 16)
DEFINE_VECTOR_BUILTIN(uint, 8, 8, 32)
DEFINE_VECTOR_BUILTIN(uint, 16, 16, 64)
DEFINE_VECTOR_BUILTIN(int, 2, 2, 8)
DEFINE_VECTOR_BUILTIN(int, 3, 4, 16)
DEFINE_VECTOR_BUILTIN(int, 4, 4, 16)
DEFINE_VECTOR_BUILTIN(int, 8, 8, 32)
DEFINE_VECTOR_BUILTIN(int, 16, 16, 64)

DEFINE_VECTOR_BUILTIN(ulong, 2, 2, 16)
DEFINE_VECTOR_BUILTIN(ulong, 3, 4, 32)
DEFINE_VECTOR_BUILTIN(ulong, 4, 4, 32)
DEFINE_VECTOR_BUILTIN(ulong, 8, 8, 64)
DEFINE_VECTOR_BUILTIN(ulong, 16, 16, 128)
DEFINE_VECTOR_BUILTIN(long, 2, 2, 16)
DEFINE_VECTOR_BUILTIN(long, 3, 4, 32)
DEFINE_VECTOR_BUILTIN(long, 4, 4, 32)
DEFINE_VECTOR_BUILTIN(long, 8, 8, 64)
DEFINE_VECTOR_BUILTIN(long, 16, 16, 128)

DEFINE_VECTOR_BUILTIN(float, 2, 2, 8)
DEFINE_VECTOR_BUILTIN(float, 3, 4, 16)
DEFINE_VECTOR_BUILTIN(float, 4, 4, 16)
DEFINE_VECTOR_BUILTIN(float, 8, 8, 32)
DEFINE_VECTOR_BUILTIN(float, 16, 16, 64)
DEFINE_VECTOR_BUILTIN_GEOMETRY(float, 2, 2, 8)
DEFINE_VECTOR_BUILTIN_GEOMETRY(float, 3, 4, 16)
DEFINE_VECTOR_BUILTIN_GEOMETRY(float, 4, 4, 16)
DEFINE_VECTOR_BUILTIN_GEOMETRY_CROSS(float, 3)
DEFINE_VECTOR_BUILTIN_GEOMETRY_CROSS(float, 4)

#ifdef DEVICE_DOUBLE
DEFINE_VECTOR_BUILTIN(double, 2, 2, 16)
DEFINE_VECTOR_BUILTIN(double, 3, 4, 32)
DEFINE_VECTOR_BUILTIN(double, 4, 4, 32)
DEFINE_VECTOR_BUILTIN(double, 8, 8, 64)
DEFINE_VECTOR_BUILTIN(double, 16, 16, 128)
DEFINE_VECTOR_BUILTIN_GEOMETRY(double, 2, 2, 16)
DEFINE_VECTOR_BUILTIN_GEOMETRY(double, 3, 4, 32)
DEFINE_VECTOR_BUILTIN_GEOMETRY(double, 4, 4, 32)
DEFINE_VECTOR_BUILTIN_GEOMETRY_CROSS(double, 3)
DEFINE_VECTOR_BUILTIN_GEOMETRY_CROSS(double, 4)
#endif // DEVICE_DOUBLE
