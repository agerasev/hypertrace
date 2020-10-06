#include "matrix.hpp"


#ifdef TEST_CATCH
#include <catch.hpp>

using namespace test;

TEST_CASE("Matrix types", "[matrix]") {
    Rng rng(0xCAFE);

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
            real4x4 m = rng.d<real4x4>().invertible();
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
    SECTION("Eigenvalues and eigenvectors") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp2x2 m = rng.d<comp2x2>().normal();
            comp2x2 l, v;
            eigen(m, &l, &v);

            REQUIRE(m[0]*v[0] + m[1]*v[2] == approx(l[0]*v[0]));
            REQUIRE(m[2]*v[0] + m[3]*v[2] == approx(l[0]*v[2]));
            REQUIRE(m[0]*v[1] + m[1]*v[3] == approx(l[3]*v[1]));
            REQUIRE(m[2]*v[1] + m[3]*v[3] == approx(l[3]*v[3]));

            comp2x2 o = dot(dot(v, l), !v);
            REQUIRE(o == approx(m));
        }
    }
    SECTION("Non-diagonalizable matrix") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp2x2 m(1, 0, 0, 1);
            if (rng.d<real>().uniform() > 0.5) {
                m[1] = rng.d<comp>().normal();
            } else {
                m[2] = rng.d<comp>().normal();
            }
            comp2x2 l, v;
            eigen(m, &l, &v);

            comp2x2 o = dot(dot(v, l), !v);
            REQUIRE(o == approx(m));
        }
    }
    SECTION("Power") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp2x2 m = rng.d<comp2x2>().normalized();
            int n = int(math::floor(8*rng.d<real>().uniform())) + 2;

            comp2x2 p = pow(m, 1_r/n);
            comp2x2 o = one<comp2x2>();

            for(int i = 0; i < n; ++i) {
                o = dot(o, p);
            }

            REQUIRE(o == approx(m));
        }
    }
    SECTION("Non-diagonalizable power") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp2x2 m(1, 0, 0, 1);
            if (rng.d<real>().uniform() > 0.5) {
                m[1] = rng.d<comp>().normal();
            } else {
                m[2] = rng.d<comp>().normal();
            }
            int p = (int)math::floor(8*rng.d<real>().uniform()) + 2;
            int q = (int)math::floor(8*rng.d<real>().uniform()) + 2;

            comp2x2 n = one<comp2x2>();
            for (int i = 0; i < p; ++i) {
                n = dot(n, m);
            }
            comp2x2 l = pow(m, real(p)/q);
            comp2x2 o = one<comp2x2>();
            for (int i = 0; i < q; ++i) {
                o = dot(o, l);
            }

            REQUIRE(o == approx(n));
        }
    }
    SECTION("Power of almost identity transformation") {
        comp2x2 m(
            1 + 8.2331e-07_i,
            4.75378e-06 + -7.47005e-07_i,
            9.3315e-08 + -2.74772e-08_i,
            1 + 9.82438e-07_i
        );

        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            int n = int(math::floor(8*rng.d<real>().uniform())) + 2;

            comp2x2 p = pow(m, 1.0/n);
            comp2x2 o = one<comp2x2>();
            for(int i = 0; i < n; ++i) {
                o = dot(o, p);
            }

            REQUIRE(o == approx(m));
        }
    }
};

#endif // TEST_CATCH
