#pragma once

#include <algebra/vector.hh>

// Shift2

typedef struct {
    real2 v;
} Shift2;

#define $Map Shift2
#define $map shf2
#define $elem real2
#include "derive.inl"
#undef $Map
#undef $map
#undef $elem

// Shift3

typedef struct {
    real3 v;
} Shift3;

#define $Map Shift3
#define $map shf3
#define $elem real3
#include "derive.inl"
#undef $Map
#undef $map
#undef $elem


#ifndef HOST
#include "rotation.cc"
#endif // !HOST
