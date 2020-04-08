#pragma once

#ifdef HOST
#include <iostream>
#include <cmath>
#endif

#include "traits.hpp"
#include "real.hpp"


template <typename T, int N>
class vector;

template <typename T, int N, typename F, typename ...Args>
vector<T, N> map(F f, Args ...args);

template <typename T, int N>
class vector {
private:
    T s[N];

public:
    template <int P=0, int M, typename ...Args>
    static void check_all_dims(vector<T, M>, Args ...args) {
        static_assert(N == M, "Wrong vector size");
        check_all_dims<P + 1>(args...);
    }
    template <int P=0>
    static void check_all_dims() {}

private:
    template <int P=0, typename ...Args>
    void unwind(T x, Args ...args) {
        static_assert(P < N, "Too many elements in the constructor");
        s[P] = x;
        unwind<P + 1>(args...);
    }
    /*
    template <int P=0, int M, typename ...Args>
    void unwind(vector<T, M> x, Args ...args) {
        static_assert(P + M <= N, "Too many elements in the constructor");
        for (int i = 0; i < M; ++i) {
            s[P + i] = x[i];
        }
        unwind<P + M>(args...);
    }
    */
    template <int P=0>
    void unwind() {
        static_assert(P == N, "Too few elements in the constructor");
    }
public:
    vector() = default;
    explicit vector(T x) {
        for (int i = 0; i < N; ++i) {
            s[i] = x;
        }
    }
    template <typename ...Args, typename enable_if<(sizeof...(Args) > 1)>::type* = nullptr>
    explicit vector(Args ...args) {
        unwind(args...);
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

    template <typename F, typename ...Args>
    static vector map(F f, Args ...args) {
        return ::map<T, N>(f, args...);
    }

    friend vector operator+(vector a) {
        return a;
    }
    friend vector operator-(vector a) {
        return map([](T x) { return -x; }, a);
    }

    friend vector operator+(vector a, vector b) {
        return map([](T x, T y) { return x + y; }, a, b);
    }
    friend vector operator+(vector a, T b) {
        return map([b](T x) { return x + b; }, a);
    }
    friend vector operator+(T a, vector b) {
        return map([a](T y) { return a + y; }, b);
    }
    friend vector operator-(vector a, vector b) {
        return map([](T x, T y) { return x - y; }, a, b);
    }
    friend vector operator-(vector a, T b) {
        return map([b](T x) { return x - b; }, a);
    }
    friend vector operator-(T a, vector b) {
        return map([a](T y) { return a - y; }, b);
    }

    friend vector operator*(vector a, vector b) {
        return map([](T x, T y) { return x * y; }, a, b);
    }
    friend vector operator*(vector a, T b) {
        return map([b](T x) { return x * b; }, a);
    }
    friend vector operator*(T a, vector b) {
        return map([a](T y) { return a * y; }, b);
    }
    friend vector operator/(vector a, vector b) {
        return map([](T x, T y) { return x / y; }, a, b);
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

template <typename T, int N, typename F, typename ...Args>
vector<T, N> map(F f, Args ...args) {
    vector<T, N>::template check_all_dims(args...);
    vector<T, N> r;
    for (int i = 0; i < N; ++i) {
        r[i] = f((args[i])...);
    }
    return r;
}

template <typename T, int N>
struct Zero<vector<T, N>> {
    static vector<T, N> zero() {
        return vector<T, N>(::zero<T>());
    } 
};

template <typename T, int N_>
struct Dim<vector<T, N_>> {
    static const int N = N_;
};
template <typename T, int N>
struct base_type<vector<T, N>> {
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
#endif


typedef vector<real, 2> real2;
typedef vector<real, 3> real3;
typedef vector<real, 4> real4;
typedef vector<real, 8> real8;
typedef vector<real, 16> real16;


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
#endif


#ifdef UNIT_TEST
#include <catch.hpp>
#include "test.hpp"

class VecTestRng : public TestRng {
public:
    inline VecTestRng(uint32_t seed) : TestRng(seed) {}
    template <int N>
    vector<real, N> vec_normal() {
        return map<real, N>([this]() { return normal(); });
    }
    template <int N>
    vector<real, N> vec_uniform() {
        return map<real, N>([this]() { return uniform(); });
    }
};

template <typename T, int N>
class VecApprox {
private:
    typedef vector<T, N> vec_t;
    vector<T, N> v;

public:
    VecApprox(vector<T, N> c) : v(c) {}

    bool operator==(vector<T, N> a) {
        for (int i = 0; i < N; ++i) {
            if (a[i] != approx(v[i])) {
                return false;
            }
        }
        return true;
    }

    friend std::ostream &operator<<(std::ostream &s, VecApprox a) {
        return s << a.v;
    }
};

TEST_DEFINE_CMP_OPS(
    template <typename T COMMA int N>,
    VecApprox<T COMMA N>, vector<T COMMA N>
)

template <typename T, int N>
VecApprox<T, N> approx(vector<T, N> v) {
    return VecApprox<T, N>(v);
}

#endif
