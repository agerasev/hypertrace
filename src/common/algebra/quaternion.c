#include <math.h>

#include "quaternion.h"


// Constructors

quaternion q_new(real r, real i, real j, real k) {
#ifdef OPENCL_DEVICE
    return (quaternion)(r, i, j, k);
#else // OPENCL_DEVICE
    quaternion c;
    c.x = r;
    c.y = i;
    c.z = j;
    c.w = k;
    return c;
#endif // OPENCL_DEVICE
}

quaternion q_new_r(real r) {
    return q_new(r, (real)0, (real)0, (real)0);
}

quaternion q_new_c(complex a) {
    return q_new(a.x, a.y, (real)0, (real)0);
}


// Operators

quaternion q_neg(quaternion a) {
#ifdef OPENCL_DEVICE
    return -a;
#else // OPENCL_DEVICE
    quaternion c;
    c.x = -a.x;
    c.y = -a.y;
    c.z = -a.z;
    c.w = -a.w;
    return c;
#endif // OPENCL_DEVICE
}

quaternion q_conj(quaternion a) {
    quaternion c;
    c.x = a.x;
    c.y = -a.y;
    c.z = -a.z;
    c.w = -a.w;
    return c;
}

real q_abs2(quaternion a) {
#ifdef OPENCL_DEVICE
    return dot(a, a);
#else // OPENCL_DEVICE
    return a.x*a.x + a.y*a.y + a.z*a.z + a.w*a.w;
#endif // OPENCL_DEVICE
}

real q_abs(quaternion a) {
#ifdef OPENCL_DEVICE
    return length(a);
#else // OPENCL_DEVICE
    return sqrt(q_abs2(a));
#endif // OPENCL_DEVICE
}

quaternion q_norm(quaternion a) {
#ifdef OPENCL_DEVICE
    return normalize(a);
#else // OPENCL_DEVICE
    return qr_div(a, q_abs(a));
#endif // OPENCL_DEVICE
}

// Addition

quaternion qq_add(quaternion a, quaternion b) {
#ifdef OPENCL_DEVICE
    return a + b;
#else // OPENCL_DEVICE
    quaternion c;
    c.x = a.x + b.x;
    c.y = a.y + b.y;
    c.z = a.z + b.z;
    c.w = a.w + b.w;
    return c;
#endif // OPENCL_DEVICE
}

quaternion qr_add(quaternion a, real b) {
    return qq_add(a, q_new_r(b));
}

quaternion rq_add(real a, quaternion b) {
    return qq_add(q_new_r(a), b);
}

quaternion qc_add(quaternion a, complex b) {
    return qq_add(a, q_new_c(b));
}

quaternion cq_add(complex a, quaternion b) {
    return qq_add(q_new_c(a), b);
}

// Subtraction

quaternion qq_sub(quaternion a, quaternion b) {
#ifdef OPENCL_DEVICE
    return a - b;
#else // OPENCL_DEVICE
    return qq_add(a, q_neg(b));
#endif // OPENCL_DEVICE
}

quaternion qr_sub(quaternion a, real b) {
    return qq_sub(a, q_new_r(b));
}

quaternion rq_sub(real a, quaternion b) {
    return qq_sub(q_new_r(a), b);
}

quaternion qc_sub(quaternion a, complex b) {
    return qq_sub(a, q_new_c(b));
}

quaternion cq_sub(complex a, quaternion b) {
    return qq_sub(q_new_c(a), b);
}

// Multiplication

quaternion qq_mul(quaternion a, quaternion b) {
    quaternion c;
    c.x = a.x*b.x - a.y*b.y - a.z*b.z - a.w*b.w;
    c.y = a.x*b.y + a.y*b.x + a.z*b.w - a.w*b.z;
    c.z = a.x*b.z + a.z*b.x - a.y*b.w + a.w*b.y;
    c.w = a.x*b.w + a.w*b.x + a.y*b.z - a.z*b.y;
    return c;
}

quaternion qr_mul(quaternion a, real b) {
#ifdef OPENCL_DEVICE
    return a*b;
#else // OPENCL_DEVICE
    quaternion c;
    c.x = a.x*b;
    c.y = a.y*b;
    c.z = a.z*b;
    c.w = a.w*b;
    return c;
#endif // OPENCL_DEVICE
}

quaternion rq_mul(real a, quaternion b) {
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

// Division

quaternion qr_div(quaternion a, real b) {
#ifdef OPENCL_DEVICE
    return a / b;
#else // OPENCL_DEVICE
    return qr_mul(a, (real)1/b);
#endif // OPENCL_DEVICE
}

quaternion q_inv(quaternion a) {
    return qr_div(q_conj(a), q_abs2(a));
}

quaternion qq_div(quaternion a, quaternion b) {
    return qq_mul(a, q_inv(b));
}

quaternion rq_div(real a, quaternion b) {
    return rq_mul(a, q_inv(b));
}

quaternion qc_div(quaternion a, complex b) {
    return qc_mul(a, c_inv(b));
}

quaternion cq_div(complex a, quaternion b) {
    return cq_mul(a, q_inv(b));
}

// Miscellaneous

real qq_dot(quaternion a, quaternion b) {
#ifdef OPENCL_DEVICE
    return dot(a, b);
#else // OPENCL_DEVICE
    return a.x*b.x + a.y*b.y + a.z*b.z + a.w*b.w;
#endif // OPENCL_DEVICE
}

real qq_dist(quaternion a, quaternion b) {
    real x = (real)1 + q_abs2(qq_sub(a, b))/(a.z*b.z);
    return log(x + sqrt(x*x - 1));
}

// Interoperability

#ifdef OPENCL_INTEROP

#ifdef OPENCL_DEVICE

quaternion_packed q_pack(quaternion a) {
    return (quaternion_packed)a;
}
quaternion q_unpack(quaternion_packed a) {
    return (quaternion)a;
}

#else // OPENCL_DEVICE

typedef cl_float4 quaternion_packed;

quaternion_packed q_pack(quaternion a) {
    quaternion_packed b;
    b.s[0] = r_pack(a.x);
    b.s[1] = r_pack(a.y);
    b.s[2] = r_pack(a.z);
    b.s[3] = r_pack(a.w);
    return b;
}
quaternion q_unpack(quaternion_packed a) {
    return q_new(
        r_unpack(a.s[0]), 
        r_unpack(a.s[1]),
        r_unpack(a.s[2]),
        r_unpack(a.s[3])
    );
}

#endif // OPENCL_DEVICE

#endif // OPENCL_INTEROP

