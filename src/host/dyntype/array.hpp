#pragma once

#include <common/types.hh>
#include "type.hpp"


namespace dyn {

template <typename T=Type>
class Array : public Type {
public:
    typedef T ItemType;
    typedef typename T::Instance ItemInstance;

private:
    typedef rstd::Box<ItemType> ItemTypeBox;
    typedef rstd::Box<ItemInstance> ItemInstanceBox;

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

        Array type_() const {
            return Array(item_type_->clone(), items_.size());
        }
        virtual Array *_type() const override {
            return new Array(type_());
        }
        rstd::Box<Array> type() const {
            return rstd::Box<Array>::_from_raw(_type());
        }

        virtual void store(uchar *dst) const override {
            for (size_t i = 0; i < items_.size(); ++i) {
                items_[i]->store(dst + i*item_type_->size().unwrap());
            }
        }
        virtual void load(const uchar *dst) override {
            *this = type_().load_(dst);
        }
    };

private:
    ItemTypeBox item_type_;
    size_t item_count_;

public:
    Array() = delete;
    Array(ItemTypeBox &&type, size_t n) :
        item_type_(std::move(type)),
        item_count_(n)
    {
        assert_(bool(item_type_));
        assert_(item_count_ > 0);
    }
    ItemTypeBox item_type() const {
        return item_type_->clone();
    }
    size_t item_count() const {
        return item_count_;
    }

    virtual Array *_clone() const override {
        return new Array(item_type_->clone(), item_count_);
    }
    rstd::Box<Array> clone() const {
        return rstd::Box<Array>::_from_raw(_clone());
    }

    virtual size_t id() const override { 
        rstd::DefaultHasher hasher;
        hasher._hash_raw(typeid(Array).hash_code());
        hasher._hash_raw(item_type_->id());
        hasher.hash(item_count_);
        return hasher.finish();
    }

    virtual rstd::Option<size_t> size() const override {
        return rstd::Some(item_type_->size().unwrap()*item_count_);
    }
    virtual size_t align() const override {
        return item_type_->align();
    }

    Instance load_(const uchar *src) const {
        Instance dst(item_type_->clone());
        for (size_t i = 0; i < item_count_; ++i) {
            dst.append(item_type_->load(src + i*item_type_->size().unwrap()));
        }
        return dst;
    }
    virtual Instance *_load(const uchar *src) const override {
        return new Instance(load_(src));
    }
    rstd::Box<Instance> load(const uchar *src) const {
        return rstd::Box<Instance>::_from_raw(_load(src));
    }

    virtual std::string name() const override {
        return format_("Array{}", id());
        // Pretty name, but it can cause collisions
        //return format_("Array_{}_{}", item_type_->name(), item_count_);
    }
    virtual std::string source() const override {
        return item_type_->source() + "\n" + format_(
            "typedef struct {{\n"
            "   {} items[{}];\n"
            "}} {};\n",
            item_type_->name(),
            item_count_,
            name()
        );
    }
};

} // namespace dyn
