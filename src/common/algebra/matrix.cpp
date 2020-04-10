#include "matrix.hpp"


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
