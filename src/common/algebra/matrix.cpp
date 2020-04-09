#include "matrix.hpp"

/*
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
    SECTION("Norm") {
        REQUIRE(norm_l1(real2x2(-1, 2, -3, 4)) == approx(10));
        REQUIRE(norm_l2(real2x2(-1, 1, -1, 1)) == approx(2));
        REQUIRE(norm_linf(real2x2(1, -2, 3, -4)) == approx(4));
    }
    SECTION("Abs") {
        REQUIRE(math::abs(real2x2(1, -2, 3, -4)) == approx(real2x2(1, 2, 3, 4)));
    }
    SECTION("Fract") {
        auto p = math::fract(real2x2(1.1, -1.8, 3.3, -3.6));
        REQUIRE(p.first == approx(real2x2(0.1, 0.2, 0.3, 0.4)));
        REQUIRE(p.second == approx(real2x2(1, -2, 3, -4)));
    }
};

#endif // UNIT_TEST
