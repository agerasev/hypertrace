#pragma once

#include "container/tuple.hpp"
#include "container/variant.hpp"


namespace core {

template <typename T>
class Option final {
private:
    Variant<Tuple<>, T> var;

public:
    Option() = default;
    Option(Variant<Tuple<>, T> &&v) :
        var(std::move(v))
    {}

    Option(const Option &) = default;
    Option &operator=(const Option &) = default;

    Option(Option &&) = default;
    Option &operator=(Option &&) = default;

    ~Option() = default;

    const Variant<Tuple<>, T> &as_variant() const {
        return var;
    }
    Variant<Tuple<>, T> &as_variant() {
        return var;
    }

    static Option None() {
        return Option(Variant<Tuple<>, T>::template create<0>(Tuple<>()));
    }
    static Option Some(T &&x) {
        return Option(Variant<Tuple<>, T>::template create<1>(std::move(x)));
    }
    static Option Some(const T &x) {
        return Option(Variant<Tuple<>, T>::template create<1>(x));
    }

    bool is_none() const {
        return var.id() == 0;
    }
    bool is_some() const {
        return var.id() == 1;
    }

    T &get() {
        return this->var.template get<1>();
    }
    const T &get() const {
        return this->var.template get<1>();
    }

    T unwrap() {
        assert(this->is_some());
        return this->var.template take<1>();
    }

    operator bool() const {
        return bool(var);
    }
};

} // namespace core
