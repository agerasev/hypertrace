#pragma once

#include "real.hh"


typedef real2 complex;
#define make_complex make_real2
#define c_new make_complex

#ifdef OPENCL_INTEROP
typedef real2_pk complex_pk;
#define pack_complex pack_real2
#define unpack_complex unpack_real2
#endif // OPENCL_INTEROP


complex c_conj(complex a) {
    return c_new(a.x, -a.y);
}
real c_abs2(complex a) {
    return dot(a, a);
}
real c_abs(complex a) {
    return length(a);
}

complex c_norm(complex a) {
    return normalize(a);
}

complex c_mul(complex a, complex b) {
    return c_new(
        a.x*b.x - a.y*b.y,
        a.x*b.y + a.y*b.x
    );
}

complex c_inv(complex a) {
    return c_conj(a)/c_abs2(a);
}

complex c_div(complex a, complex b) {
    return c_mul(a, c_inv(b));
}

complex c_sqrt(complex a) {
    real r = sqrt(c_abs(a));
    real phi = 0.5f*atan2(a.y, a.x);
    return c_new(r*cos(phi), r*sin(phi));
}


#ifdef UNIT_TEST
#ifdef __cplusplus

#include <catch.hpp>

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
    } while(c_abs2(a) < EPS);
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
    friend bool operator==(c_approx a, complex b){
        return b == a;
    }
    friend std::ostream &operator<<(std::ostream &s, c_approx a) {
        return s << a.v;
    }
};


TEST_CASE("Complex numbers", "[complex]") {
    Rng rng(0xbeef);

    SECTION("Constructor") {
        complex a = c_new(0.0, 1.0);
        REQUIRE(a.x == Approx(0.0));
        REQUIRE(a.y == Approx(1.0));
    }

    SECTION("Inversion") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            complex a = rand_c_nonzero(rng);
            std::cout << c_div(a, a) << std::endl;
            c_new(1.0, 0.0);
            //REQUIRE(c_div(a, a) == c_approx(c_new(1, 0)));
        }
    }
};

#endif // __cplusplus
#endif // UNIT_TEST
