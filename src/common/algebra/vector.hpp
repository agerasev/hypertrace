#pragma once

#include "traits.hpp"

#ifdef HOST
#include <iostream>
#include <cmath>
#endif // HOST


template <typename T, int N>
class vector {
private:
    T s[N];

    template <int P, typename ...Args>
    void unwind(T x, Args ...args) {
        static_assert(P < N, "Too many elements in the constructor");
        s[P] = x;
        unwind<P + 1>(args...);
    }
    /*
    template <int P, int M, typename ...Args>
    void unwind(vector<T, M> x, Args ...args) {
        static_assert(P + M <= N, "Too many elements in the constructor");
        for (int i = 0; i < M; ++i) {
            s[P + i] = x[i];
        }
        unwind<P + M>(args...);
    }
    */
    template <int P>
    void unwind() {
        static_assert(P == N, "Too few elements in the constructor");
    }
public:
    vector() = default;
    vector(T x) {
        for (int i = 0; i < N; ++i) {
            s[i] = x;
        }
    }
    template <typename ...Args>
    vector(Args ...args) {
        unwind<0>(args...);
    }

    T *data() {
        return s;
    }
    const T *data() const {
        return s;
    }
    T &operator[](int i) {
        return s[i];
    }
    const T &operator[](int i) const {
        return s[i];
    }

    static constexpr int size() {
        return N;
    }
    template <int P>
    T &elem() {
        static_assert(P >= 0 && P < N, "Index is out of bounds");
        return s[P];
    }
    template <int P>
    const T &elem() const {
        static_assert(P >= 0 && P < N, "Index is out of bounds");
        return s[P];
    }
    template <int P, int S>
    vector<T, S> &slice() {
        static_assert(P >= 0 && S > 0 && P + S <= N, "Indices is out of bounds");
        static_assert(P == 0 || (P % S) == 0, "Slicing breaks alignment");
        return *reinterpret_cast<vector<T, S>*>(s + P);
    }
    template <int P, int S>
    const vector<T, S> &slice() const {
        static_assert(P >= 0 && S > 0 && P + S <= N, "Indices is out of bounds");
        static_assert(P == 0 || (P % S) == 0, "Slicing breaks alignment");
        return *reinterpret_cast<const vector<T, S>*>(s + P);
    }

    static vector load(const T *data, int stride=1) {
        vector v;
        for (int i = 0; i < N; ++i) {
            v[i] = data[i*stride];
        }
        return v;
    }
    void store(T *data, int stride=1) const {
        for (int i = 0; i < N; ++i) {
            data[i*stride] = (*this)[i];
        }
    }

    template <typename F>
    friend vector map(F f, vector a) {
        vector r;
        for (int i = 0; i < N; ++i) {
            r[i] = f(a[i]);
        }
        return r;
    }
    template <typename F>
    friend vector map2(F f, vector a, vector b) {
        vector r;
        for (int i = 0; i < N; ++i) {
            r[i] = f(a[i], b[i]);
        }
        return r;
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

    vector &operator+=(vector a) {
        return *this = *this + a;
    }
    vector &operator+=(T a) {
        return *this = *this + a;
    }
    vector &operator-=(vector a) {
        return *this = *this - a;
    }
    vector &operator-=(T a) {
        return *this = *this - a;
    }
    vector &operator*=(vector a) {
        return *this = *this * a;
    }
    vector &operator*=(T a) {
        return *this = *this * a;
    }
    vector &operator/=(vector a) {
        return *this = *this / a;
    }
    vector &operator/=(T a) {
        return *this = *this / a;
    }
};

template <typename T, int N>
class Zero<vector<T, N>> {
    static vector<T, N> zero() {
        return vector<T, N>(::zero<T>());
    } 
};

template <typename T, int N_>
class Dim<vector<T, N_>> {
    static const int N = N_;
};
template <typename T, int N>
class BaseType<vector<T, N>> {
    typedef T type;
};

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
    return vector<T, 4>(
        a[1]*b[2] - a[2]*b[1],
        a[2]*b[0] - a[0]*b[2],
        a[0]*b[1] - a[1]*b[0],
        zero<T>()
    );
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

#ifdef HOST
template <typename T, int N>
std::ostream &operator<<(std::ostream &s, vector<T, N> v) {
    s << "(";
    for (int i = 0; i < N - 1; ++i) {
        s << v[i] << ", ";
    }
    s << v[N - 1] << ")";
    return s;
}
#endif // HOST

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
