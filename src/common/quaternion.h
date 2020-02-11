#pragma once

#include "complex.h"


// # Quaternion

// ## Definition

#ifdef OPENCL

typedef float4 quaternion;

#else // OPENCL

#include <math.h>

typedef struct {
    float x, y, z, w;
} quaternion;

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
