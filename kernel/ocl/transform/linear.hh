#pragma once

#include <algebra/matrix.hh>
#include "shift.hh"

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

void lin2_shf2_reorder(Linear2 *a, Shift2 *b);
void shf2_lin2_reorder(Shift2 *a, Linear2 *b);

// Linear3

typedef struct {
    real3x3 v;
} Linear3;

#define $Map Linear3
#define $map lin3
#define $elem real3
#include "derive.inl"
#undef $Map
#undef $map
#undef $elem

Linear3 lin3_look_to(real3 dir);
Linear3 lin3_look_to_up(real3 dir, real3 up);

void lin3_shf3_reorder(Linear3 *a, Shift3 *b);
void shf3_lin3_reorder(Shift3 *a, Linear3 *b);

#ifndef HOST
#include "linear.cc"
#endif // !HOST
