#pragma once

#include <cstdint>
#include <typeinfo>
#include <rstd/prelude.hpp>
#include <common/geometry/geometry.hpp>


class Type;
class Instance;


class Type {
public:
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
    virtual std::string prefix() = 0;
    virtual std::string source() = 0;
};

class EmptyType : public Type {
public:
    virtual rstd::Box<Instance> instance() const = 0;

    inline virtual bool empty() const override { return true; }
    inline virtual size_t size() const override { return 0; }
    inline virtual size_t align() const override { return 0; }

    inline virtual rstd::Box<Instance> load(const void *) const override { return this->instance(); }

    inline virtual std::string name() override { return ""; }
};


struct Instance {
    Instance() = default;
    virtual ~Instance() = default;

    virtual rstd::Box<Type> type() const = 0;

    // Dynamic size of instance. If instance is statically sized, then returns static size.
    virtual size_t dyn_size() const = 0;
    // Stores the instance to the device. The `dst` pointer should be properly aligned.
    virtual void store(void *dst) const = 0;
};

struct EmptyInstance : public virtual Instance {
    virtual size_t dyn_size() const { return 0; }
    virtual void store(void *) const {}
};

struct SizedInstance : public virtual Instance {
    inline virtual size_t dyn_size() const override {
        return this->type()->size();
    }
};
