#pragma once


// # Complex

// ## Definition

#ifdef OPENCL_DEVICE

typedef float real;
typedef float2 complex;

#else // OPENCL_DEVICE

typedef double real;

#ifdef __cplusplus
#include <cmath>
#include <iostream>
#else // __cplusplus
#include <math.h>
#endif // __cplusplus

typedef struct complex complex;


#ifdef __cplusplus
extern "C" {
#endif // __cplusplus
complex c_new(real r, real i);
complex c_new_r(real r);

complex c_neg(complex a);
complex c_conj(complex a);
real c_abs2(complex a);
real c_abs(complex a);
complex c_norm(complex a);

complex cc_add(complex a, complex b);
complex cr_add(complex a, real b);
complex rc_add(real a, complex b);

complex cc_sub(complex a, complex b);
complex cr_sub(complex a, real b);
complex rc_sub(real a, complex b);

complex cr_mul(complex a, real b);
complex rc_mul(real a, complex b);
complex cc_mul(complex a, complex b);

complex cr_div(complex a, real b);
complex c_inv(complex a);
complex cc_div(complex a, complex b);
complex rc_div(real a, complex b);

complex c_sqrt(complex a);
real cc_dot(complex a, complex b);
#ifdef __cplusplus
};
#endif // __cplusplus


struct complex {
    real x, y;
#ifdef __cplusplus
    static const complex i;

    complex() = default;
    complex(real r, real i) {
        *this = c_new(r, i);
    }
    complex(real r) {
        *this = c_new_r(r);
    }

    complex conj() const {
        return c_conj(*this);
    }
    real abs2() const {
        return c_abs2(*this);
    }
    real abs() const {
        return c_abs(*this);
    }
    complex norm() const {
        return c_norm(*this);
    }
    complex inv() const {
        return c_inv(*this);
    }

    complex &operator+=(complex b) {
        return *this = cc_add(*this, b);
    }
    complex &operator+=(real b) {
        return *this = cr_add(*this, b);
    }
    complex &operator-=(complex b) {
        return *this = cc_sub(*this, b);
    }
    complex &operator-=(real b) {
        return *this = cr_sub(*this, b);
    }
    complex &operator*=(complex b) {
        return *this = cc_mul(*this, b);
    }
    complex &operator*=(real b) {
        return *this = cr_mul(*this, b);
    }
    complex &operator/=(complex b) {
        return *this = cc_div(*this, b);
    }
    complex &operator/=(real b) {
        return *this = cr_div(*this, b);
    }
#endif // __cplusplus
};

#ifdef __cplusplus
const complex complex::i = complex(real(0), real(1));

complex operator+(complex a) {
    return a;
}
complex operator-(complex a) {
    return c_neg(a);
}

complex operator+(complex a, complex b) {
    return cc_add(a, b);
}
complex operator+(complex a, real b) {
    return cr_add(a, b);
}
complex operator+(real a, complex b) {
    return rc_add(a, b);
}

complex operator-(complex a, complex b) {
    return cc_sub(a, b);
}
complex operator-(complex a, real b) {
    return cr_sub(a, b);
}
complex operator-(real a, complex b) {
    return rc_sub(a, b);
}

complex operator*(complex a, real b) {
    return cr_mul(a, b);
}
complex operator*(real a, complex b) {
    return rc_mul(a, b);
}
complex operator*(complex a, complex b) {
    return cc_mul(a, b);
}

complex operator/(complex a, real b) {
    return cr_div(a, b);
}
complex operator/(complex a, complex b) {
    return cc_div(a, b);
}
complex operator/(real a, complex b) {
    return rc_div(a, b);
}

complex sqrt(complex a) {
    return c_sqrt(a);
}
real dot(complex a, complex b) {
    return cc_dot(a, b);
}

std::ostream &operator<<(std::ostream &s, complex c) {
    return s << "(" << c.x << ", " << c.y << ")";
}

complex operator "" _i(long double i) {
    return complex((real)0, real(i));
}
complex operator "" _i(unsigned long long i) {
    return complex((real)0, real(i));
}

#endif // __cplusplus

#endif // OPENCL_DEVICE


// ## Constructors

complex c_new(real r, real i) {
#ifdef OPENCL_DEVICE
    return (complex)(r, i);
#else // OPENCL_DEVICE
    complex c;
    c.x = r;
    c.y = i;
    return c;
#endif // OPENCL_DEVICE
}

complex c_new_r(real r) {
    return c_new(r, (real)0);
}


// ## Operators

complex c_neg(complex a) {
#ifdef OPENCL_DEVICE
    return -a;
#else // OPENCL_DEVICE
    complex c;
    c.x = -a.x;
    c.y = -a.y;
    return c;
#endif // OPENCL_DEVICE
}

complex c_conj(complex a) {
    complex c;
    c.x = a.x;
    c.y = -a.y;
    return c;
}

real c_abs2(complex a) {
#ifdef OPENCL_DEVICE
    return dot(a, a);
#else // OPENCL_DEVICE
    return a.x*a.x + a.y*a.y;
#endif // OPENCL_DEVICE
}

real c_abs(complex a) {
#ifdef OPENCL_DEVICE
    return length(a);
#else // OPENCL_DEVICE
    return sqrt(c_abs2(a));
#endif // OPENCL_DEVICE
}

complex c_norm(complex a) {
#ifdef OPENCL_DEVICE
    return normalize(a);
#else // OPENCL_DEVICE
    return cr_div(a, c_abs(a));
#endif // OPENCL_DEVICE
}

// ## Addition

complex cc_add(complex a, complex b) {
#ifdef OPENCL_DEVICE
    return a + b;
#else // OPENCL_DEVICE
    complex c;
    c.x = a.x + b.x;
    c.y = a.y + b.y;
    return c;
#endif // OPENCL_DEVICE
}

complex cr_add(complex a, real b) {
    return cc_add(a, c_new_r(b));
}

complex rc_add(real a, complex b) {
    return cc_add(c_new_r(a), b);
}

// ## Subtraction

complex cc_sub(complex a, complex b) {
#ifdef OPENCL_DEVICE
    return a - b;
#else // OPENCL_DEVICE
    return cc_add(a, c_neg(b));
#endif // OPENCL_DEVICE
}

complex cr_sub(complex a, real b) {
    return cc_sub(a, c_new_r(b));
}

complex rc_sub(real a, complex b) {
    return cc_sub(c_new_r(a), b);
}

// ## Multiplication

complex cr_mul(complex a, real b) {
#ifdef OPENCL_DEVICE
    return a * b;
#else // OPENCL_DEVICE
    complex c;
    c.x = a.x*b;
    c.y = a.y*b;
    return c;
#endif // OPENCL_DEVICE
}

complex rc_mul(real a, complex b) {
    return cr_mul(b, a);
}

complex cc_mul(complex a, complex b) {
    complex c;
    c.x = a.x*b.x - a.y*b.y;
    c.y = a.x*b.y + a.y*b.x;
    return c;
}

// ## Division

complex cr_div(complex a, real b) {
#ifdef OPENCL_DEVICE
    return a / b;
#else // OPENCL_DEVICE
    return cr_mul(a, (real)1/b);
#endif // OPENCL_DEVICE
}

complex c_inv(complex a) {
    return cr_div(c_conj(a), c_abs2(a));
}

complex cc_div(complex a, complex b) {
    return cc_mul(a, c_inv(b));
}

complex rc_div(real a, complex b) {
    return rc_mul(a, c_inv(b));
}

// ## Miscellanous

complex c_sqrt(complex a) {
    real r = sqrt(c_abs(a));
    real phi = 0.5f*atan2(a.y, a.x);
    return c_new(r*cos(phi), r*sin(phi));
}

real cc_dot(complex a, complex b) {
#ifdef OPENCL_DEVICE
    return dot(a, b);
#else // OPENCL_DEVICE
    return a.x*b.x + a.y*b.y;
#endif // OPENCL_DEVICE
}


// # Interoperability

#ifdef OPENCL_INTEROP

#ifdef OPENCL_DEVICE

typedef real real_packed;
typedef complex complex_packed;

#else // OPENCL_DEVICE

#include <CL/cl.h>

typedef cl_float real_packed;
typedef cl_float2 complex_packed;

#endif // OPENCL_DEVICE

real_packed r_pack(real a) {
    return (real_packed)a;
}
real r_unpack(real_packed a) {
    return (real)a;
}

#ifdef OPENCL_DEVICE

complex_packed c_pack(complex a) {
    return (complex_packed)a;
}
complex c_unpack(complex_packed a) {
    return (complex)a;
}

#else // OPENCL_DEVICE

complex_packed c_pack(complex a) {
    complex_packed b;
    b.s[0] = r_pack(a.x);
    b.s[1] = r_pack(a.y);
    return b;
}
complex c_unpack(complex_packed a) {
    return c_new(
        r_unpack(a.s[0]), 
        r_unpack(a.s[1])
    );
}

#endif // OPENCL_DEVICE

#endif // OPENCL_INTEROP

// # Tests

#ifdef UNIT_TEST
#ifdef __cplusplus
#include <catch.hpp>

#include <random>

#define EPS 1e-8
#define PI 3.14159265358979323846
#define ATTEMPTS 16

class Rng {
private:
    std::minstd_rand rng;
    std::uniform_real_distribution<> unif;
    std::normal_distribution<> norm;

public:
    Rng(uint32_t seed) : rng(seed) {}
    Rng() : Rng(0xdeadbeef) {}

    real uniform() {
        return unif(rng);
    }
    real normal() {
        return norm(rng);
    }
};

complex rand_c_normal(Rng &rng) {
    return complex(rng.normal(), rng.normal());
}
complex rand_c_unit(Rng &rng) {
    real phi = 2*PI*rng.uniform();
    return complex(cos(phi), sin(phi));
}
complex rand_c_nonzero(Rng &rng) {
    complex a;
    do {
        a = rand_c_normal(rng);
    } while(a.abs2() < EPS);
    return a;
}

class c_approx {
private:
    complex v;
public:
    c_approx(complex c) : v(c) {}

    friend bool operator==(complex a, c_approx b) {
        return a.x == Approx(b.v.x) && a.y == Approx(b.v.y);
    }
    friend bool operator==(c_approx a, complex b) {
        return b == a;
    }
    friend std::ostream &operator<<(std::ostream &s, c_approx a) {
        return s << a.v;
    }
};

TEST_CASE("Complex numbers", "[complex.h]") {
    Rng rng(0xbeef);

    SECTION("Inversion") {
        for (int i = 0; i < ATTEMPTS; ++i) {
            complex a = rand_c_nonzero(rng);
            REQUIRE(a/a == c_approx(1));
        }
    }
    SECTION("Literal") {
        REQUIRE(-2_i == c_approx(complex(0, -2)));
    }
};

#endif // __cplusplus
#endif // UNIT_TEST
