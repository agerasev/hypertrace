#pragma once

#include "material.hpp"
#include "material.hh"
#include <host/dyntype/primitive.hpp>
#include <host/dyntype/struct.hpp>


template <typename Self, typename Base, typename D>
class ImplModifier : public Base {
public:
    typedef Base BaseType;
    typedef typename Base::Instance BaseInstance;
    typedef rstd::Box<BaseType> BaseTypeBox;
    typedef rstd::Box<BaseInstance> BaseInstanceBox;

    class Instance : public BaseInstance {
    public:
        dyn::Struct<>::Instance content;

        Instance(BaseInstanceBox &&inner) {
            content.append("inner", std::move(inner));
            content.append("data", dyn::Primitive<D>::Instance());
        }

        Self type_() const {
            return Self(content.fields[i]->type()->clone().template downcast<BaseInstance>().unwrap());
        }
        virtual Self *_type() const override { return new Self(_type()); }
        
        virtual void load(const uchar *src) const override {
            content.load(src);
        }
        virtual void store(uchar *dst) const override {
            content.store(dst);
        }
    };
private:
    dyn::Struct<> struct_;

public:
    ImplModifier(BaseTypeBox &&inner) {
        struct_.append("inner", std::move(inner));
        struct_.append("data", dyn::Primitive<D>());
    }
    const BaseTypeBox &inner() const { return inner_; }

    virtual Self *_clone() const override { return new Self(inner_->clone()); }
    rstd::Box<Self> clone() const { return rstd::Box<Self>::_from_raw(_clone()); }

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
        Instance inst;
        inst.inner = inner_->load(src);
        dev_load<Data>(
            &inst.data,
            (const dev_type<Data> *)(src + upper_multiple(align(), inner_->size()))
        );
        return inst;
    }
    virtual Instance *_load(const uchar *src) const override {
        return new Instance(load_(src));
    }
    rstd::Box<Instance> load(const uchar *src) const {
        return rstd::Box<Instance>::_from_raw(_load(src));
    }
};

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

/*
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
