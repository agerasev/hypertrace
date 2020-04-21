#include "complex.hpp"
#include "matrix.hpp"


#ifdef UNIT_TEST
#include <catch.hpp>

#include <iostream>
#include <vector>
#include <functional>

using namespace test;

TEST_CASE("Complex numbers", "[complex]") {
    Rng rng(0xBEEF);

    SECTION("Constructor") {
        comp a(0.0, 1.0);
        REQUIRE(a.re() == Approx(0.0));
        REQUIRE(a.im() == Approx(1.0));
    }
    SECTION("Inversion") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp a = random<comp>(rng).nonzero();
            REQUIRE(a/a == approx(1));
        }
    }
    SECTION("Square root") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp a = random<comp>(rng).normal();
            comp b = math::sqrt(a);
            REQUIRE(b*b == approx(a));
        }
    }
    SECTION("Power") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp a = random<comp>(rng).normal();
            int n = int(math::floor(2 + 10*random<real>(rng).uniform()));
            comp b = math::pow(a, 1.0/n);
            comp c(real(1));
            for (int i = 0; i < n; ++i) {
                c *= b;
            }
            REQUIRE(c == approx(a));
        }
    }
    SECTION("Norm") {
        REQUIRE(norm_l1(comp(-1, 2)) == approx(3));
        REQUIRE(norm_l2(comp(3, -4)) == approx(5));
        REQUIRE(norm_linf(comp(1, -2)) == approx(2));
    }
    SECTION("Matrix representation") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp a = random<comp>(rng).normal();
            comp b = random<comp>(rng).normal();
            REQUIRE(a.lower() == approx(a.to_matrix()));
            REQUIRE(a.to_matrix() + b.to_matrix() == approx((a + b).to_matrix()));
            REQUIRE(dot(a.to_matrix(), b.to_matrix()) == approx((a*b).to_matrix()));
            REQUIRE(transpose(a.to_matrix()) == approx((~a).to_matrix()));
            REQUIRE(inverse(a.to_matrix()) == approx((!a).to_matrix()));
        }
    }
};

TEST_CASE("Quaternions", "[quaternion]") {
    Rng rng(0xFEED);

    SECTION("Imaginary units") {
        REQUIRE(1_i * 1_i == approx(-1_r));
        REQUIRE(1_j * 1_j == approx(-1_r));
        REQUIRE(1_k * 1_k == approx(-1_r));
        REQUIRE(1_i * 1_j * 1_k == approx(-1_r));

        REQUIRE(1_i * 1_j == approx(1_k));
        REQUIRE(1_j * 1_k == approx(1_i));
        REQUIRE(1_k * 1_i == approx(1_j));
        
        REQUIRE(1_j * 1_i == approx(-1_k));
        REQUIRE(1_k * 1_j == approx(-1_i));
        REQUIRE(1_i * 1_k == approx(-1_j));
    }
    SECTION("Inversion") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat a = random<quat>(rng).nonzero();
            REQUIRE(a/a == approx(1));
        }
    }
    SECTION("Law of cosines") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat a = random<quat>(rng).normal(), b = random<quat>(rng).normal();
            REQUIRE(length2(a) + length2(b) + 2*dot(a, b) == approx(length2(a + b)));
        }
    }
    SECTION("Derivation") {
        std::vector<std::pair<
            std::function<quat(quat)>,
            std::function<quat(quat, quat)>
        >> cases = {
            std::make_pair(
                [](quat p) { return p; },
                [](quat, quat v) { return v; }
            ),
            std::make_pair(
                [](quat p) { return p*p; },
                [](quat p, quat v) { return p*v + v*p; }
            ),
            std::make_pair(
                [](quat p) { return !p; },
                [](quat p, quat v) {
                    real p2 = length2(p);
                    return (~v - (2*dot(p, v)/p2)*~p)/p2;
                }
            )
        };

        for (auto p : cases) {
            auto f = p.first;
            auto dfdv = p.second;
            for (int i = 0; i < TEST_ATTEMPTS; ++i) {
                quat p = random<quat>(rng).normal();
                quat v = random<quat>(rng).unit();
                REQUIRE((f(p + EPS*v) - f(p))/EPS == approx(dfdv(p, v)));
            }
        }
    }
    SECTION("Norm") {
        REQUIRE(norm_l1(quat(-1, 2, -3, 4)) == approx(10));
        REQUIRE(norm_l2(quat(1, -1, 1, -1)) == approx(2));
        REQUIRE(norm_linf(quat(1, -2, 3, -4)) == approx(4));
    }
    SECTION("Degree lowering") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat a = random<quat>(rng).normal();
            quat b = random<quat>(rng).normal();
            REQUIRE(a.lower() + b.lower() == approx((a + b).lower()));
            REQUIRE(dot(a.lower(), b.lower()) == approx((a*b).lower()));
            //REQUIRE(transpose(a.lower()) == approx((~a).lower()));
            REQUIRE(inverse(a.lower()) == approx((!a).lower()));
        }
    }
    SECTION("Matrix representation") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat a = random<quat>(rng).normal();
            quat b = random<quat>(rng).normal();
            REQUIRE(a.to_matrix() + b.to_matrix() == approx((a + b).to_matrix()));
            REQUIRE(dot(a.to_matrix(), b.to_matrix()) == approx((a*b).to_matrix()));
            REQUIRE(transpose(a.to_matrix()) == approx((~a).to_matrix()));
            REQUIRE(inverse(a.to_matrix()) == approx((!a).to_matrix()));
        }
    }
};

#endif // UNIT_TEST
