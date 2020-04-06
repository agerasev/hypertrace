#pragma once

#include <types.hpp>

#ifdef HOST
#include <iostream>
#include <cmath>
#endif // HOST

template <typename T, int N>
class vector;

template <typename T, int N, typename F>
vector<T, N> map(F f, vector<T, N> a);
template <typename T, int N, typename F>
vector<T, N> map2(F f, vector<T, N> a, vector<T, N> b);


template <typename T, int N>
class vector {
    public:
    T s[N];

    T &operator[](int i) {
        return s[i];
    }
    const T &operator[](int i) const {
        return s[i];
    }

    static vector load(const T *data) {
        vector v;
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

    friend vector operator+(vector a) {
        return a;
    }
    friend vector operator-(vector a) {
        return map([](T x) { return -x; }, a);
    }

    friend vector operator+(vector a, vector b) {
        return map2([](T x, T y) { return x + y; }, a, b);
    }
    friend vector operator+(vector a, T b) {
        return map([b](T x) { return x + b; }, a);
    }
    friend vector operator+(T a, vector b) {
        return map([a](T y) { return a + y; }, b);
    }
    friend vector operator-(vector a, vector b) {
        return map2([](T x, T y) { return x - y; }, a, b);
    }
    friend vector operator-(vector a, T b) {
        return map([b](T x) { return x - b; }, a);
    }
    friend vector operator-(T a, vector b) {
        return map([a](T y) { return a - y; }, b);
    }

    friend vector operator*(vector a, vector b) {
        return map2([](T x, T y) { return x * y; }, a, b);
    }
    friend vector operator*(vector a, T b) {
        return map([b](T x) { return x * b; }, a);
    }
    friend vector operator*(T a, vector b) {
        return map([a](T y) { return a * y; }, b);
    }
    friend vector operator/(vector a, vector b) {
        return map2([](T x, T y) { return x / y; }, a, b);
    }
    friend vector operator/(vector a, T b) {
        return map([b](T x) { return x / b; }, a);
    }
    friend vector operator/(T a, vector b) {
        return map([a](T y) { return a / y; }, b);
    }

    vector &operator+=(vector v) {
        return *this = *this + v;
    }
    vector &operator+=(T s) {
        return *this = *this + s;
    }
    vector &operator-=(vector v) {
        return *this = *this - v;
    }
    vector &operator-=(T s) {
        return *this = *this - s;
    }
    vector &operator*=(vector v) {
        return *this = *this * v;
    }
    vector &operator*=(T s) {
        return *this = *this * s;
    }
    vector &operator/=(vector v) {
        return *this = *this / v;
    }
    vector &operator/=(T s) {
        return *this = *this / s;
    }

#ifdef HOST
    friend std::ostream &operator<<(std::ostream &s, vector<T, N> v) {
        s << "(";
        for (int i = 0; i < N - 1; ++i) {
            s << v[i] << ", ";
        }
        s << v[N - 1] << ")";
        return s;
    }
#endif // HOST
};

template <typename T, int N, typename F>
vector<T, N> map(F f, vector<T, N> a) {
    vector<T, N> r;
    for (int i = 0; i < N; ++i) {
        r[i] = f(a[i]);
    }
    return r;
}
template <typename T, int N, typename F>
vector<T, N> map2(F f, vector<T, N> a, vector<T, N> b) {
    vector<T, N> r;
    for (int i = 0; i < N; ++i) {
        r[i] = f(a[i], b[i]);
    }
    return r;
}

template <typename T, int N>
T dot(vector<T, N> a, vector<T, N> b) {
    T c = (T)0;
    for (int i = 0; i < N; ++i) {
        c += a[i]*b[i];
    }
    return c;
}

template <typename T>
vector<T, 3> cross(vector<T, 3> a, vector<T, 3> b) {
    return vector<T, 3>(
        a[1]*b[2] - a[2]*b[1],
        a[2]*b[0] - a[0]*b[2],
        a[0]*b[1] - a[1]*b[0]
    );
}
template <typename T>
vector<T, 4> cross(vector<T, 4> a, vector<T, 4> b) {
    return vector<T, 4>(cross(a.xyz, b.xyz), (T)0);
}

template <typename T, int N>
T length(vector<T, N> a) {
    return sqrt(dot(a, a));
}

template <typename T, int N>
vector<T, N> normalize(vector<T, N> a) {
    return a/length(a);
}


#ifdef DEVICE
#include "vector_builtin.hpp"
#endif // DEVICE


#ifdef UNIT_TEST
#include <catch.hpp>

template <typename T, int N>
class VecApprox {
    public:
    typedef vector<T, N> vtype;
    vtype v;
    VecApprox(vtype c) : v(c) {}
    friend bool operator==(vtype a, VecApprox b) {
        for (int i = 0; i < N; ++i) {
            if (a[i] != Approx(b.v[i])) {
                return false;
            }
        }
        return true;
    }
    friend bool operator==(VecApprox a, vtype b){
        return b == a;
    }
    friend bool operator!=(vtype a, VecApprox b){
        return !(a == b);
    }
    friend bool operator!=(VecApprox a, vtype b){
        return a != b;
    }
    friend std::ostream &operator<<(std::ostream &s, VecApprox a) {
        return s << a.v;
    }
};
template <typename T, int N>
VecApprox<T, N> ApproxV(vector<T, N> v) {
    return VecApprox<T, N>(v);
}
#endif // UNIT_TEST
