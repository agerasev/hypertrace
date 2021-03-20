#pragma once

#include <algebra/vector.hh>
#include <random.hh>
#include <render/light.hh>
#include <render/context.hh>

typedef void Absorbing;
typedef void Transparent;
typedef void Specular;
typedef void Lambertian;

bool absorbing_interact  (__global const Absorbing   *material, Context *context, real3 normal, LightLocal *light, float3 *emission);
bool transparent_interact(__global const Transparent *material, Context *context, real3 normal, LightLocal *light, float3 *emission);
bool specular_interact   (__global const Specular    *material, Context *context, real3 normal, LightLocal *light, float3 *emission);
bool lambertian_interact (__global const Lambertian  *material, Context *context, real3 normal, LightLocal *light, float3 *emission);

#ifndef HOST
#include "material.cc"
#endif // !HOST
