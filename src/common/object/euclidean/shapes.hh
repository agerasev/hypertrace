#pragma once

#include <common/algebra/vector.hh>
#include <common/geometry/euclidean.hh>
#include <common/render/context.hh>
#include <common/render/light.hh>


typedef void Sphere;

#define SPHERE_REPEATED true

real sphere_detect(Context *context, real3 *normal, LightEu *light);


typedef void Cube;

#define CUBE_REPEATED true

real cube_detect(Context *context, real3 *normal, LightEu *light);


#ifndef HOST
#include "shapes.cc"
#endif // !HOST
