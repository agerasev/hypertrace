#include "matrix.hh"


comp2x2 comp2x2_zero() {
    return MAKE(comp2x2)(R0);
}
comp2x2 comp2x2_one() {
    return MAKE(comp2x2)(
        C1, C0,
        C0, C1
    );
}

comp2x2 comp2x2_transpose(comp2x2 m) {
    return MAKE(comp2x2)(
        m.s01, m.s45,
        m.s23, m.s67
    );
}
comp2x2 comp2x2_mul(comp2x2 m, comp c) {
    return MAKE(comp2x2)(
        c_mul(m.s01, c), c_mul(m.s23, c),
        c_mul(m.s45, c), c_mul(m.s67, c)
    );
}
comp2x2 comp2x2_div(comp2x2 m, comp c) {
    return comp2x2_mul(m, c_inverse(c));
}
real comp2x2_norm_l1(comp2x2 m) {
    real8 a8 = fabs(m);
    real4 a4 = a8.s0123 + a8.s4567;
    real2 a2 = a4.xy + a4.zw;
    return a2.x + a2.y;
}

comp2x2 comp2x2_dot(comp2x2 a, comp2x2 b) {
    return MAKE(comp2x2)(
        c_mul(a.s01, b.s01) + c_mul(a.s23, b.s45),
        c_mul(a.s01, b.s23) + c_mul(a.s23, b.s67),
        c_mul(a.s45, b.s01) + c_mul(a.s67, b.s45),
        c_mul(a.s45, b.s23) + c_mul(a.s67, b.s67)
    );
}

comp comp2x2_det(comp2x2 m) {
    return c_mul(m.s01, m.s67) - c_mul(m.s23, m.s45);
}
comp2x2 comp2x2_normalize(comp2x2 m) {
    return comp2x2_div(m, c_sqrt(comp2x2_det(m)));
}

comp2x2 comp2x2_inverse(comp2x2 m) {
    return comp2x2_div(MAKE(comp2x2)(
        m.s67, -m.s23,
        -m.s45, m.s01
    ), comp2x2_det(m));
}
comp2x2 comp2x2_inverse_n(comp2x2 m) {
    return MAKE(comp2x2)(
        m.s67, -m.s23,
        -m.s45, m.s01
    );
}

void _comp2x2_eigen_c(
    comp2x2 m,
    comp2x2 *l,
    comp2x2 *v,
    comp det
) {
    if (c_norm_l1(m.s23) < EPS && c_norm_l1(m.s45) < EPS) {
        *l = MAKE(comp2x2)(m.s01, C0, C0, m.s67);
        *v = MAKE(comp2x2)(C1, C0, C0, C1);
    } else {
        comp m0p3 = m.s01 + m.s67;
        comp ad = m0p3/2;
        comp dis = c_sqrt(c_mul(ad, ad) - det);
        if (c_norm_l1(dis) > EPS) {
            *l = MAKE(comp2x2)(ad + dis, C0, C0, ad - dis);
            if (c_norm_l1(m.s23) > EPS) {
                *v = MAKE(comp2x2)(
                    m.s23,
                    m.s23,
                    l->s01 - m.s01,
                    l->s67 - m.s01
                );
            } else {
                *v = MAKE(comp2x2)(
                    l->s01 - m.s67,
                    l->s67 - m.s67,
                    m.s45,
                    m.s45
                );
            }
        } else {
            comp m0m3 = m.s01 - m.s67;
            *l = MAKE(comp2x2)(ad, C1, C0, ad);
            if (c_norm_l1(m.s23) > EPS) {
                comp g = 4*c_mul(m.s23, m.s23) + c_mul(m0m3, m0m3);
                *v = MAKE(comp2x2)(
                    m.s23,
                    c_div(2*c_mul(m.s23, m0m3), g),
                    -m0m3/2,
                    c_div(4*c_mul(m.s23, m.s23), g)
                );
            } else {
                comp g = 4*c_mul(m.s45, m.s45) + c_mul(m0m3, m0m3);
                *v = MAKE(comp2x2)(
                    m0m3/2,
                    c_div(4*c_mul(m.s45, m.s45), g),
                    m.s45,
                    -c_div(2*c_mul(m.s45, m0m3), g)
                );
            }
        }
    }
}
void comp2x2_eigen(comp2x2 m, comp2x2 *l, comp2x2 *v) {
    _comp2x2_eigen_c(m, l, v, comp2x2_det(m));
}
void comp2x2_eigen_n(comp2x2 m, comp2x2 *l, comp2x2 *v) {
    _comp2x2_eigen_c(m, l, v, C1);
    *v = comp2x2_normalize(*v);
}

comp2x2 _comp2x2_pow_c(
    comp2x2 m,
    real p,
    comp2x2 j,
    comp2x2 v
) {
    comp2x2 x = m - comp2x2_one();
    real y = comp2x2_norm_l1(x);
    // FIXME: Why 1e4?
    if (y*y > 1e4*EPS) {
        comp2x2 k;
        if (c_norm_l1(j.s23) < EPS) {
            k = MAKE(comp2x2)(
                c_powr(j.s01, p),
                C0,
                C0,
                c_powr(j.s67, p)
            );
        } else {
            // Assume j.s01 == j.s67
            comp l = c_powr(j.s01, p);
            k = MAKE(comp2x2)(
                l,
                p*c_mul(c_div(l, j.s01), j.s23),
                C0,
                l
            );
        }
        return comp2x2_dot(comp2x2_dot(v, k), comp2x2_inverse(v));
    } else {
        return comp2x2_one() + x*p;
    }
}
comp2x2 comp2x2_pow(comp2x2 m, real p) {
    comp2x2 j, v;
    comp2x2_eigen(m, &j, &v);
    return _comp2x2_pow_c(m, p, j, v);
}
comp2x2 comp2x2_pow_n(comp2x2 m, real p) {
    comp2x2 j, v;
    comp2x2_eigen_n(m, &j, &v);
    comp2x2 r = _comp2x2_pow_c(m, p, j, v);
    r = comp2x2_normalize(r);
    return r;
}
