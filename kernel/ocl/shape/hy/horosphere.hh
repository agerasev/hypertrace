#pragma once

#include <geometry/hyperbolic.hh>
#include <render/light/hy.hh>
#include <shape/interface.hh>

typedef void Horosphere;

define_shape_interface(
    Horosphere, horosphere,
    Hy, hy
)

#ifndef HOST
#include "horosphere.cc"
#endif // !HOST
