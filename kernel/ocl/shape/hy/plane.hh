#pragma once

#include <geometry/hyperbolic.hh>
#include <render/light/hy.hh>
#include <shape/primitive.hh>
#include <shape/interface.hh>

typedef Plane PlaneHy;

define_shape_interface(
    PlaneHy, plane_hy,
    Hy, hy
)

#ifndef HOST
#include "plane.cc"
#endif // !HOST
