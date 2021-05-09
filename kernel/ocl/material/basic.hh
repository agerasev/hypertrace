#pragma once

#include <types.hh>
#include <algebra/vector.hh>
#include <random.hh>
#include <render/light/base.hh>
#include <render/context.hh>

typedef Empty Absorbing;
typedef Empty Transparent;
typedef Empty Specular;
typedef Empty Lambertian;

bool absorbing_interact  (__global const Absorbing   *material, Context *context, real3 normal, LightLocal *light, float3 *emission);
bool transparent_interact(__global const Transparent *material, Context *context, real3 normal, LightLocal *light, float3 *emission);
bool specular_interact   (__global const Specular    *material, Context *context, real3 normal, LightLocal *light, float3 *emission);
bool lambertian_interact (__global const Lambertian  *material, Context *context, real3 normal, LightLocal *light, float3 *emission);

#ifndef HOST
#include "basic.cc"
#endif // !HOST
