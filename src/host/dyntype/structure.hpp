#pragma once

#include <rstd/prelude.hpp>
#include <common/types.hh>
#include "type.hpp"


class Structure : public Type {
public:
    class Instance : public Type::Instance {
    public:
        Instance() = default;
        virtual ~Instance() = default;

        virtual rstd::Box<Type> type() const = 0;

        virtual size_t size() const { return 0; };
        virtual void store(void *dst) const = 0;
    };

    std::vector<rstd::Box<Type>> fields;

    inline virtual size_t id() const override {
        rstd::DefaultHasher hasher;
        hasher._hash_raw(typeid(Structure).hash_code());
        for (const rstd::Box<Type> &t : fields) {
            hasher._hash_raw(t->id());
        }
        return hasher.finish();
    }

    inline virtual rstd::Option<size_t> size() const override {
        return rstd::Some(iter_ref(fields)
        .fold((size_t)0, [](size_t pos, const rstd::Box<Type> *ty) {
            size_t s = (*ty)->size().unwrap(), a = (*ty)->align();
            assert_eq_(s % a, (size_t)0);
            if (pos != 0) {
                return ((pos - 1)/a + 1)*a + s;
            } else {
                return s;
            }
        }));
    }
    inline virtual size_t align() const {
        return iter_ref(fields)
        .map([](const rstd::Box<Type> *ty) { return (*ty)->align(); })
        .max().unwrap_or(0);
    }

    virtual rstd::Box<Instance> load(const void *src) const = 0;

    virtual std::string name() = 0;
    virtual std::string source() = 0;
};
