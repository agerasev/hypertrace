#include "matrix.hh"


// real2x2

real2x2 r22_zero() {
    return r22_new(R0);
}
real2x2 r22_one() {
    return r22_new(R1, R0, R0, R1);
}

real2x2 r22_transpose(real2x2 m) {
    return r22_new(
        m.x, m.z,
        m.y, m.w
    );
}

real r22_norm_l1(real2x2 m) {
    real2x2 am = fabs(m);
    real2 r2 = am.xy + am.zw;
    return r2.x + r2.y;
}

real2x2 r22_dot(real2x2 a, real2x2 b) {
    return r22_new(
        a.x*b.x + a.y*b.z,
        a.x*b.y + a.y*b.w,
        a.z*b.x + a.w*b.z,
        a.z*b.y + a.w*b.w
    );
}
real2 r22_dot_mv(real2x2 a, real2 b) {
    return r2_new(
        a.x*b.x + a.y*b.y,
        a.z*b.x + a.w*b.y
    );
}
real2 r22_dot_vm(real2 a, real2x2 b) {
    return r2_new(
        a.x*b.x + a.y*b.z,
        a.x*b.y + a.y*b.w
    );
}
real2x2 r22_outer(real2 a, real2 b) {
    return r22_new(
        a.x*b.x, a.x*b.y,
        a.y*b.x, a.y*b.y
    );
}

real r22_det(real2x2 m) {
    return m.x*m.w - m.y*m.z;
}

real2x2 r22_inverse(real2x2 m) {
    return r22_new(
        m.w, -m.y,
        -m.z, m.x
    )/r22_det(m);
}

real2x2 r22_from_comp(comp c) {
    return r22_new(
        c.x, c.y,
        -c.y, c.x
    );
}


// real3x3

real3x3 r33_zero() {
    return r44_zero();
}
real3x3 r33_one() {
    return r44_one();
}

real3x3 r33_transpose(real3x3 m) {
    return r44_transpose(m);
}

real r33_norm_l1(real3x3 m) {
    real3x3 am = fabs(m);
    real3 r3 = (am.s0123 + am.s4567 + am.s89ab).xyz;
    return r3.x + r3.y + r3.z;
}

real3x3 r33_dot(real3x3 a, real3x3 b) {
    return r44_dot(a, b);
}
real3 r33_dot_mv(real3x3 a, real3 b) {
    return r44_dot_mv(a, r4_new(b, R0)).xyz;
}
real3 r33_dot_vm(real3 a, real3x3 b) {
    return r44_dot_vm(r4_new(a, R0), b).xyz;
}
real3x3 r33_outer(real3 a, real3 b) {
    real4x4 m = r44_outer(r4_new(a, R0), r4_new(b, R0));
    m.sf = R1;
    return m;
}

real r33_det(real4x4 m) {
    return
    + m.s0*m.s5*m.sa + m.s1*m.s6*m.s8 + m.s2*m.s4*m.s9
    - m.s0*m.s6*m.s9 - m.s1*m.s4*m.sa - m.s2*m.s5*m.s8;
}

real3x3 r33_inverse(real3x3 m) {
    real4x4 r = r44_new(
        m.s5*m.sa - m.s6*m.s9, m.s2*m.s9 - m.s1*m.sa, m.s1*m.s6 - m.s2*m.s5, R0,
        m.s6*m.s8 - m.s4*m.sa, m.s0*m.sa - m.s2*m.s8, m.s2*m.s4 - m.s0*m.s6, R0,
        m.s4*m.s9 - m.s5*m.s8, m.s1*m.s8 - m.s0*m.s9, m.s0*m.s5 - m.s1*m.s4, R0,
        R0, R0, R0, R0
    )/r33_det(m);
    r.sf = R1;
    return r;
}

// real4x4

real4x4 r44_zero() {
    return r44_new(R0);
}

real4x4 r44_one() {
    return r44_new(
        R1, R0, R0, R0,
        R0, R1, R0, R0,
        R0, R0, R1, R0,
        R0, R0, R0, R1
    );
}

real4x4 r44_transpose(real4x4 m) {
    return r44_new(
        m.s0, m.s4, m.s8, m.sc,
        m.s1, m.s5, m.s9, m.sd,
        m.s2, m.s6, m.sa, m.se,
        m.s3, m.s7, m.sb, m.sf
    );
}

real r44_norm_l1(real4x4 m) {
    real16 r16 = fabs(m);
    real8 r8 = r16.s01234567 + r16.s89abcdef;
    real4 r4 = r8.s0123 + r8.s4567;
    real2 r2 = r4.xy + r4.zw;
    return r2.x + r2.y;
}

real4x4 r44_dot(real4x4 a, real4x4 b) {
    real4x4 tb = r44_transpose(b);
    return r44_new(
        r44_dot_mv(tb, a.s0123),
        r44_dot_mv(tb, a.s4567),
        r44_dot_mv(tb, a.s89ab),
        r44_dot_mv(tb, a.scdef)
    );
}
real4 r44_dot_mv(real4x4 a, real4 b) {
    return r4_new(
        dot(a.s0123, b),
        dot(a.s4567, b),
        dot(a.s89ab, b),
        dot(a.scdef, b)
    );
}
real4 r44_dot_vm(real4 a, real4x4 b) {
    return r44_dot_mv(r44_transpose(b), a);
}

real4x4 r44_outer(real4 a, real4 b) {
    return r44_new(a.x*b, a.y*b, a.z*b, a.w*b);
}

real4x4 r44_clip_to_r33(real4x4 m) {
    m *= r44_new(
        R1, R1, R1, R0,
        R1, R1, R1, R0,
        R1, R1, R1, R0,
        R0, R0, R0, R0
    );
    m.sf = R1;
    return m;
}

real4x4 r44_from_quat(quat q) {
    return r44_new(
        q.x, q.y, q.z, q.w,
        -q.y, q.x, -q.w, q.z,
        -q.z, q.w, q.x, -q.y,
        -q.w, -q.z, q.y, q.x
    );
}
