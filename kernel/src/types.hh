#pragma once

#ifdef HOST

#include <cstdint>
#include <cstdbool>

#define __global
#define __local
#define __constant const

typedef unsigned char  uchar;
typedef unsigned short ushort;
typedef unsigned int   uint;

typedef uint64_t ulong;
typedef int64_t  long_;

static_assert(sizeof(char) == 1);
static_assert(sizeof(short) == 2);
static_assert(sizeof(int) == 4);
static_assert(sizeof(long_) == 8);

static_assert(sizeof(uchar) == 1);
static_assert(sizeof(ushort) == 2);
static_assert(sizeof(uint) == 4);
static_assert(sizeof(ulong) == 8);

typedef std::size_t    usize;
typedef std::ptrdiff_t isize;

#ifdef DOUBLE_SUPPORT
typedef double real;
#else
typedef float real;
#endif

#else // !HOST

#define NULL 0

#if ADDRESS_WIDTH == 32
typedef uint  usize;
typedef int   isize;
#elif ADDRESS_WIDTH == 64
typedef ulong usize;
typedef long  isize;
#endif

#ifdef DOUBLE_SUPPORT
typedef float real;
#else
typedef double real;
#endif

#define offsetof(st, m) \
    ((ulong)&(((st *)0)->m))

#endif // HOST

#ifdef HOST
#define _ALLOW_UNUSED_PARAMETERS_ _Pragma("GCC diagnostic ignored \"-Wunused-parameter\"");
#define _ALLOW_MULTIPLE_DEFINITIONS_ inline
#else // !HOST
#define _ALLOW_UNUSED_PARAMETERS_
#define _ALLOW_MULTIPLE_DEFINITIONS_
#endif // HOST
