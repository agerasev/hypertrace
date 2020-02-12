#pragma once

#include "complex.h"


// # Quaternion

// ## Definition

#ifdef OPENCL

typedef float4 quaternion;

#else // OPENCL

#include <math.h>

typedef struct quaternion quaternion;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus
quaternion q_new(float r, float i, float j, float k);
quaternion q_new_r(float r);
quaternion q_new_c(complex a);

quaternion q_neg(quaternion a);
quaternion q_conj(quaternion a);
float q_abs2(quaternion a);
float q_abs(quaternion a);

quaternion qq_add(quaternion a, quaternion b);
quaternion qr_add(quaternion a, float b);
quaternion rq_add(float a, quaternion b);
quaternion qc_add(quaternion a, complex b);
quaternion cq_add(complex a, quaternion b);

quaternion qq_sub(quaternion a, quaternion b);
quaternion qr_sub(quaternion a, float b);
quaternion rq_sub(float a, quaternion b);
quaternion qc_sub(quaternion a, complex b);
quaternion cq_sub(complex a, quaternion b);

quaternion qq_mul(quaternion a, quaternion b);
quaternion qr_mul(quaternion a, float b);
quaternion rq_mul(float a, quaternion b);
quaternion qc_mul(quaternion a, complex b);
quaternion cq_mul(complex a, quaternion b);

quaternion qr_div(quaternion a, float b);
quaternion q_inv(quaternion a);
quaternion qq_div(quaternion a, quaternion b);
quaternion rq_div(float a, quaternion b);
quaternion qc_div(quaternion a, complex b);
quaternion cq_div(complex a, quaternion b);
#ifdef __cplusplus
};
#endif // __cplusplus


struct quaternion {
    float x, y, z, w;
#ifdef __cplusplus
    static const quaternion i, j, k;
    quaternion() = default;
    quaternion(float r, float i, float j, float k) {
        *this = q_new(r, i, j, k);
    }
    quaternion(float r) {
        *this = q_new_r(r);
    }
    quaternion(complex c) {
        *this = q_new_c(c);
    }

    quaternion conj() const {
        return q_conj(*this);
    }
    float abs2() const {
        return q_abs2(*this);
    }
    float abs() const {
        return q_abs(*this);
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
    quaternion &operator+=(float b) {
        return *this = qr_add(*this, b);
    }
    quaternion &operator-=(quaternion b) {
        return *this = qq_sub(*this, b);
    }
    quaternion &operator-=(complex b) {
        return *this = qc_sub(*this, b);
    }
    quaternion &operator-=(float b) {
        return *this = qr_sub(*this, b);
    }
    quaternion &operator*=(quaternion b) {
        return *this = qq_mul(*this, b);
    }
    quaternion &operator*=(complex b) {
        return *this = qc_mul(*this, b);
    }
    quaternion &operator*=(float b) {
        return *this = qr_mul(*this, b);
    }
    quaternion &operator/=(quaternion b) {
        return *this = qq_div(*this, b);
    }
    quaternion &operator/=(complex b) {
        return *this = qc_div(*this, b);
    }
    quaternion &operator/=(float b) {
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
    quaternion operator+(quaternion a, float b) {
        return qr_add(a, b);
    }
    quaternion operator+(float a, quaternion b) {
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
    quaternion operator-(quaternion a, float b) {
        return qr_sub(a, b);
    }
    quaternion operator-(float a, quaternion b) {
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
    quaternion operator*(quaternion a, float b) {
        return qr_mul(a, b);
    }
    quaternion operator*(float a, quaternion b) {
        return rq_mul(a, b);
    }
    quaternion operator*(quaternion a, complex b) {
        return qc_mul(a, b);
    }
    quaternion operator*(complex a, quaternion b) {
        return cq_mul(a, b);
    }

    quaternion operator/(quaternion a, float b) {
        return qr_div(a, b);
    }
    quaternion operator/(quaternion a, quaternion b) {
        return qq_div(a, b);
    }
    quaternion operator/(float a, quaternion b) {
        return rq_div(a, b);
    }
    quaternion operator/(quaternion a, complex b) {
        return qc_div(a, b);
    }
    quaternion operator/(complex a, quaternion b) {
        return cq_div(a, b);
    }

    quaternion operator "" _j(long double v) {
        return float(v)*quaternion::j;
    }
    quaternion operator "" _j(unsigned long long v) {
        return float(v)*quaternion::j;
    }
    quaternion operator "" _k(long double v) {
        return float(v)*quaternion::k;
    }
    quaternion operator "" _k(unsigned long long v) {
        return float(v)*quaternion::k;
    }
#endif // __cplusplus

#endif // OPENCL


// ## Constructors

quaternion q_new(float r, float i, float j, float k) {
#ifdef OPENCL
    return (quaternion)(r, i, j, k);
#else // OPENCL
    quaternion c;
    c.x = r;
    c.y = i;
    c.z = j;
    c.w = k;
    return c;
#endif // OPENCL
}

quaternion q_new_r(float r) {
    return q_new(r, 0.0f, 0.0f, 0.0f);
}

quaternion q_new_c(complex a) {
    return q_new(a.x, a.y, 0.0f, 0.0f);
}


// ## Unary operators

quaternion q_neg(quaternion a) {
#ifdef OPENCL
    return -a;
#else // OPENCL
    quaternion c;
    c.x = -a.x;
    c.y = -a.y;
    c.z = -a.z;
    c.w = -a.w;
    return c;
#endif // OPENCL
}

quaternion q_conj(quaternion a) {
    quaternion c;
    c.x = a.x;
    c.y = -a.y;
    c.z = -a.z;
    c.w = -a.w;
    return c;
}

float q_abs2(quaternion a) {
    return a.x*a.x + a.y*a.y + a.z*a.z + a.w*a.w;
}

float q_abs(quaternion a) {
    return sqrt(q_abs(a));
}


// ## Binary operators

// ### Addition

quaternion qq_add(quaternion a, quaternion b) {
#ifdef OPENCL
    return a + b;
#else // OPENCL
    quaternion c;
    c.x = a.x + b.x;
    c.y = a.y + b.y;
    c.z = a.z + b.z;
    c.w = a.w + b.w;
    return c;
#endif // OPENCL
}

quaternion qr_add(quaternion a, float b) {
    return qq_add(a, q_new_r(b));
}

quaternion rq_add(float a, quaternion b) {
    return qq_add(q_new_r(a), b);
}

quaternion qc_add(quaternion a, complex b) {
    return qq_add(a, q_new_c(b));
}

quaternion cq_add(complex a, quaternion b) {
    return qq_add(q_new_c(a), b);
}

// ### Subtraction

quaternion qq_sub(quaternion a, quaternion b) {
#ifdef OPENCL
    return a - b;
#else // OPENCL
    return qq_add(a, q_neg(b));
#endif // OPENCL
}

quaternion qr_sub(quaternion a, float b) {
    return qq_sub(a, q_new_r(b));
}

quaternion rq_sub(float a, quaternion b) {
    return qq_sub(q_new_r(a), b);
}

quaternion qc_sub(quaternion a, complex b) {
    return qq_sub(a, q_new_c(b));
}

quaternion cq_sub(complex a, quaternion b) {
    return qq_sub(q_new_c(a), b);
}

// ### Multiplication

quaternion qq_mul(quaternion a, quaternion b) {
    quaternion c;
    c.x = a.x*b.x - a.y*b.y - a.z*b.z - a.w*b.w;
    c.y = a.x*b.y + a.y*b.x + a.z*b.w - a.w*b.z;
    c.z = a.x*b.z + a.z*b.x - a.y*b.w + a.w*b.y;
    c.w = a.x*b.w + a.w*b.x + a.y*b.z - a.z*b.y;
    return c;
}

quaternion qr_mul(quaternion a, float b) {
#ifdef OPENCL
    return a*b;
#else // OPENCL
    quaternion c;
    c.x = a.x*b;
    c.y = a.y*b;
    c.z = a.z*b;
    c.w = a.w*b;
    return c;
#endif // OPENCL
}

quaternion rq_mul(float a, quaternion b) {
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

// ### Division

quaternion qr_div(quaternion a, float b) {
#ifdef OPENCL
    return a / b;
#else // OPENCL
    return qr_mul(a, 1.0f/b);
#endif // OPENCL
}

quaternion q_inv(quaternion a) {
    return qr_div(q_conj(a), q_abs2(a));
}

quaternion qq_div(quaternion a, quaternion b) {
    return qq_mul(a, q_inv(b));
}

quaternion rq_div(float a, quaternion b) {
    return rq_mul(a, q_inv(b));
}

quaternion qc_div(quaternion a, complex b) {
    return qc_mul(a, c_inv(b));
}

quaternion cq_div(complex a, quaternion b) {
    return cq_mul(a, q_inv(b));
}

// ## Tests

#ifdef TEST
#ifdef __cplusplus
#include <catch.hpp>

class qapprox {
private:
    quaternion v;
public:
    qapprox(quaternion q) : v(q) {}

    friend bool operator==(quaternion a, qapprox b) {
        return
            a.x == Approx(b.v.x) &&
            a.y == Approx(b.v.y) &&
            a.z == Approx(b.v.z) &&
            a.w == Approx(b.v.w);
    }
    friend bool operator==(qapprox a, quaternion b) {
        return b == a;
    }
};

TEST_CASE("Quaternions", "[quaternion.h]") {
    SECTION("Imaginary units") {
        REQUIRE(1_i*1_i == qapprox(-1));
        REQUIRE(1_j*1_j == qapprox(-1));
        REQUIRE(1_k*1_k == qapprox(-1));
        REQUIRE(1_i*1_j*1_k == qapprox(-1));

        REQUIRE(1_i*1_j == qapprox(1_k));
        REQUIRE(1_j*1_k == qapprox(1_i));
        REQUIRE(1_k*1_i == qapprox(1_j));
        
        REQUIRE(1_j*1_i == qapprox(-1_k));
        REQUIRE(1_k*1_j == qapprox(-1_i));
        REQUIRE(1_i*1_k == qapprox(-1_j));
    }
    SECTION("Inversion") {
        quaternion a(1, -2, -3, -4);
        quaternion c = a*a.inv();
        REQUIRE(c == qapprox(1));
    }
    SECTION("Literal") {
        REQUIRE(-2_i == qapprox(quaternion(0,-2,0,0)));
        REQUIRE(-3_j == qapprox(quaternion(0,0,-3,0)));
        REQUIRE(-4_k == qapprox(quaternion(0,0,0,-4)));
    }
};

#endif // __cplusplus
#endif // TEST
