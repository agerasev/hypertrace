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


class Black final : public ImplEmptyType<Black, SurfaceMaterial> {
public:
    inline virtual std::string prefix() override { return "black"; };
    inline virtual std::string source() override { return "#include <common/object/material.hh>"; };
};

class Transparent final : public ImplEmptyType<Black, SurfaceMaterial> {
public:
    inline virtual std::string prefix() override { return "transparent"; };
    inline virtual std::string source() override { return "#include <common/object/material.hh>"; };
};

class Specular final : public ImplEmptyType<Black, SurfaceMaterial> {
public:
    inline virtual std::string prefix() override { return "specular"; };
    inline virtual std::string source() override { return "#include <common/object/material.hh>"; };
};

class Lambertian final : public ImplEmptyType<Black, SurfaceMaterial> {
public:
    inline virtual std::string prefix() override { return "lambertian"; };
    inline virtual std::string source() override { return "#include <common/object/material.hh>"; };
};

#endif // HOST

#ifndef HOST
#include "material.cc"
#endif // !HOST
