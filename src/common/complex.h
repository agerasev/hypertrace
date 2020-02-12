#pragma once


// # Complex

// ## Definition

#ifdef OPENCL

typedef float2 complex;

#else // OPENCL

#include <math.h>

typedef struct complex complex;


#ifdef __cplusplus
extern "C" {
#endif // __cplusplus
complex c_new(float r, float i);
complex c_new_r(float r);

complex c_neg(complex a);
complex c_conj(complex a);
float c_abs2(complex a);
float c_abs(complex a);

complex cc_add(complex a, complex b);
complex cr_add(complex a, float b);
complex rc_add(float a, complex b);

complex cc_sub(complex a, complex b);
complex cr_sub(complex a, float b);
complex rc_sub(float a, complex b);

complex cr_mul(complex a, float b);
complex rc_mul(float a, complex b);
complex cc_mul(complex a, complex b);

complex cr_div(complex a, float b);
complex c_inv(complex a);
complex cc_div(complex a, complex b);
complex rc_div(float a, complex b);
#ifdef __cplusplus
};
#endif // __cplusplus


struct complex {
    float x, y;
#ifdef __cplusplus
    static const complex i;

    complex() = default;
    complex(float r, float i) {
        *this = c_new(r, i);
    }
    complex(float r) {
        *this = c_new_r(r);
    }

    complex conj() const {
        return c_conj(*this);
    }
    float abs2() const {
        return c_abs2(*this);
    }
    float abs() const {
        return c_abs(*this);
    }
    complex inv() const {
        return c_inv(*this);
    }

    complex &operator+=(complex b) {
        return *this = cc_add(*this, b);
    }
    complex &operator+=(float b) {
        return *this = cr_add(*this, b);
    }
    complex &operator-=(complex b) {
        return *this = cc_sub(*this, b);
    }
    complex &operator-=(float b) {
        return *this = cr_sub(*this, b);
    }
    complex &operator*=(complex b) {
        return *this = cc_mul(*this, b);
    }
    complex &operator*=(float b) {
        return *this = cr_mul(*this, b);
    }
    complex &operator/=(complex b) {
        return *this = cc_div(*this, b);
    }
    complex &operator/=(float b) {
        return *this = cr_div(*this, b);
    }
#endif // __cplusplus
};

#ifdef __cplusplus
const complex complex::i = complex(0.0f, 1.0f);

complex operator+(complex a) {
    return a;
}
complex operator-(complex a) {
    return c_neg(a);
}

complex operator+(complex a, complex b) {
    return cc_add(a, b);
}
complex operator+(complex a, float b) {
    return cr_add(a, b);
}
complex operator+(float a, complex b) {
    return rc_add(a, b);
}

complex operator-(complex a, complex b) {
    return cc_sub(a, b);
}
complex operator-(complex a, float b) {
    return cr_sub(a, b);
}
complex operator-(float a, complex b) {
    return rc_sub(a, b);
}

complex operator*(complex a, float b) {
    return cr_mul(a, b);
}
complex operator*(float a, complex b) {
    return rc_mul(a, b);
}
complex operator*(complex a, complex b) {
    return cc_mul(a, b);
}

complex operator/(complex a, float b) {
    return cr_div(a, b);
}
complex operator/(complex a, complex b) {
    return cc_div(a, b);
}
complex operator/(float a, complex b) {
    return rc_div(a, b);
}

complex operator "" _i(long double i) {
    return complex(0.0f, float(i));
}
complex operator "" _i(unsigned long long i) {
    return complex(0.0f, float(i));
}

#endif // __cplusplus

#endif // OPENCL


// ## Constructors

complex c_new(float r, float i) {
#ifdef OPENCL
    return (complex)(r, i);
#else // OPENCL
    complex c;
    c.x = r;
    c.y = i;
    return c;
#endif // OPENCL
}

complex c_new_r(float r) {
    return c_new(r, 0.0f);
}


// ## Unary operators

complex c_neg(complex a) {
#ifdef OPENCL
    return -a;
#else // OPENCL
    complex c;
    c.x = -a.x;
    c.y = -a.y;
    return c;
#endif // OPENCL
}

complex c_conj(complex a) {
    complex c;
    c.x = a.x;
    c.y = -a.y;
    return c;
}

float c_abs2(complex a) {
    return a.x*a.x + a.y*a.y;
}

float c_abs(complex a) {
    return sqrt(c_abs(a));
}


// ## Binary operators

// ### Addition

complex cc_add(complex a, complex b) {
#ifdef OPENCL
    return a + b;
#else // OPENCL
    complex c;
    c.x = a.x + b.x;
    c.y = a.y + b.y;
    return c;
#endif // OPENCL
}

complex cr_add(complex a, float b) {
    return cc_add(a, c_new_r(b));
}

complex rc_add(float a, complex b) {
    return cc_add(c_new_r(a), b);
}

// ### Subtraction

complex cc_sub(complex a, complex b) {
#ifdef OPENCL
    return a - b;
#else // OPENCL
    return cc_add(a, c_neg(b));
#endif // OPENCL
}

complex cr_sub(complex a, float b) {
    return cc_sub(a, c_new_r(b));
}

complex rc_sub(float a, complex b) {
    return cc_sub(c_new_r(a), b);
}

// ### Multiplication

complex cr_mul(complex a, float b) {
#ifdef OPENCL
    return a * b;
#else // OPENCL
    complex c;
    c.x = a.x*b;
    c.y = a.y*b;
    return c;
#endif // OPENCL
}

complex rc_mul(float a, complex b) {
    return cr_mul(b, a);
}

complex cc_mul(complex a, complex b) {
    complex c;
    c.x = a.x*b.x - a.y*b.y;
    c.y = a.x*b.y + a.y*b.x;
    return c;
}

// ### Division

complex cr_div(complex a, float b) {
#ifdef OPENCL
    return a / b;
#else // OPENCL
    return cr_mul(a, 1.0f/b);
#endif // OPENCL
}

complex c_inv(complex a) {
    return cr_div(c_conj(a), c_abs2(a));
}

complex cc_div(complex a, complex b) {
    return cc_mul(a, c_inv(b));
}

complex rc_div(float a, complex b) {
    return rc_mul(a, c_inv(b));
}


// ## Tests

#ifdef TEST
#ifdef __cplusplus
#include <catch.hpp>

class capprox {
public:
    complex v;
    capprox(complex c) : v(c) {}
};

bool operator==(complex a, capprox b) {
    return a.x == Approx(b.v.x) && a.y == Approx(b.v.y);
}
bool operator==(capprox a, complex b) {
    return b == a;
}

TEST_CASE("Complex numbers", "[complex.h]") {
    SECTION("Inversion") {
        complex a(1, -2);
        complex c = a*a.inv();
        REQUIRE(c == capprox(1));
    }
    SECTION("Literal") {
        REQUIRE(-2_i == capprox(complex(0, -2)));
    }
};

#endif // __cplusplus
#endif // TEST
