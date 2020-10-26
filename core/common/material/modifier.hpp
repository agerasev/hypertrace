#pragma once

#include "material.hpp"
#include "material.hh"
#include <host/dyntype/primitive.hpp>
#include <host/dyntype/static_struct.hpp>


template <typename Self, typename Base, typename Data>
class ImplMaterialModifier : public Base {
public:
    typedef Base BaseType;
    typedef typename Base::Instance BaseInstance;
    typedef rs::Box<BaseType> BaseTypeBox;
    typedef rs::Box<BaseInstance> BaseInstanceBox;

    typedef dyn::Primitive<Data> DataType;
    typedef typename dyn::Primitive<Data>::Instance DataInstance;
    typedef rs::Box<DataType> DataTypeBox;
    typedef rs::Box<DataInstance> DataInstanceBox;

    typedef dyn::StaticStruct<BaseType, DataType> InnerType;
    typedef typename InnerType::Instance InnerInstance;


    class Instance : public BaseInstance {
    private:
        InnerInstance content_;

    public:
        Instance(BaseInstanceBox &&inner, DataInstanceBox &&data) :
            content_(
                rs::Tuple(std::string("inner"), std::move(inner)),    
                rs::Tuple(std::string("data"), std::move(data))
            )
        {}
        explicit Instance(InnerInstance &&cont) : content_(std::move(cont)) {}

        BaseInstance &inner() { return content_.fields().template get<0>().get(); }
        const BaseInstance &inner() const { return content_.fields().template get<0>().get(); }

        DataInstance &data() { return content_.fields().template get<1>().get(); }
        const DataInstance &data() const { return content_.fields().template get<1>().get(); }

        Self type_() const {
            return Self(content_.type_());
        }
        virtual Base *_type() const override { return new Self(type_()); }
        
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
    ImplMaterialModifier(BaseTypeBox &&inner, DataTypeBox &&data) :
        content_(
            rs::Tuple(std::string("inner"), std::move(inner)),
            rs::Tuple(std::string("data"), std::move(data))
        )
    {}
    ImplMaterialModifier(InnerType &&cont) :
        content_(std::move(cont))
    {}

    BaseType &inner() { return content_.fields().template get<0>().get(); }
    const BaseType &inner() const { return content_.fields().template get<0>().get(); }

    DataType &data() { return content_.fields().template get<1>().get(); }
    const DataType &data() const { return content_.fields().template get<1>().get(); }

    virtual Base *_clone() const override { return new Self(content_.clone_()); }

    virtual size_t id() const override {
        rs::DefaultHasher hasher;
        hasher._hash_raw(typeid(Self).hash_code());
        hasher._hash_raw(content_.id());
        return hasher.finish();
    }

    virtual size_t align() const override { return content_.align(); };
    virtual rs::Option<size_t> size() const override { return content_.size(); };

    Instance load_(const uchar *src) const {
        return Instance(content_.load_(src));
    }
    virtual Instance *_load(const uchar *src) const override {
        return new Instance(load_(src));
    }
    rs::Box<Instance> load(const uchar *src) const {
        return rs::Box<Instance>::_from_raw(_load(src));
    }

    virtual std::string modifier_code() const = 0;
    virtual dyn::Source source() const override {
        std::string lname = this->prefix();
        std::string fname = format_("generated/object/material_{}.hxx", lname);
        dyn::Source src = content_.source();
        
        std::string inner_elem;
        if (inner().size().unwrap() > 0) {
            inner_elem = "&material->inner";
        } else {
            inner_elem = "NULL";
        }
        
        src.insert(fname, format_(
            "#include <{}>\n"
            "typedef {} {};\n"
            "\n"
            "bool {}_interact(__global const {} *material, Context *context, real3 normal, LightLocal *light, float3 *emission) {{\n"
            "{}"
            "    return {}_interact({}, context, normal, light, emission);\n"
            "}}\n",
            src.name(),
            content_.name(), this->name(),
            lname, this->name(),
            modifier_code(),
            inner().prefix(), inner_elem
        )).unwrap();
        src.set_name(fname);
        return src;
    }
};

class Colored final : public ImplMaterialModifier<Colored, Material, float3> {
public:
    Colored() = default;
    explicit Colored(rs::Box<Material> &&inner, rs::Box<dyn::Primitive<float3>> &&data) :
        ImplMaterialModifier<Colored, Material, float3>(std::move(inner), std::move(data))
    {}
    Colored(dyn::StaticStruct<Material, dyn::Primitive<float3>> &&content) :
        ImplMaterialModifier<Colored, Material, float3>(std::move(content))
    {}
    virtual std::string name() const override {
        return format_("Colored{}", id());
    }
    virtual std::string modifier_code() const override {
        return std::string(
            "    light->base.intensity *= material->data;\n"
        );
    }
};

class Emissive final : public ImplMaterialModifier<Emissive, Material, float3> {
public:
    Emissive() = default;
    explicit Emissive(rs::Box<Material> &&inner, rs::Box<dyn::Primitive<float3>> &&data) :
        ImplMaterialModifier<Emissive, Material, float3>(std::move(inner), std::move(data))
    {}
    Emissive(dyn::StaticStruct<Material, dyn::Primitive<float3>> &&content) :
        ImplMaterialModifier<Emissive, Material, float3>(std::move(content))
    {}
    virtual std::string name() const override {
        return format_("Emissive{}", id());
    }
    virtual std::string modifier_code() const override {
        return std::string(
            "    *emission += light->base.intensity * material->data;\n"
        );
    }
};
