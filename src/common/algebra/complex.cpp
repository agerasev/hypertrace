#include "complex.hpp"


#ifdef UNIT_TEST
#include <catch.hpp>

#include <iostream>
#include <vector>
#include <functional>

comp CompTestRng::comp_normal() {
    return comp(vec_normal<2>());
}
comp CompTestRng::comp_unit() {
    real phi = 2*PI*uniform();
    return comp(cos(phi), sin(phi));
}
comp CompTestRng::comp_nonzero() {
    comp a;
    do {
        a = comp_normal();
    } while(length2(a) < EPS);
    return a;
}

quat QuatTestRng::quat_normal() {
    return quat(vec_normal<4>());
}
quat QuatTestRng::quat_unit() {
    quat q = quat_nonzero();
    return q/length(q);
}
quat QuatTestRng::quat_nonzero() {
    quat a;
    do {
        a = quat_normal();
    } while(length2(a) < EPS);
    return a;
}

TEST_CASE("Complex numbers", "[complex]") {
    CompTestRng rng(0xbeef);

    SECTION("Constructor") {
        comp a(0.0, 1.0);
        REQUIRE(a.re() == Approx(0.0));
        REQUIRE(a.im() == Approx(1.0));
    }
    SECTION("Inversion") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp a = rng.comp_nonzero();
            REQUIRE(a/a == approx(1));
        }
    }
    SECTION("Square root") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp a = rng.comp_normal();
            comp b = math::sqrt(a);
            REQUIRE(b*b == approx(a));
        }
    }
    SECTION("Power") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp a = rng.comp_normal();
            int n = int(floor(2 + 10*rng.uniform()));
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
};

TEST_CASE("Quaternions", "[quaternion]") {
    QuatTestRng rng(0xfeed);

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
            quat a = rng.quat_nonzero();
            REQUIRE(a/a == approx(1));
        }
    }
    SECTION("Law of cosines") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat a = rng.quat_normal(), b = rng.quat_normal();
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
                quat p = rng.quat_normal();
                quat v = rng.quat_unit();
                REQUIRE((f(p + EPS*v) - f(p))/EPS == approx(dfdv(p, v)));
            }
        }
    }
    SECTION("Norm") {
        REQUIRE(norm_l1(quat(-1, 2, -3, 4)) == approx(10));
        REQUIRE(norm_l2(quat(1, -1, 1, -1)) == approx(2));
        REQUIRE(norm_linf(quat(1, -2, 3, -4)) == approx(4));
    }
};

#endif // UNIT_TEST
