#pragma once

#include <cassert>

#include <core/traits.hpp>
#include "base.hpp"


// Union for types with non-trivial ctors/dtors
template <typename ...Elems>
class Union final {
public:
    static const bool copyable = all_v<container::is_copyable_v<Elems>...>;

private:
    struct __attribute__((aligned(container::common_align<Elems...>))) {
        char bytes[container::common_size<Elems...>];
    } data;
#ifdef DEBUG
    bool stored_ = false;
#endif // DEBUG

public:
    Union() = default;

    Union(const Union &) = delete;
    Union &operator=(const Union &) = delete;

    // FIXME: Call move constructor of stored object
    Union(Union &&u) :
        data(u.data)
#ifdef DEBUG
        ,stored_(u.stored_)
#endif // DEBUG
    {
#ifdef DEBUG
        u.stored_ = false;
#endif // DEBUG
    }
    // FIXME: Call move constructor of stored object
    Union &operator=(Union &&u) {
#ifdef DEBUG
        assert(!this->stored_);
        this->stored_ = u.stored_;
        u.stored_ = false;
#endif // DEBUG
        this->data = u.data;
        return *this;
    }

    ~Union() {
#ifdef DEBUG
        assert(!this->stored_);
#endif // DEBUG
    }

    static constexpr size_t size() {
        return sizeof...(Elems);
    }
#ifdef DEBUG
    bool stored() const {
        return this->stored_;
    }
#endif // DEBUG

    template <size_t P>
    void put(nth_type<P, Elems...> &&x) {
#ifdef DEBUG
        assert(!this->stored_);
        this->stored_ = true;
#endif // DEBUG
        new (reinterpret_cast<nth_type<P, Elems...> *>(&this->data))
            nth_type<P, Elems...>(std::move(x));
    }
    template <size_t P>
    void put(const nth_type<P, Elems...> &x) {
        nth_type<P, Elems...> cx(x);
        this->put<P>(std::move(cx));
    }

    template <size_t P>
    const nth_type<P, Elems...> &get() const {
#ifdef DEBUG
        assert(this->stored_);
#endif // DEBUG
        return *reinterpret_cast<const nth_type<P, Elems...> *>(&this->data);
    }
    template <size_t P>
    nth_type<P, Elems...> &get() {
#ifdef DEBUG
        assert(this->stored_);
#endif // DEBUG
        return *reinterpret_cast<nth_type<P, Elems...> *>(&this->data);
    }

    template <size_t P>
    static Union create(nth_type<P, Elems...> &&x) {
        Union u;
        u.put<P>(std::move(x));
        return u;
    }
    template <size_t P>
    static Union create(const nth_type<P, Elems...> &x) {
        nth_type<P, Elems...> cx(x);
        return create<P>(std::move(cx));
    }

    template <size_t P>
    nth_type<P, Elems...> take() {
#ifdef DEBUG
        assert(this->stored_);
        this->stored_ = false;
#endif // DEBUG
        typedef nth_type<P, Elems...> T;
        T *px = reinterpret_cast<T *>(&this->data);
        T x = std::move(*px);
        px->~T();
        return x;
    }
    template <size_t P>
    void destroy() {
        this->take<P>();
    }
};
