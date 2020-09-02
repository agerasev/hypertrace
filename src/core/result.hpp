#pragma once

#include "container/tuple.hpp"
#include "container/variant.hpp"


namespace core {

template <typename T = Tuple<>, typename E = Tuple<>>
class Result final {
private:
    Variant<T, E> var;

public:
    Result() = default;
    Result(Variant<T, E> &&v) :
        var(std::move(v))
    {}

    Result(const Result &) = default;
    Result &operator=(const Result &) = default;

    Result(Result &&) = default;
    Result &operator=(Result &&) = default;

    ~Result() = default;

    const Variant<T, E> &as_variant() const {
        return var;
    }
    Variant<T, E> &as_variant() {
        return var;
    }

    static Result Ok(T &&x) {
        return Result(Variant<T, E>::template create<0>(std::move(x)));
    }
    static Result Ok(const T &x) {
        return Result(Variant<T, E>::template create<0>(x));
    }
    static Result Err(E &&x) {
        return Result(Variant<T, E>::template create<1>(std::move(x)));
    }
    static Result Err(const E &x) {
        return Result(Variant<T, E>::template create<1>(x));
    }

    bool is_ok() const {
        return var.id() == 0;
    }
    bool is_err() const {
        return var.id() == 1;
    }

    T &get() {
        return this->var.template get<0>();
    }
    const T &get() const {
        return this->var.template get<0>();
    }
    E &get_err() {
        return this->var.template get<1>();
    }
    const E &get_err() const {
        return this->var.template get<1>();
    }

    T unwrap() {
        assert(this->is_ok());
        return this->var.template take<0>();
    }
    E unwrap_err() {
        assert(this->is_err());
        return this->var.template take<1>();
    }

    operator bool() const {
        return bool(var);
    }
};

} // namespace core
