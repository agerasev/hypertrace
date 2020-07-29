#include "complex.hh"


comp c_conj(comp a) {
    return c_new(a.x, -a.y);
}
real c_abs2(comp a) {
    return dot(a, a);
}
real c_abs(comp a) {
    return length(a);
}
real c_norm_l1(comp a) {
    return fabs(a.x) + fabs(a.y);
}

comp c_mul(comp a, comp b) {
    return c_new(
        a.x*b.x - a.y*b.y,
        a.x*b.y + a.y*b.x
    );
}

comp c_inverse(comp a) {
    return c_conj(a)/c_abs2(a);
}

comp c_div(comp a, comp b) {
    return c_mul(a, c_inverse(b));
}

comp c_exp(comp p) {
    return exp(p.x)*c_new(cos(p.y), sin(p.y));
}
comp c_powr(comp a, real p) {
    real r = pow(c_abs2(a), p/2);
    real phi = p*atan2(a.y, a.x);
    return c_new(r*cos(phi), r*sin(phi));
}
comp c_sqrt(comp a) {
    real r = sqrt(c_abs(a));
    real phi = 0.5f*atan2(a.y, a.x);
    return c_new(r*cos(phi), r*sin(phi));
}


#ifdef UNIT_TEST
#include <catch.hpp>

#include <iostream>

TEST_CASE("Complex numbers", "[comp]") {
    TestRng<comp> crng(0xbeef);

    SECTION("Constructor") {
        comp a = c_new(0.0, 1.0);
        REQUIRE(a.x == Approx(0.0));
        REQUIRE(a.y == Approx(1.0));
    }

    SECTION("Inversion") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp a = crng.nonzero();
            REQUIRE(c_div(a, a) == approx(C1));
        }
    }
};
#endif // UNIT_TEST
