#pragma once

#include "material.hpp"
#include "material.hh"


template <typename Self, typename Base, typename Data>
class ImplModifier : public Base {
public:
    typedef typename Base::Instance BaseInstance;
    typedef rstd::Box<Base> BaseBox;
    typedef rstd::Box<BaseInstance> BaseInstanceBox;

    class Instance : public BaseInstance {
    public:
        BaseInstanceBox inner;
        Data data;

        virtual Base *_type() const override { return new Self(inner->type()); }
        
        virtual void load(const void *src) const override = 0;
        virtual void store(void *dst) const override = 0;
    };
private:
    BaseBox inner_;

public:
    ImplModifier(BaseBox &&inner) : inner_(std::move(inner)) {}
    const BaseBox &inner() const { return inner_; }

    virtual Base *_clone() const override { return new Self(inner_->clone()); }
    BaseBox clone() const { return BaseBox::_from_raw(_clone()); }

    virtual size_t id() const override { return typeid(Self).hash_code(); }

    virtual size_t align() const override {
        return std::max(inner_->align(), alignof(dev_type<Data>));
    };
    virtual rstd::Option<size_t> size() const override {
        return 
            upper_multiple(align(), inner_->size()) +
            upper_multiple(align(), sizeof(dev_type<Data>));
    };

    Instance load_(const uchar *src) const {
        
    }
    virtual BaseInstance *_load(const uchar *src) const override {
        return new Instance(load_());
    }
    BaseInstanceBox load(const uchar *src) const { return BaseInstanceBox::_from_raw(_load(src)); }

    virtual std::string name() const = 0;
    virtual Source source() const = 0;

    virtual std::string name() { return EmptyType::name(); }
    virtual std::string source() override { return "#include <common/object/material.hh>"; };

    virtual rstd::Box<::Instance> instance() const { return rstd::Box(Instance()); }
};

/*
class Colored final : public virtual Material {
public:
    struct Instance : public Material::Instance {
        float3 color;
        rstd::Box<Material::Instance> inner;

        virtual rstd::Box<Type> type() const { return rstd::Box(Colored(inner->type())); }
    };
private:
    rstd::Box<Material> inner_;
public:
    Colored(rstd::Box<Material> &&inner) : inner_(std::move(inner)) {}
    const Material &inner() const { return inner_.get(); }
};

class Emissive final : public virtual Material {
private:
    rstd::Box<Material> inner_;
public:
    Emissive(rstd::Box<Material> &&inner) : inner_(std::move(inner)) {}
    const Material &inner() const { return inner_.get(); }
};

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
