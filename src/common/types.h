#pragma once


#ifdef OPENCL_DEVICE

typedef uchar  uint8_t;
typedef char   int8_t;
typedef ushort uint16_t;
typedef short  int16_t;
typedef uint   uint32_t;
typedef int    int32_t;

typedef uint8_t  uint8_packed;
typedef int8_t   int8_packed;
typedef uint16_t uint16_packed;
typedef int16_t  int16_packed;
typedef uint32_t uint32_packed;
typedef int32_t  int32_packed;

typedef float    float_packed;

#else // OPENCL_DEVICE

#include <stdint.h>
#include <stdbool.h>

#include <CL/cl.h>

typedef cl_uchar  uint8_packed;
typedef cl_char   int8_packed;
typedef cl_ushort uint16_packed;
typedef cl_short  int16_packed;
typedef cl_uint   uint32_packed;
typedef cl_int    int32_packed;

typedef cl_float  float_packed;

#endif // OPENCL_DEVICE
