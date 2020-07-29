#include "matrix.hh"


#include <catch.hpp>

TEST_CASE("Matrix types", "[matrix]") {
    TestRng<real> rng(0xAA);
    TestRng<comp> crng(0xBB);
    TestRngComp2x2 mrng(0xCAFE);

    SECTION("One") {
        REQUIRE((MAKE(comp2x2)(C1, C0, C0, C1)) == approx(comp2x2_one()));
    }
    SECTION("Determinant") {
        REQUIRE(comp2x2_det(MAKE(comp2x2)(C1, 2*C1, 3*C1, 4*C1)) == approx(-2*C1));
    }
    SECTION("Inversion") {
        for (int k = 0; k < TEST_ATTEMPTS; ++k) {
            comp2x2 m = mrng.invertible();
            REQUIRE(comp2x2_dot(m, comp2x2_inverse(m)) == approx(comp2x2_one()));
            REQUIRE(comp2x2_dot(comp2x2_inverse(m), m) == approx(comp2x2_one()));
        }
    }
    SECTION("Norm") {
        REQUIRE(comp2x2_norm_l1(MAKE(comp2x2)(-1*C1, 2*C1, -3*C1, 4*C1)) == approx(10));
        REQUIRE(length(MAKE(comp2x2)(-1*C1, 1*C1, -1*C1, 1*C1)) == approx(2));
    }
    SECTION("Abs") {
        REQUIRE(
            fabs(MAKE(comp2x2)(1*C1, -2*C1, 3*C1, -4*C1)) ==
            approx(MAKE(comp2x2)(1*C1, 2*C1, 3*C1, 4*C1))
        );
    }
    SECTION("Eigenvalues and eigenvectors") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp2x2 m = mrng.normal();
            comp2x2 l, v;
            comp2x2_eigen(m, &l, &v);

            comp2x2 x = comp2x2_dot(m, v);
            comp2x2 y = comp2x2_dot(v, MAKE(comp2x2)(l.s01, C0, C0, l.s67));
            REQUIRE(x == approx(y));

            comp2x2 o = comp2x2_dot(comp2x2_dot(v, l), comp2x2_inverse(v));
            REQUIRE(o == approx(m));
        }
    }
    SECTION("Non-diagonalizable matrix") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp2x2 m(1, 0, 0, 1);
            if (rng.uniform() > 0.5) {
                m.s23 = crng.normal();
            } else {
                m.s45 = crng.normal();
            }
            comp2x2 l, v;
            comp2x2_eigen(m, &l, &v);

            comp2x2 o = comp2x2_dot(comp2x2_dot(v, l), comp2x2_inverse(v));
            REQUIRE(o == approx(m));
        }
    }
    SECTION("Power") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp2x2 m = mrng.normalized();
            int n = int(floor(8*rng.uniform())) + 2;

            comp2x2 p = comp2x2_pow(m, R1/n);
            comp2x2 o = comp2x2_one();

            for(int i = 0; i < n; ++i) {
                o = comp2x2_dot(o, p);
            }

            REQUIRE(o == approx(m));
        }
    }
    SECTION("Non-diagonalizable power") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp2x2 m(1, 0, 0, 1);
            if (rng.uniform() > 0.5) {
                m.s23 = crng.normal();
            } else {
                m.s45 = crng.normal();
            }
            int p = int(floor(8*rng.uniform())) + 2;
            int q = int(floor(8*rng.uniform())) + 2;

            comp2x2 n = comp2x2_one();
            for (int i = 0; i < p; ++i) {
                n = comp2x2_dot(n, m);
            }
            comp2x2 l = comp2x2_pow(m, (real)p/q);
            comp2x2 o = comp2x2_one();
            for (int i = 0; i < q; ++i) {
                o = comp2x2_dot(o, l);
            }

            REQUIRE(o == approx(n));
        }
    }
    SECTION("Power of almost identity transformation") {
        comp2x2 m(
            c_new(1, 8.2331e-07),
            c_new(4.75378e-06, -7.47005e-07),
            c_new(9.3315e-08, -2.74772e-08),
            c_new(1, 9.82438e-07)
        );

        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            int n = int(floor(8*rng.uniform())) + 2;

            comp2x2 p = comp2x2_pow(m, R1/n);
            comp2x2 o = comp2x2_one();
            for(int i = 0; i < n; ++i) {
                o = comp2x2_dot(o, p);
            }

            REQUIRE(o == approx(m));
        }
    }
};
