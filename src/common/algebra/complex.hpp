#pragma once


#include <real.h>
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
template <typename T>
struct Degree {
    static const int value = 0;
};
template <typename T, int D>
struct Degree<complex<T, D>> {
    static const int value = D;
};
template <typename T>
constexpr int degree() {
    return Degree<T>::value;
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

template <typename T, int M, int N>
class matrix;

template <typename T, int D>
matrix<complex_type<T, D - 1>, 2, 2> lower(complex<T, D> c);
template <typename T, int D>
enable_if<
    !is_complex<complex_type<T, D - 1>>(),
    matrix<T, (1 << D), (1 << D)>
> to_matrix(complex<T, D> c);
template <typename T, int D>
enable_if<
    is_complex<complex_type<T, D - 1>>(),
    matrix<T, (1 << D), (1 << D)>
> to_matrix(complex<T, D> c);


template <typename T, int D>
class complex {
private:
    typedef complex_type<T, D - 1> C;
    static const int CN = dim<C>();
    static const int N = 1 << D;
    
    vector<T, N> v;

    inline T forward(T x) {
        return x;
    }
    template <int E>
    vector<T, dim<complex<T, E>>()> forward(complex<T, E> c) {
        return c.vec();
    }
    template <int N>
    vector<T, N> forward(vector<T, N> v) {
        return v;
    }
public:
    complex() = default;
    complex(T r) : v(zero<T>()) {
        v[0] = r;
    }
    template <int E>
    complex(complex<T, E> c) : v(zero<T>()) {
        v.template slice<0, dim<complex<T, E>>()>() = c.vec();
    }
    explicit complex(vector<T, N> x) : v(x) {}
    template <typename ...Args, enable_if<(sizeof...(Args) > 1)>* = nullptr>
    explicit complex(Args ...args) : v(forward(args)...) {}

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

    matrix<C, 2, 2> lower() const {
        return ::lower(*this);
    }
    matrix<T, N, N> to_matrix() const {
        return ::to_matrix(*this);
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
struct Conj<complex<T, D>> {
    static complex<T, D> conj(complex<T, D> z) {
        return complex<T, D>(::conj(z.re()), -z.im());
    }
};

template <typename T, int D>
struct Dim<complex<T, D>> {
    static const int value = 1 << D;
};
template <typename T, int D>
struct BaseType<complex<T, D>> {
    typedef base_type<T> type;
};
template <typename T, int D>
struct ElementType<complex<T, D>> {
    typedef T type;
};

template <typename S, typename T, int D>
struct Convert<complex<S, D>, complex<T, D>> {
    static complex<S, D> convert(complex<T, D> v) {
        return complex<S, D>(::convert<vector<S, (1<<D)>>(v.vec()));
    }
};

template <typename T, int D>
T length(complex<T, D> x) {
    return length(x.vec());
}
template <typename T, int D>
T length2(complex<T, D> x) {
    return length2(x.vec());
}
template <typename T, int D>
complex<T, D> normalize(complex<T, D> x) {
    return x/length(x);
}
template <typename T, int D>
complex<T, D> inverse(complex<T, D> x) {
    return conj(x)/length2(x);
}

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
    T r = math::pow(length2(a), p/2);
    T phi = p*math::atan2(a.im(), a.re());
    return complex<T>(r*math::cos(phi), r*math::sin(phi));
}
template <typename T>
enable_if<!is_complex<T>(), complex<T>> sqrt(complex<T> a) {
    T r = math::sqrt(length(a));
    T phi = T(0.5)*math::atan2(a.im(), a.re());
    return complex<T>(r*math::cos(phi), r*math::sin(phi));
}

} // namespace math


#ifdef HOST

namespace device {
template <typename T, int D=1>
using complex = device::vector<T, (1<<D)>;
}

template <typename T, int D>
struct ToDevice<complex<T, D>> {
    typedef device::complex<device_type<T>, D> type;
    static type to_device(complex<T, D> v) {
        return ::to_device(v.vec());
    }
};

#endif

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
        return complex<T, D>(d<V>().normal());
    }
    complex<T, D> nonzero() {
        return complex<T, D>(d<V>().nonzero());
    }
    complex<T, D> unit() {
        return complex<T, D>(d<V>().unit());
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
    template <int E>
    bool operator==(complex<T, E> a) {
        static const int G = E > D ? E : D;
        return CompApprox<T, G>(v) == complex<T, G>(a);
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
    CompApprox<T COMMA D>, complex<T COMMA D>
)
TEST_DEFINE_CMP_OPS(
    template <typename T COMMA int D COMMA int E>,
    CompApprox<T COMMA D>, complex<T COMMA E>
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
