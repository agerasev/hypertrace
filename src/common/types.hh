#pragma once


#ifdef OPENCL

typedef uchar  uint8_t;
typedef char   int8_t;
typedef ushort uint16_t;
typedef short  int16_t;
typedef uint   uint32_t;
typedef int    int32_t;

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

typedef float   float_pk;
typedef float2  float2_pk;
typedef float3  float3_pk;
typedef float4  float4_pk;

#define pack_float2 (float2)
#define pack_float3 (float3)
#define pack_float4 (float4)
#define unpack_float2 (float2)
#define unpack_float3 (float3)
#define unpack_float4 (float4)

#endif // OPENCL_INTEROP

#else // OPENCL

#include <stdint.h>
#include <stdbool.h>

#include <vectype.hpp>


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

template <typename ... Args>
float2 make_float2(Args ...args) { return float2(args...); }
template <typename ... Args>
float3 make_float3(Args ...args) { return float3(args...); }
template <typename ... Args>
float4 make_float4(Args ...args) { return float4(args...); }

template <typename ... Args>
double2 make_double2(Args ...args) { return double2(args...); }
template <typename ... Args>
double3 make_double3(Args ...args) { return double3(args...); }
template <typename ... Args>
double4 make_double4(Args ...args) { return double4(args...); }

#ifdef OPENCL_INTEROP

#include <CL/cl.h>

typedef cl_uchar  uchar_pk;
typedef cl_char   char_pk;
typedef cl_ushort ushort_pk;
typedef cl_short  short_pk;
typedef cl_uint   uint_pk;
typedef cl_int    int_pk;

typedef cl_float  float_pk;
typedef cl_float2 float2_pk;
typedef cl_float3 float3_pk;
typedef cl_float4 float4_pk;

float2_pk pack_float2(float2 v);
float3_pk pack_float3(float3 v);
float4_pk pack_float4(float4 v);
float2 unpack_float2(float2_pk v);
float3 unpack_float3(float3_pk v);
float4 unpack_float4(float2_pk v);

typedef float_pk  double_pk;
typedef float2_pk double2_pk;
typedef float3_pk double3_pk;
typedef float4_pk double4_pk;

float2_pk pack_double2(double2 v);
float3_pk pack_double3(double3 v);
float4_pk pack_double4(double4 v);
double2 unpack_double2(float2_pk v);
double3 unpack_double3(float3_pk v);
double4 unpack_double4(float2_pk v);

#endif // OPENCL_INTEROP

#endif // OPENCL
