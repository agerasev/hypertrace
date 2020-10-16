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
typedef unsigned long int ulong;

static_assert(sizeof(char) == 1);
static_assert(sizeof(short) == 2);
static_assert(sizeof(int) == 4);
static_assert(sizeof(long) == 8);

static_assert(sizeof(uchar) == 1);
static_assert(sizeof(ushort) == 2);
static_assert(sizeof(uint) == 4);
static_assert(sizeof(ulong) == 8);

#ifdef INTEROP

template <typename H>
struct Interop;

template <typename H>
using dev_type = typename Interop<H>::Dev;
template <typename H>
inline static const char *const dev_name = Interop<H>::name;
template <typename H>
void dev_load(H *dst, const dev_type<H> *src) {
    Interop<H>::load(dst, src);
}
template <typename H>
void dev_store(dev_type<H> *dst, const H *src) {
    Interop<H>::store(dst, src);
}

#define PRIMITIVE_INTEROP(H, D, N) \
template <> \
struct Interop<H> { \
    typedef H Host; \
    typedef D Dev; \
    inline static const char *const name = N; \
    static void load(Host *dst, const Dev *src) { \
        *dst = (Host)(*src); \
    } \
    static void store(Dev *dst, const Host *src) { \
        *dst = (Dev)(*src); \
    } \
}

#include <CL/cl.h>

PRIMITIVE_INTEROP(uchar,  cl_uchar,  "uchar" );
PRIMITIVE_INTEROP(char,   cl_char,   "char"  );
PRIMITIVE_INTEROP(ushort, cl_ushort, "ushort");
PRIMITIVE_INTEROP(ulong,  cl_ulong,  "ulong" );
PRIMITIVE_INTEROP(short,  cl_short,  "short" );
PRIMITIVE_INTEROP(uint,   cl_uint,   "uint"  );
PRIMITIVE_INTEROP(int,    cl_int,    "int"   );
PRIMITIVE_INTEROP(long,   cl_long,   "long"  );
PRIMITIVE_INTEROP(float,  cl_float,  "float" );
#ifdef DEV_F64
PRIMITIVE_INTEROP(double, cl_double, "double");
#else // DEV_F64
PRIMITIVE_INTEROP(double, cl_float,  "float" );
#endif // DEV_F64


#endif // INTEROP

#endif // HOST


#ifdef HOST
#define _ALLOW_UNUSED_PARAMETERS_ _Pragma("GCC diagnostic ignored \"-Wunused-parameter\"");
#else // !HOST
#define _ALLOW_UNUSED_PARAMETERS_
#endif // HOST
