#pragma once

#ifdef HOST
#include <cmath>
#endif

#include "vector.hpp"

template <typename C>
class complex {
private:
    typedef typename BaseType<C>::type T;
    static const int CN = Dim<C>::N;
    static const int N = 2*CN;
    vector<T, N> v;

public:
    complex() = default;
    complex(vector<T, N> x) : v(x) {}
    complex(C r, C i=zero<C>()) {
        re() = r;
        im() = i;
    }
    template <typename ...Args>
    complex(Args ...args) : v(args...) {}

    template <typename S>
    friend S &re(complex<S> &z);
    template <typename S>
    friend const S &re(const complex<S> &z);
    template <typename S>
    friend S &im(complex<S> &z);
    template <typename S>
    friend const S &im(const complex<S> &z);

    C &re() {
        return re(*this);
    }
    const C &re() const {
        return re(*this);
    }
    C &im() {
        return im(*this);
    }
    const C &im() const {
        return im(*this);
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
            a.re()*b.re() - ~b.im()*a.im(),
            b.im()*a.re() + a.im()*~b.re()
        );
    }
    friend complex operator*(complex a, T b) {
        return complex(a.v*b);
    }
    friend complex operator*(T a, complex b) {
        return complex(a*b.v);
    }
    friend complex operator/(complex a, complex b) {
        return a*!b;
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
    return z.v[0];
}
template <typename T>
const T &re(const complex<T> &z) {
    return z.v[0];
}
template <typename T>
T &im(complex<T> &z) {
    return z.v[1];
}
template <typename T>
const T &im(const complex<T> &z) {
    return z.v[1];
}
template <typename T>
complex<T> &re(complex<complex<T>> &z) {
    return *reinterpret_cast<complex<T>*>(
        &z.v.template slice<0, complex<T>::N>()
    );
}
template <typename T>
const complex<T> &re(const complex<complex<T>> &z) {
    return *reinterpret_cast<const complex<T>*>(
        &z.v.template slice<0, complex<T>::N>()
    );
}
template <typename T>
complex<T> &im(complex<complex<T>> &z) {
    return *reinterpret_cast<complex<T>*>(
        &z.v.template slice<complex<T>::N, complex<T>::N>()
    );
}
template <typename T>
const complex<T> &im(const complex<complex<T>> &z) {
    return *reinterpret_cast<const complex<T>*>(
        &z.v.template slice<complex<T>::N, complex<T>::N>()
    );
}

template <typename C>
class Dim<complex<C>> {
    static const int N = 2*Dim<C>::N;
};
template <typename C>
class BaseType<complex<C>> {
    typedef typename BaseType<C>::type type;
};
template <typename C>
class Conj<complex<C>> {
    static complex<C> conj(complex<C> z) {
        return complex<C>(conj(z.re()), -z.im());
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

/*
#ifdef UNIT_TEST
#include <catch.hpp>

complex rand_c_normal(TestRng &rng);
complex rand_c_unit(TestRng &rng);
complex rand_c_nonzero(TestRng &rng);

#endif // UNIT_TEST
*/
