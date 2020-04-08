#pragma once

#include <types.h>
#include <builtin.h>


#define DEFINE_VECTOR_BUILTIN(T, N, LEN, ALIGN) \
template <> \
class __attribute__((aligned(ALIGN))) vector<T, N> { \
private: \
    T s[LEN]; \
    template <int P=0, int M, typename ...Args> \
    static void check_all_dims(vector<T, M>, Args ...args) { \
        static_assert(N == M, "Wrong vector size"); \
        check_all_dims<P + 1>(args...); \
    } \
    template <int P=0> \
    static void check_all_dims() {} \
public: \
    inline vector() = default; \
    inline explicit vector(T x) { \
        ocl_ctor_##T##N##_##T(s, x); \
    } \
    template <typename ...Args, enable_if<(sizeof...(Args) > 1)>* = nullptr> \
    inline explicit vector(Args ...args) { \
        ocl_ctor_##T##N##_##T##_##N(s, args...); \
    } \
    T &operator[](int i) { \
        return s[i]; \
    } \
    const T &operator[](int i) const { \
        return s[i]; \
    } \
    T *data() { \
        return s; \
    } \
    const T *data() const { \
        return s; \
    } \
    static constexpr int size() { \
        return N; \
    } \
    template <int P> \
    T &elem() { \
        static_assert(P >= 0 && P < N, "Index is out of bounds"); \
        return s[P]; \
    } \
    template <int P> \
    const T &elem() const { \
        static_assert(P >= 0 && P < N, "Index is out of bounds"); \
        return s[P]; \
    } \
    template <int P, int S> \
    vector<T, S> &slice() { \
        static const int SA = S + (S == 3); \
        static_assert(P >= 0 && S > 0 && P + SA <= N, "Indices is out of bounds"); \
        static_assert((LEN % SA) == 0 && (P % SA) == 0, "Slicing breaks alignment"); \
        return *reinterpret_cast<vector<T, S>*>(s + P); \
    } \
    template <int P, int S> \
    const vector<T, S> &slice() const { \
        static const int SA = S + (S == 3); \
        static_assert(P >= 0 && S > 0 && P + SA <= N, "Indices is out of bounds"); \
        static_assert((LEN % SA) == 0 && (P % SA) == 0, "Slicing breaks alignment"); \
        return *reinterpret_cast<const vector<T, S>*>(s + P); \
    } \
    static vector load(const T *data) { \
        vector v; \
        ocl_load_##T##N(v.s, data); \
        return v; \
    } \
    void store(T *data) const { \
        ocl_store_##T##N(s, data); \
    } \
    static vector load(const T *data, int stride) { \
        vector v; \
        for (int i = 0; i < N; ++i) { \
            v[i] = data[i*stride]; \
        } \
        return v; \
    } \
    void store(T *data, int stride) const { \
        for (int i = 0; i < N; ++i) { \
            data[i*stride] = (*this)[i]; \
        } \
    } \
    template <typename F, typename ...Args> \
    static vector map(F f, Args ...args) { \
        check_all_dims(args...); \
        vector r; \
        for (int i = 0; i < N; ++i) { \
            r[i] = f((args[i])...); \
        } \
        return r; \
    } \
    inline friend vector operator+(vector a) { \
        return a; \
    } \
    inline friend vector operator-(vector a) { \
        vector o; \
        ocl_neg_##T##N(o.s, a.s); \
        return o; \
    } \
    inline friend vector operator+(vector a, vector b) { \
        vector o; \
        ocl_add_##T##N##_##T##N(o.s, a.s, b.s); \
        return o; \
    } \
    inline friend vector operator+(vector a, T b) { \
        vector o; \
        ocl_add_##T##N##_##T(o.s, a.s, b); \
        return o; \
    } \
    inline friend vector operator+(T a, vector b) { \
        vector o; \
        ocl_add_##T##_##T##N(o.s, a, b.s); \
        return o; \
    } \
    inline friend vector operator-(vector a, vector b) { \
        vector o; \
        ocl_sub_##T##N##_##T##N(o.s, a.s, b.s); \
        return o; \
    } \
    inline friend vector operator-(vector a, T b) { \
        vector o; \
        ocl_sub_##T##N##_##T(o.s, a.s, b); \
        return o; \
    } \
    inline friend vector operator-(T a, vector b) { \
        vector o; \
        ocl_sub_##T##_##T##N(o.s, a, b.s); \
        return o; \
    } \
    inline friend vector operator*(vector a, vector b) { \
        vector o; \
        ocl_mul_##T##N##_##T##N(o.s, a.s, b.s); \
        return o; \
    } \
    inline friend vector operator*(vector a, T b) { \
        vector o; \
        ocl_mul_##T##N##_##T(o.s, a.s, b); \
        return o; \
    } \
    inline friend vector operator*(T a, vector b) { \
        vector o; \
        ocl_mul_##T##_##T##N(o.s, a, b.s); \
        return o; \
    } \
    inline friend vector operator/(vector a, vector b) { \
        vector o; \
        ocl_div_##T##N##_##T##N(o.s, a.s, b.s); \
        return o; \
    } \
    inline friend vector operator/(vector a, T b) { \
        vector o; \
        ocl_div_##T##N##_##T(o.s, a.s, b); \
        return o; \
    } \
    inline friend vector operator/(T a, vector b) { \
        vector o; \
        ocl_div_##T##_##T##N(o.s, a, b.s); \
        return o; \
    } \
    inline vector &operator+=(vector a) { \
        ocl_addassign_##T##N##_##T##N(s, a.s); \
        return *this; \
    } \
    inline vector &operator+=(T a) { \
        ocl_addassign_##T##N##_##T(s, a); \
        return *this; \
    } \
    inline vector &operator-=(vector a) { \
        ocl_subassign_##T##N##_##T##N(s, a.s); \
        return *this; \
    } \
    inline vector &operator-=(T a) { \
        ocl_subassign_##T##N##_##T(s, a); \
        return *this; \
    } \
    inline vector &operator*=(vector a) { \
        ocl_mulassign_##T##N##_##T##N(s, a.s); \
        return *this; \
    } \
    inline vector &operator*=(T a) { \
        ocl_mulassign_##T##N##_##T(s, a); \
        return *this; \
    } \
    inline vector &operator/=(vector a) { \
        ocl_divassign_##T##N##_##T##N(s, a.s); \
        return *this; \
    } \
    inline vector &operator/=(T a) { \
        ocl_divassign_##T##N##_##T(s, a); \
        return *this; \
    } \
}; \


#define DEFINE_VECTOR_BUILTIN_GEOMETRY(T, N, L, A) \
template <> \
inline T dot(vector<T, N> a, vector<T, N> b) { \
    return ocl_dot_##T##N(a.data(), b.data()); \
} \
template <> \
inline T length(vector<T, N> a) { \
    return ocl_length_##T##N(a.data()); \
} \
template <> \
inline vector<T, N> normalize(vector<T, N> a) { \
    vector<T, N> o; \
    ocl_normalize_##T##N(o.data(), a.data()); \
    return o; \
} \


#define DEFINE_VECTOR_BUILTIN_GEOMETRY_CROSS(T, N) \
template <> \
inline vector<T, N> cross<T>(vector<T, N> a, vector<T, N> b) { \
    vector<T, N> o; \
    ocl_cross_##T##N(o.data(), a.data(), b.data()); \
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
