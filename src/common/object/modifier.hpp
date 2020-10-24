#pragma once

#include "material.hpp"
#include "material.hh"
#include <host/dyntype/primitive.hpp>
#include <host/dyntype/static_struct.hpp>


template <typename Self, typename Base, typename D>
class ImplModifier : public Base {
public:
    typedef Base BaseType;
    typedef typename Base::Instance BaseInstance;
    typedef rstd::Box<BaseType> BaseTypeBox;
    typedef rstd::Box<BaseInstance> BaseInstanceBox;
    typedef dyn::StaticStruct<Base, dyn::Primitive<D>> InnerType;
    typedef typename InnerType::Instance InnerInstance;


    class Instance : public BaseInstance {
    private:
        InnerInstance content_;

    public:
        Instance(BaseInstanceBox &&inner, D data) :
            content_(
                rstd::Tuple(std::string("inner"), std::move(inner)),    
                rstd::Tuple(std::string("data"), dyn::Primitive<D>::Instance(data))
            )
        {}
        explicit Instance(InnerInstance &&cont) : content_(cont) {}

        BaseInstanceBox &inner() { content_.fields().template get<0>(); }
        const BaseInstanceBox &inner() const { content_.fields().template get<0>(); }

        D &data() { content_.fields().template get<1>()->value; }
        const D &data() const { content_.fields().template get<1>()->value; }

        Self type_() const {
            return Self(content_.fields().template get<0>()->type());
        }
        virtual Base *_type() const override { return new Self(_type()); }
        
        virtual void load(const uchar *src) override {
            content_.load(src);
        }
        virtual void store(uchar *dst) const override {
            content_.store(dst);
        }
    };
private:
    InnerType content_;

public:
    ImplModifier(BaseTypeBox &&inner) :
        content_(
            rstd::Tuple(std::string("inner"), std::move(inner)),    
            rstd::Tuple(std::string("data"), dyn::Primitive<D>())
        )
    {}
    const BaseTypeBox &inner() const { return content_.fields().template get<0>(); }
    virtual Base *_clone() const override { return new Self(inner()->clone()); }

    virtual size_t id() const override { return typeid(Self).hash_code(); }

    virtual size_t align() const override { return content_.align(); };
    virtual rstd::Option<size_t> size() const override { return content_.size(); };

    Instance load_(const uchar *src) const {
        return Include(content_.load_(src));
    }
    virtual Instance *_load(const uchar *src) const override {
        return new Instance(load_(src));
    }
    rstd::Box<Instance> load(const uchar *src) const {
        return rstd::Box<Instance>::_from_raw(_load(src));
    }

    virtual std::string modifier_code() const = 0;
    virtual dyn::Source source() const override {
        std::string lname = rstd::to_lower(this->name());
        std::string fname = format_("generated/object/material_{}.hxx", lname);
        dyn::Source src = content_.source();
        
        std::string inner_elem;
        if (inner()->size().unwrap() > 0) {
            inner_elem = "&material->inner";
        } else {
            inner_elem = "NULL";
        }
        
        src.insert(fname, format_(
            "#include <{}>\n"
            "typedef {} {}\n"
            "\n"
            "bool {}_interact(__global const {} *material, Context *context, real3 normal, LightLocal *light, float3 *emission) {{\n"
            "{}"
            "    return {}_interact({}, context, normal, light, emission);\n"
            "}}\n",
            src.name(),
            content_.name(), this->name(),
            lname,
            modifier_code(),
            rstd::to_lower(inner()->name()), inner_elem
        )).unwrap();
        src.set_name(fname);
        return src;
    }
};

class Colored final : public ImplModifier<Colored, Material, float3> {
    virtual std::string name() const override {
        return format_("Colored{}", inner()->name());
    }
    virtual std::string modifier_code() const override {
        return std::string(
            "    light->base->intensity *= material->data;\n"
        );
    }
};

class Emissive final : public ImplModifier<Emissive, Material, float3> {
    virtual std::string name() const override {
        return format_("Emissive{}", inner()->name());
    }
    virtual std::string modifier_code() const override {
        return std::string(
            "    *emission += light->base->intensity * material->data;\n"
        );
    }
};
