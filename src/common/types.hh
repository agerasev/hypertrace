#pragma once


#ifdef OPENCL

typedef uchar  uint8_t;
typedef char   int8_t;
typedef ushort uint16_t;
typedef short  int16_t;
typedef uint   uint32_t;
typedef int    int32_t;

#define make_int2 (int2)
#define make_int3 (int3)
#define make_int4 (int4)
#define make_uint2 (uint2)
#define make_uint3 (uint3)
#define make_uint4 (uint4)
#define make_float2 (float2)
#define make_float3 (float3)
#define make_float4 (float4)

#ifdef OPENCL_INTEROP

typedef uchar  uchar_pk;
typedef char   char_pk;
typedef ushort ushort_pk;
typedef short  short_pk;
typedef uint   uint_pk;
typedef int    int_pk;
typedef float  float_pk;

#define DEF_VEC_PACK(type, n, size) \
typedef struct { type s[size]; } type##n##_pk; \
type##n##_pk pack_##type##n(type##n v) { type##n##_pk p; vstore##n(v, 0, p.s); return p; } \
type##n unpack_##type##n(type##n##_pk p) { return vload##n(0, p.s); }

#define DEF_VEC_PACK_234(type) \
    DEF_VEC_PACK(type, 2, 2) \
    DEF_VEC_PACK(type, 3, 4) \
    DEF_VEC_PACK(type, 4, 4)

DEF_VEC_PACK_234(int)
DEF_VEC_PACK_234(uint)
DEF_VEC_PACK_234(float)

#endif // OPENCL_INTEROP

#else // OPENCL

#include <stdint.h>
#include <stdbool.h>

#include <vectype.hpp>

#define __global
#define __local
#define __constant const

typedef unsigned char  uchar;
typedef unsigned short ushort;
typedef unsigned int   uint;

typedef vectype<char, 2> char2;
typedef vectype<char, 3> char3;
typedef vectype<char, 4> char4;
typedef vectype<uchar, 2> uchar2;
typedef vectype<uchar, 3> uchar3;
typedef vectype<uchar, 4> uchar4;

typedef vectype<short, 2> short2;
typedef vectype<short, 3> short3;
typedef vectype<short, 4> short4;
typedef vectype<ushort, 2> ushort2;
typedef vectype<ushort, 3> ushort3;
typedef vectype<ushort, 4> ushort4;

typedef vectype<int, 2> int2;
typedef vectype<int, 3> int3;
typedef vectype<int, 4> int4;
typedef vectype<uint, 2> uint2;
typedef vectype<uint, 3> uint3;
typedef vectype<uint, 4> uint4;

typedef vectype<float, 2> float2;
typedef vectype<float, 3> float3;
typedef vectype<float, 4> float4;

typedef vectype<double, 2> double2;
typedef vectype<double, 3> double3;
typedef vectype<double, 4> double4;


#define make_int2 int2
#define make_int3 int3
#define make_int4 int4
#define make_uint2 uint2
#define make_uint3 uint3
#define make_uint4 uint4

#define make_float2 float2
#define make_float3 float3
#define make_float4 float4

#define make_double2 double2
#define make_double3 double3
#define make_double4 double4


#define convert_int2(x) (x).cast<int>()
#define convert_int3(x) (x).cast<int>()
#define convert_int4(x) (x).cast<int>()
#define convert_uint2(x) (x).cast<uint>()
#define convert_uint3(x) (x).cast<uint>()
#define convert_uint4(x) (x).cast<uint>()

#define convert_float2(x) (x).cast<float>()
#define convert_float3(x) (x).cast<float>()
#define convert_float4(x) (x).cast<float>()

#define convert_double2(x) (x).cast<double>()
#define convert_double3(x) (x).cast<double>()
#define convert_double4(x) (x).cast<double>()

#ifdef OPENCL_INTEROP

#include <CL/cl.h>

typedef cl_uchar  uchar_pk;
typedef cl_char   char_pk;
typedef cl_ushort ushort_pk;
typedef cl_short  short_pk;
typedef cl_uint   uint_pk;
typedef cl_int    int_pk;
typedef cl_float  float_pk;
typedef float_pk  double_pk;

typedef cl_int2 int2_pk;
typedef cl_int3 int3_pk;
typedef cl_int4 int4_pk;
typedef cl_uint2 uint2_pk;
typedef cl_uint3 uint3_pk;
typedef cl_uint4 uint4_pk;
typedef cl_float2 float2_pk;
typedef cl_float3 float3_pk;
typedef cl_float4 float4_pk;
typedef float2_pk double2_pk;
typedef float3_pk double3_pk;
typedef float4_pk double4_pk;

template <typename T, typename P, int N>
P pack_vectype(T t) {
    P p;
    for (int i = 0; i < N; ++i) {
        p.s[i] = t[i];
    }
    return p;
}
template <typename T, typename P, int N>
T unpack_vectype(P p) {
    T t;
    for (int i = 0; i < N; ++i) {
        t[i] = p.s[i];
    }
    return t;
}

#define pack_int2 pack_vectype<uint2, uint2_pk, 2>
#define pack_int3 pack_vectype<uint3, uint3_pk, 3>
#define pack_int4 pack_vectype<uint4, uint4_pk, 4>
#define unpack_int2 unpack_vectype<uint2, uint2_pk, 2>
#define unpack_int3 unpack_vectype<uint3, uint3_pk, 3>
#define unpack_int4 unpack_vectype<uint4, uint4_pk, 4>

#define pack_uint2 pack_vectype<int2, int2_pk, 2>
#define pack_uint3 pack_vectype<int3, int3_pk, 3>
#define pack_uint4 pack_vectype<int4, int4_pk, 4>
#define unpack_uint2 unpack_vectype<int2, int2_pk, 2>
#define unpack_uint3 unpack_vectype<int3, int3_pk, 3>
#define unpack_uint4 unpack_vectype<int4, int4_pk, 4>

#define pack_float2 pack_vectype<float2, float2_pk, 2>
#define pack_float3 pack_vectype<float3, float3_pk, 3>
#define pack_float4 pack_vectype<float4, float4_pk, 4>
#define unpack_float2 unpack_vectype<float2, float2_pk, 2>
#define unpack_float3 unpack_vectype<float3, float3_pk, 3>
#define unpack_float4 unpack_vectype<float4, float4_pk, 4>

inline double2_pk pack_double2(double2 v) { return pack_float2(v.cast<float>()); }
inline double3_pk pack_double3(double3 v) { return pack_float3(v.cast<float>()); }
inline double4_pk pack_double4(double4 v) { return pack_float4(v.cast<float>()); }
inline double2 unpack_double2(double2_pk v) { return unpack_float2(v).cast<double>(); }
inline double3 unpack_double3(double3_pk v) { return unpack_float3(v).cast<double>(); }
inline double4 unpack_double4(double4_pk v) { return unpack_float4(v).cast<double>(); }


#endif // OPENCL_INTEROP

#endif // OPENCL

#ifdef OPENCL_INTEROP

#define _PACKED_ALIGNMENT_ 4
#define _PACKED_STRUCT_ATTRIBUTE_ __attribute__((packed, aligned(_PACKED_ALIGNMENT_)))
#define _PACKED_FIELD_ATTRIBUTE_ __attribute__((packed, aligned(_PACKED_ALIGNMENT_)))

#endif // OPENCL_INTEROP

#define INTERPOLATE_FIELD(o, a, b, field, t) \
    ((o).field = (a).field*(1 - (t)) + (b).field*(t))

inline int mod(int a, int b) {
    int r = a % b;
    return r >= 0 ? r : r + b;
}


