#pragma once

#include <common/types.hh>
#include <common/object/type.hpp>


class Structure : public Type {
public:
    class Instance : public Type::Instance {
    public:
        Instance() = default;
        virtual ~Instance() = default;

        virtual rstd::Box<Type> type() const = 0;

        // Dynamic size of instance. If instance is statically sized, then returns static size.
        virtual size_t size() const { return type()->size(); };
        // Stores the instance to the device. The `dst` pointer should be properly aligned.
        virtual void store(void *dst) const = 0;
    };

    Type() = default;
    virtual ~Type() = default;

    virtual size_t id() const = 0;

    inline virtual bool empty() const { return false; }
    // Static size of instance.
    // If instance is empty or its size is dynamic then method returns 0.
    virtual size_t size() const = 0;
    virtual size_t align() const = 0;

    // Loads instance from device. The `src` pointer should be properly aligned.
    virtual rstd::Box<Instance> load(const void *src) const = 0;

    virtual std::string name() = 0;
    virtual std::string source() = 0;
};
