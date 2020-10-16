#pragma once

#include <cstdint>
#include <typeinfo>
#include <rstd/prelude.hpp>
#include <common/geometry/geometry.hpp>


class Type {
public:
    class Instance {
    public:
        Instance() = default;
        virtual ~Instance() = default;

        virtual rstd::Box<Type> type() const = 0;

        // Dynamic size of instance. If instance is statically sized, then returns 0.
        virtual size_t size() const { return 0; };
        // Stores the instance to the device. The `dst` pointer should be properly aligned.
        virtual void store(void *dst) const = 0;
    };

    Type() = default;
    virtual ~Type() = default;

    virtual size_t id() const = 0;

    // Static size of instance.
    // If the size is dynamic then the method returns None.
    virtual rstd::Option<size_t> size() const = 0;
    virtual size_t align() const = 0;

    // Loads instance from device. The `src` pointer should be properly aligned.
    virtual rstd::Box<Instance> load(const void *src) const = 0;

    virtual std::string name() = 0;
    virtual std::string source() = 0;
};

template <typename Self, typename Base>
class ImplEmptyType : public Base {
    class Instance final : public Base::Instance {
    public:
        inline virtual rstd::Box<Type> type() const override { return rstd::Box(Self()); }
        inline virtual void store(void *) const override {}
    };

    inline virtual size_t id() const override { return typeid(Self).hash_code(); }

    inline virtual rstd::Option<size_t> size() const override { return rstd::Some<size_t>(0); }
    inline virtual size_t align() const override { return 0; }
    inline virtual rstd::Box<Type::Instance> load(const void *) const override { return rstd::Box(Instance()); }

    inline virtual std::string name() override { return ""; }
};
