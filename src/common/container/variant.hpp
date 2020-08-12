#pragma once

#include "utils.hpp"


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
        union_(std::move(v.union_)),
        id_(v.id_)
        v.id_ = size();
    }
    Variant &operator=(Variant &&v) {
        this->assert_empty();
        v.assert_valid();
        this->union_ = std::move(v.union_);
        this->id_ = v.id_;
        v.id_ = size();
    }

    ~Union() {
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
    void put(container::nth_type<P, Elems...> &&x) {
        this->assert_empty();
        this->union_;
    }
    template <size_t P>
    void put(const container::nth_type<P, Elems...> &x) {
        container::nth_type<P, Elems...> cx(x);
        this->template put<P>(std::move(cx));
    }

    template <size_t P>
    const container::nth_type<P, Elems...> &get() const {
#ifdef DEBUG
        assert(this->stored_);
#endif // DEBUG
        return *reinterpret_cast<const container::nth_type<P, Elems...> *>(&this->data);
    }
    template <size_t P>
    container::nth_type<P, Elems...> &get() {
#ifdef DEBUG
        assert(this->stored_);
#endif // DEBUG
        return *reinterpret_cast<container::nth_type<P, Elems...> *>(&this->data);
    }

    template <size_t P>
    static Union create(container::nth_type<P, Elems...> &&x) {
        Union u;
        u.put<P>(std::move(x));
        return u;
    }
    template <size_t P>
    static Union create(const container::nth_type<P, Elems...> &x) {
        container::nth_type<P, Elems...> cx(x);
        return create<P>(std::move(cx));
    }

    template <size_t P>
    container::nth_type<P, Elems...> take() {
#ifdef DEBUG
        assert(this->stored_);
        this->stored_ = false;
#endif // DEBUG
        return std::move(*reinterpret_cast<container::nth_type<P, Elems...> *>(&this->data));
    }
    template <size_t P>
    void destroy() {
        this->template take<P>();
    }
};
