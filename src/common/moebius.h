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

complex moebius_apply_c(const moebius *m, complex c) {
    return cc_div(
        cc_add(cc_mul(m->a, c), m->b),
        cc_add(cc_mul(m->c, c), m->d)
    );
}

quaternion moebius_apply_q(const moebius *m, quaternion q) {
    return qq_div(
        qc_add(cq_mul(m->a, q), m->b),
        qc_add(cq_mul(m->c, q), m->d)
    );
}

void moebius_chain(moebius *o, const moebius *k, const moebius *l) {
    o->a = cc_add(cc_mul(k->a, l->a), cc_mul(k->b, l->c));
    o->b = cc_add(cc_mul(k->a, l->b), cc_mul(k->b, l->d));
    o->c = cc_add(cc_mul(k->c, l->a), cc_mul(k->d, l->c));
    o->d = cc_add(cc_mul(k->c, l->b), cc_mul(k->d, l->d));
}
