#pragma once

#include <algebra/real.hh>
#include "shift.hh"

typedef struct {
    real v;
} Scale;

#ifdef HOST
template <>
struct Distance<Scale> {
    static real distance(Scale a, Scale b) {
        return std::abs(a.v - b.v);
    }
};
#endif // HOST

// Scale2

typedef Scale Scale2;

#define $Map Scale2
#define $map scl2
#define $elem real2
#include "interface.inl"
#undef $Map
#undef $map
#undef $elem

void scl2_shf2_reorder(Scale2 *a, Shift2 *b);
void shf2_scl2_reorder(Shift2 *a, Scale2 *b);

// Scale3

typedef Scale Scale3;

#define $Map Scale3
#define $map scl3
#define $elem real3
#include "interface.inl"
#undef $Map
#undef $map
#undef $elem

void scl3_shf3_reorder(Scale3 *a, Shift3 *b);
void shf3_scl3_reorder(Shift3 *a, Scale3 *b);

#ifndef HOST
#include "scale.cc"
#endif // !HOST
