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

template <typename H>
struct Interop;

template <typename H>
using dev_type = typename Interop<H>::dev;

template <typename H>
void dev_load(H *dst, const dev_type<H> *src) {
    Interop<H>::load(dst, src);
}

template <typename H>
void dev_store(dev_type<H> *dst, const H *src) {
    Interop<H>::store(dst, src);
}

#define PRIMITIVE_INTEROP(H, D) \
template <> \
struct Interop<H> { \
    typedef H Host; \
    typedef D Dev; \
    static void load(Host *dst, const Dev *src) { \
        *dst = (Host)(*src); \
    } \
    static void store(Dev *dst, const Host *src) { \
        *dst = (Dev)(*dst); \
    } \
}

#include <CL/cl.h>

PRIMITIVE_INTEROP(uchar,  cl_uchar);
PRIMITIVE_INTEROP(char,   cl_char);
PRIMITIVE_INTEROP(ushort, cl_ushort);
PRIMITIVE_INTEROP(ulong,  cl_ulong);
PRIMITIVE_INTEROP(short,  cl_short);
PRIMITIVE_INTEROP(uint,   cl_uint);
PRIMITIVE_INTEROP(int,    cl_int);
PRIMITIVE_INTEROP(long,   cl_long);
PRIMITIVE_INTEROP(float,  cl_float);
#ifdef DEV_F64
PRIMITIVE_INTEROP(double, cl_double);
#else // DEV_F64
PRIMITIVE_INTEROP(double, cl_float);
#endif // DEV_F64


#endif // INTEROP

#endif // HOST
