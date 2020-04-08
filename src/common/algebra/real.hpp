#pragma once

#include <types.h>


#if defined(DEVICE) && !defined(DEVICE_DOUBLE)

typedef float real;

#define PI 3.14159265358979323846f
#define EPS 1e-6f

#else

typedef double real;

#define PI 3.14159265358979323846
#define EPS 1e-8

#endif
