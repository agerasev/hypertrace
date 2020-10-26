#pragma once

#include <rstd.hpp>
#include "tuple.hpp"


namespace dyn {

template <typename T=Type>
class Struct : public dyn::Tuple<T> {
protected:
    typedef dyn::Tuple<T> Base;
    typedef typename dyn::Tuple<T>::Instance BaseInstance;

public:
    typedef typename Base::ItemType ItemType;
    typedef typename Base::ItemInstance ItemInstance;

protected:
    typedef rs::Box<ItemType> ItemTypeBox;
    typedef rs::Box<ItemInstance> ItemInstanceBox;

public:
    class Instance : public BaseInstance {
    public:
        std::vector<std::string> field_names_;

        Instance() = default;
        template <typename I>
        Instance(I &&field_iter) {
            for (auto &&t : field_iter) {
                append(t.template get<0>(), std::move(t.template get<1>()));
            }
        }
        void append(const std::string &str, ItemInstanceBox &&i) {
            BaseInstance::append(std::move(i));
            field_names_.push_back(str);
        }
        
        std::vector<ItemInstanceBox> &fields() { return BaseInstance::fields(); }
        const std::vector<ItemInstanceBox> &fields() const { return BaseInstance::fields(); }

        std::vector<ItemInstanceBox> &field_names() {
            return field_names_;
        }
        const std::vector<ItemInstanceBox> &field_names() const {
            return field_names_;
        }

        Struct type_() const {
            return Struct(
                rs::iter_ref(field_names_).zip(rs::iter_ref(fields()))
                .map([](auto &&nf) { return rs::Tuple<std::string, ItemTypeBox>(rs::clone(*nf.template get<0>()), (*nf.template get<1>())->type()); })
            );
        }
        virtual Struct *_type() const override {
            return new Struct(type_());
        }
        rs::Box<Struct> type() const {
            return rs::Box<Struct>::_from_raw(_type());
        }
    };

private:
    std::vector<std::string> field_names_;

public:
    Struct() = default;
    template <typename I>
    Struct(I &&field_iter) {
        for (auto &&t : field_iter) {
            append(t.template get<0>(), std::move(t.template get<1>()));
        }
    }
    void append(const std::string &str, ItemTypeBox &&i) {
        Base::append(std::move(i));
        field_names_.push_back(str);
    }
    
    std::vector<ItemTypeBox> &fields() { return Base::fields(); }
    const std::vector<ItemTypeBox> &fields() const { return Base::fields(); }

    std::vector<ItemTypeBox> &field_names() {
        return field_names_;
    }
    const std::vector<ItemTypeBox> &field_names() const {
        return field_names_;
    }

    virtual Struct *_clone() const override {
        return new Struct(
            rs::iter_ref(field_names_).zip(rs::iter_ref(fields()))
            .map([](auto &&nf) { return rs::Tuple<std::string, ItemTypeBox>(rs::clone(*nf.template get<0>()), (*nf.template get<1>())->clone()); })
        );
    }
    rs::Box<Struct> clone() const {
        return rs::Box<Struct>::_from_raw(_clone());
    }

    virtual size_t id() const override { 
        rs::DefaultHasher hasher;
        hasher._hash_raw(Base::id());
        hasher._hash_raw(typeid(Struct).hash_code());
        for (const std::string &n : field_names_) {
            hasher.hash(n);
        }
        return hasher.finish();
    }

    Instance load_(const uchar *src) const {
        BaseInstance dtup = Base::load_(src);
        Instance dst;
        for (size_t i = 0; i < field_names_.size(); ++i) {
            dst.append(field_names_[i], std::move(dtup.fields()[i]));
        }
        return dst;
    }
    virtual Instance *_load(const uchar *src) const override {
        return new Instance(load_(src));
    }
    rs::Box<Instance> load(const uchar *src) const {
        return rs::Box<Instance>::_from_raw(_load(src));
    }

    virtual std::string name() const override {
        return format_("Struct{}", id());
    }
    virtual Source source() const override {
        return Base::source_with_names([&](size_t i){ return field_names_[i]; });
    }
};

} // namespace dyn
