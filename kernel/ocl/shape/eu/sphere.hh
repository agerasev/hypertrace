#pragma once

#include <algebra/vector.hh>
#include <geometry/euclidean.hh>
#include <render/context.hh>
#include <render/light/eu.hh>


typedef void SphereEu;
real sphere_eu_detect(__global const SphereEu *shape, Context *context, real3 *normal, RayEu *ray);


#ifndef HOST
#include "sphere.cc"
#endif // !HOST
