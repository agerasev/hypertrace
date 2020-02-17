#include "complex.h"


const complex complex::i = complex(real(0), real(1));

complex::complex() = default;
complex::complex(real r, real i) {
    *this = c_new(r, i);
}
complex::complex(real r) {
    *this = c_new_r(r);
}

complex complex::conj() const {
    return c_conj(*this);
}
real complex::abs2() const {
    return c_abs2(*this);
}
real complex::abs() const {
    return c_abs(*this);
}
complex complex::norm() const {
    return c_norm(*this);
}
complex complex::inv() const {
    return c_inv(*this);
}

complex &complex::operator+=(complex b) {
    return *this = cc_add(*this, b);
}
complex &complex::operator+=(real b) {
    return *this = cr_add(*this, b);
}
complex &complex::operator-=(complex b) {
    return *this = cc_sub(*this, b);
}
complex &complex::operator-=(real b) {
    return *this = cr_sub(*this, b);
}
complex &complex::operator*=(complex b) {
    return *this = cc_mul(*this, b);
}
complex &complex::operator*=(real b) {
    return *this = cr_mul(*this, b);
}
complex &complex::operator/=(complex b) {
    return *this = cc_div(*this, b);
}
complex &complex::operator/=(real b) {
    return *this = cr_div(*this, b);
}

#ifdef OPENCL_INTEROP
complex_packed complex::pack() const {
    return c_pack(*this);
}
#endif // OPENCL_INTEROP


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


#ifdef UNIT_TEST

#include <catch.hpp>


complex rand_c_normal(Rng &rng) {
    return complex(rng.normal(), rng.normal());
}
complex rand_c_unit(Rng &rng) {
    real phi = 2*TEST_PI*rng.uniform();
    return complex(cos(phi), sin(phi));
}
complex rand_c_nonzero(Rng &rng) {
    complex a;
    do {
        a = rand_c_normal(rng);
    } while(a.abs2() < TEST_EPS);
    return a;
}


c_approx::c_approx(complex c) : v(c) {}

bool operator==(complex a, c_approx b) {
    return a.x == Approx(b.v.x) && a.y == Approx(b.v.y);
}
bool operator==(c_approx a, complex b) {
    return b == a;
}
std::ostream &operator<<(std::ostream &s, c_approx a) {
    return s << a.v;
}

TEST_CASE("Complex numbers", "[complex]") {
    Rng rng(0xbeef);

    SECTION("Inversion") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            complex a = rand_c_nonzero(rng);
            REQUIRE(a/a == c_approx(1));
        }
    }
    SECTION("Literal") {
        REQUIRE(-2_i == c_approx(complex(0, -2)));
    }
};

#endif // UNIT_TEST
