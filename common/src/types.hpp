#pragma once

#include <traits.hpp>

#ifndef KERNEL

#include <cstdlib>

typedef unsigned char uchar;
typedef unsigned short ushort;
typedef unsigned int uint;
typedef unsigned long ulong;

#define __kernel
#define __address_space(as)
#define __private
#define __global
#define __constant
#define __local

#else // KERNEL

static_assert(sizeof(void *) == 4 || sizeof(void *) == 8);
using size_t = std::conditional_t<sizeof(void *) == 4, uint, ulong>;

#endif // KERNEL


static_assert(sizeof(char) == 1);
static_assert(sizeof(short) == 2);
static_assert(sizeof(int) == 4);
static_assert(sizeof(long) == 8);

static_assert(sizeof(uchar) == 1);
static_assert(sizeof(ushort) == 2);
static_assert(sizeof(uint) == 4);
static_assert(sizeof(ulong) == 8);

static_assert(sizeof(float) == 4);
static_assert(sizeof(double) == 8);

static_assert(sizeof(size_t) == sizeof(void *));
