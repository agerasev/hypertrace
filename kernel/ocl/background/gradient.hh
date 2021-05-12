#pragma once

#include <types.hh>
#include <algebra/color.hh>
#include <geometry/euclidean.hh>
#include <render/light/eu.hh>

typedef struct GradBg {
    real3 direction;
    color3 colors[2];
    float power;
} GradBg;

#define $Self GradBg
#define $self grad_bg
#define $Geo Eu
#define $geo eu
#include "interface.inl"
#undef $Self
#undef $self
#undef $Geo
#undef $geo

#ifndef HOST
#include "gradient.cc"
#endif // HOST
