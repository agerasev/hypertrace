#pragma once

#ifdef HOST
#include <cmath>
#endif

#include "real.hpp"
#include "vector.hpp"


template <typename C>
class complex;

template <typename T>
struct is_complex {
    static constexpr bool value = false;
};
template <typename T>
struct is_complex<complex<T>> {
    static constexpr bool value = true;
};

template <typename C>
class complex {
private:
    typedef typename base_type<C>::type T;
    static const int CN = Dim<C>::N;
    static const int N = 2*CN;
    vector<T, N> v;

public:
    complex() = default;
    // Remove duplicate constructor if T == U
    template<typename U = T>
    explicit complex(
        typename enable_if<is_complex<C>::value, U>::type r
    ) : v(zero<T>()) {
        v[0] = r;
    }
    explicit complex(C r, C i=zero<C>()) {
        re() = r;
        im() = i;
    }
    explicit complex(vector<T, N> x) : v(x) {}
    template <typename ...Args, typename enable_if<(sizeof...(Args) > 2)>::type* = nullptr>
    explicit complex(Args ...args) : v(args...) {}

    template <typename U = C>
    typename enable_if<!is_complex<U>::value, U>::type &re() {
        return v[0];
    }
    template <typename U = C>
    const typename enable_if<!is_complex<U>::value, U>::type &re() const {
        return v[0];
    }
    template <typename U = C>
    typename enable_if<!is_complex<U>::value, U>::type &im() {
        return v[1];
    }
    template <typename U = C>
    const typename enable_if<!is_complex<U>::value, U>::type &im() const {
        return v[1];
    }
    template <typename U = C>
    typename enable_if<is_complex<U>::value, U>::type &re() {
        return *reinterpret_cast<U*>(
            &v.template slice<0, U::size()>()
        );
    }
    template <typename U = C>
    const typename enable_if<is_complex<U>::value, U>::type &re() const {
        return *reinterpret_cast<const U*>(
            &v.template slice<0, U::size()>()
        );
    }
    template <typename U = C>
    typename enable_if<is_complex<U>::value, U>::type &im() {
        return *reinterpret_cast<U*>(
            &v.template slice<U::size(), U::size()>()
        );
    }
    template <typename U = C>
    const typename enable_if<is_complex<U>::value, U>::type &im() const {
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

    friend T abs(complex x) {
        return length(x.v);
    }
    friend T abs2(complex x) {
        return dot(x.v, x.v);
    }
    friend complex inverse(complex x) {
        return conj(x)/abs2(x);
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
    friend complex operator+(
        complex a,
        typename enable_if<is_complex<C>::value, U>::type b
    ) {
        a[0] += b;
        return a;
    }
    template <typename U = T>
    friend complex operator+(
        typename enable_if<is_complex<C>::value, U>::type a,
        complex b
    ) {
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
    friend complex operator-(
        complex a,
        typename enable_if<is_complex<C>::value, U>::type b
    ) {
        a[0] -= b;
        return a;
    }
    template <typename U = T>
    friend complex operator-(
        typename enable_if<is_complex<C>::value, U>::type a,
        complex b
    ) {
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
    friend complex operator*(
        complex a,
        typename enable_if<is_complex<C>::value, U>::type b
    ) {
        return complex(a.re()*b, a.im()*conj(b));
    }
    template <typename U = C>
    friend complex operator*(
        typename enable_if<is_complex<C>::value, U>::type a,
        complex b
    ) {
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
    friend complex operator/(
        complex a,
        typename enable_if<is_complex<C>::value, U>::type b
    ) {
        return a*inverse(b);
    }
    template <typename U = C>
    friend complex operator/(
        typename enable_if<is_complex<C>::value, U>::type a,
        complex b
    ) {
        return a*inverse(b);
    }
    friend complex operator/(complex a, T b) {
        return complex(a.v/b);
    }
    friend complex operator/(T a, complex b) {
        return complex(conj(b).v*(a/abs2(b)));
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


template <typename C>
struct Zero<complex<C>> {
    static complex<C> zero() {
        return complex<C>(0);
    }
};

template <typename C>
struct One<complex<C>> {
    static complex<C> one() {
        return complex<C>(1);
    }
};

template <typename C>
struct Dim<complex<C>> {
    static const int N = 2*Dim<C>::N;
};
template <typename C>
struct base_type<complex<C>> {
    typedef typename base_type<C>::type type;
};
template <typename C>
struct Conj<complex<C>> {
    static complex<C> conj(complex<C> z) {
        return complex<C>(::conj(z.re()), -z.im());
    }
};

template <typename C>
typename base_type<C>::type dot(complex<C> a, complex<C> b) {
    return dot(a.vec(), b.vec());
}

template <typename T>
typename enable_if<!is_complex<T>::value, complex<T>>::type exp(complex<T> p) {
    return exp(p.re())*complex<T>(cos(p.im()), sin(p.im()));
}
// TODO: Add (complex, complex) power.
template <typename T>
typename enable_if<!is_complex<T>::value, complex<T>>::type pow(complex<T> a, T p) {
    T r = pow(abs2(a), p/2);
    T phi = p*atan2(a.im(), a.re());
    return complex<T>(r*cos(phi), r*sin(phi));
}
template <typename T>
typename enable_if<!is_complex<T>::value, complex<T>>::type sqrt(complex<T> a) {
    T r = sqrt(abs(a));
    T phi = T(0.5)*atan2(a.im(), a.re());
    return complex<T>(r*cos(phi), r*sin(phi));
}

#ifdef HOST
template <typename C>
std::ostream &operator<<(std::ostream &s, complex<C> c) {
    return s << c.vec();
}
#endif

template <typename T>
using quaternion = complex<complex<T>>;
template <typename T>
using octonion = complex<complex<complex<T>>>;
template <typename T>
using sedenion = complex<complex<complex<complex<T>>>>;

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

class CompTestRng : public VecTestRng {
public:
    inline CompTestRng(uint32_t seed) : VecTestRng(seed) {}
    comp comp_normal();
    comp comp_unit();
    comp comp_nonzero();
};

class QuatTestRng : public CompTestRng {
public:
    inline QuatTestRng(uint32_t seed) : CompTestRng(seed) {}
    quat quat_normal();
    quat quat_unit();
    quat quat_nonzero();
};

template <typename C>
class CompApprox {
private:
    complex<C> v;

public:
    CompApprox(complex<C> c) : v(c) {}

    bool operator==(complex<C> a) {
        for (int i = 0; i < v.size(); ++i) {
            if (a[i] != approx(v[i])) {
                return false;
            }
        }
        return true;
    }
    bool operator==(complex<complex<C>> a) {
        return (*this) == a.re() && approx(zero<complex<C>>()) == a.im();
    }
    template <typename U = C>
    bool operator==(typename enable_if<is_complex<C>::value, U>::type a) {
        return approx(v.re()) == a && approx(v.im()) == zero<C>();
    }
    bool operator==(typename base_type<C>::type a) {
        if (Approx(v[0]) != a) {
            return false;
        }
        for (int i = 1; i < v.size(); ++i) {
            if (Approx(v[i]) != 0) {
                return false;
            }
        }
        return true;
    }

    friend std::ostream &operator<<(std::ostream &s, CompApprox a) {
        return s << a.v;
    }
};

template <typename C>
bool operator==(Approx a, complex<C> b) {
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
    template <typename C>,
    CompApprox<C>, complex<complex<C>>
)
TEST_DEFINE_CMP_OPS(
    template <typename C>,
    CompApprox<C>, complex<C>
)
TEST_DEFINE_CMP_OPS(
    template <typename C>,
    CompApprox<C>,
    typename enable_if<is_complex<C>::value COMMA C>::type
)
TEST_DEFINE_CMP_OPS(
    template <typename C>,
    CompApprox<C>, typename base_type<C>::type
)
TEST_DEFINE_CMP_OPS(
    template <typename C>,
    Approx, complex<C>
)

template <typename C>
CompApprox<C> approx(complex<C> c) {
    return CompApprox<C>(c);
}

#endif

