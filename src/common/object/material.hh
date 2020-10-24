#pragma once

#include <common/algebra/vector.hh>
#include <common/random.hh>
#include <common/render/light.hh>
#include <common/render/context.hh>


typedef void _Black;
bool black_interact(__global const _Black *material, Context *context, real3 normal, LightLocal *light, float3 *emission);

typedef void _Transparent;
bool transparent_interact(__global const _Transparent *material, Context *context, real3 normal, LightLocal *light, float3 *emission);

typedef void _Specular;
bool specular_interact(__global const _Specular *material, Context *context, real3 normal, LightLocal *light, float3 *emission);

typedef void _Lambertian;
bool lambertian_interact(__global const _Lambertian *material, Context *context, real3 normal, LightLocal *light, float3 *emission);


#ifdef HOST

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

#else // !HOST

typedef _Black Black;
typedef _Transparent Transparent;
typedef _Specular Specular;
typedef _Lambertian Lambertian;

#endif // HOST


#ifndef HOST
#include "material.cc"
#endif // !HOST
