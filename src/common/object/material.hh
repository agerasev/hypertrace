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

template <typename Self>
class EmptySurfaceMaterial : public virtual SurfaceMaterial, public virtual EmptyType {
    class Instance final : public virtual Material::Instance, public virtual EmptyInstance {
    public:
        inline virtual rstd::Box<Type> type() const { return rstd::Box(Self()); }
    };

    inline virtual size_t id() const override { return typeid(Self).hash_code(); }

    inline virtual bool empty() const override { return EmptyType::empty(); }
    inline virtual size_t size() const override { return EmptyType::size(); }
    inline virtual size_t align() const override { return EmptyType::align(); }
    inline virtual rstd::Box<::Instance> load(const void *) const { return EmptyType::load(); }

    virtual std::string name() { return EmptyType::name(); }
    inline virtual std::string source() override { return "#include <common/object/material.hh>"; };

    inline virtual rstd::Box<::Instance> instance() const { return rstd::Box(Instance()); }
};

class Black final : public virtual EmptySurfaceMaterial<Black> {
public:
    inline virtual std::string prefix() override { return "black"; };
};

class Transparent final : public virtual EmptySurfaceMaterial<Transparent> {
public:
    virtual std::string prefix() override { return "transparent"; };
};

class Specular final : public virtual EmptySurfaceMaterial<Specular> {
public:
    virtual std::string prefix() override { return "specular"; };
};

class Lambertian final : public virtual EmptySurfaceMaterial<Lambertian> {
public:
    virtual std::string prefix() override { return "lambertian"; };
};

#endif // HOST

#ifndef HOST
#include "material.cc"
#endif // !HOST
