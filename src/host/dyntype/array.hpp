#pragma once

#include <common/types.hh>
#include "type.hpp"


namespace dyn {

class Array : public Type {
public:
    class Instance : public Type::Instance {
    public:
        TypeBox item_type_; 
        std::vector<InstanceBox> items_;

        Instance() = default;
        Instance(TypeBox &&type) :
            item_type_(std::move(type))
        {}
        template <typename I>
        Instance(I &&item_iter) {
            items_ = item_iter.collect<std::vector>();
            if (items_.size() > 0) {
                item_type_ = items_[0]->type();
            }
            for (auto &i : items_) {
                assert_eq_(item_type_->id(), i->type()->id());
            }
        }

        void append(InstanceBox &&i) {
            if (items_.size() > 0) {
                assert_eq_(item_type_->id(), i->type()->id());
            } else {
                item_type_ = i->type();
            }
        }
        
        TypeBox item_type() {
            return item_type_;
        }
        std::vector<InstanceBox>

        Array type_() const {
            return Array();
        }
        virtual TypeBox type() const override {
            return rstd::Box(type_());
        }
        virtual void store(void *dst) const override {
            assert_eq_((size_t)dst % alignof(dev_type<T>), 0);
            dev_store((dev_type<T> *)dst, &value);
        }
        virtual void load(const void *dst) override {
            *this = type_().load_(dst);
        }
    };

    inline virtual size_t id() const override { return typeid(Primitive<T>).hash_code(); }

    virtual rstd::Option<size_t> size() const override { return rstd::Some(sizeof(dev_type<T>)); }
    virtual size_t align() const override { return alignof(dev_type<T>); }

    Instance load_(const void *src) const {
        assert_eq_((size_t)src % alignof(dev_type<T>), 0);
        T v;
        dev_load(&v, (const dev_type<T> *)src);
        return Instance(v);
    }
    virtual rstd::Box<Type::Instance> load(const void *src) const override {
        return rstd::Box(load_(src));
    }

    virtual std::string name() override { return dev_name<T>; }
    virtual std::string source() override {
        return 
            "#include <common/types.hh>\n"
            "#include <common/algebra/real.hh>\n"
            "#include <common/algebra/vector.hh>\n";
    }
};

} // namespace dyn
