#pragma once

#include <common/types.hh>
#include "type.hpp"


namespace dyn {

template <typename T>
class Primitive : public Type {
public:
    class Instance : public Type::Instance {
    public:
        T value;

        Instance() = default;
        Instance(T v) : value(v) {}

        Primitive type_() const {
            return Primitive();
        }
        virtual Type *_type() const override {
            return new Primitive(type_());
        }
        virtual void store(uchar *dst) const override {
            assert_eq_((size_t)dst % alignof(dev_type<T>), 0);
            dev_store((dev_type<T> *)dst, &value);
        }
        virtual void load(const uchar *dst) override {
            *this = type_().load_(dst);
        }
    };

    virtual Type *_clone() const override { return new Primitive<T>(); }

    virtual size_t id() const override { return typeid(Primitive<T>).hash_code(); }

    virtual rstd::Option<size_t> size() const override { return rstd::Some(sizeof(dev_type<T>)); }
    virtual size_t align() const override { return alignof(dev_type<T>); }

    Instance load_(const uchar *src) const {
        assert_eq_((size_t)src % alignof(dev_type<T>), 0);
        T v;
        dev_load(&v, (const dev_type<T> *)src);
        return Instance(v);
    }
    virtual Type::Instance *_load(const uchar *src) const override {
        return new Instance(load_(src));
    }

    virtual std::string name() const override { return dev_name<T>; }
    virtual Source source() const override {
        return Source("common/algebra/vector.hh", {});
    }
};

} // namespace dyn
