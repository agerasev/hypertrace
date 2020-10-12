#pragma once

#include <common/algebra/vector.hh>
#include <common/geometry/euclidean.hh>
#include <common/render/context.hh>
#include <common/render/light.hh>


typedef void SphereEu;

#define SPHEREEU_REPEATED true

real sphereeu_detect(Context *context, real3 *normal, LightEu *light);


typedef void Cube;

#define CUBE_REPEATED true

real cube_detect(Context *context, real3 *normal, LightEu *light);


#ifndef HOST
#include "shapes.cc"
#endif // !HOST
