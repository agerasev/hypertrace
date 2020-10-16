#pragma once

#include <common/types.hh>
#include <common/object/type.hpp>


template <typename T>
class Primitive : public Type {
public:
    class Instance : public Type::Instance {
    public:
        T value;

        Instance(T v) : value(v) {}

        virtual rstd::Box<Type> type() const override { return rstd::Box(Primitive<T>()); }
        virtual void store(void *dst) const override {
            assert_eq_((size_t)dst % alignof(dev_type<T>), 0);
            dev_store((dev_type<T> *)dst, &value);
        }
    };

    inline virtual size_t id() const override { return typeid(Primitive<T>).hash_code(); }

    virtual size_t size() const override { return sizeof(dev_type<T>); }
    virtual size_t align() const override { return alignof(dev_type<T>); }

    virtual rstd::Box<Instance> load(const void *src) const override {
        assert_eq_((size_t)src % alignof(dev_type<T>), 0);
        T v;
        dev_load(&v, (const dev_type<T> *)src);
        return Instance(v);
    }

    virtual std::string name() { return dev_name<T>; }
    virtual std::string source() {
        return 
            "#include <common/types.hh>\n"
            "#include <common/algebra/real.hh>\n"
            "#include <common/algebra/vector.hh>\n";
    }
};
