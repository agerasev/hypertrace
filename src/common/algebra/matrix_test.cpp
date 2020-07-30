#ifdef UNIT_TEST

#include "matrix.hh"

#include <catch.hpp>


comp2x2 TestRngComp2x2::normal() {
    return rng.normal();
}
comp2x2 TestRngComp2x2::uniform() {
    return rng.uniform();
}
comp2x2 TestRngComp2x2::invertible() {
    comp2x2 r;
    do {
        r = normal();
    } while(c_norm_l1(c22_det(r)) < EPS);
    return r;
}
comp2x2 TestRngComp2x2::normalized() {
    return normalize(invertible());
}

real4x4 TestRngReal3x3::normal() {
    return r33_clip(rng.normal());
}
real4x4 TestRngReal3x3::uniform() {
    return r33_clip(rng.uniform());
}
real4x4 TestRngReal3x3::invertible() {
    real4x4 r;
    do {
        r = normal();
    } while(abs(r33_det(r)) < EPS);
    return r;
}

real4x4 TestRngReal4x4::normal() {
    return rng.normal();
}
real4x4 TestRngReal4x4::uniform() {
    return rng.normal();
}


TEST_CASE("Matrix types", "[matrix]") {
    TestRng<real> rng(0xAA);
    TestRng<comp> crng(0xBB);

    SECTION("Complex matrices") {
        TestRngComp2x2 mrng(0xCAFE);
        
        SECTION("One") {
            REQUIRE((c22_new(C1, C0, C0, C1)) == approx(c22_one()));
        }
        SECTION("Determinant") {
            REQUIRE(c22_det(c22_new(C1, 2*C1, 3*C1, 4*C1)) == approx(-2*C1));
        }
        SECTION("Inversion") {
            for (int k = 0; k < TEST_ATTEMPTS; ++k) {
                comp2x2 m = mrng.invertible();
                REQUIRE(c22_dot(m, c22_inverse(m)) == approx(c22_one()));
                REQUIRE(c22_dot(c22_inverse(m), m) == approx(c22_one()));
            }
        }
        SECTION("Norm") {
            REQUIRE(c22_norm_l1(c22_new(-1*C1, 2*C1, -3*C1, 4*C1)) == approx(10));
            REQUIRE(length(c22_new(-1*C1, 1*C1, -1*C1, 1*C1)) == approx(2));
        }
        SECTION("Abs") {
            REQUIRE(
                fabs(c22_new(1*C1, -2*C1, 3*C1, -4*C1)) ==
                approx(c22_new(1*C1, 2*C1, 3*C1, 4*C1))
            );
        }
        SECTION("Eigenvalues and eigenvectors") {
            for (int i = 0; i < TEST_ATTEMPTS; ++i) {
                comp2x2 m = mrng.normal();
                comp2x2 l, v;
                c22_eigen(m, &l, &v);

                comp2x2 x = c22_dot(m, v);
                comp2x2 y = c22_dot(v, c22_new(l.s01, C0, C0, l.s67));
                REQUIRE(x == approx(y));

                comp2x2 o = c22_dot(c22_dot(v, l), c22_inverse(v));
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
                c22_eigen(m, &l, &v);

                comp2x2 o = c22_dot(c22_dot(v, l), c22_inverse(v));
                REQUIRE(o == approx(m));
            }
        }
        SECTION("Power") {
            for (int i = 0; i < TEST_ATTEMPTS; ++i) {
                comp2x2 m = mrng.normalized();
                int n = int(floor(8*rng.uniform())) + 2;

                comp2x2 p = c22_pow(m, R1/n);
                comp2x2 o = c22_one();

                for(int i = 0; i < n; ++i) {
                    o = c22_dot(o, p);
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

                comp2x2 n = c22_one();
                for (int i = 0; i < p; ++i) {
                    n = c22_dot(n, m);
                }
                comp2x2 l = c22_pow(m, (real)p/q);
                comp2x2 o = c22_one();
                for (int i = 0; i < q; ++i) {
                    o = c22_dot(o, l);
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

                comp2x2 p = c22_pow(m, R1/n);
                comp2x2 o = c22_one();
                for(int i = 0; i < n; ++i) {
                    o = c22_dot(o, p);
                }

                REQUIRE(o == approx(m));
            }
        }
    }
    SECTION("Real matrices") {
        TestRngReal3x3 r3rng(0xABC3);
        TestRngReal4x4 r4rng(0xABC4);

        SECTION("Transpose") {
            for (int k = 0; k < TEST_ATTEMPTS; ++k) {
                real4x4 m = r4rng.normal();
                REQUIRE(r44_transpose(r44_transpose(m)) == approx(m));
            }
        }
        SECTION("Dot product") {
            REQUIRE(r44_dot(r44_one(), r44_one()) == approx(r44_one()));

            REQUIRE(r44_dot(
                r44_new(1, -2, 0, 0, 3, 4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1),
                r44_new(0.4, 0.2, 0, 0, -0.3, 0.1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
            ) == approx(r44_one()));

            for (int k = 0; k < TEST_ATTEMPTS; ++k) {
                real4x4 a = r4rng.normal(), b = r4rng.normal();
                REQUIRE(
                    r44_transpose(r44_dot(r44_transpose(b), r44_transpose(a))) ==
                    approx(r44_dot(a, b))
                );
            }
        }
        SECTION("Determinant") {
            real4x4 r2 = r44_one();
            r2.s01 = MAKE(real2)(1, 2);
            r2.s45 = MAKE(real2)(3, 4);
            REQUIRE(r33_det(r2) == approx(-2));

            for (int k = 0; k < TEST_ATTEMPTS; ++k) {
                real4x4 a = r3rng.normal(), b = r3rng.normal();
                REQUIRE(r33_det(a)*r33_det(b) == approx(r33_det(r44_dot(a, b))));
            }
        }
        SECTION("Inversion") {
            REQUIRE(r33_inverse(r44_one()) == approx(r44_one()));

            REQUIRE(
                r33_inverse(r44_new(1, -2, 0, 0, 3, 4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)) ==
                approx(r44_new(0.4, 0.2, 0, 0, -0.3, 0.1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1))
            );
            REQUIRE(
                r33_inverse(r44_new(1, 0, 0, 0, 0, 1, -2, 0, 0, 3, 4, 0, 0, 0, 0, 1)) ==
                approx(r44_new(1, 0, 0, 0, 0, 0.4, 0.2, 0, 0, -0.3, 0.1, 0, 0, 0, 0, 1))
            );

            for (int k = 0; k < TEST_ATTEMPTS; ++k) {
                real4x4 m = r3rng.invertible();
                REQUIRE(r44_dot(m, r33_inverse(m)) == approx(r44_one()));
                REQUIRE(r44_dot(r33_inverse(m), m) == approx(r44_one()));
            }
        }
        SECTION("Outer product") {
            real4 a = MAKE(real4)(0, 1, 2, 4);
            real4 b = MAKE(real4)(0, 1, 2, 3);
            real4x4 c = r44_new(
                0, 0, 0, 0,
                0, 1, 2, 3,
                0, 2, 4, 6,
                0, 4, 8, 12
            );
            REQUIRE(r44_outer(a, b) == approx(c));
        }
    }
};

#endif // UNIT_TEST
