#pragma once

#include <common/algebra/vector.hh>
#include <common/random.hh>
#include <common/render/light.hh>
#include <common/render/context.hh>

#ifndef DYNTYPE

typedef void Black;
bool black_interact(__global const Black *material, Context *context, real3 normal, LightLocal *light, float3 *emission);

typedef void Transparent;
bool transparent_interact(__global const Transparent *material, Context *context, real3 normal, LightLocal *light, float3 *emission);

typedef void Specular;
bool specular_interact(__global const Specular *material, Context *context, real3 normal, LightLocal *light, float3 *emission);

typedef void Lambertian;
bool lambertian_interact(__global const Lambertian *material, Context *context, real3 normal, LightLocal *light, float3 *emission);

#else // DYNTYPE

#include "material.hpp"


class Black final : public dyn::ImplEmptyType<Black, SurfaceMaterial> {
public:
    inline virtual std::string name() const override { return "Black"; };
    inline virtual dyn::Source source() const override { return dyn::Source("common/object/material.hh"); };
};

class Transparent final : public dyn::ImplEmptyType<Black, SurfaceMaterial> {
public:
    inline virtual std::string name() const override { return "Transparent"; };
    inline virtual dyn::Source source() const override { return dyn::Source("common/object/material.hh"); };
};

class Specular final : public dyn::ImplEmptyType<Black, SurfaceMaterial> {
public:
    inline virtual std::string name() const override { return "Specular"; };
    inline virtual dyn::Source source() const override { return dyn::Source("common/object/material.hh"); };
};

class Lambertian final : public dyn::ImplEmptyType<Black, SurfaceMaterial> {
public:
    inline virtual std::string name() const override { return "Lambertian"; };
    inline virtual dyn::Source source() const override { return dyn::Source("common/object/material.hh"); };
};

#endif // !DYNTYPE


#ifndef HOST
#include "material.cc"
#endif // !HOST
