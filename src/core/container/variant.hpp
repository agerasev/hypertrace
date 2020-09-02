#pragma once

#include <type_traits>
#include <core/traits.hpp>
#include "base.hpp"
#include "union.hpp"


namespace core {

// Safe union with id (similar to Rust enum type)

template <bool C, typename ...Elems>
class _Variant {
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
    static const bool copyable = false;

private:
    template <size_t P>
    struct Destroyer{
        static void call(Union<Elems...> &u) {
            u.template destroy<P>();
        }
    };

public:
    _Variant() = default;

    _Variant(_Variant &&v) {
        v.assert_valid();
        this->union_ = std::move(v.union_);
        this->id_ = v.id_;
        v.id_ = size();
    }
    _Variant &operator=(_Variant &&v) {
        this->assert_empty();
        v.assert_valid();
        this->union_ = std::move(v.union_);
        this->id_ = v.id_;
        v.id_ = size();
        return *this;
    }

    _Variant(const _Variant &var) = delete;
    _Variant &operator=(const _Variant &var) = delete;

    ~_Variant() {
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
    template <size_t P, std::enable_if_t<core::is_copyable_v<nth_type<P, Elems...>>, int> = 0>
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
    nth_type<P, Elems...> take() {
        this->assert_variant<P>();
        this->id_ = size();
        return std::move(this->union_.template take<P>());
    }

    void destroy() {
        this->assert_valid();
        core::Dispatcher<Destroyer, size()>::dispatch(this->id_, this->union_);
        this->id_ = size();
    }

    operator bool() const {
        return this->id_ < size();
    }
};

template <typename ...Elems>
class _Variant<true, Elems...> : public _Variant<false, Elems...> {
public:
    static const bool copyable = true;

private:
    template <size_t P>
    struct CopyCreator {
        static void call(Union<Elems...> &dst, const Union<Elems...> &src) {
            dst.template put<P>(src.template get<P>());
        }
    };
    template <size_t P>
    struct CopyAssigner {
        static void call(Union<Elems...> &dst, const Union<Elems...> &src) {
            dst.template get<P>() = src.template get<P>();
        }
    };

public:
    _Variant() = default;

    _Variant(const _Variant &var) {
        var.assert_valid();
        core::Dispatcher<CopyCreator, this->size()>::dispatch(var.id_, this->union_, var.union_);
        this->id_ = var.id_;
    }
    _Variant &operator=(const _Variant &var) {
        this->assert_valid();
        var.assert_valid();
        core::Dispatcher<CopyAssigner, this->size()>::dispatch(var.id_, this->union_, var.union_);
        this->id_ = var.id_;
    }

    _Variant(_Variant &&v) = default;
    _Variant &operator=(_Variant &&v) = default;

    ~_Variant() = default;
};

template <typename ...Elems>
class Variant final : public _Variant<all_v<core::is_copyable_v<Elems>...>, Elems...> {
public:
    Variant() = default;

    Variant(const Variant &var) = default;
    Variant &operator=(const Variant &var) = default;

    Variant(Variant &&v) = default;
    Variant &operator=(Variant &&v) = default;

    ~Variant() = default;

    template <size_t P>
    static Variant create(nth_type<P, Elems...> &&x) {
        Variant v;
        v.template put<P>(std::move(x));
        return v;
    }
    template <size_t P, std::enable_if_t<core::is_copyable_v<nth_type<P, Elems...>>, int> = 0>
    static Variant create(const nth_type<P, Elems...> &x) {
        nth_type<P, Elems...> cx(x);
        return create<P>(std::move(cx));
    }
};

} // namespace core
