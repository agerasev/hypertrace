#pragma once

#include <types.hpp>

#define DECLARE_BUILTIN_VECTOR(T, N) \
void ocl_load_##T##N(T *v, const T *d); \
void ocl_store_##T##N(const T *v, T *d); \
void ocl_neg_##T##N(T *o, const T *a); \
void ocl_add_##T##N##_##T##N(T *o, const T *a, const T *b); \
void ocl_add_##T##N##_##T(T *o, const T *a, T b); \
void ocl_add_##T##_##T##N(T *o, T a, const T *b); \
void ocl_sub_##T##N##_##T##N(T *o, const T *a, const T *b); \
void ocl_sub_##T##N##_##T(T *o, const T *a, T b); \
void ocl_sub_##T##_##T##N(T *o, T a, const T *b); \
void ocl_mul_##T##N##_##T##N(T *o, const T *a, const T *b); \
void ocl_mul_##T##N##_##T(T *o, const T *a, T b); \
void ocl_mul_##T##_##T##N(T *o, T a, const T *b); \
void ocl_div_##T##N##_##T##N(T *o, const T *a, const T *b); \
void ocl_div_##T##N##_##T(T *o, const T *a, T b); \
void ocl_div_##T##_##T##N(T *o, T a, const T *b); \
void ocl_addassign_##T##N##_##T##N(T *o, const T *a); \
void ocl_addassign_##T##N##_##T(T *o, T a); \
void ocl_subassign_##T##N##_##T##N(T *o, const T *a); \
void ocl_subassign_##T##N##_##T(T *o, T a); \
void ocl_mulassign_##T##N##_##T##N(T *o, const T *a); \
void ocl_mulassign_##T##N##_##T(T *o, T a); \
void ocl_divassign_##T##N##_##T##N(T *o, const T *a); \
void ocl_divassign_##T##N##_##T(T *o, T a); \

#define DECLARE_BUILTIN_VECTOR_GEOMETRY(T, N) \
T ocl_dot_##T##N(const T *a, const T *b); \
T ocl_length_##T##N(const T *a); \
void ocl_normalize_##T##N(T *o, const T *a); \

#define DECLARE_BUILTIN_VECTOR_GEOMETRY_CROSS(T, N) \
void ocl_cross_##T##N(T *o, const T *a, const T *b); \

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

DECLARE_BUILTIN_VECTOR(uchar, 2)
DECLARE_BUILTIN_VECTOR(uchar, 3)
DECLARE_BUILTIN_VECTOR(uchar, 4)
DECLARE_BUILTIN_VECTOR(uchar, 8)
DECLARE_BUILTIN_VECTOR(uchar, 16)
DECLARE_BUILTIN_VECTOR(char, 2)
DECLARE_BUILTIN_VECTOR(char, 3)
DECLARE_BUILTIN_VECTOR(char, 4)
DECLARE_BUILTIN_VECTOR(char, 8)
DECLARE_BUILTIN_VECTOR(char, 16)

DECLARE_BUILTIN_VECTOR(ushort, 2)
DECLARE_BUILTIN_VECTOR(ushort, 3)
DECLARE_BUILTIN_VECTOR(ushort, 4)
DECLARE_BUILTIN_VECTOR(ushort, 8)
DECLARE_BUILTIN_VECTOR(ushort, 16)
DECLARE_BUILTIN_VECTOR(short, 2)
DECLARE_BUILTIN_VECTOR(short, 3)
DECLARE_BUILTIN_VECTOR(short, 4)
DECLARE_BUILTIN_VECTOR(short, 8)
DECLARE_BUILTIN_VECTOR(short, 16)

DECLARE_BUILTIN_VECTOR(uint, 2)
DECLARE_BUILTIN_VECTOR(uint, 3)
DECLARE_BUILTIN_VECTOR(uint, 4)
DECLARE_BUILTIN_VECTOR(uint, 8)
DECLARE_BUILTIN_VECTOR(uint, 16)
DECLARE_BUILTIN_VECTOR(int, 2)
DECLARE_BUILTIN_VECTOR(int, 3)
DECLARE_BUILTIN_VECTOR(int, 4)
DECLARE_BUILTIN_VECTOR(int, 8)
DECLARE_BUILTIN_VECTOR(int, 16)

DECLARE_BUILTIN_VECTOR(ulong, 2)
DECLARE_BUILTIN_VECTOR(ulong, 3)
DECLARE_BUILTIN_VECTOR(ulong, 4)
DECLARE_BUILTIN_VECTOR(ulong, 8)
DECLARE_BUILTIN_VECTOR(ulong, 16)
DECLARE_BUILTIN_VECTOR(long, 2)
DECLARE_BUILTIN_VECTOR(long, 3)
DECLARE_BUILTIN_VECTOR(long, 4)
DECLARE_BUILTIN_VECTOR(long, 8)
DECLARE_BUILTIN_VECTOR(long, 16)

DECLARE_BUILTIN_VECTOR(float, 2)
DECLARE_BUILTIN_VECTOR(float, 3)
DECLARE_BUILTIN_VECTOR(float, 4)
DECLARE_BUILTIN_VECTOR(float, 8)
DECLARE_BUILTIN_VECTOR(float, 16)
DECLARE_BUILTIN_VECTOR_GEOMETRY(float, 2)
DECLARE_BUILTIN_VECTOR_GEOMETRY(float, 3)
DECLARE_BUILTIN_VECTOR_GEOMETRY(float, 4)
DECLARE_BUILTIN_VECTOR_GEOMETRY_CROSS(float, 3)
DECLARE_BUILTIN_VECTOR_GEOMETRY_CROSS(float, 4)

#ifdef DEVICE_DOUBLE
DECLARE_BUILTIN_VECTOR(double, 2)
DECLARE_BUILTIN_VECTOR(double, 3)
DECLARE_BUILTIN_VECTOR(double, 4)
DECLARE_BUILTIN_VECTOR(double, 8)
DECLARE_BUILTIN_VECTOR(double, 16)
DECLARE_BUILTIN_VECTOR_GEOMETRY(double, 2)
DECLARE_BUILTIN_VECTOR_GEOMETRY(double, 3)
DECLARE_BUILTIN_VECTOR_GEOMETRY(double, 4)
DECLARE_BUILTIN_VECTOR_GEOMETRY_CROSS(double, 3)
DECLARE_BUILTIN_VECTOR_GEOMETRY_CROSS(double, 4)
#endif // DEVICE_DOUBLE

#ifdef __cplusplus
};
#endif // __cplusplus
