#pragma once


// # Complex

// ## Definition

#ifdef OPENCL

typedef float2 complex;

#else // OPENCL

#include <math.h>

typedef struct {
    float x, y;
} complex;

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
