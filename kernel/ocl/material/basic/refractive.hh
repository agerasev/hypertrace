#pragma once

#include <algebra/real.hh>

typedef struct Refractive {
    real index;
} Refractive;

#define $Self Refractive
#define $self refractive
#include <material/interface.inl>
#undef $Self
#undef $self

#ifndef HOST
#include "refractive.cc"
#endif // !HOST
