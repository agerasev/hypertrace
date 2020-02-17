#pragma once


// # Quaternion

// ## Definition

#include "complex.h"


#ifdef OPENCL_DEVICE

typedef float4 quaternion;

#else // OPENCL_DEVICE

#ifdef __cplusplus
#include <cmath>
#include <iostream>
#else // __cplusplus
#include <math.h>
#endif // __cplusplus

typedef struct quaternion quaternion;


quaternion q_new(real r, real i, real j, real k);
quaternion q_new_r(real r);
quaternion q_new_c(complex a);

quaternion q_neg(quaternion a);
quaternion q_conj(quaternion a);
real q_abs2(quaternion a);
real q_abs(quaternion a);
quaternion q_norm(quaternion a);

quaternion qq_add(quaternion a, quaternion b);
quaternion qr_add(quaternion a, real b);
quaternion rq_add(real a, quaternion b);
quaternion qc_add(quaternion a, complex b);
quaternion cq_add(complex a, quaternion b);

quaternion qq_sub(quaternion a, quaternion b);
quaternion qr_sub(quaternion a, real b);
quaternion rq_sub(real a, quaternion b);
quaternion qc_sub(quaternion a, complex b);
quaternion cq_sub(complex a, quaternion b);

quaternion qq_mul(quaternion a, quaternion b);
quaternion qr_mul(quaternion a, real b);
quaternion rq_mul(real a, quaternion b);
quaternion qc_mul(quaternion a, complex b);
quaternion cq_mul(complex a, quaternion b);

quaternion qr_div(quaternion a, real b);
quaternion q_inv(quaternion a);
quaternion qq_div(quaternion a, quaternion b);
quaternion rq_div(real a, quaternion b);
quaternion qc_div(quaternion a, complex b);
quaternion cq_div(complex a, quaternion b);

real qq_dot(quaternion a, quaternion b);


struct quaternion {
    real x, y, z, w;
#ifdef __cplusplus
    static const quaternion i, j, k;
    quaternion() = default;
    quaternion(real r, real i, real j, real k) {
        *this = q_new(r, i, j, k);
    }
    quaternion(real r) {
        *this = q_new_r(r);
    }
    quaternion(complex c) {
        *this = q_new_c(c);
    }

    quaternion conj() const {
        return q_conj(*this);
    }
    real abs2() const {
        return q_abs2(*this);
    }
    real abs() const {
        return q_abs(*this);
    }
    quaternion norm() const {
        return q_norm(*this);
    }
    quaternion inv() const {
        return q_inv(*this);
    }

    quaternion &operator+=(quaternion b) {
        return *this = qq_add(*this, b);
    }
    quaternion &operator+=(complex b) {
        return *this = qc_add(*this, b);
    }
    quaternion &operator+=(real b) {
        return *this = qr_add(*this, b);
    }
    quaternion &operator-=(quaternion b) {
        return *this = qq_sub(*this, b);
    }
    quaternion &operator-=(complex b) {
        return *this = qc_sub(*this, b);
    }
    quaternion &operator-=(real b) {
        return *this = qr_sub(*this, b);
    }
    quaternion &operator*=(quaternion b) {
        return *this = qq_mul(*this, b);
    }
    quaternion &operator*=(complex b) {
        return *this = qc_mul(*this, b);
    }
    quaternion &operator*=(real b) {
        return *this = qr_mul(*this, b);
    }
    quaternion &operator/=(quaternion b) {
        return *this = qq_div(*this, b);
    }
    quaternion &operator/=(complex b) {
        return *this = qc_div(*this, b);
    }
    quaternion &operator/=(real b) {
        return *this = qr_div(*this, b);
    }
#endif // __cplusplus
};

#ifdef __cplusplus
const quaternion
    quaternion::i = quaternion(0,1,0,0),
    quaternion::j = quaternion(0,0,1,0),
    quaternion::k = quaternion(0,0,0,1);

quaternion operator+(quaternion a) {
    return a;
}
quaternion operator-(quaternion a) {
    return q_neg(a);
}

quaternion operator+(quaternion a, quaternion b) {
    return qq_add(a, b);
}
quaternion operator+(quaternion a, real b) {
    return qr_add(a, b);
}
quaternion operator+(real a, quaternion b) {
    return rq_add(a, b);
}
quaternion operator+(quaternion a, complex b) {
    return qc_add(a, b);
}
quaternion operator+(complex a, quaternion b) {
    return cq_add(a, b);
}

quaternion operator-(quaternion a, quaternion b) {
    return qq_sub(a, b);
}
quaternion operator-(quaternion a, real b) {
    return qr_sub(a, b);
}
quaternion operator-(real a, quaternion b) {
    return rq_sub(a, b);
}
quaternion operator-(quaternion a, complex b) {
    return qc_sub(a, b);
}
quaternion operator-(complex a, quaternion b) {
    return cq_sub(a, b);
}

quaternion operator*(quaternion a, quaternion b) {
    return qq_mul(a, b);
}
quaternion operator*(quaternion a, real b) {
    return qr_mul(a, b);
}
quaternion operator*(real a, quaternion b) {
    return rq_mul(a, b);
}
quaternion operator*(quaternion a, complex b) {
    return qc_mul(a, b);
}
quaternion operator*(complex a, quaternion b) {
    return cq_mul(a, b);
}

quaternion operator/(quaternion a, real b) {
    return qr_div(a, b);
}
quaternion operator/(quaternion a, quaternion b) {
    return qq_div(a, b);
}
quaternion operator/(real a, quaternion b) {
    return rq_div(a, b);
}
quaternion operator/(quaternion a, complex b) {
    return qc_div(a, b);
}
quaternion operator/(complex a, quaternion b) {
    return cq_div(a, b);
}

real dot(quaternion a, quaternion b) {
    return qq_dot(a, b);
}

std::ostream &operator<<(std::ostream &s, quaternion c) {
    return s << "(" << c.x << ", " << c.y << ", " << c.z << ", " << c.w << ")";
}

quaternion operator "" _j(long double v) {
    return real(v)*quaternion::j;
}
quaternion operator "" _j(unsigned long long v) {
    return real(v)*quaternion::j;
}
quaternion operator "" _k(long double v) {
    return real(v)*quaternion::k;
}
quaternion operator "" _k(unsigned long long v) {
    return real(v)*quaternion::k;
}
#endif // __cplusplus

#endif // OPENCL_DEVICE


// ## Constructors

quaternion q_new(real r, real i, real j, real k) {
#ifdef OPENCL_DEVICE
    return (quaternion)(r, i, j, k);
#else // OPENCL_DEVICE
    quaternion c;
    c.x = r;
    c.y = i;
    c.z = j;
    c.w = k;
    return c;
#endif // OPENCL_DEVICE
}

quaternion q_new_r(real r) {
    return q_new(r, (real)0, (real)0, (real)0);
}

quaternion q_new_c(complex a) {
    return q_new(a.x, a.y, (real)0, (real)0);
}


// # Operators

quaternion q_neg(quaternion a) {
#ifdef OPENCL_DEVICE
    return -a;
#else // OPENCL_DEVICE
    quaternion c;
    c.x = -a.x;
    c.y = -a.y;
    c.z = -a.z;
    c.w = -a.w;
    return c;
#endif // OPENCL_DEVICE
}

quaternion q_conj(quaternion a) {
    quaternion c;
    c.x = a.x;
    c.y = -a.y;
    c.z = -a.z;
    c.w = -a.w;
    return c;
}

real q_abs2(quaternion a) {
#ifdef OPENCL_DEVICE
    return dot(a, a);
#else // OPENCL_DEVICE
    return a.x*a.x + a.y*a.y + a.z*a.z + a.w*a.w;
#endif // OPENCL_DEVICE
}

real q_abs(quaternion a) {
#ifdef OPENCL_DEVICE
    return length(a);
#else // OPENCL_DEVICE
    return sqrt(q_abs2(a));
#endif // OPENCL_DEVICE
}

quaternion q_norm(quaternion a) {
#ifdef OPENCL_DEVICE
    return normalize(a);
#else // OPENCL_DEVICE
    return qr_div(a, q_abs(a));
#endif // OPENCL_DEVICE
}

// ## Addition

quaternion qq_add(quaternion a, quaternion b) {
#ifdef OPENCL_DEVICE
    return a + b;
#else // OPENCL_DEVICE
    quaternion c;
    c.x = a.x + b.x;
    c.y = a.y + b.y;
    c.z = a.z + b.z;
    c.w = a.w + b.w;
    return c;
#endif // OPENCL_DEVICE
}

quaternion qr_add(quaternion a, real b) {
    return qq_add(a, q_new_r(b));
}

quaternion rq_add(real a, quaternion b) {
    return qq_add(q_new_r(a), b);
}

quaternion qc_add(quaternion a, complex b) {
    return qq_add(a, q_new_c(b));
}

quaternion cq_add(complex a, quaternion b) {
    return qq_add(q_new_c(a), b);
}

// ## Subtraction

quaternion qq_sub(quaternion a, quaternion b) {
#ifdef OPENCL_DEVICE
    return a - b;
#else // OPENCL_DEVICE
    return qq_add(a, q_neg(b));
#endif // OPENCL_DEVICE
}

quaternion qr_sub(quaternion a, real b) {
    return qq_sub(a, q_new_r(b));
}

quaternion rq_sub(real a, quaternion b) {
    return qq_sub(q_new_r(a), b);
}

quaternion qc_sub(quaternion a, complex b) {
    return qq_sub(a, q_new_c(b));
}

quaternion cq_sub(complex a, quaternion b) {
    return qq_sub(q_new_c(a), b);
}

// ## Multiplication

quaternion qq_mul(quaternion a, quaternion b) {
    quaternion c;
    c.x = a.x*b.x - a.y*b.y - a.z*b.z - a.w*b.w;
    c.y = a.x*b.y + a.y*b.x + a.z*b.w - a.w*b.z;
    c.z = a.x*b.z + a.z*b.x - a.y*b.w + a.w*b.y;
    c.w = a.x*b.w + a.w*b.x + a.y*b.z - a.z*b.y;
    return c;
}

quaternion qr_mul(quaternion a, real b) {
#ifdef OPENCL_DEVICE
    return a*b;
#else // OPENCL_DEVICE
    quaternion c;
    c.x = a.x*b;
    c.y = a.y*b;
    c.z = a.z*b;
    c.w = a.w*b;
    return c;
#endif // OPENCL_DEVICE
}

quaternion rq_mul(real a, quaternion b) {
    return qr_mul(b, a);
}

quaternion qc_mul(quaternion a, complex b) {
    quaternion c;
    c.x = a.x*b.x - a.y*b.y;
    c.y = a.x*b.y + a.y*b.x;
    c.z = a.z*b.x + a.w*b.y;
    c.w = a.w*b.x - a.z*b.y;
    return c;
}

quaternion cq_mul(complex a, quaternion b) {
    quaternion c;
    c.x = a.x*b.x - a.y*b.y;
    c.y = a.x*b.y + a.y*b.x;
    c.z = a.x*b.z - a.y*b.w;
    c.w = a.x*b.w + a.y*b.z;
    return c;
}

// ## Division

quaternion qr_div(quaternion a, real b) {
#ifdef OPENCL_DEVICE
    return a / b;
#else // OPENCL_DEVICE
    return qr_mul(a, (real)1/b);
#endif // OPENCL_DEVICE
}

quaternion q_inv(quaternion a) {
    return qr_div(q_conj(a), q_abs2(a));
}

quaternion qq_div(quaternion a, quaternion b) {
    return qq_mul(a, q_inv(b));
}

quaternion rq_div(real a, quaternion b) {
    return rq_mul(a, q_inv(b));
}

quaternion qc_div(quaternion a, complex b) {
    return qc_mul(a, c_inv(b));
}

quaternion cq_div(complex a, quaternion b) {
    return cq_mul(a, q_inv(b));
}

// ## Miscellanous

real qq_dot(quaternion a, quaternion b) {
#ifdef OPENCL_DEVICE
    return dot(a, b);
#else // OPENCL_DEVICE
    return a.x*b.x + a.y*b.y + a.z*b.z + a.w*b.w;
#endif // OPENCL_DEVICE
}


// # Interoperability

#ifdef OPENCL_INTEROP

#ifdef OPENCL_DEVICE

typedef quaternion quaternion_packed;

quaternion_packed q_pack(quaternion a) {
    return (quaternion_packed)a;
}
quaternion q_unpack(quaternion_packed a) {
    return (quaternion)a;
}

#else // OPENCL_DEVICE

#include <CL/cl.h>

typedef cl_float4 quaternion_packed;

quaternion_packed q_pack(quaternion a) {
    quaternion_packed b;
    b.s[0] = r_pack(a.x);
    b.s[1] = r_pack(a.y);
    b.s[2] = r_pack(a.z);
    b.s[3] = r_pack(a.w);
    return b;
}
quaternion q_unpack(quaternion_packed a) {
    return q_new(
        r_unpack(a.s[0]), 
        r_unpack(a.s[1]),
        r_unpack(a.s[2]),
        r_unpack(a.s[3])
    );
}

#endif // OPENCL_DEVICE

#endif // OPENCL_INTEROP


// # Tests

#ifdef UNIT_TEST
#ifdef __cplusplus
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
    } while(a.abs2() < EPS);
    return a;
}
quaternion rand_q_unit(Rng &rng) {
    quaternion q = rand_q_nonzero(rng);
    return q/q.abs();
}

TEST_CASE("Quaternions", "[quaternion.h]") {
    Rng rng(0xfeed);

    SECTION("Imaginary units") {
        REQUIRE(1_i*1_i == q_approx(-1));
        REQUIRE(1_j*1_j == q_approx(-1));
        REQUIRE(1_k*1_k == q_approx(-1));
        REQUIRE(1_i*1_j*1_k == q_approx(-1));

        REQUIRE(1_i*1_j == q_approx(1_k));
        REQUIRE(1_j*1_k == q_approx(1_i));
        REQUIRE(1_k*1_i == q_approx(1_j));
        
        REQUIRE(1_j*1_i == q_approx(-1_k));
        REQUIRE(1_k*1_j == q_approx(-1_i));
        REQUIRE(1_i*1_k == q_approx(-1_j));
    }
    SECTION("Inversion") {
        for (int i = 0; i < ATTEMPTS; ++i) {
            quaternion a = rand_q_nonzero(rng);
            REQUIRE(a/a == q_approx(1));
        }
    }
    SECTION("Law of cosines") {
        for (int i = 0; i < ATTEMPTS; ++i) {
            quaternion a = rand_q_normal(rng), b = rand_q_normal(rng);
            REQUIRE(a.abs2() + b.abs2() + 2*dot(a, b) == Approx((a + b).abs2()));
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
                [](quaternion p) { return p*p; },
                [](quaternion p, quaternion v) { return p*v + v*p; }
            ),
            std::make_pair(
                [](quaternion p) { return 1/p; },
                [](quaternion p, quaternion v) {
                    real p2 = p.abs2();
                    return (v.conj() - (2*dot(p, v)/p2)*p.conj())/p2;
                }
            )
        };

        for (auto p : cases) {
            auto f = p.first;
            auto dfdv = p.second;
            for (int i = 0; i < ATTEMPTS; ++i) {
                quaternion p = rand_q_normal(rng);
                quaternion v = rand_q_unit(rng);
                REQUIRE((f(p + EPS*v) - f(p))/EPS == q_approx(dfdv(p, v)));
            }
        }
    }
    SECTION("Literal") {
        REQUIRE(-2_i == q_approx(quaternion(0,-2,0,0)));
        REQUIRE(-3_j == q_approx(quaternion(0,0,-3,0)));
        REQUIRE(-4_k == q_approx(quaternion(0,0,0,-4)));
    }
};

#endif // __cplusplus
#endif // UNIT_TEST
