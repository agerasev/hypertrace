#pragma once

#include <types.hh>
#include <algebra/color.hh>
#include <geometry/euclidean.hh>
#include <render/light/eu.hh>
#include "interface.hh"

typedef struct GradBg {
    real3 direction;
    color3 colors[2];
    float power;
} GradBg;

define_background_interface(
    GradBg, grad_bg,
    Eu, eu
)

#ifndef HOST
#include "gradient.cc"
#endif // HOST
