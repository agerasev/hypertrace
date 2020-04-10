#pragma once

#ifdef HOST
#include <cmath>
#endif

#include "real.hpp"
#include "vector.hpp"


template <typename T, int D=1>
class complex;

// Is complex
template <typename X>
struct IsComplex {
    static constexpr bool value = false;
};
template <typename T, int D>
struct IsComplex<complex<T, D>> {
    static constexpr bool value = true;
};
template <typename C>
constexpr bool is_complex() {
    return IsComplex<C>::value;
}

// Conjugate
template <typename T>
struct Conj {
    static T conj(T x) {
        return x;
    }
};
template <typename T>
T conj(T x) {
    return Conj<T>::conj(x);
}

template <typename T, int D>
struct ComplexType {
    typedef complex<T, D> type;
};
template <typename T>
struct ComplexType<T, 0> {
    typedef T type;
};
template <typename T, int D>
using complex_type = typename ComplexType<T, D>::type;


template <typename T, int D>
class complex {
private:
    typedef complex_type<T, D - 1> C;
    static const int CN = dim<C>();
    static const int N = 1 << D;
    
    vector<T, N> v;

public:
    complex() = default;
    // Remove duplicate constructor if T == U
    template<typename U = T>
    complex(enable_if<is_complex<C>(), U> r) : v(zero<T>()) {
        v[0] = r;
    }
    complex(C r, C i=zero<C>()) {
        re() = r;
        im() = i;
    }
    explicit complex(vector<T, N> x) : v(x) {}
    template <typename ...Args, enable_if<(sizeof...(Args) > 2)>* = nullptr>
    explicit complex(Args ...args) : v(args...) {}

    template <typename U = C>
    enable_if<!is_complex<U>(), U> &re() {
        return v[0];
    }
    template <typename U = C>
    const enable_if<!is_complex<U>(), U> &re() const {
        return v[0];
    }
    template <typename U = C>
    enable_if<!is_complex<U>(), U> &im() {
        return v[1];
    }
    template <typename U = C>
    const enable_if<!is_complex<U>(), U> &im() const {
        return v[1];
    }
    template <typename U = C>
    enable_if<is_complex<U>(), U> &re() {
        return *reinterpret_cast<U*>(
            &v.template slice<0, U::size()>()
        );
    }
    template <typename U = C>
    const enable_if<is_complex<U>(), U> &re() const {
        return *reinterpret_cast<const U*>(
            &v.template slice<0, U::size()>()
        );
    }
    template <typename U = C>
    enable_if<is_complex<U>(), U> &im() {
        return *reinterpret_cast<U*>(
            &v.template slice<U::size(), U::size()>()
        );
    }
    template <typename U = C>
    const enable_if<is_complex<U>(), U> &im() const {
        return *reinterpret_cast<const U*>(
            &v.template slice<U::size(), U::size()>()
        );
    }

    T *data() {
        return v.data();
    }
    const T *data() const {
        return v.data();
    }
    T &operator[](int i) {
        return v[i];
    }
    const T &operator[](int i) const {
        return v[i];
    }
    static constexpr int size() {
        return N;
    }
    vector<T, N> &vec() {
        return v;
    }
    const vector<T, N> &vec() const {
        return v;
    }

    friend T length(complex x) {
        return length(x.v);
    }
    friend T length2(complex x) {
        return length2(x.v);
    }
    friend complex normalize(complex x) {
        return x/length(x);
    }
    friend complex inverse(complex x) {
        return conj(x)/length2(x);
    }

    friend complex operator~(complex a) {
        return conj(a);
    }
    friend complex operator!(complex a) {
        return inverse(a);
    }

    // Addition
    friend complex operator+(complex a) {
        return a;
    }
    friend complex operator-(complex a) {
        return complex(-a.v);
    }

    friend complex operator+(complex a, complex b) {
        return complex(a.v + b.v);
    }
    friend complex operator+(complex a, C b) {
        a.re() += b;
        return a;
    }
    friend complex operator+(C a, complex b) {
        b.re() = a + b.re();
        return b;
    }
    template <typename U = T>
    friend complex operator+(complex a, enable_if<is_complex<C>(), U> b) {
        a[0] += b;
        return a;
    }
    template <typename U = T>
    friend complex operator+(enable_if<is_complex<C>(), U> a, complex b) {
        b[0] = a + b[0];
        return b;
    }

    // Subtraction
    friend complex operator-(complex a, complex b) {
        return complex(a.v - b.v);
    }
    friend complex operator-(complex a, C b) {
        a.re() -= b;
        return a;
    }
    friend complex operator-(C a, complex b) {
        b.re() = a - b.re();
        return b;
    }
    template <typename U = T>
    friend complex operator-(complex a, enable_if<is_complex<C>(), U> b) {
        a[0] -= b;
        return a;
    }
    template <typename U = T>
    friend complex operator-(enable_if<is_complex<C>(), U> a, complex b) {
        b[0] = a - b[0];
        return b;
    }

    // Multiplication
    friend complex operator*(complex a, complex b) {
        // Cayley-Dickson construction
        return complex(
            a.re()*b.re() - conj(b.im())*a.im(),
            b.im()*a.re() + a.im()*conj(b.re())
        );
    }
    template <typename U = C>
    friend complex operator*(complex a, enable_if<is_complex<C>(), U> b) {
        return complex(a.re()*b, a.im()*conj(b));
    }
    template <typename U = C>
    friend complex operator*(enable_if<is_complex<C>(), U> a, complex b) {
        return complex(a*b.re(), b.im()*a);
    }
    friend complex operator*(complex a, T b) {
        return complex(a.v*b);
    }
    friend complex operator*(T a, complex b) {
        return complex(a*b.v);
    }

    // Division
    friend complex operator/(complex a, complex b) {
        return a*inverse(b);
    }
    template <typename U = C>
    friend complex operator/(complex a, enable_if<is_complex<C>(), U> b) {
        return a*inverse(b);
    }
    template <typename U = C>
    friend complex operator/(enable_if<is_complex<C>(), U> a, complex b) {
        return a*inverse(b);
    }
    friend complex operator/(complex a, T b) {
        return complex(a.v/b);
    }
    friend complex operator/(T a, complex b) {
        return complex(conj(b).v*(a/length2(b)));
    }

    complex &operator+=(complex a) {
        return *this = *this + a;
    }
    complex &operator+=(T a) {
        return *this = *this + a;
    }
    complex &operator-=(complex a) {
        return *this = *this - a;
    }
    complex &operator-=(T a) {
        return *this = *this - a;
    }
    complex &operator*=(complex a) {
        return *this = *this * a;
    }
    complex &operator*=(T a) {
        return *this = *this * a;
    }
    complex &operator/=(complex a) {
        return *this = *this / a;
    }
    complex &operator/=(T a) {
        return *this = *this / a;
    }
};

// Complex of degree 0 is real number
template <typename T>
class complex<T, 0> {};

template <typename T, int D>
struct Zero<complex<T, D>> {
    static complex<T, D> zero() {
        return complex<T, D>(::zero<T>());
    }
};

template <typename T, int D>
struct One<complex<T, D>> {
    static complex<T, D> one() {
        return complex<T, D>(::one<T>());
    }
};

template <typename T, int D>
struct Dim<complex<T, D>> {
    static const int N = 1 << D;
};
template <typename T, int D>
struct BaseType<complex<T, D>> {
    typedef base_type<T> type;
};
template <typename T, int D>
struct Conj<complex<T, D>> {
    static complex<T, D> conj(complex<T, D> z) {
        return complex<T, D>(::conj(z.re()), -z.im());
    }
};

template <typename T, int D>
T dot(complex<T, D> a, complex<T, D> b) {
    return dot(a.vec(), b.vec());
}

template <typename X, int D>
struct Sequence<complex<X, D>> {
private:
    typedef base_type<X> T;
    typedef complex<X, D> C;
    typedef vector<T, dim<C>()> V;
public:
    template <typename F>
    static C map(F f, C a) {
        return C(Sequence<V>::map(f, a.vec()));
    }
    template <typename F>
    static T reduce(F f, T t, C a) {
        return Sequence<V>::reduce(f, t, a.vec());
    }
};

namespace math {

template <typename T>
enable_if<!is_complex<T>(), complex<T>> exp(complex<T> p) {
    return exp(p.re())*complex<T>(cos(p.im()), sin(p.im()));
}
// TODO: Add (complex, complex) power.
template <typename T>
enable_if<!is_complex<T>(), complex<T>> pow(complex<T> a, T p) {
    T r = pow(length2(a), p/2);
    T phi = p*atan2(a.im(), a.re());
    return complex<T>(r*cos(phi), r*sin(phi));
}
template <typename T>
enable_if<!is_complex<T>(), complex<T>> sqrt(complex<T> a) {
    T r = sqrt(length(a));
    T phi = T(0.5)*atan2(a.im(), a.re());
    return complex<T>(r*cos(phi), r*sin(phi));
}

} // namespace math

#ifdef HOST
template <typename T, int D>
std::ostream &operator<<(std::ostream &s, complex<T, D> c) {
    return s << c.vec();
}
#endif

template <typename T>
using quaternion = complex<T, 2>;
template <typename T>
using octonion = complex<T, 3>;
template <typename T>
using sedenion = complex<T, 4>;

typedef complex<real> comp;
typedef quaternion<real> quat;


inline comp operator ""_i(unsigned long long x) {
    return comp(real(0), real(x));
}
inline comp operator ""_i(long double x) {
    return comp(real(0), real(x));
}

inline quat operator ""_j(unsigned long long x) {
    return quat(real(0), real(0), real(x), real(0));
}
inline quat operator ""_j(long double x) {
    return quat(real(0), real(0), real(x), real(0));
}

inline quat operator ""_k(unsigned long long x) {
    return quat(real(0), real(0), real(0), real(x));
}
inline quat operator ""_k(long double x) {
    return quat(real(0), real(0), real(0), real(x));
}


#ifdef UNIT_TEST
#include <catch.hpp>

#include "test.hpp"

namespace test {

using namespace math;

template <typename T, int D>
class Distrib<complex<T, D>> : public Rng {
private:
    typedef vector<T, dim<complex<T, D>>()> V;

public:
    complex<T, D> normal() {
        return complex<T, D>(distrib<V>().normal());
    }
    complex<T, D> nonzero() {
        V a;
        do {
            a = distrib<V>().normal();
        } while(length2(a) < EPS);
        return complex<T, D>(a);
    }
    complex<T, D> unit() {
        return normalize(nonzero());
    }
};

template <typename T, int D>
class CompApprox;
template <typename T, int D>
CompApprox<T, D> approx(complex<T, D> c);

template <typename T, int D>
class CompApprox {
private:
    complex<T, D> v;

public:
    CompApprox(complex<T, D> c) : v(c) {}

    bool operator==(complex<T, D> a) {
        for (int i = 0; i < v.size(); ++i) {
            if (a[i] != approx(v[i])) {
                return false;
            }
        }
        return true;
    }
    bool operator==(complex<T, D + 1> a) {
        return (*this) == a.re() && approx(zero<complex<T, D>>()) == a.im();
    }
    template <typename U = complex_type<T, D - 1>>
    bool operator==(enable_if<is_complex<U>(), U> a) {
        return approx(v.re()) == a && approx(v.im()) == zero<U>();
    }
    bool operator==(T a) {
        if (approx(v[0]) != a) {
            return false;
        }
        for (int i = 1; i < v.size(); ++i) {
            if (approx(v[i]) != 0) {
                return false;
            }
        }
        return true;
    }

    friend std::ostream &operator<<(std::ostream &s, CompApprox a) {
        return s << a.v;
    }
};

template <typename T, int D>
bool operator==(Approx a, complex<T, D> b) {
    if (a != b[0]) {
        return false;
    }
    for (int i = 1; i < b.size(); ++i) {
        if (Approx(0) != b[i]) {
            return false;
        }
    }
    return true;
}

TEST_DEFINE_CMP_OPS(
    template <typename T COMMA int D>,
    CompApprox<T COMMA D>, complex<T COMMA D + 1>
)
TEST_DEFINE_CMP_OPS(
    template <typename T COMMA int D>,
    CompApprox<T COMMA D>, complex<T COMMA D>
)
TEST_DEFINE_CMP_OPS(
    template <typename T COMMA int D>,
    CompApprox<T COMMA D>,
    enable_if<is_complex<complex_type<T COMMA D - 1>>() COMMA complex_type<T COMMA D - 1>>
)
TEST_DEFINE_CMP_OPS(
    template <typename T COMMA int D>,
    CompApprox<T COMMA D>, T
)
TEST_DEFINE_CMP_OPS(
    template <typename T COMMA int D>,
    Approx, complex<T COMMA D>
)

template <typename T, int D>
CompApprox<T, D> approx(complex<T, D> c) {
    return CompApprox<T, D>(c);
}

} // namespace test

#endif

