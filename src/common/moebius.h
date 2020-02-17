#pragma once

#include "complex.h"
#include "quaternion.h"


typedef struct Moebius Moebius;

#ifdef OPENCL_INTEROP

typedef struct MoebiusPacked MoebiusPacked;

void moebius_pack(MoebiusPacked *p, const Moebius *u);
void moebius_unpack(Moebius *u, const MoebiusPacked *p);

#endif // OPENCL_INTEROP


void moebius_new(Moebius *o, complex a, complex b, complex c, complex d);

void moebius_new_identity(Moebius *o);
void moebius_new_zshift(Moebius *o, real l);
void moebius_new_xshift(Moebius *o, real l);
void moebius_new_yshift(Moebius *o, real l);
void moebius_new_zrotate(Moebius *o, real phi);
void moebius_new_xcoil(Moebius *o, real theta);
void moebius_new_ycoil(Moebius *o, real theta);

quaternion moebius_apply(const Moebius *m, quaternion p);
quaternion moebius_deriv(const Moebius *m, quaternion p, quaternion v);

complex moebius_det(const Moebius *m);
void moebius_inverse(Moebius *o, const Moebius *m);

void moebius_chain(Moebius *o, const Moebius *k, const Moebius *l);


struct Moebius {
    complex a, b, c, d;

#ifdef __cplusplus
    Moebius() = default;
    Moebius(complex a, complex b, complex c, complex d) {
        moebius_new(this, a, b, c, d);
    }

    quaternion apply(quaternion p) const {
        return moebius_apply(this, p);
    }

    quaternion deriv(quaternion p, quaternion v) const {
        return moebius_deriv(this, p, v);
    }

    Moebius operator*(const Moebius &other) const {
        Moebius r;
        moebius_chain(&r, this, &other);
        return r;
    }
    Moebius &operator*=(const Moebius &other) {
        Moebius t;
        moebius_chain(&t, this, &other);
        *this = t;
        return *this;
    }

    complex det() const {
        return moebius_det(this);
    }
    Moebius inverse() const {
        Moebius r;
        moebius_inverse(&r, this);
        return r;
    }

    static Moebius identity() {
        Moebius r;
        moebius_new_identity(&r);
        return r;
    }
    static Moebius zshift(real l) {
        Moebius r;
        moebius_new_zshift(&r, l);
        return r;
    }
    static Moebius xshift(real l) {
        Moebius r;
        moebius_new_xshift(&r, l);
        return r;
    }
    static Moebius yshift(real l) {
        Moebius r;
        moebius_new_yshift(&r, l);
        return r;
    }
    static Moebius zrotate(real phi) {
        Moebius r;
        moebius_new_zrotate(&r, phi);
        return r;
    }
    static Moebius xcoil(real theta) {
        Moebius r;
        moebius_new_xcoil(&r, theta);
        return r;
    }
    static Moebius ycoil(real theta) {
        Moebius r;
        moebius_new_ycoil(&r, theta);
        return r;
    }
/*
#ifdef OPENCL_INTEROP
    MoebiusPacked pack() const {
        MoebiusPacked p;
        moebius_pack(&p, this);
        return p;
    }
#endif // OPENCL_INTEROP
*/
#endif // __cplusplus
};

#ifdef OPENCL_INTEROP

struct __attribute__ ((packed)) MoebiusPacked {
    complex_packed a, b, c, d;
/*
#ifdef __cplusplus
    Moebius unpack() const {
        Moebius u;
        moebius_unpack(&u, this);
        return u;
    }
#endif // __cplusplus
*/
};

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

#ifdef __cplusplus
std::ostream &operator<<(std::ostream &s, const Moebius &m) {
    return s << "[" << m.a << ", " << m.b << ", " << m.c << ", " << m.d << "]";
}
#endif // __cplusplus

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
    moebius_new(o,
        c_new_r(exp(l2)),
        c_new_r((real)0),
        c_new_r((real)0),
        c_new_r(exp(-l2))
    );
}

void moebius_new_xshift(Moebius *o, real l) {
    real l2 = l/(real)2;
    moebius_new(o,
        c_new_r(cosh(l2)),
        c_new_r(sinh(l2)),
        c_new_r(sinh(l2)),
        c_new_r(cosh(l2))
    );
}

void moebius_new_yshift(Moebius *o, real l) {
    real l2 = l/(real)2;
    moebius_new(o,
        c_new_r(cosh(l2)),
        c_new((real)0, sinh(l2)),
        c_new((real)0, -sinh(l2)),
        c_new_r(cosh(l2))
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

void moebius_new_xcoil(Moebius *o, real theta) {
    real c = cos(theta/(real)2), s = sin(theta/(real)2);
    moebius_new(o,
        c_new_r(c),
        c_new_r(-s),
        c_new_r(s),
        c_new_r(c)
    );
}

void moebius_new_ycoil(Moebius *o, real theta) {
    real c = cos(theta/(real)2), s = sin(theta/(real)2);
    moebius_new(o,
        c_new_r(c),
        c_new((real)0, s),
        c_new((real)0, s),
        c_new_r(c)
    );
}


#ifdef UNIT_TEST
#ifdef __cplusplus
#include <catch.hpp>


Moebius random_moebius(Rng &rng) {
    return Moebius(
        rand_c_normal(rng),
        rand_c_normal(rng),
        rand_c_normal(rng),
        rand_c_normal(rng)
    );
}

TEST_CASE("Moebius transformation", "[Moebius.h]") {
    Rng rng;

    SECTION("Chaining") {
        for (int i = 0; i < ATTEMPTS; ++i) {
            Moebius a = random_moebius(rng), b = random_moebius(rng);
            quaternion c = rand_q_normal(rng);
            REQUIRE((a*b).apply(c) == q_approx(a.apply(b.apply(c))));
        }
    }

    SECTION("Derivation") {
        for (int i = 0; i < ATTEMPTS; ++i) {
            Moebius a = random_moebius(rng);
            quaternion p = rand_q_normal(rng);
            quaternion v = rand_q_nonzero(rng);
            
            REQUIRE(a.deriv(p, v) == q_approx((a.apply(p + EPS*v) - a.apply(p))/EPS));
        }
    }

    SECTION("Rotation of derivative") {
        for (int i = 0; i < ATTEMPTS; ++i) {
            quaternion q = q_norm(q_new(rng.normal(), rng.normal(), 1, 0));
            real phi = -atan2(q.y, q.x);
            real theta = -atan2(sqrt(q.x*q.x + q.y*q.y), q.z);

            Moebius a, b, c;
            moebius_new_zrotate(&a, phi);
            moebius_new_xcoil(&b, theta);
            moebius_chain(&c, &b, &a);
            REQUIRE(c.deriv(quaternion::j, q) == q_approx(quaternion::j));
        }
    }
};

#endif // __cplusplus
#endif // UNIT_TEST
