#pragma once

#include <geometry/euclidean.hh>
#include <render/light/eu.hh>
#include <shape/primitive.hh>
#include <shape/interface.hh>

typedef Sphere SphereEu;

define_shape_interface(
    SphereEu, sphere_eu,
    Eu, eu
)

#ifndef HOST
#include "sphere.cc"
#endif // !HOST
