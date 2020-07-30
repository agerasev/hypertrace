#include "matrix.hh"


comp2x2 c22_zero() {
    return c22_new(R0);
}
comp2x2 c22_one() {
    return c22_new(
        C1, C0,
        C0, C1
    );
}

comp2x2 c22_transpose(comp2x2 m) {
    return c22_new(
        m.s01, m.s45,
        m.s23, m.s67
    );
}
comp2x2 c22_mul(comp2x2 m, comp c) {
    return c22_new(
        c_mul(m.s01, c), c_mul(m.s23, c),
        c_mul(m.s45, c), c_mul(m.s67, c)
    );
}
comp2x2 c22_div(comp2x2 m, comp c) {
    return c22_mul(m, c_inverse(c));
}
real c22_norm_l1(comp2x2 m) {
    real8 a8 = fabs(m);
    real4 a4 = a8.s0123 + a8.s4567;
    real2 a2 = a4.xy + a4.zw;
    return a2.x + a2.y;
}

comp2x2 c22_dot(comp2x2 a, comp2x2 b) {
    return c22_new(
        c_mul(a.s01, b.s01) + c_mul(a.s23, b.s45),
        c_mul(a.s01, b.s23) + c_mul(a.s23, b.s67),
        c_mul(a.s45, b.s01) + c_mul(a.s67, b.s45),
        c_mul(a.s45, b.s23) + c_mul(a.s67, b.s67)
    );
}

comp c22_det(comp2x2 m) {
    return c_mul(m.s01, m.s67) - c_mul(m.s23, m.s45);
}
comp2x2 c22_normalize(comp2x2 m) {
    return c22_div(m, c_sqrt(c22_det(m)));
}

comp2x2 c22_inverse(comp2x2 m) {
    return c22_div(c22_new(
        m.s67, -m.s23,
        -m.s45, m.s01
    ), c22_det(m));
}
comp2x2 c22_inverse_n(comp2x2 m) {
    return c22_new(
        m.s67, -m.s23,
        -m.s45, m.s01
    );
}

void _c22_eigen_c(
    comp2x2 m,
    comp2x2 *l,
    comp2x2 *v,
    comp det
) {
    if (c_norm_l1(m.s23) < EPS && c_norm_l1(m.s45) < EPS) {
        *l = c22_new(m.s01, C0, C0, m.s67);
        *v = c22_new(C1, C0, C0, C1);
    } else {
        comp m0p3 = m.s01 + m.s67;
        comp ad = m0p3/2;
        comp dis = c_sqrt(c_mul(ad, ad) - det);
        if (c_norm_l1(dis) > EPS) {
            *l = c22_new(ad + dis, C0, C0, ad - dis);
            if (c_norm_l1(m.s23) > EPS) {
                *v = c22_new(
                    m.s23,
                    m.s23,
                    l->s01 - m.s01,
                    l->s67 - m.s01
                );
            } else {
                *v = c22_new(
                    l->s01 - m.s67,
                    l->s67 - m.s67,
                    m.s45,
                    m.s45
                );
            }
        } else {
            comp m0m3 = m.s01 - m.s67;
            *l = c22_new(ad, C1, C0, ad);
            if (c_norm_l1(m.s23) > EPS) {
                comp g = 4*c_mul(m.s23, m.s23) + c_mul(m0m3, m0m3);
                *v = c22_new(
                    m.s23,
                    c_div(2*c_mul(m.s23, m0m3), g),
                    -m0m3/2,
                    c_div(4*c_mul(m.s23, m.s23), g)
                );
            } else {
                comp g = 4*c_mul(m.s45, m.s45) + c_mul(m0m3, m0m3);
                *v = c22_new(
                    m0m3/2,
                    c_div(4*c_mul(m.s45, m.s45), g),
                    m.s45,
                    -c_div(2*c_mul(m.s45, m0m3), g)
                );
            }
        }
    }
}
void c22_eigen(comp2x2 m, comp2x2 *l, comp2x2 *v) {
    _c22_eigen_c(m, l, v, c22_det(m));
}
void c22_eigen_n(comp2x2 m, comp2x2 *l, comp2x2 *v) {
    _c22_eigen_c(m, l, v, C1);
    *v = c22_normalize(*v);
}

comp2x2 _c22_pow_c(
    comp2x2 m,
    real p,
    comp2x2 j,
    comp2x2 v
) {
    comp2x2 x = m - c22_one();
    real y = c22_norm_l1(x);
    // FIXME: Why 1e4?
    if (y*y > 1e4*EPS) {
        comp2x2 k;
        if (c_norm_l1(j.s23) < EPS) {
            k = c22_new(
                c_powr(j.s01, p),
                C0,
                C0,
                c_powr(j.s67, p)
            );
        } else {
            // Assume j.s01 == j.s67
            comp l = c_powr(j.s01, p);
            k = c22_new(
                l,
                p*c_mul(c_div(l, j.s01), j.s23),
                C0,
                l
            );
        }
        return c22_dot(c22_dot(v, k), c22_inverse(v));
    } else {
        return c22_one() + x*p;
    }
}
comp2x2 c22_pow(comp2x2 m, real p) {
    comp2x2 j, v;
    c22_eigen(m, &j, &v);
    return _c22_pow_c(m, p, j, v);
}
comp2x2 c22_pow_n(comp2x2 m, real p) {
    comp2x2 j, v;
    c22_eigen_n(m, &j, &v);
    comp2x2 r = _c22_pow_c(m, p, j, v);
    r = c22_normalize(r);
    return r;
}


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

real4x4 r44_dot(real4x4 a, real4x4 b) {
    real4x4 tb = r44_transpose(b);
    return r44_transpose(r44_new(
        r44_dot_mv(tb, a.s0123),
        r44_dot_mv(tb, a.s4567),
        r44_dot_mv(tb, a.s89ab),
        r44_dot_mv(tb, a.scdef)
    ));
}
real4 r44_dot_mv(real4x4 a, real4 b) {
    return MAKE(real4)(
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

real r33_det(real4x4 m) {
    return
    + m.s0*m.s5*m.sa + m.s1*m.s6*m.s8 + m.s2*m.s4*m.s9
    - m.s0*m.s6*m.s9 - m.s1*m.s4*m.sa - m.s2*m.s5*m.s8;
}

real4x4 r33_inverse(real4x4 m) {
    real4x4 r = r44_new(
        m.s5*m.sa - m.s6*m.s9, m.s2*m.s9 - m.s1*m.sa, m.s1*m.s6 - m.s2*m.s5, R0,
        m.s6*m.s8 - m.s4*m.sa, m.s0*m.sa - m.s2*m.s8, m.s2*m.s4 - m.s0*m.s6, R0,
        m.s4*m.s9 - m.s5*m.s8, m.s1*m.s8 - m.s0*m.s9, m.s0*m.s5 - m.s1*m.s4, R0,
        R0, R0, R0, R0
    )/r33_det(m);
    r.sf = R1;
    return r;
}

real4x4 r33_clip(real4x4 m) {
    m *= r44_new(
        R1, R1, R1, R0,
        R1, R1, R1, R0,
        R1, R1, R1, R0,
        R0, R0, R0, R0
    );
    m.sf = R1;
    return m;
}
