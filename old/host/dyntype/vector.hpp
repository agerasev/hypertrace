#pragma once

#include <common/types.hh>
#include "type.hpp"


namespace dyn {

template <typename T=Type>
class Vector : public Type {
public:
    typedef T ItemType;
    typedef typename T::Instance ItemInstance;

private:
    typedef rs::Box<ItemType> ItemTypeBox;
    typedef rs::Box<ItemInstance> ItemInstanceBox;

public:
    class Instance : public Type::Instance {
    public:
        ItemTypeBox item_type_; 
        std::vector<ItemInstanceBox> items_;

        Instance() = default;
        Instance(ItemTypeBox &&type) :
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

        void append(ItemInstanceBox &&i) {
            if (items_.size() > 0) {
                assert_eq_(item_type_->id(), i->type()->id());
            } else {
                item_type_ = i->type();
            }
            items_.push_back(std::move(i));
        }
        
        ItemTypeBox item_type() const {
            return item_type_->clone();
        }
        std::vector<ItemInstanceBox> &items() {
            return items_;
        }
        const std::vector<ItemInstanceBox> &items() const {
            return items_;
        }

        Vector type_() const {
            return Vector(item_type_->clone());
        }
        virtual Vector *_type() const override {
            return new Vector(type_());
        }
        rs::Box<Vector> type() const {
            return rs::Box<Vector>::_from_raw(_type());
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
    ItemTypeBox item_type_;

public:
    Vector() = delete;
    Vector(ItemTypeBox &&type) :
        item_type_(std::move(type))
    {
        assert_(bool(item_type_));
    }
    ItemTypeBox item_type() const {
        return item_type_->clone();
    }

    virtual Vector *_clone() const override {
        return new Vector(item_type_->clone());
    }
    rs::Box<Vector> clone() const {
        return rs::Box<Vector>::_from_raw(_clone());
    }

    virtual size_t id() const override { 
        rs::DefaultHasher hasher;
        hasher._hash_raw(typeid(Vector).hash_code());
        hasher._hash_raw(item_type_->id());
        return hasher.finish();
    }

    virtual rs::Option<size_t> size() const override {
        return rs::None();
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
    virtual Instance *_load(const uchar *src) const override {
        return new Instance(load_(src));
    }
    rs::Box<Instance> load(const uchar *src) const {
        return rs::Box<Instance>::_from_raw(_load(src));
    }

    virtual std::string name() const override {
        return format_("Vector{}", id());
    }
    virtual Source source() const override {
        std::string lname = rs::to_lower(name());
        Source src = item_type_->source();

        std::stringstream ss;
        writeln_(ss, "#include <{}>\n", src.name());
        writeln_(ss,
            "typedef struct {{\n"
            "   ulong size;\n"
            "   {} first;\n"
            "}} {};\n",
            item_type_->name(), name()
        );
        writeln_(ss, "#define {}_size(ptr) ((ptr)->size)", lname);
        writeln_(ss, "#define {}_get(ptr, idx) (&(ptr)->first + idx)", lname);

        std::string fname = format_("generated/{}.hxx", lname);
        src.insert(fname, ss.str()).unwrap();
        src.set_name(fname);

        return src;
    }
};

} // namespace dyn
