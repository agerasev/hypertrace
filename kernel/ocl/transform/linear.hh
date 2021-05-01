#pragma once

#include <algebra/matrix.hh>
#include "shift.hh"

// Linear2

typedef struct {
    real2x2 v;
} Linear2;

Linear2 lin2_new(real2x2 v);

#define $Self Linear2
#define $self lin2
#define $elem real2
#include "interface.inl"
#undef $Self
#undef $self
#undef $elem

void lin2_shf2_reorder(Linear2 *a, Shift2 *b);
void shf2_lin2_reorder(Shift2 *a, Linear2 *b);

#ifdef HOST
template <>
struct Distance<Linear2> {
    static real distance(Linear2 a, Linear2 b) {
        return ::distance(a.v, b.v);
    }
};
#endif // HOST

// Linear3

typedef struct {
    real3x3 v;
} Linear3;

Linear3 lin3_new(real3x3 v);

#define $Self Linear3
#define $self lin3
#define $elem real3
#include "interface.inl"
#undef $Self
#undef $self
#undef $elem

Linear3 lin3_look_at(real3 dir);
Linear3 lin3_look_at_up(real3 dir, real3 up);

void lin3_shf3_reorder(Linear3 *a, Shift3 *b);
void shf3_lin3_reorder(Shift3 *a, Linear3 *b);

#ifdef HOST
template <>
struct Distance<Linear3> {
    static real distance(Linear3 a, Linear3 b) {
        return ::distance(a.v, b.v);
    }
};
#endif // HOST

#ifndef HOST
#include "linear.cc"
#endif // !HOST
