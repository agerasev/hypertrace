#pragma once


#ifdef HOST

#include <stdint.h>
#include <stdbool.h>

#define __global
#define __local
#define __constant const

typedef unsigned char  uchar;
typedef unsigned short ushort;
typedef unsigned int   uint;

#ifdef INTEROP

#include <CL/cl.h>

typedef cl_uchar  uchar_dev;
typedef cl_char   char_dev;
typedef cl_ushort ushort_dev;
typedef cl_short  short_dev;
typedef cl_uint   uint_dev;
typedef cl_int    int_dev;
typedef cl_float  float_dev;
typedef float_dev double_dev;

#define float_load(x)   ((float)(x))
#define float_store(x)  ((float_dev)(x))
#define double_load(x)  ((double)(x))
#define double_store(x) ((double_dev)(x))

#endif // INTEROP

#endif // HOST
