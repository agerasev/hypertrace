#pragma once

#ifndef OPENCL
#include <math.h>
#endif // OPENCL

#include "complex.h"


typedef real4 quaternion;
#define make_quaternion make_real4
#define q_new make_quaternion

#ifdef OPENCL_INTEROP
typedef cl_float4 quaternion_packed;
#define pack_quaternion pack_real4
#define unpack_quaternion unpack_real4
#define q_pack pack_quaternion
#define q_unpack unpack_quaternion
#endif // OPENCL_INTEROP

#define Q0 q_new(R0, R0, R0, R0)
#define Q1 q_new(R1, R0, R0, R0)
#define QI q_new(R0, R1, R0, R0)
#define QJ q_new(R0, R0, R1, R0)
#define QK q_new(R0, R0, R0, R1)


quaternion q_conj(quaternion a) {
    return q_new(a.x, -a.yzw);
}
real q_abs2(quaternion a) {
    return dot(a, a);
}
real q_abs(quaternion a) {
    return length(a);
}


quaternion q_mul(quaternion a, quaternion b) {
    return q_new(
        a.x*b.x - a.y*b.y - a.z*b.z - a.w*b.w,
        a.x*b.y + a.y*b.x + a.z*b.w - a.w*b.z,
        a.x*b.z + a.z*b.x - a.y*b.w + a.w*b.y,
        a.x*b.w + a.w*b.x + a.y*b.z - a.z*b.y
    );
}

quaternion qc_mul(quaternion a, complex b) {
    return q_new(
        a.x*b.x - a.y*b.y,
        a.x*b.y + a.y*b.x,
        a.z*b.x + a.w*b.y,
        a.w*b.x - a.z*b.y
    );
}

quaternion cq_mul(complex a, quaternion b) {
    return q_new(
        a.x*b.x - a.y*b.y,
        a.x*b.y + a.y*b.x,
        a.x*b.z - a.y*b.w,
        a.x*b.w + a.y*b.z
    );
}

quaternion q_inverse(quaternion a) {
    return q_conj(a)/q_abs2(a);
}

quaternion q_div(quaternion a, quaternion b) {
    return q_mul(a, q_inverse(b));
}

quaternion qc_div(quaternion a, complex b) {
    return qc_mul(a, c_inverse(b));
}

quaternion cq_div(complex a, quaternion b) {
    return cq_mul(a, q_inverse(b));
}


#ifdef UNIT_TEST
#include <catch.hpp>

#include <vector>
#include <utility>
#include <functional>

class q_approx {
private:
    quaternion v;
public:
    q_approx(quaternion q) : v(q) {}
    friend bool operator==(quaternion a, q_approx b) {
        return
            a.x == Approx(b.v.x) &&
            a.y == Approx(b.v.y) &&
            a.z == Approx(b.v.z) &&
            a.w == Approx(b.v.w);
    }
    friend bool operator==(q_approx a, quaternion b) {
        return b == a;
    }
    friend std::ostream &operator<<(std::ostream &s, q_approx a) {
        return s << a.v;
    }
};

quaternion rand_q_normal(Rng &rng) {
    return quaternion(rng.normal(), rng.normal(), rng.normal(), rng.normal());
}
quaternion rand_q_nonzero(Rng &rng) {
    quaternion a;
    do {
        a = rand_q_normal(rng);
    } while(q_abs2(a) < EPS);
    return a;
}
quaternion rand_q_unit(Rng &rng) {
    quaternion q = rand_q_nonzero(rng);
    return q/q_abs(q);
}

TEST_CASE("Quaternions", "[quaternion]") {
    Rng rng(0xfeed);

    SECTION("Imaginary units") {
        REQUIRE(q_mul(QI, QI) == q_approx(-Q1));
        REQUIRE(q_mul(QJ, QJ) == q_approx(-Q1));
        REQUIRE(q_mul(QK, QK) == q_approx(-Q1));
        REQUIRE(q_mul(q_mul(QI, QJ), QK) == q_approx(-Q1));

        REQUIRE(q_mul(QI, QJ) == q_approx(QK));
        REQUIRE(q_mul(QJ, QK) == q_approx(QI));
        REQUIRE(q_mul(QK, QI) == q_approx(QJ));
        
        REQUIRE(q_mul(QJ, QI) == q_approx(-QK));
        REQUIRE(q_mul(QK, QJ) == q_approx(-QI));
        REQUIRE(q_mul(QI, QK) == q_approx(-QJ));
    }
    SECTION("Inversion") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quaternion a = rand_q_nonzero(rng);
            REQUIRE(a/a == q_approx(1));
        }
    }
    SECTION("Law of cosines") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quaternion a = rand_q_normal(rng), b = rand_q_normal(rng);
            REQUIRE(q_abs2(a) + q_abs2(b) + 2*dot(a, b) == Approx(q_abs2(a + b)));
        }
    }
    SECTION("Derivation") {
        std::vector<std::pair<
            std::function<quaternion(quaternion)>,
            std::function<quaternion(quaternion, quaternion)>
        >> cases = {
            std::make_pair(
                [](quaternion p) { return p; },
                [](quaternion, quaternion v) { return v; }
            ),
            std::make_pair(
                [](quaternion p) { return q_mul(p, p); },
                [](quaternion p, quaternion v) { return q_mul(p, v) + q_mul(v, p); }
            ),
            std::make_pair(
                [](quaternion p) { return q_inverse(p); },
                [](quaternion p, quaternion v) {
                    real p2 = q_abs2(p);
                    return (q_conj(v) - (2*dot(p, v)/p2)*q_conj(p))/p2;
                }
            )
        };

        for (auto p : cases) {
            auto f = p.first;
            auto dfdv = p.second;
            for (int i = 0; i < TEST_ATTEMPTS; ++i) {
                quaternion p = rand_q_normal(rng);
                quaternion v = rand_q_unit(rng);
                REQUIRE((f(p + EPS*v) - f(p))/EPS == q_approx(dfdv(p, v)));
            }
        }
    }
};
#endif // UNIT_TEST
