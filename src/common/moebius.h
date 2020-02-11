#pragma once

#include "complex.h"
#include "quaternion.h"


typedef struct {
    complex a, b, c, d;
} moebius;

void moebius_new(moebius *o, complex a, complex b, complex c, complex d) {
    o->a = a;
    o->b = b;
    o->c = c;
    o->d = d;
}

quaternion moebius_apply_pos(const moebius *m, quaternion p) {
    return qq_div(
        qc_add(cq_mul(m->a, p), m->b),
        qc_add(cq_mul(m->c, p), m->d)
    );
}
/*
quaternion moebius_apply_dir(const moebius *m, quaternion p, quaternion v) {
    quaternion den = q_inv(qc_add(cq_mul(m->c, p), m->d));
    return normalize(qq_mul(qq_sub(
        cq_mul(m->a, v),
        qq_mul(qq_mul(qc_add(cq_mul(m->a, p), m->b), cq_mul(m->c, v)), den)
    ), den));
}
*/
quaternion moebius_apply_dir(const moebius *m, quaternion p, quaternion v) {
   quaternion one = q_new(0.0f, 0.0f, 1.0f, 0.0f);
   quaternion nv = qq_add(one, rq_mul(1e-4f, v));
   nv = moebius_apply_pos(m, nv);
   return normalize(qq_sub(nv, one));
}

void moebius_chain(moebius *o, const moebius *k, const moebius *l) {
    o->a = cc_add(cc_mul(k->a, l->a), cc_mul(k->b, l->c));
    o->b = cc_add(cc_mul(k->a, l->b), cc_mul(k->b, l->d));
    o->c = cc_add(cc_mul(k->c, l->a), cc_mul(k->d, l->c));
    o->d = cc_add(cc_mul(k->c, l->b), cc_mul(k->d, l->d));
}

void moebius_inverse(moebius *o, const moebius *m) {
    o->a = m->d;
    o->b = -m->b;
    o->c = -m->c;
    o->d = m->a;
}
