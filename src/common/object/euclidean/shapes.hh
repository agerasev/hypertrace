#pragma once

#include <common/algebra/vector.hh>
#include <common/geometry/euclidean.hh>
#include <common/render/context.hh>
#include <common/render/light.hh>


typedef void SphereEu; // : Shape

#define SPHEREEU_REPEATED true

real sphereeu_detect(Context *context, real3 *normal, LightEu *light);


typedef void CubeEu; // : Shape

#define CUBEEU_REPEATED true

real cubeeu_detect(Context *context, real3 *normal, LightEu *light);


#ifndef HOST
#include "shapes.cc"
#endif // !HOST
