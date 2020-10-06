#pragma once

#include <real.h>
#include "complex_base.hpp"

namespace math {

template <typename T>
complex<T> exp(complex<T> p) {
    return exp(p.re())*complex<T>(cos(p.im()), sin(p.im()));
}
// TODO: Add (complex, complex) power.
template <typename T>
complex<T> pow(complex<T> a, T p) {
    T r = math::pow(length2(a), p/2);
    T phi = p*math::atan2(a.im(), a.re());
    return complex<T>(r*math::cos(phi), r*math::sin(phi));
}
template <typename T>
complex<T> sqrt(complex<T> a) {
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


#ifdef TEST_CATCH

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
    TEST_DEFINE_CMP_OPS(, friend, CompApprox, complex<T COMMA D>)

    template <int E>
    bool operator==(complex<T, E> a) {
        static const int G = E > D ? E : D;
        return CompApprox<T, G>(v) == complex<T, G>(a);
    }
    TEST_DEFINE_CMP_OPS(template <int E>, friend, CompApprox, complex<T COMMA E>)

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
    TEST_DEFINE_CMP_OPS(, friend, CompApprox, T)

    friend std::ostream &operator<<(std::ostream &s, CompApprox a) {
        return s << a.v;
    }
};

template <typename T, int D>
CompApprox<T, D> approx(complex<T, D> c) {
    return CompApprox<T, D>(c);
}

} // namespace test

#endif
