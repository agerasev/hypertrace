#pragma once

#include <rstd.hpp>
#include <common/types.hh>
#include "type.hpp"


namespace dyn {

template <typename T=Type>
class Variant : public Type {
public:
    typedef T ItemType;
    typedef typename T::Instance ItemInstance;

private:
    typedef rs::Box<ItemType> ItemTypeBox;
    typedef rs::Box<ItemInstance> ItemInstanceBox;

public:
    class Instance : public Type::Instance {
    public:
        std::vector<ItemTypeBox> types_;
        rs::Option<size_t> idx_ = rs::None();
        ItemInstanceBox value_;

        Instance() = default;
        template <typename I>
        Instance(I &&type_iter) :
            types_(type_iter.template collect<std::vector>())
        {}
        template <typename I>
        Instance(I &&type_iter, size_t i, InstanceBox &&v) :
            Instance(std::move(type_iter))
        {
            set_value(i, std::move(v));
        }

        void append_vartype(ItemTypeBox &&i) {
            types_.push_back(std::move(i));
        }
        
        std::vector<ItemTypeBox> &vartypes() {
            return types_;
        }
        const std::vector<ItemTypeBox> &vartypes() const {
            return types_;
        }

        void set_value(size_t i, ItemInstanceBox &&v) {
            assert_(i < types_.size());
            assert_(types_[i]->id() == v->type()->id());
            idx_ = rs::Some(i);
            value_ = std::move(v);
        }
        size_t index() const {
            return idx_.get();
        }
        ItemInstanceBox &value() {
            return value_;
        }
        const ItemInstanceBox &value() const {
            return value_;
        }

        Variant type_() const {
            return Variant(iter_ref(types_).map([](const TypeBox *f) { return (*f)->clone(); }));
        }
        virtual Variant *_type() const override {
            return new Variant(type_());
        }
        rs::Box<Variant> type() const {
            return rs::Box<Variant>::_from_raw(_type());
        }

        virtual void store(uchar *dst) const override {
            assert_eq_((size_t)dst % alignof(dev_type<ulong>), 0);
            ulong i = index();
            dev_store((dev_type<ulong> *)dst, &i);
            value_->store(dst + upper_multiple(type_().align(), sizeof(dev_type<ulong>)));
        }
        virtual void load(const uchar *dst) override {
            *this = type_().load_(dst);
        }
    };

private:
    std::vector<ItemTypeBox> types_;

public:
    Variant() = default;
    template <typename I>
    Variant(I &&type_iter) {
        types_ = type_iter.template collect<std::vector>();
    }

    void append_vartype(ItemTypeBox &&i) {
        types_.push_back(std::move(i));
    }
    
    std::vector<ItemTypeBox> &vartypes() {
        return types_;
    }
    const std::vector<ItemTypeBox> &vartypes() const {
        return types_;
    }

    virtual Variant *_clone() const override {
        return new Variant(iter_ref(types_).map([](const TypeBox *f) { return (*f)->clone(); }));
    }
    rs::Box<Variant> clone() const {
        return rs::Box<Variant>::_from_raw(_clone());
    }

    virtual size_t id() const override { 
        rs::DefaultHasher hasher;
        hasher._hash_raw(typeid(Variant).hash_code());
        for (const TypeBox &f : types_) {
            hasher._hash_raw(f->id());
        }
        return hasher.finish();
    }

    virtual rs::Option<size_t> size() const override {
        return rs::Some(
            upper_multiple(align(), sizeof(dev_type<ulong>)) +
            upper_multiple(align(),
                iter_ref(types_)
                .map([](const TypeBox *f) { return (*f)->size().unwrap(); })
                .max().unwrap()
            )
        );
    }
    virtual size_t align() const override {
        return std::max(alignof(dev_type<ulong>),
            iter_ref(types_)
            .map([](const TypeBox *f) { return (*f)->align(); })
            .max().unwrap()
        );
    }

    Instance instance_() const {
        return Instance(iter_ref(types_).map([](const TypeBox *f) { return (*f)->clone(); }));
    }
    Instance load_(const uchar *src) const {
        Instance dst = instance_();
        assert_eq_((size_t)src % alignof(dev_type<ulong>), 0);
        ulong i;
        dev_load(&i, (const dev_type<ulong> *)src);
        assert_(i < types_.size());
        InstanceBox v = types_[i]->load(src + upper_multiple(align(), sizeof(dev_type<ulong>)));
        dst.set_value(i, std::move(v));
        return dst;
    }
    virtual Instance *_load(const uchar *src) const override {
        return new Instance(load_(src));
    }
    rs::Box<Instance> load(const uchar *src) const {
        return rs::Box<Instance>::_from_raw(_load(src));
    }

    virtual std::string name() const override {
        return format_("Variant{}", id());
    }
    template <typename F>
    Source source_with_names(F field_name) const {
        std::stringstream ss;
        Source src;

        for (const TypeBox &f : types_) {
            Source fsrc = f->source();
            src.append(fsrc.into_files()).unwrap();
            writeln_(ss, "#include <{}>", fsrc.name());
        }
        writeln_(ss, "typedef struct {{");
        writeln_(ss, "    ulong index;");
        writeln_(ss, "    union {{");
        for (size_t i = 0; i < types_.size(); ++i) {
            writeln_(ss, "        {} {};", types_[i]->name(), field_name(i));
        }
        writeln_(ss, "    }} variants;");
        writeln_(ss, "}} {};", name());

        std::string fname = rs::to_lower(name());
        src.insert(fname, ss.str()).unwrap();
        src.set_name(fname);
        
        return src;
    }
    virtual Source source() const override {
        return source_with_names([](size_t i){ return format_("variant{}", i); });
    }
};

} // namespace dyn
