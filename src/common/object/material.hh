#pragma once

#include <common/algebra/vector.hh>
#include <common/random.hh>
#include <common/render/light.hh>
#include <common/render/context.hh>


bool black_interact(Context *context, real3 normal, LightLocal *light, float3 *emission);
bool transparent_interact(Context *context, real3 normal, LightLocal *light, float3 *emission);
bool specular_interact(Context *context, real3 normal, LightLocal *light, float3 *emission);
bool lambertian_interact(Context *context, real3 normal, LightLocal *light, float3 *emission);


#ifdef HOST

#include "material.hpp"


class Black final : public dyn::ImplEmptyType<Black, SurfaceMaterial> {
public:
    inline virtual std::string prefix() const override { return "black"; };
    inline virtual std::string source() const override { return "#include <common/object/material.hh>"; };
};

class Transparent final : public dyn::ImplEmptyType<Black, SurfaceMaterial> {
public:
    inline virtual std::string prefix() const override { return "transparent"; };
    inline virtual std::string source() const override { return "#include <common/object/material.hh>"; };
};

class Specular final : public dyn::ImplEmptyType<Black, SurfaceMaterial> {
public:
    inline virtual std::string prefix() const override { return "specular"; };
    inline virtual std::string source() const override { return "#include <common/object/material.hh>"; };
};

class Lambertian final : public dyn::ImplEmptyType<Black, SurfaceMaterial> {
public:
    inline virtual std::string prefix() const override { return "lambertian"; };
    inline virtual std::string source() const override { return "#include <common/object/material.hh>"; };
};

#endif // HOST

#ifndef HOST
#include "material.cc"
#endif // !HOST
