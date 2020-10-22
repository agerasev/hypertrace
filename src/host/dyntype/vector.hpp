#pragma once

#include <common/types.hh>
#include "type.hpp"


namespace dyn {

class Vector : public Type {
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
            items_ = item_iter.template collect<std::vector>();
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
            items_.push_back(std::move(i));
        }
        
        TypeBox item_type() const {
            return item_type_->clone();
        }
        std::vector<InstanceBox> &items() {
            return items_;
        }
        const std::vector<InstanceBox> &items() const {
            return items_;
        }

        Vector type_() const {
            return Vector(item_type_->clone());
        }
        virtual TypeBox type() const override {
            return TypeBox(type_());
        }
        virtual size_t size() const override {
            return 
                upper_multiple(type_().align(), sizeof(dev_type<ulong_>))
                + items_.size()*item_type_->size().unwrap();
        }
        virtual void store(uchar *dst) const override {
            assert_eq_((size_t)dst % alignof(dev_type<ulong_>), 0);
            ulong_ n = items_.size();
            dev_store<ulong_>((dev_type<ulong_> *)dst, &n);
            size_t shift = upper_multiple(type_().align(), sizeof(dev_type<ulong_>));
            for (size_t i = 0; i < items_.size(); ++i) {
                items_[i]->store(dst + shift + i*item_type_->size().unwrap());
            }
        }
        virtual void load(const uchar *dst) override {
            *this = type_().load_(dst);
        }
    };

private:
    TypeBox item_type_;

public:
    Vector() = delete;
    Vector(TypeBox &&type) :
        item_type_(std::move(type))
    {
        assert_(bool(item_type_));
    }
    TypeBox item_type() const {
        return item_type_->clone();
    }

    inline virtual TypeBox clone() const override {
        return TypeBox(Vector(item_type_->clone()));
    }
    inline virtual size_t id() const override { 
        rstd::DefaultHasher hasher;
        hasher._hash_raw(typeid(Vector).hash_code());
        hasher._hash_raw(item_type_->id());
        return hasher.finish();
    }

    virtual rstd::Option<size_t> size() const override {
        return rstd::None();
    }
    virtual size_t align() const override {
        return std::max(alignof(dev_type<ulong_>), item_type_->align());
    }

    Instance load_(const uchar *src) const {
        assert_eq_((size_t)src % alignof(dev_type<ulong_>), 0);
        Instance dst(item_type_->clone());
        ulong_ n;
        dev_load<ulong_>(&n, (const dev_type<ulong_> *)src);
        size_t shift = upper_multiple(align(), sizeof(dev_type<ulong_>));
        for (size_t i = 0; i < n; ++i) {
            dst.append(item_type_->load(src + shift + i*item_type_->size().unwrap()));
        }
        return dst;
    }
    virtual InstanceBox load(const uchar *src) const override {
        return InstanceBox(load_(src));
    }

    virtual std::string name() const override {
        return format_("Vector{}", id());
    }
    virtual std::string source() const override {
        std::string lname = rstd::to_lower(name());
        std::stringstream ss;
        writeln_(ss, item_type_->source());
        writeln_(ss,
            "typedef struct {{\n"
            "   ulong size;\n"
            "   {} first;\n"
            "}} {};\n",
            item_type_->name(), name()
        );
        writeln_(ss, "#define {}_size(ptr) ((ptr)->size)", lname);
        writeln_(ss, "#define {}_get(ptr, idx) (&(ptr)->first + idx)", lname);
        return ss.str();
    }
};

} // namespace dyn
