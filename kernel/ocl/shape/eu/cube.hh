#pragma once

#include <algebra/vector.hh>
#include <geometry/euclidean.hh>
#include <render/context.hh>
#include <render/light/eu.hh>


typedef void CubeEu;
real cube_eu_detect(__global const CubeEu *shape, Context *context, real3 *normal, RayEu *ray);


#ifndef HOST
#include "cube.cc"
#endif // !HOST
