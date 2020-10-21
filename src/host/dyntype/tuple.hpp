#pragma once

#include <rstd/prelude.hpp>
#include <common/types.hh>
#include "type.hpp"


namespace dyn {

class Tuple : public Type {
public:
    class Instance : public Type::Instance {
    public:
        std::vector<InstanceBox> fields_;

        Instance() = default;
        template <typename I>
        Instance(I &&field_iter) {
            fields_ = field_iter.template collect<std::vector>();
        }

        void append(InstanceBox &&i) {
            fields_.push_back(std::move(i));
        }
        
        std::vector<InstanceBox> &fields() {
            return fields_;
        }
        const std::vector<InstanceBox> &fields() const {
            return fields_;
        }

        Tuple type_() const {
            return Tuple(iter_ref(fields_).map([](const InstanceBox *f) { return (*f)->type(); }));
        }
        virtual TypeBox type() const override {
            return TypeBox(type_());
        }
        virtual void store(uchar *dst) const override {
            std::vector<size_t> offt = type_().offsets();
            for (size_t i = 0; i < fields_.size(); ++i) {
                fields_[i]->store(dst + offt[i]);
            }
        }
        virtual void load(const uchar *dst) override {
            *this = type_().load_(dst);
        }
    };

private:
    std::vector<TypeBox> fields_;

public:
    Tuple() = delete;
    template <typename I>
    Tuple(I &&field_iter) {
        fields_ = field_iter.template collect<std::vector>();
    }

    void append(TypeBox &&i) {
        fields_.push_back(std::move(i));
    }
    
    std::vector<TypeBox> &fields() {
        return fields_;
    }
    const std::vector<TypeBox> &fields() const {
        return fields_;
    }

    inline virtual TypeBox clone() const override {
        return TypeBox(Tuple(iter_ref(fields_).map([](const TypeBox *f) { return (*f)->clone(); })));
    }
    inline virtual size_t id() const override { 
        rstd::DefaultHasher hasher;
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
                //println_("{} offt: {}", (*f)->name(), offt);
                *p += (*f)->size().unwrap();
                return rstd::Some(offt);
            })
            .template collect<std::vector>();
    }
    virtual rstd::Option<size_t> size() const override {
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
        return rstd::Some(csize);
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
    virtual InstanceBox load(const uchar *src) const override {
        return InstanceBox(load_(src));
    }

    virtual std::string name() override {
        return format_("Tuple{}", id());
    }
    template <typename F>
    std::string source_with_names(F field_name) {
        std::stringstream ss;
        for (TypeBox &f : fields_) {
            writeln_(ss, f->source());
        }
        writeln_(ss, "typedef struct {{");
        for (size_t i = 0; i < fields_.size(); ++i) {
            writeln_(ss, "    {} {};", fields_[i]->name(), field_name(i));
        }
        writeln_(ss, "}} {};", name());
        return ss.str();
    }
    virtual std::string source() override {
        return source_with_names([](size_t i){ return format_("field{}", i); });
    }
};

} // namespace dyn
