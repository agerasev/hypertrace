#pragma once

#include <memory>
#include "prelude.hpp"


namespace core {

// Wrapper over std::unique_ptr
template <typename T>
class Box final {
private:
    std::unique_ptr<T> base;

public:
    Box() = default;
    Box(T &&v) : base(new T(std::move(v))) {}
    Box(const T &v) : base(new T(v)) {}
    ~Box() = default;

    Box(Box &&) = default;
    Box &operator=(Box &&) = default;

    Box(const Box &) = default;
    Box &operator=(const Box &) = default;

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
        base = std::unique_ptr<T>();
    }
    Option<T> take() {
        auto ret = Option<T>::Some(T(std::move(*base)));
        drop();
        return ret;
    }

    operator bool() const {
        return bool(base);
    }
};

} // namespace core
