#pragma once

#include "matrix_base.hpp"
#include "complex.hpp"

#ifdef HOST
#include <iostream>
#endif


#define DEFINE_MATRIX_MATH_FUNCTION_A(F) \
template <typename T, int M, int N> \
matrix<T, M, N> F(matrix<T, M, N> a) { \
    return matrix<T, M, N>::map([](T x) { return F(x); }, a); \
} \

#define DEFINE_MATRIX_MATH_FUNCTION_AB(F) \
template <typename T, int M, int N> \
matrix<T, M, N> F(matrix<T, M, N> a, matrix<T, M, N> b) { \
    return matrix<T, M, N>::map([](T x, T y) { return F(x, y); }, a, b); \
} \

namespace math {

DEFINE_MATRIX_MATH_FUNCTION_A(abs)
DEFINE_MATRIX_MATH_FUNCTION_AB(max)
DEFINE_MATRIX_MATH_FUNCTION_AB(min)
DEFINE_MATRIX_MATH_FUNCTION_A(acos)
DEFINE_MATRIX_MATH_FUNCTION_A(acosh)
DEFINE_MATRIX_MATH_FUNCTION_A(asin)
DEFINE_MATRIX_MATH_FUNCTION_A(asinh)
DEFINE_MATRIX_MATH_FUNCTION_A(atan)
DEFINE_MATRIX_MATH_FUNCTION_AB(atan2)
DEFINE_MATRIX_MATH_FUNCTION_A(atanh)
DEFINE_MATRIX_MATH_FUNCTION_A(ceil)
DEFINE_MATRIX_MATH_FUNCTION_A(cos)
DEFINE_MATRIX_MATH_FUNCTION_A(cosh)
DEFINE_MATRIX_MATH_FUNCTION_A(erfc)
DEFINE_MATRIX_MATH_FUNCTION_A(erf)
DEFINE_MATRIX_MATH_FUNCTION_A(exp)
DEFINE_MATRIX_MATH_FUNCTION_A(floor)
DEFINE_MATRIX_MATH_FUNCTION_AB(mod)
DEFINE_MATRIX_MATH_FUNCTION_A(log)
DEFINE_MATRIX_MATH_FUNCTION_AB(pow)
DEFINE_MATRIX_MATH_FUNCTION_A(round)
DEFINE_MATRIX_MATH_FUNCTION_A(sqrt)
DEFINE_MATRIX_MATH_FUNCTION_A(sin)
DEFINE_MATRIX_MATH_FUNCTION_A(sinh)
DEFINE_MATRIX_MATH_FUNCTION_A(tan)
DEFINE_MATRIX_MATH_FUNCTION_A(tanh)
DEFINE_MATRIX_MATH_FUNCTION_A(tgamma)
template <typename T, int M, int N>
matrix<T, M, N> clamp(matrix<T, M, N> a, matrix<T, M, N> b, matrix<T, M, N> c) {
    return matrix<T, M, N>::map(
        [](T x, T y, T z) { return clamp(x, y, z); },
        a, b, c
    );
}
template <typename T, int M, int N>
pair<matrix<T, M, N>, matrix<T, M, N>> fract(matrix<T, M, N> a) {
    pair<matrix<T, M, N>, matrix<T, M, N>> p;
    for (int i = 0; i < M*N; ++i) {
        pair<T, T> x = fract(a[i]);
        p.first[i] = x.first;
        p.second[i] = x.second;
    }
    return p;
}

} // namespace math


template <typename T, int N>
matrix<T, N, N> normalize(matrix<T, N, N> m) {
    return m/math::pow(det(m), 1_r/N);
}
template <typename T>
matrix<T, 2, 2> normalize(matrix<T, 2, 2> m) {
    return m/math::sqrt(det(m));
}

template <typename T>
void eigen(
    matrix<T, 2, 2> m,
    matrix<T, 2, 2> *l,
    matrix<T, 2, 2> *v,
    bool normalized=false
) {
    const T T0 = zero<T>(), T1 = one<T>();
    const T D = normalized ? T1 : det(m);
    if (norm_l1(m[1]) < EPS && norm_l1(m[2]) < EPS) {
        *l = matrix<T, 2, 2>(m[0], T0, T0, m[3]);
        *v = matrix<T, 2, 2>(T1, T0, T0, T1);
    } else {
        T m0p3 = m[0] + m[3];
        T ad = m0p3/2;
        T dis = math::sqrt(ad*ad - D);
        if (norm_l1(dis) > EPS) {
            *l = matrix<T, 2, 2>(ad + dis, T0, T0, ad - dis);
            if (norm_l1(m[1]) > EPS) {
                *v = matrix<T, 2, 2>(
                    m[1], m[1],
                    (*l)[0] - m[0], (*l)[3] - m[0]
                );
            } else {
                *v = matrix<T, 2, 2>(
                    (*l)[0] - m[3], (*l)[3] - m[3],
                    m[2], m[2]
                );
            }
        } else {
            T m0m3 = m[0] - m[3];
            *l = matrix<T, 2, 2>(ad, T1, T0, ad);
            if (norm_l1(m[1]) > EPS) {
                T g = 4*m[1]*m[1] + m0m3*m0m3;
                *v = matrix<T, 2, 2>(
                    m[1], 2*m[1]*m0m3/g,
                    -m0m3/2, 4*m[1]*m[1]/g
                );
            } else {
                T g = 4*m[2]*m[2] + m0m3*m0m3;
                *v = matrix<T, 2, 2>(
                    m0m3/2, 4*m[2]*m[2]/g,
                    m[2], -2*m[2]*m0m3/g
                );
            }
        }
        if (normalized) {
            *v = normalize(*v);
        }
    }
}

template <typename T>
matrix<T, 2, 2> pow(matrix<T, 2, 2> m, base_type<T> p, bool normalized=false) {
    matrix<T, 2, 2> r;
    matrix<T, 2, 2> x = m - one<matrix<T, 2, 2>>();
    base_type<T> y = norm_l1(x);
    // FIXME: Why 1e4?
    if (y*y > 1e4*EPS) {
        const T T0 = zero<T>();
        matrix<T, 2, 2> k, j, v;
        eigen(m, &j, &v, normalized);

        if (norm_l1(j[1]) < EPS) {
            k = matrix<T, 2, 2>(
                math::pow(j[0], p), T0,
                T0, math::pow(j[3], p)
            );
        } else {
            // Assume j[0] == j[3]
            T l = math::pow(j[0], p);
            k = matrix<T, 2, 2>(
                l, p*(l/j[0])*j[1],
                T0, l
            );
        }
        r = dot(dot(v, k), !v);
    } else {
        r = one<matrix<T, 2, 2>>() + p*x;
        if (normalized) {
            r = normalize(r);
        }
    }
    return r;
}


#ifdef HOST

namespace device {
template <typename T, int M, int N>
struct matrix {
    T s[M*N];
};
}

template <typename T, int M, int N>
struct ToDevice<matrix<T, M, N>> {
    typedef device::matrix<device_type<T>, M, N> type;
    static type to_device(matrix<T, M, N> v) {
        type o;
        for (int i = 0; i < M*N; ++i) {
            o.s[i] = ::to_device(v[i]);
        }
        return o;
    }
};

#endif

#define DEFINE_MATRIX_TYPENAMES(T) \
typedef matrix<T, 2, 2> T##2x2; \
typedef matrix<T, 2, 3> T##2x3; \
typedef matrix<T, 2, 4> T##2x4; \
typedef matrix<T, 3, 2> T##3x2; \
typedef matrix<T, 3, 3> T##3x3; \
typedef matrix<T, 3, 4> T##3x4; \
typedef matrix<T, 4, 2> T##4x2; \
typedef matrix<T, 4, 3> T##4x3; \
typedef matrix<T, 4, 4> T##4x4; \

DEFINE_MATRIX_TYPENAMES(real)
DEFINE_MATRIX_TYPENAMES(comp)


#ifdef HOST
template <typename T, int M, int N>
std::ostream &operator<<(std::ostream &s, matrix<T, M, N> v) {
    s << std::endl;
    for (int i = 0; i < M; ++i) {
        s << "|\t";
        for (int j = 0; j < N; ++j) {
            s << v(i, j) << "\t";
        }
        s << "|" << std::endl;
    }
    return s;
}
#endif


#ifdef TEST_UNIT
#include <catch.hpp>
#include "test.hpp"

namespace test {

template <typename T, int M, int N>
class Distrib<matrix<T, M, N>> : public Rng {
public:
    matrix<T, M, N> normal() {
        return matrix<T, M, N>::map([this]() {
            return d<T>().normal();
        });
    }
    matrix<T, M, N> uniform() {
        return matrix<T, M, N>::map([this]() {
            return d<T>().uniform();
        });
    }
    enable_if<M == N, matrix<T, M, N>> invertible() {
        matrix<T, M, N> r;
        do {
            r = normal();
        } while(norm_l1(det(r)) < EPS);
        return r;
    }
    enable_if<M == N, matrix<T, M, N>> normalized() {
        return normalize(invertible());
    }
};

template <typename T, int M, int N>
class MatApprox {
private:
    matrix<T, M, N> v;

public:
    MatApprox(matrix<T, M, N> c) : v(c) {}

    bool operator==(matrix<T, M, N> a) const {
        for (int i = 0; i < M; ++i) {
            for (int j = 0; j < N; ++j) {
                if (a(i, j) != approx(v(i, j))) {
                    return false;
                }
            }
        }
        return true;
    }
    TEST_DEFINE_CMP_OPS(, friend, MatApprox, matrix<T COMMA M COMMA N>)

    friend std::ostream &operator<<(std::ostream &s, MatApprox a) {
        return s << a.v;
    }
};

template <typename T, int M, int N>
MatApprox<T, M, N> approx(matrix<T, M, N> v) {
    return MatApprox<T, M, N>(v);
}

} // namespace test

#endif
