#pragma once

#include "vector.hh"


typedef float3 color3;
typedef float4 color4;

color3 color_new_gamma(float3 c, float g);

#define color_new(x) color_new_gamma((x), 2.2f)

color3 color_from_uint(uint c);
