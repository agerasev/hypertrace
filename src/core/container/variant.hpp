#pragma once

#include <type_traits>
#include <core/traits.hpp>
#include "base.hpp"
#include "union.hpp"


// Safe union with id (similar to Rust enum type)
template <typename ...Elems>
class Variant final {
private:
    Union<Elems...> union_;
    size_t id_ = size();

    void assert_valid() const {
#ifdef DEBUG
        assert(this->id_ < size());
#endif // DEBUG
    }
    template <size_t P>
    void assert_variant() const {
        this->assert_valid();
#ifdef DEBUG
        assert(this->id_ == P);
#endif // DEBUG
    }
    void assert_empty() const {
#ifdef DEBUG
        assert(this->id_ == size());
#endif // DEBUG
    }

public:
    Variant() = default;

    Variant(const Variant &) = delete;
    Variant &operator=(const Variant &) = delete;

    Variant(Variant &&v) {
        v.assert_valid();
        this->union_ = std::move(v.union_);
        this->id_ = v.id_;
        v.id_ = size();
    }
    Variant &operator=(Variant &&v) {
        this->assert_empty();
        v.assert_valid();
        this->union_ = std::move(v.union_);
        this->id_ = v.id_;
        v.id_ = size();
        return *this;
    }

    ~Variant() {
        if (this->id_ < size()) {
            this->destroy();
        }
    }

    static constexpr size_t size() {
        return sizeof...(Elems);
    }
    size_t id() const {
        return this->id_;
    }

    template <size_t P>
    void put(nth_type<P, Elems...> &&x) {
        this->assert_empty();
        this->union_.template put<P>(std::move(x));
        this->id_ = P;
    }
    template <size_t P>
    void put(const nth_type<P, Elems...> &x) {
        nth_type<P, Elems...> cx(x);
        this->put<P>(std::move(cx));
    }

    template <size_t P>
    const nth_type<P, Elems...> &get() const {
        this->assert_variant<P>();
        return this->union_.template get<P>();
    }
    template <size_t P>
    nth_type<P, Elems...> &get() {
        this->assert_variant<P>();
        return this->union_.template get<P>();
    }

    template <size_t P>
    static Variant create(nth_type<P, Elems...> &&x) {
        Variant v;
        v.put<P>(std::move(x));
        return v;
    }
    template <size_t P>
    static Variant create(const nth_type<P, Elems...> &x) {
        nth_type<P, Elems...> cx(x);
        return create<P>(std::move(cx));
    }

    template <size_t P>
    nth_type<P, Elems...> take() {
        this->assert_variant<P>();
        this->id_ = size();
        return std::move(this->union_.template take<P>());
    }

private:
    template <size_t P>
    struct Destroyer{
        static void call(Union<Elems...> &u) {
            u.template destroy<P>();
        }
    };

public:
    void destroy() {
        this->assert_valid();
        container::Dispatcher<Destroyer, size()>::dispatch(this->id_, this->union_);
        this->id_ = size();
    }
};
