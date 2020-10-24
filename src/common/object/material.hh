#pragma once

#include <common/algebra/vector.hh>
#include <common/random.hh>
#include <common/render/light.hh>
#include <common/render/context.hh>

#ifndef DYNTYPE

typedef void Absorbing;
bool absorbing_interact(__global const Absorbing *material, Context *context, real3 normal, LightLocal *light, float3 *emission);

typedef void Transparent;
bool transparent_interact(__global const Transparent *material, Context *context, real3 normal, LightLocal *light, float3 *emission);

typedef void Specular;
bool specular_interact(__global const Specular *material, Context *context, real3 normal, LightLocal *light, float3 *emission);

typedef void Lambertian;
bool lambertian_interact(__global const Lambertian *material, Context *context, real3 normal, LightLocal *light, float3 *emission);

#else // DYNTYPE

#include "material.hpp"


class Absorbing final : public dyn::ImplEmptyType<Absorbing, SurfaceMaterial> {
public:
    inline virtual std::string name() const override { return "Absorbing"; };
    inline virtual dyn::Source source() const override { return dyn::Source("common/object/material.hh"); };
};

class Transparent final : public dyn::ImplEmptyType<Transparent, SurfaceMaterial> {
public:
    inline virtual std::string name() const override { return "Transparent"; };
    inline virtual dyn::Source source() const override { return dyn::Source("common/object/material.hh"); };
};

class Specular final : public dyn::ImplEmptyType<Specular, SurfaceMaterial> {
public:
    inline virtual std::string name() const override { return "Specular"; };
    inline virtual dyn::Source source() const override { return dyn::Source("common/object/material.hh"); };
};

class Lambertian final : public dyn::ImplEmptyType<Lambertian, SurfaceMaterial> {
public:
    inline virtual std::string name() const override { return "Lambertian"; };
    inline virtual dyn::Source source() const override { return dyn::Source("common/object/material.hh"); };
};

#endif // !DYNTYPE


#ifndef HOST
#include "material.cc"
#endif // !HOST
