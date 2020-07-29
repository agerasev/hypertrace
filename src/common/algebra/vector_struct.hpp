#pragma once

#include <iostream>

#include "vector_base.hpp"


template <typename T, int N>
class vec : public _vec<T, N> {
private:
    // check vec alignment is the same as the element type
    static_assert(std::alignment_of_v<T> == std::alignment_of_v<_vec<T, N>>);

public:
    static const int SIZE = N; 

    vec() = default;
    template <typename ... Args>
    vec(Args ...args) : _vec<T, N>(args...) {}

    T &operator[](int i) {
        return this->s[i];
    }
    const T &operator[](int i) const {
        return this->s[i];
    }

    static vec load(const T *data) {
        vec v;
        for (int i = 0; i < N; ++i) {
            v[i] = data[i];
        }
        return v;
    }
    void store(T *data) const {
        for (int i = 0; i < N; ++i) {
            data[i] = (*this)[i];
        }
    }

    template <typename S>
    vec<S, N> convert() const {
        vec<S, N> r;
        for (int i = 0; i < N; ++i) {
            r[i] = (S)(*this)[i];
        }
        return r;
    }

    template <typename S=T, typename F, typename ...Ts>
    static vec<S, N> map(F f, vec<Ts, N> ...args) {
        vec<S, N> r;
        for (int i = 0; i < N; ++i) {
            r[i] = f((args[i])...);
        }
        return r;
    }

    friend vec<T, N> operator+(vec<T, N> a) {
        return a;
    }
    friend vec<T, N> operator-(vec<T, N> a) {
        return map([](T x) { return -x; }, a);
    }

    friend vec<T, N> operator+(vec<T, N> a, vec<T, N> b) {
        return map([](T x, T y) { return x + y; }, a, b);
    }
    friend vec<T, N> operator+(vec<T, N> a, T b) {
        return map([b](T x) { return x + b; }, a);
    }
    friend vec<T, N> operator+(T a, vec<T, N> b) {
        return map([a](T y) { return a + y; }, b);
    }
    friend vec<T, N> operator-(vec<T, N> a, vec<T, N> b) {
        return map([](T x, T y) { return x - y; }, a, b);
    }
    friend vec<T, N> operator-(vec<T, N> a, T b) {
        return map([b](T x) { return x - b; }, a);
    }
    friend vec<T, N> operator-(T a, vec<T, N> b) {
        return map([a](T y) { return a - y; }, b);
    }

    friend vec<T, N> operator*(vec<T, N> a, vec<T, N> b) {
        return map([](T x, T y) { return x * y; }, a, b);
    }
    friend vec<T, N> operator*(vec<T, N> a, T b) {
        return map([b](T x) { return x * b; }, a);
    }
    friend vec<T, N> operator*(T a, vec<T, N> b) {
        return map([a](T y) { return a * y; }, b);
    }
    friend vec<T, N> operator/(vec<T, N> a, vec<T, N> b) {
        return map([](T x, T y) { return x / y; }, a, b);
    }
    friend vec<T, N> operator/(vec<T, N> a, T b) {
        return map([b](T x) { return x / b; }, a);
    }
    friend vec<T, N> operator/(T a, vec<T, N> b) {
        return map([a](T y) { return a / y; }, b);
    }

    vec &operator+=(vec v) {
        return *this = *this + v;
    }
    vec &operator+=(T s) {
        return *this = *this + s;
    }
    vec &operator-=(vec v) {
        return *this = *this - v;
    }
    vec &operator-=(T s) {
        return *this = *this - s;
    }
    vec &operator*=(vec v) {
        return *this = *this * v;
    }
    vec &operator*=(T s) {
        return *this = *this * s;
    }
    vec &operator/=(vec v) {
        return *this = *this / v;
    }
    vec &operator/=(T s) {
        return *this = *this / s;
    }

    friend std::ostream &operator<<(std::ostream &s, vec<T, N> v) {
        s << "(";
        for (int i = 0; i < N - 1; ++i) {
            s << v[i] << ", ";
        }
        s << v[N - 1] << ")";
        return s;
    }
};
