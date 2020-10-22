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

typedef int64_t  long_;
typedef uint64_t ulong_;

static_assert(sizeof(char) == 1);
static_assert(sizeof(short) == 2);
static_assert(sizeof(int) == 4);
static_assert(sizeof(long_) == 8);

static_assert(sizeof(uchar) == 1);
static_assert(sizeof(ushort) == 2);
static_assert(sizeof(uint) == 4);
static_assert(sizeof(ulong_) == 8);

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

#define PRIMITIVE_INTEROP(H, D, A) \
template <> \
struct Interop<H> { \
    typedef H Host; \
    typedef D Dev; \
    inline static const char *const name = A; \
    static void load(Host *dst, const Dev *src) { \
        *dst = (Host)(*src); \
    } \
    static void store(Dev *dst, const Host *src) { \
        *dst = (Dev)(*src); \
    } \
}

#include <CL/cl.h>

PRIMITIVE_INTEROP(uint8_t,  cl_uchar,  "uchar" );
PRIMITIVE_INTEROP(int8_t,   cl_char,   "char"  );
PRIMITIVE_INTEROP(uint16_t, cl_ushort, "ushort");
PRIMITIVE_INTEROP(int16_t,  cl_short,  "short" );
PRIMITIVE_INTEROP(uint32_t, cl_uint,   "uint"  );
PRIMITIVE_INTEROP(int32_t,  cl_int,    "int"   );
PRIMITIVE_INTEROP(uint64_t, cl_ulong,  "ulong" );
PRIMITIVE_INTEROP(int64_t,  cl_long,   "long"  );
PRIMITIVE_INTEROP(float,    cl_float,  "float" );
#ifdef DEV_F64
PRIMITIVE_INTEROP(double,   cl_double, "double");
#else // DEV_F64
PRIMITIVE_INTEROP(double,   cl_float,  "float" );
#endif // DEV_F64


#endif // INTEROP

#else // !HOST

typedef long  long_;
typedef ulong ulong_;

#define offsetof(st, m) \
    ((ulong)&(((st *)0)->m))

#endif // HOST


#ifdef HOST
#define _ALLOW_UNUSED_PARAMETERS_ _Pragma("GCC diagnostic ignored \"-Wunused-parameter\"");
#else // !HOST
#define _ALLOW_UNUSED_PARAMETERS_
#endif // HOST
