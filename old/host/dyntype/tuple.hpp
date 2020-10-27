#pragma once

#include <rstd.hpp>
#include <common/types.hh>
#include "type.hpp"


namespace dyn {

template <typename T=Type>
class Tuple : public Type {
public:
    typedef T ItemType;
    typedef typename T::Instance ItemInstance;

protected:
    typedef rs::Box<ItemType> ItemTypeBox;
    typedef rs::Box<ItemInstance> ItemInstanceBox;

public:
    class Instance : public Type::Instance {
    public:
        std::vector<ItemInstanceBox> fields_;

        Instance() = default;
        template <typename I>
        Instance(I &&field_iter) {
            fields_ = field_iter.template collect<std::vector>();
        }

        void append(ItemInstanceBox &&i) {
            fields_.push_back(std::move(i));
        }
        
        std::vector<ItemInstanceBox> &fields() {
            return fields_;
        }
        const std::vector<ItemInstanceBox> &fields() const {
            return fields_;
        }

        Tuple type_() const {
            return Tuple(iter_ref(fields_).map([](const ItemInstanceBox *f) { return (*f)->type(); }));
        }
        virtual Tuple *_type() const override {
            return new Tuple(type_());
        }
        rs::Box<Tuple> type() const {
            return rs::Box<Tuple>::_from_raw(_type());
        }

        virtual void store(uchar *dst) const override {
            std::vector<size_t> offt = type_().offsets();
            for (size_t i = 0; i < fields_.size(); ++i) {
                fields_[i]->store(dst + offt[i]);
            }
        }
        virtual void load(const uchar *src) override {
            *this = type_().load_(src);
        }
    };

private:
    std::vector<ItemTypeBox> fields_;

public:
    Tuple() = default;
    template <typename I>
    Tuple(I &&field_iter) {
        fields_ = field_iter.template collect<std::vector>();
    }

    void append(ItemTypeBox &&i) {
        fields_.push_back(std::move(i));
    }
    
    std::vector<ItemTypeBox> &fields() {
        return fields_;
    }
    const std::vector<ItemTypeBox> &fields() const {
        return fields_;
    }

    virtual Tuple *_clone() const override {
        return new Tuple(iter_ref(fields_).map([](const TypeBox *f) { return (*f)->clone(); }));
    }
    rs::Box<Tuple> clone() const {
        return rs::Box<Tuple>::_from_raw(_clone());
    }

    virtual size_t id() const override { 
        rs::DefaultHasher hasher;
        hasher._hash_raw(typeid(Tuple).hash_code());
        for (const TypeBox &f : fields_) {
            hasher._hash_raw(f->id());
        }
        return hasher.finish();
    }

    std::vector<size_t> offsets() const {
        return
            iter_ref(fields_)
            .scan((size_t)0, [](size_t *p, const TypeBox *f) {
                size_t offt = upper_multiple((*f)->align(), *p);
                *p = offt + (*f)->size().unwrap();
                return rs::Some(offt);
            })
            .template collect<std::vector>();
    }
    virtual rs::Option<size_t> size() const override {
        size_t csize = upper_multiple(align(),
            iter_ref(fields_)
            .fold((size_t)0, [](size_t p, const TypeBox *f) {
                const TypeBox &rf = *f;
                return upper_multiple(rf->align(), p) + rf->size().unwrap();
            })
        );
        if (csize == 0) {
            // empty Tuple has 1-byte size
            csize = 1;
        }
        return rs::Some(csize);
    }
    virtual size_t align() const override {
        return 
            iter_ref(fields_)
            .map([](const TypeBox *f) { return (*f)->align(); })
            .max().unwrap_or(1);
            // empty Tuple is byte-aligned
    }

    Instance load_(const uchar *src) const {
        Instance dst;
        std::vector<size_t> offt = offsets();
        for (size_t i = 0; i < fields_.size(); ++i) {
            dst.append(fields_[i]->load(src + offt[i]));
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
        return format_("Tuple{}", id());
    }
    template <typename F>
    Source source_with_names(F field_name) const {
        std::stringstream ss;
        Source src;

        for (const TypeBox &f : fields_) {
            Source fsrc = f->source();
            src.append(fsrc.into_files()).unwrap();
            writeln_(ss, "#include <{}>", fsrc.name());
        }
        writeln_(ss, "\ntypedef struct {{");
        for (size_t i = 0; i < fields_.size(); ++i) {
            if (fields_[i]->size().unwrap() > 0) {
                writeln_(ss, "    {} {};", fields_[i]->name(), field_name(i));
            }
        }
        writeln_(ss, "}} {};", name());

        std::string fname = rs::to_lower(name());
        src.insert(fname, ss.str()).unwrap();
        src.set_name(fname);
        
        return src;
    }
    virtual Source source() const override {
        return source_with_names([](size_t i){ return format_("field{}", i); });
    }
};

} // namespace dyn