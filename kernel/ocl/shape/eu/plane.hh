#pragma once

#include <geometry/euclidean.hh>
#include <render/light/eu.hh>
#include <shape/primitive.hh>
#include <shape/interface.hh>

typedef Plane PlaneEu;

define_shape_interface(
    PlaneEu, plane_eu,
    Eu, eu
)

#ifndef HOST
#include "plane.cc"
#endif // !HOST
