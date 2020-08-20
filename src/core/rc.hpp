#pragma once

#include <memory>
#include "prelude.hpp"


// Wrapper over std::shared_ptr
template <typename T>
class Rc final {
private:
    std::shared_ptr<T> base;

public:
    Rc() = default;
    Rc(T &&v) : base(new T(std::move(v))) {}
    Rc(const T &v) : base(new T(v)) {}
    ~Rc() = default;

    Rc(Rc &&rc) = default;
    Rc &operator=(Rc &&rc) = default;

    Rc(const Rc &rc) = default;
    Rc &operator=(const Rc &rc) = default;

    T &operator*() {
        return *base;
    }
    const T &operator*() const {
        return *base;
    }
    T *operator->() {
        return &*base;
    }
    const T *operator->() const {
        return &*base;
    }

    void drop() {
        base = std::shared_ptr<T>();
    }
    Option<T> try_take() {
        if (base.use_count() == 1) {
            auto ret = Option<T>::Some(T(std::move(*base)));
            drop();
            return ret;
        } else {
            return Option<T>::None();
        }
    }

    operator bool() const {
        return bool(base);
    }
};
