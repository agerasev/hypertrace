#pragma once

#include <algebra/matrix.hh>

// Linear2

typedef struct {
    real2x2 v;
} Linear2;

#define $Map Linear2
#define $map lin2
#define $elem real2
#include "derive.inl"
#undef $Map
#undef $map
#undef $elem

// Linear3

typedef struct {
    real3x3 v;
} Linear3;

Linear3 lin3_look_to(real3 dir);
Linear3 lin3_look_to_up(real3 dir, real3 up);

#define $Map Linear3
#define $map lin3
#define $elem real3
#include "derive.inl"
#undef $Map
#undef $map
#undef $elem

#ifndef HOST
#include "linear.cc"
#endif // !HOST
