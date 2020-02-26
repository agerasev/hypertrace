#ifndef OPENCL
#include <math.h>
#endif // OPENCL

#include "complex.hh"


complex c_conj(complex a) {
    return c_new(a.x, -a.y);
}
real c_abs2(complex a) {
    return dot(a, a);
}
real c_abs(complex a) {
    return length(a);
}

complex c_mul(complex a, complex b) {
    return c_new(
        a.x*b.x - a.y*b.y,
        a.x*b.y + a.y*b.x
    );
}

complex c_inverse(complex a) {
    return c_conj(a)/c_abs2(a);
}

complex c_div(complex a, complex b) {
    return c_mul(a, c_inverse(b));
}

complex c_exp(complex p) {
    return exp(p.x)*c_new(cos(p.y), sin(p.y));
}
complex c_powr(complex a, real p) {
    real r = pow(c_abs2(a), p/2);
    real phi = p*atan2(a.y, a.x);
    return c_new(r*cos(phi), r*sin(phi));
}
complex c_sqrt(complex a) {
    real r = sqrt(c_abs(a));
    real phi = 0.5f*atan2(a.y, a.x);
    return c_new(r*cos(phi), r*sin(phi));
}


#ifdef UNIT_TEST
#include <catch.hpp>

#include <iostream>

complex rand_c_normal(TestRng &rng) {
    return complex(rng.normal(), rng.normal());
}
complex rand_c_unit(TestRng &rng) {
    real phi = 2*PI*rng.uniform();
    return complex(cos(phi), sin(phi));
}
complex rand_c_nonzero(TestRng &rng) {
    complex a;
    do {
        a = rand_c_normal(rng);
    } while(c_abs2(a) < EPS);
    return a;
}

TEST_CASE("Complex numbers", "[complex]") {
    TestRng rng(0xbeef);

    SECTION("Constructor") {
        complex a = c_new(0.0, 1.0);
        REQUIRE(a.x == Approx(0.0));
        REQUIRE(a.y == Approx(1.0));
    }

    SECTION("Inversion") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            complex a = rand_c_nonzero(rng);
            REQUIRE(c_div(a, a) == ApproxV(C1));
        }
    }
};
#endif // UNIT_TEST
