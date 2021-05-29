#pragma once

#include <geometry/euclidean.hh>
#include <render/light/eu.hh>
#include <shape/primitive.hh>
#include <shape/interface.hh>

typedef Cube CubeEu;

define_shape_interface(
    CubeEu, cube_eu,
    Eu, eu
)

#ifndef HOST
#include "cube.cc"
#endif // !HOST
