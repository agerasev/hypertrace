#pragma once

#include "material.hpp"
#include "material.hh"


template <typename Self>
class MaterialModifier final : public virtual Material {
public:
    struct Instance : public Material::Instance {
        rstd::Box<Material::Instance> inner;
        float3 color;

        inline virtual rstd::Box<Type> type() const { return rstd::Box(Self(inner->type())); }
    };
private:
    rstd::Box<Material> inner_;
public:
    Colored(rstd::Box<Material> &&inner) : inner_(std::move(inner)) {}
    inline const Material &inner() const { return inner_.get(); }

    inline virtual size_t id() const override { return typeid(Self).hash_code(); }

    inline virtual bool empty() const override { return false; }
    inline virtual size_t size() const override { return ; }
    inline virtual size_t align() const override { return std::max(inner().align(), alignof(float3)); }
    inline virtual rstd::Box<::Instance> load(const void *) const { return EmptyType::load(); }

    virtual std::string name() { return EmptyType::name(); }
    inline virtual std::string source() override { return "#include <common/object/material.hh>"; };

    inline virtual rstd::Box<::Instance> instance() const { return rstd::Box(Instance()); }
};

class Colored final : public virtual Material {
public:
    struct Instance : public Material::Instance {
        float3 color;
        rstd::Box<Material::Instance> inner;

        inline virtual rstd::Box<Type> type() const { return rstd::Box(Colored(inner->type())); }
    };
private:
    rstd::Box<Material> inner_;
public:
    Colored(rstd::Box<Material> &&inner) : inner_(std::move(inner)) {}
    inline const Material &inner() const { return inner_.get(); }
};

class Emissive final : public virtual Material {
private:
    rstd::Box<Material> inner_;
public:
    Emissive(rstd::Box<Material> &&inner) : inner_(std::move(inner)) {}
    inline const Material &inner() const { return inner_.get(); }
};

/*
template <typename M, bool Z=is_empty<M>()>
class Colored {
// : public Material
public:
    M base;
    float3 color;

    Colored() = default;
    Colored(float3 v, const M &b) : base(b), color(v) {}

    template <typename Context>
    bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) const {
        light.intensity *= color;
        return base.interact(context, normal, light, emission);
    }
};
template <typename M>
class Colored<M, true> {
// : public Material
public:
    float3 color;

    Colored() = default;
    Colored(float3 v) : color(v) {}

    template <typename Context>
    bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) const {
        light.intensity *= color;
        return M::interact(context, normal, light, emission);
    }
};

template <typename M, bool Z=is_empty<M>()>
class Emissive {
// : public Material
public:
    M base;
    float3 intensity;

    Emissive() = default;
    Emissive(float3 v, const M &b) : base(b), intensity(v) {}

    template <typename Context>
    bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) const {
        emission += intensity*light.intensity;
        return base.interact(context, normal, light, emission);
    }
};
template <typename M>
class Emissive<M, true> {
// : public Material
public:
    float3 intensity;

    Emissive() = default;
    Emissive(float3 v) : intensity(v) {}

    template <typename Context>
    bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) const {
        emission += intensity*light.intensity;
        return M::interact(context, normal, light, emission);
    }
};
*/
