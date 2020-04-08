#pragma once

#ifdef HOST
#include <cmath>
#endif

#include "real.hpp"
#include "vector.hpp"


template <typename C>
class complex;

template <typename T>
T &re(complex<T> &z);
template <typename T>
const T &re(const complex<T> &z);
template <typename T>
T &im(complex<T> &z);
template <typename T>
const T &im(const complex<T> &z);


template <typename C>
class complex {
private:
    typedef typename BaseType<C>::type T;
    static const int CN = Dim<C>::N;
    static const int N = 2*CN;
    vector<T, N> v;

public:
    complex() = default;
    explicit complex(C r, C i=zero<C>()) {
        re() = r;
        im() = i;
    }
    explicit complex(vector<T, N> x) : v(x) {}
    template <typename ...Args>
    explicit complex(Args ...args) : v(args...) {}

    C &re() {
        return ::re(*this);
    }
    const C &re() const {
        return ::re(*this);
    }
    C &im() {
        return ::im(*this);
    }
    const C &im() const {
        return ::im(*this);
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

    friend complex operator+(complex a) {
        return a;
    }
    friend complex operator-(complex a) {
        return complex(-a.v);
    }

    // TODO: Add optimized versions of operators for particular cases.

    friend complex operator+(complex a, complex b) {
        return complex(a.v + b.v);
    }
    friend complex operator-(complex a, complex b) {
        return complex(a.v - b.v);
    }

    friend complex operator*(complex a, complex b) {
        // Cayley-Dickson construction
        return complex(
            a.re()*b.re() - conj(b.im())*a.im(),
            b.im()*a.re() + a.im()*conj(b.re())
        );
    }
    friend complex operator*(complex a, T b) {
        return complex(a.v*b);
    }
    friend complex operator*(T a, complex b) {
        return complex(a*b.v);
    }
    friend complex operator/(complex a, complex b) {
        return a*inverse(b);
    }
    friend complex operator/(complex a, T b) {
        return complex(a.v/b);
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

template <typename T>
T &re(complex<T> &z) {
    return z[0];
}
template <typename T>
const T &re(const complex<T> &z) {
    return z[0];
}
template <typename T>
T &im(complex<T> &z) {
    return z[1];
}
template <typename T>
const T &im(const complex<T> &z) {
    return z[1];
}
template <typename T>
complex<T> &re(complex<complex<T>> &z) {
    return *reinterpret_cast<complex<T>*>(
        &z.vec().template slice<0, complex<T>::N>()
    );
}
template <typename T>
const complex<T> &re(const complex<complex<T>> &z) {
    return *reinterpret_cast<const complex<T>*>(
        &z.vec().template slice<0, complex<T>::N>()
    );
}
template <typename T>
complex<T> &im(complex<complex<T>> &z) {
    return *reinterpret_cast<complex<T>*>(
        &z.vec().template slice<complex<T>::N, complex<T>::N>()
    );
}
template <typename T>
const complex<T> &im(const complex<complex<T>> &z) {
    return *reinterpret_cast<const complex<T>*>(
        &z.vec().template slice<complex<T>::N, complex<T>::N>()
    );
}

template <typename C>
struct Dim<complex<C>> {
    static const int N = 2*Dim<C>::N;
};
template <typename C>
struct BaseType<complex<C>> {
    typedef typename BaseType<C>::type type;
};
template <typename C>
struct Conj<complex<C>> {
    static complex<C> conj(complex<C> z) {
        return complex<C>(::conj(z.re()), -z.im());
    }
};

template <typename T>
complex<T> exp(complex<T> p) {
    return exp(p.re())*complex<T>(cos(p.im()), sin(p.im()));
}
// TODO: Add (complex, complex) power.
template <typename T>
complex<T> pow(complex<T> a, T p) {
    T r = pow(abs2(a), p/2);
    T phi = p*atan2(a.im(), a.re());
    return complex<T>(r*cos(phi), r*sin(phi));
}
template <typename T>
complex<T> sqrt(complex<T> a) {
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


#ifdef UNIT_TEST
#include <catch.hpp>

#include "test.hpp"

class CompTestRng : public VecTestRng {
public:
    inline CompTestRng(uint32_t seed) : VecTestRng(seed) {}
    comp complex_normal();
    comp complex_unit();
    comp complex_nonzero();
};

template <typename C>
class CompApprox {
public:
    complex<C> v;
    CompApprox(complex<C> c) : v(c) {}
    bool operator==(complex<C> a) {
        for (int i = 0; i < v.vec().size(); ++i) {
            if (a[i] != approx(v[i])) {
                return false;
            }
        }
        return true;
    }
    bool operator==(typename BaseType<C>::type a) {
        if (Approx(v[0]) != a) {
            return false;
        }
        for (int i = 1; i < v.vec().size(); ++i) {
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
bool operator==(complex<C> a, CompApprox<C> b) {
    return b == a;
}
template <typename C>
bool operator!=(CompApprox<C> a, complex<C> b) {
    return !(a == b);
}
template <typename C>
bool operator!=(complex<C> a, CompApprox<C> b) {
    return b != a;
}
template <typename C>
bool operator==(typename BaseType<C>::type a, CompApprox<C> b) {
    return b == a;
}
template <typename C>
bool operator!=(CompApprox<C> a, typename BaseType<C>::type b) {
    return !(a == b);
}
template <typename C>
bool operator!=(typename BaseType<C>::type a, CompApprox<C> b) {
    return b != a;
}

template <typename C>
bool operator==(Approx a, complex<C> b) {
    if (b[0] != a) {
        return false;
    }
    for (int i = 1; i < b.vec().size(); ++i) {
        if (b[i] != Approx(0)) {
            return false;
        }
    }
    return true;
}
template <typename C>
bool operator==(complex<C> a, Approx b) {
    return b == a;
}
template <typename C>
bool operator!=(Approx a, complex<C> b) {
    return !(a == b);
}
template <typename C>
bool operator!=(complex<C> a, Approx b) {
    return b != a;
}

template <typename C>
CompApprox<C> approx(complex<C> c) {
    return CompApprox<C>(c);
}

#endif

