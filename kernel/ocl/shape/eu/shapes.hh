#pragma once

#include <algebra/vector.hh>
#include <geometry/euclidean.hh>
#include <render/context.hh>
#include <render/light.hh>


typedef void SphereEu;
real sphere_eu_detect(__global const SphereEu *shape, Context *context, real3 *normal, LightEu *light);

typedef void CubeEu;
real cube_eu_detect(__global const CubeEu *shape, Context *context, real3 *normal, LightEu *light);


#ifndef HOST
#include "shapes.cc"
#endif // !HOST
