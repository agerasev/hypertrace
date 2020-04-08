#include "quaternion.hh"

#ifndef OPENCL
#include <math.h>
#endif // OPENCL


quaternion q_conj(quaternion a) {
    return q_new(a.x, -a.yzw);
}
real q_abs2(quaternion a) {
    return dot(a, a);
}
real q_abs(quaternion a) {
    return length(a);
}


quaternion q_mul(quaternion a, quaternion b) {
    return q_new(
        a.x*b.x - a.y*b.y - a.z*b.z - a.w*b.w,
        a.x*b.y + a.y*b.x + a.z*b.w - a.w*b.z,
        a.x*b.z + a.z*b.x - a.y*b.w + a.w*b.y,
        a.x*b.w + a.w*b.x + a.y*b.z - a.z*b.y
    );
}

quaternion qc_mul(quaternion a, complex b) {
    return q_new(
        a.x*b.x - a.y*b.y,
        a.x*b.y + a.y*b.x,
        a.z*b.x + a.w*b.y,
        a.w*b.x - a.z*b.y
    );
}

quaternion cq_mul(complex a, quaternion b) {
    return q_new(
        a.x*b.x - a.y*b.y,
        a.x*b.y + a.y*b.x,
        a.x*b.z - a.y*b.w,
        a.x*b.w + a.y*b.z
    );
}

quaternion q_inverse(quaternion a) {
    return q_conj(a)/q_abs2(a);
}

quaternion q_div(quaternion a, quaternion b) {
    return q_mul(a, q_inverse(b));
}

quaternion qc_div(quaternion a, complex b) {
    return qc_mul(a, c_inverse(b));
}

quaternion cq_div(complex a, quaternion b) {
    return cq_mul(a, q_inverse(b));
}


#ifdef UNIT_TEST
#include <catch.hpp>

#include <vector>
#include <utility>
#include <functional>

quaternion rand_q_normal(TestRng &rng) {
    return quaternion(rng.normal(), rng.normal(), rng.normal(), rng.normal());
}
quaternion rand_q_nonzero(TestRng &rng) {
    quaternion a;
    do {
        a = rand_q_normal(rng);
    } while(q_abs2(a) < EPS);
    return a;
}
quaternion rand_q_unit(TestRng &rng) {
    quaternion q = rand_q_nonzero(rng);
    return q/q_abs(q);
}

#endif // UNIT_TEST
