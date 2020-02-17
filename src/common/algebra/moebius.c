#include <math.h>

#include "moebius.h"


void moebius_new(Moebius *o, complex a, complex b, complex c, complex d) {
    o->a = a;
    o->b = b;
    o->c = c;
    o->d = d;
}

void moebius_new_identity(Moebius *o) {
    moebius_new(o,
        c_new_r((real)1),
        c_new_r((real)0),
        c_new_r((real)0),
        c_new_r((real)1)
    );
}

quaternion moebius_apply(const Moebius *m, quaternion p) {
    return qq_div(
        qc_add(cq_mul(m->a, p), m->b),
        qc_add(cq_mul(m->c, p), m->d)
    );
}

quaternion moebius_deriv(const Moebius *m, quaternion p, quaternion v) {
    quaternion u = qc_add(cq_mul(m->a, p), m->b);
    quaternion d = qc_add(cq_mul(m->c, p), m->d);
    real d2 = q_abs2(d);
    quaternion s1 = qq_div(cq_mul(m->a, v), d);
    quaternion s21 = q_conj(cq_mul(m->c, v));
    quaternion s22 = rq_mul((real)2*qq_dot(d, cq_mul(m->c, v))/d2, q_conj(d));
    quaternion s2 = qq_mul(u, qr_div(qq_sub(s21, s22), d2));
    return qq_add(s1, s2);
}

complex moebius_det(const Moebius *m) {
    return cc_sub(cc_mul(m->a, m->d), cc_mul(m->b, m->c));
}

void moebius_inverse(Moebius *o, const Moebius *m) {
    o->a = m->d;
    o->b = c_neg(m->b);
    o->c = c_neg(m->c);
    o->d = m->a;
}

void moebius_chain(Moebius *o, const Moebius *k, const Moebius *l) {
    o->a = cc_add(cc_mul(k->a, l->a), cc_mul(k->b, l->c));
    o->b = cc_add(cc_mul(k->a, l->b), cc_mul(k->b, l->d));
    o->c = cc_add(cc_mul(k->c, l->a), cc_mul(k->d, l->c));
    o->d = cc_add(cc_mul(k->c, l->b), cc_mul(k->d, l->d));
}

void moebius_new_zshift(Moebius *o, real l) {
    real l2 = l/(real)2;
    real e = exp(l2);
    moebius_new(o,
        c_new_r(e),
        c_new_r((real)0),
        c_new_r((real)0),
        c_new_r((real)1/e)
    );
}

void moebius_new_xshift(Moebius *o, real l) {
    real l2 = l/(real)2;
    real c = cosh(l2), s = sinh(l2);
    moebius_new(o,
        c_new_r(c),
        c_new_r(s),
        c_new_r(s),
        c_new_r(c)
    );
}

void moebius_new_yshift(Moebius *o, real l) {
    real l2 = l/(real)2;
    real c = cosh(l2), s = sinh(l2);
    moebius_new(o,
        c_new_r(c),
        c_new((real)0, s),
        c_new((real)0, -s),
        c_new_r(c)
    );
}

void moebius_new_zrotate(Moebius *o, real phi) {
    real c = cos(phi/(real)2), s = sin(phi/(real)2);
    moebius_new(o,
        c_new(c, s),
        c_new_r((real)0),
        c_new_r((real)0),
        c_new(c, -s)
    );
}

void moebius_new_xrotate(Moebius *o, real theta) {
    real c = cos(theta/(real)2), s = sin(theta/(real)2);
    moebius_new(o,
        c_new_r(c),
        c_new_r(-s),
        c_new_r(s),
        c_new_r(c)
    );
}

void moebius_new_yrotate(Moebius *o, real theta) {
    real c = cos(theta/(real)2), s = sin(theta/(real)2);
    moebius_new(o,
        c_new_r(c),
        c_new((real)0, s),
        c_new((real)0, s),
        c_new_r(c)
    );
}

void moebius_new_lookat(Moebius *o, quaternion dir) {
	// At first we look at the top (along the z axis)
	real phi = -atan2(dir.y, dir.x);
	real theta = -atan2(sqrt(dir.x*dir.x + dir.y*dir.y), dir.z);

	Moebius a, b;
	moebius_new_zrotate(&a, phi);
	moebius_new_xrotate(&b, theta);
	moebius_chain(o, &b, &a);
}


#ifdef OPENCL_INTEROP

void moebius_pack(MoebiusPacked *p, const Moebius *u) {
    p->a = c_pack(u->a);
    p->b = c_pack(u->b);
    p->c = c_pack(u->c);
    p->d = c_pack(u->d);
}
void moebius_unpack(Moebius *u, const MoebiusPacked *p) {
    u->a = c_unpack(p->a);
    u->b = c_unpack(p->b);
    u->c = c_unpack(p->c);
    u->d = c_unpack(p->d);
}

#endif // OPENCL_INTEROP
