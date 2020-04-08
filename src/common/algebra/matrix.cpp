#include "matrix.hpp"

/*
#ifndef OPENCL
#include <math.h>
#endif // OPENCL

// Common functions

#define MAT_COMMON_C(T,M,N) \
\
MAT_MAP_C(T,M,N,neg,TF_(T,neg)) \
MAT_MAP2_C(T,M,N,add,TF_(T,add)) \
MAT_MAP2_C(T,M,N,sub,TF_(T,sub)) \
MAT_MAPC_C(T,M,N,mul,TF_(T,mul)) \
MAT_MAPC_C(T,M,N,div,TF_(T,div)) \


// Square matrix

#define MAT_SQUARE_C(T,N) \
MAT_(T,N,N) MATF_(T,N,N,one)() { \
    MAT_(T,N,N) r = MATF_(T,N,N,zero)(); \
    for (int i = 0; i < N; ++i) { \
        r.s[i*(N + 1)] = TF_(T,one)(); \
    } \
    return r; \
} \

#define MAT_SQUARE2_C(T) \
MAT_SQUARE_C(T,2) \
\
T MATF_(T,2,2,det)(MAT_(T,2,2) m) { \
    return TF_(T,sub)( \
        TF_(T,mul)(m.s[0], m.s[3]), \
        TF_(T,mul)(m.s[1], m.s[2]) \
    ); \
} \
\
MAT_(T,2,2) MATF_(T,2,2,inverse)(MAT_(T,2,2) m) { \
    T det = MATF_(T,2,2,det)(m); \
    return (MAT_(T,2,2)){.s = { \
        TF_(T,div)(m.s[3], det), TF_(T,div)(TF_(T,neg)(m.s[1]), det), \
        TF_(T,div)(TF_(T,neg)(m.s[2]), det), TF_(T,div)(m.s[0], det) \
    }}; \
} \


#define MAT_SQUARE3_C(T) \
MAT_SQUARE_C(T,3) \

#define MAT_SQUARE4_C(T) \
MAT_SQUARE_C(T,4) \


// Dot product

#define MAT_DOT_C(T,L,M,N) \
MAT_DOT_H(T,L,M,N) { \
    MAT_(T,L,N) r; \
    for (int i = 0; i < L; ++i) { \
        for (int j = 0; j < N; ++j) { \
            T c = TF_(T,zero)(); \
            for (int k = 0; k < M; ++k) { \
                c = TF_(T,add)(c, TF_(T,mul)(a.s[i*M + k], b.s[k*N + j])); \
            } \
            r.s[i*N + j] = c; \
        } \
    } \
    return r; \
} \


// All in one up to specified dimension

#define MAT2_C(T) \
\
MAT_COMMON_C(T,1,1) \
MAT_COMMON_C(T,1,2) \
MAT_COMMON_C(T,2,1) \
MAT_COMMON_C(T,2,2) \
\
MAT_DOT_C(T,1,1,1) \
MAT_DOT_C(T,1,1,2) \
MAT_DOT_C(T,1,2,1) \
MAT_DOT_C(T,1,2,2) \
MAT_DOT_C(T,2,1,1) \
MAT_DOT_C(T,2,1,2) \
MAT_DOT_C(T,2,2,1) \
MAT_DOT_C(T,2,2,2) \
\
MAT_SQUARE2_C(T); \


#define MAT3_C(T) \
\
MAT2_C(T) \
\
MAT_COMMON_C(T,1,3) \
MAT_COMMON_C(T,2,3) \
MAT_COMMON_C(T,3,1) \
MAT_COMMON_C(T,3,2) \
MAT_COMMON_C(T,3,3) \
\
MAT_DOT_C(T,1,1,3) \
MAT_DOT_C(T,1,2,3) \
MAT_DOT_C(T,1,3,1) \
MAT_DOT_C(T,1,3,2) \
MAT_DOT_C(T,1,3,3) \
MAT_DOT_C(T,2,1,3) \
MAT_DOT_C(T,2,2,3) \
MAT_DOT_C(T,2,3,1) \
MAT_DOT_C(T,2,3,2) \
MAT_DOT_C(T,2,3,3) \
MAT_DOT_C(T,3,1,1) \
MAT_DOT_C(T,3,1,2) \
MAT_DOT_C(T,3,1,3) \
MAT_DOT_C(T,3,2,1) \
MAT_DOT_C(T,3,2,2) \
MAT_DOT_C(T,3,2,3) \
MAT_DOT_C(T,3,3,1) \
MAT_DOT_C(T,3,3,2) \
MAT_DOT_C(T,3,3,3) \
\
MAT_SQUARE3_C(T); \


#define MAT4_C(T) \
\
MAT3_C(T) \
\
MAT_COMMON_C(T,1,4) \
MAT_COMMON_C(T,2,4) \
MAT_COMMON_C(T,3,4) \
MAT_COMMON_C(T,4,1) \
MAT_COMMON_C(T,4,2) \
MAT_COMMON_C(T,4,3) \
MAT_COMMON_C(T,4,4) \
\
MAT_DOT_C(T,1,1,4) \
MAT_DOT_C(T,1,2,4) \
MAT_DOT_C(T,1,3,4) \
MAT_DOT_C(T,1,4,1) \
MAT_DOT_C(T,1,4,2) \
MAT_DOT_C(T,1,4,3) \
MAT_DOT_C(T,1,4,4) \
MAT_DOT_C(T,2,1,4) \
MAT_DOT_C(T,2,2,4) \
MAT_DOT_C(T,2,3,4) \
MAT_DOT_C(T,2,4,1) \
MAT_DOT_C(T,2,4,2) \
MAT_DOT_C(T,2,4,3) \
MAT_DOT_C(T,2,4,4) \
MAT_DOT_C(T,3,1,4) \
MAT_DOT_C(T,3,2,4) \
MAT_DOT_C(T,3,3,4) \
MAT_DOT_C(T,3,4,1) \
MAT_DOT_C(T,3,4,2) \
MAT_DOT_C(T,3,4,3) \
MAT_DOT_C(T,3,4,4) \
MAT_DOT_C(T,4,1,1) \
MAT_DOT_C(T,4,1,2) \
MAT_DOT_C(T,4,1,3) \
MAT_DOT_C(T,4,1,4) \
MAT_DOT_C(T,4,2,1) \
MAT_DOT_C(T,4,2,2) \
MAT_DOT_C(T,4,2,3) \
MAT_DOT_C(T,4,2,4) \
MAT_DOT_C(T,4,3,1) \
MAT_DOT_C(T,4,3,2) \
MAT_DOT_C(T,4,3,3) \
MAT_DOT_C(T,4,3,4) \
MAT_DOT_C(T,4,4,1) \
MAT_DOT_C(T,4,4,2) \
MAT_DOT_C(T,4,4,3) \
MAT_DOT_C(T,4,4,4) \
\
MAT_SQUARE4_C(T); \


// Packing

#ifdef OPENCL_INTEROP

#define MAT_PK_C(T,M,N) \
MAT_PK_(T,M,N) MATF_PK_(pack,T,M,N)(MAT_(T,M,N) m) { \
    MAT_PK_(T,M,N) r; \
    for (int i = 0; i < M*N; ++i) { \
        r.s[i] = TF_PK_(pack,T)(m.s[i]); \
    } \
    return r; \
} \
MAT_(T,M,N) MATF_PK_(unpack,T,M,N)(MAT_PK_(T,M,N) m) { \
    MAT_(T,M,N) r; \
    for (int i = 0; i < M*N; ++i) { \
        r.s[i] = TF_PK_(unpack,T)(m.s[i]); \
    } \
    return r; \
} \

#define MAT2_PK_C(T) \
MAT_PK_C(T,1,1) \
MAT_PK_C(T,1,2) \
MAT_PK_C(T,2,1) \
MAT_PK_C(T,2,2) \

#define MAT3_PK_C(T) \
MAT2_PK_C(T) \
MAT_PK_C(T,1,3) \
MAT_PK_C(T,2,3) \
MAT_PK_C(T,3,1) \
MAT_PK_C(T,3,2) \
MAT_PK_C(T,3,3) \

#define MAT4_PK_C(T) \
MAT3_PK_C(T) \
MAT_PK_C(T,1,4) \
MAT_PK_C(T,2,4) \
MAT_PK_C(T,3,4) \
MAT_PK_C(T,4,1) \
MAT_PK_C(T,4,2) \
MAT_PK_C(T,4,3) \
MAT_PK_C(T,4,4) \

#endif // OPENCL_INTEROP


// Instantiation

#define real_zero() R0
#define real_one() R1
#define real_neg(x) (-(x))
#define real_add(x, y) ((x) + (y))
#define real_sub(x, y) ((x) - (y))
#define real_mul(x, y) ((x)*(y))
#define real_div(x, y) ((x)/(y))
#define real_sqrt sqrt

#define complex_zero() C0
#define complex_one() C1
#define complex_neg(x) (-(x))
#define complex_add(x, y) ((x) + (y))
#define complex_sub(x, y) ((x) - (y))
#define complex_mul c_mul
#define complex_div c_div
#define complex_sqrt c_sqrt


MAT4_C(real)
MAT2_C(complex)

#ifdef OPENCL_INTEROP
MAT4_PK_C(real)
MAT2_PK_C(complex)
#endif // OPENCL_INTEROP

complex2x2 complex2x2_new(complex a, complex b, complex c, complex d) {
    return (complex2x2){.s = {a, b, c, d}};
}

real complex2x2_fabs(complex2x2 m) {
    real d = R0;
    for (int i = 0; i < 2*2; ++i) {
        d += c_fabs(m.s[i]);
    }
    return d;
}

complex2x2 complex2x2_normalize(complex2x2 m) {
    complex d = c_sqrt(complex2x2_det(m));
    for (int i = 0; i < 2*2; ++i) {
        m.s[i] = c_div(m.s[i], d);
    }
    return m;
}
complex2x2 complex2x2_inverse_normalized(complex2x2 m) {
    return complex2x2_new(
        m.s[3], -m.s[1],
        -m.s[2], m.s[0]
    );
}

void _complex2x2_eigen_common(
    complex2x2 m,
    complex2x2 *l,
    complex2x2 *v,
    complex d // determinant of `m`
) {
    if (c_fabs(m.s[1]) < EPS && c_fabs(m.s[2]) < EPS) {
        *l = complex2x2_new(m.s[0], C0, C0, m.s[3]);
        *v = complex2x2_new(C1, C0, C0, C1);
    } else {
        complex ad = (m.s[0] + m.s[3])/2;
        complex D = c_sqrt(c_mul(ad, ad) - d);
        if (c_fabs(D) > EPS) {
            *l = complex2x2_new(ad + D, C0, C0, ad - D);
            if (c_fabs(m.s[1]) > EPS) {
                *v = complex2x2_new(m.s[1], m.s[1], l->s[0] - m.s[0], l->s[3] - m.s[0]);
            } else {
                *v = complex2x2_new(l->s[0] - m.s[3], l->s[3] - m.s[3], m.s[2], m.s[2]);
            }
        } else {
            *l = complex2x2_new(ad, C1, C0, ad);
            if (c_fabs(m.s[1]) > EPS) {
                complex g = 4*c_mul(m.s[1], m.s[1]) + c_mul(m.s[0] - m.s[3], m.s[0] - m.s[3]);
                *v = complex2x2_new(
                    m.s[1],
                    c_div(2*c_mul(m.s[1], m.s[0] - m.s[3]), g),
                    (m.s[3] - m.s[0])/2,
                    c_div(4*c_mul(m.s[1], m.s[1]), g)
                );
            } else {
                complex g = 4*c_mul(m.s[2], m.s[2]) + c_mul(m.s[3] - m.s[0], m.s[3] - m.s[0]);
                *v = complex2x2_new(
                    (m.s[0] - m.s[3])/2,
                    c_div(4*c_mul(m.s[2], m.s[2]), g),
                    m.s[2],
                    c_div(2*c_mul(m.s[2], m.s[3] - m.s[0]), g)
                );
            }
        }
    }
}

void complex2x2_eigen(complex2x2 m, complex2x2 *l, complex2x2 *v) {
    _complex2x2_eigen_common(m, l, v, complex2x2_det(m));
}

void complex2x2_eigen_normalized(complex2x2 m, complex2x2 *l, complex2x2 *v) {
    _complex2x2_eigen_common(m, l, v, C1);
    *v = complex2x2_normalize(*v);
}

complex2x2 _complex2x2_pow_common(
    complex2x2 m,
    real p,
    complex2x2 j,
    complex2x2 v
) {
    complex2x2 x = complex2x2_sub(m, complex2x2_one());
    real y = complex2x2_fabs(x);
    // FIXME: Why 1e4?
    if (y*y > 1e4*EPS) {
        complex2x2 k;
        if (c_fabs(j.s[1]) < EPS) {
            k = complex2x2_new(c_powr(j.s[0], p), C0, C0, c_powr(j.s[3], p));
        } else {
            // Assume j.s[0] == j.s[3]
            k = complex2x2_new(
                c_powr(j.s[0], p),
                p*c_mul(c_powr(j.s[0], p - 1), j.s[1]),
                C0,
                c_powr(j.s[3], p)
            );
        }
        return complex2x2x2_dot(
            complex2x2x2_dot(v, k),
            complex2x2_inverse(v)
        );
    } else {
        return complex2x2_add(
            complex2x2_one(),
            complex2x2_mul(x, c_new(p, 0))
        );
    }
}

complex2x2 complex2x2_pow(complex2x2 m, real p) {
    complex2x2 j, v;
    complex2x2_eigen(m, &j, &v);
    return _complex2x2_pow_common(m, p, j, v);
}

complex2x2 complex2x2_pow_normalized(complex2x2 m, real p) {
    complex2x2 j, v;
    complex2x2_eigen_normalized(m, &j, &v);
    complex2x2 r = _complex2x2_pow_common(m, p, j, v);
    //r = complex2x2_normalize(r);
    return r;
}
*/

#ifdef UNIT_TEST
#include <catch.hpp>

TEST_CASE("Matrix types", "[matrix]") {
    MatTestRng rng(0xCAFE);

    SECTION("Contruction") {
        auto a = matrix<int, 3, 4>(-1);
        for (int i = 0; i < a.size(); ++i) {
            REQUIRE(a[i] == -1);
        }
        auto b = matrix<int, 2, 2>(0, 1, 2, 3);
        for (int i = 0; i < b.shape<0>(); ++i) {
            for (int j = 0; j < b.shape<1>(); ++j) {
                REQUIRE(b(i, j) == b.shape<1>()*i + j);
            }
        }
        auto c = matrix<int, 2, 3>(
            vector<int, 3>(0, 1, 2),
            vector<int, 3>(3, 4, 5)
        );
        for (int i = 0; i < c.shape<0>(); ++i) {
            for (int j = 0; j < c.shape<1>(); ++j) {
                REQUIRE(c(i, j) == c.shape<1>()*i + j);
            }
        }
    }
    SECTION("Compare") {
        REQUIRE(real2x2(1, 2, 3, 4) == approx(real2x2(1, 2, 3, 4)));
    }
    SECTION("One") {
        REQUIRE((matrix<int, 2, 2>(1, 0, 0, 1)) == approx(one<matrix<int, 2, 2>>()));
    }
    SECTION("Determinant") {
        REQUIRE(det(real2x2(1, 2, 3, 4)) == approx(-2));
    }
    SECTION("Inversion") {
        for (int k = 0; k < TEST_ATTEMPTS; ++k) {
            real4x4 m = rng.mat_invertible<4>();
            REQUIRE(dot(m, !m) == approx(one<real4x4>()));
            REQUIRE(dot(!m, m) == approx(one<real4x4>()));
        }
    }
    SECTION("Outer product") {
        real3 a(0, 1, 2);
        real4 b(0, 1, 2, 3);
        real3x4 c(
            0, 0, 0, 0,
            0, 1, 2, 3,
            0, 2, 4, 6
        );
        REQUIRE(outer(a, b) == approx(c));
    }
};

#endif // UNIT_TEST
