#pragma once

#include <algebra/vector.hh>

// Shift2

typedef struct {
    real2 v;
} Shift2;

#define $Map Shift2
#define $map shf2
#define $elem real2
#include "interface.inl"
#undef $Map
#undef $map
#undef $elem

#ifdef HOST
template <>
struct Distance<Shift2> {
    static real distance(Shift2 a, Shift2 b) {
        return ::distance(a.v, b.v);
    }
};
#endif // HOST

// Shift3

typedef struct {
    real3 v;
} Shift3;

#define $Map Shift3
#define $map shf3
#define $elem real3
#include "interface.inl"
#undef $Map
#undef $map
#undef $elem

#ifdef HOST
template <>
struct Distance<Shift3> {
    static real distance(Shift3 a, Shift3 b) {
        return ::distance(a.v, b.v);
    }
};
#endif // HOST


#ifndef HOST
#include "rotation.cc"
#endif // !HOST
