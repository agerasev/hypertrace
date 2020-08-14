#pragma once

#include <cassert>
#include <core/traits.hpp>
#include "base.hpp"


template <typename ...Elems>
class Tuple {
public:
    static constexpr int size() {
        return 0;
    }
};

template <size_t P, typename T, typename ...Elems>
struct _TupleGetter;

template <typename T, typename ...Elems>
class Tuple<T, Elems...> {
private:
    template <size_t P, typename T_, typename ...Elems_>
    friend struct _TupleGetter;
    template <size_t P>
    using Getter = _TupleGetter<P, T, Elems...>;

    T value;
    Tuple<Elems...> tail;

public:
    Tuple() = default;
    Tuple(const T &v, Elems &&...args) :
        value(v),
        tail(std::forward<Elems>(args)...)
    {}
    Tuple(T &&v, Elems &&...args) :
        value(std::move(v)),
        tail(std::forward<Elems>(args)...)
    {}

    Tuple(const Tuple &) = delete;
    Tuple &operator=(const Tuple &) = delete;

    Tuple(Tuple &&t) :
        value(std::move(t.value)),
        tail(std::move(t.tail))
    {}
    Tuple &operator=(Tuple &&t) {
        this->value = std::move(t.value);
        this->tail = std::move(t.tail);
        return *this;
    }

    ~Tuple() = default;

    static constexpr size_t size() {
        return 1 + sizeof...(Elems);
    }
    
    template <size_t P>
    const nth_type<P, T, Elems...> &get() const {
        return Getter<P>::get(*this);
    }
    template <size_t P>
    nth_type<P, T, Elems...> &get() {
        return Getter<P>::get(*this);
    }
};

template <size_t P, typename T, typename ...Elems>
struct _TupleGetter {
    static const nth_type<P - 1, Elems...> &get(const Tuple<T, Elems...> &t) {
        return _TupleGetter<P - 1, Elems...>::get(t.tail);
    }
    static nth_type<P - 1, Elems...> &get(Tuple<T, Elems...> &t) {
        return _TupleGetter<P - 1, Elems...>::get(t.tail);
    }
};

template <typename T, typename ...Elems>
struct _TupleGetter<0, T, Elems...> {
    static const T &get(const Tuple<T, Elems...> &t) {
        return t.value;
    }
    static T &get(Tuple<T, Elems...> &t) {
        return t.value;
    }
};
