#include <math.h>

#include "complex.h"


// Constructors

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


// Operators

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

// Addition

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

// Subtraction

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

// Multiplication

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

// Division

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

// Miscellaneous

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


// Interoperability

#ifdef OPENCL_INTEROP

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
