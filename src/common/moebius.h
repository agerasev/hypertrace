#pragma once

#include "complex.h"
#include "quaternion.h"


typedef struct moebius moebius;


#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void moebius_new(moebius *o, complex a, complex b, complex c, complex d);
void moebius_new_zshift(moebius *o, real l);
void moebius_new_zrotate(moebius *o, real phi);
void moebius_new_xcoil(moebius *o, real theta);

quaternion moebius_apply(const moebius *m, quaternion p);
quaternion moebius_deriv(const moebius *m, quaternion p, quaternion v);

complex moebius_det(const moebius *m);
void moebius_inverse(moebius *o, const moebius *m);

void moebius_chain(moebius *o, const moebius *k, const moebius *l);


#ifdef __cplusplus
};
#endif // __cplusplus


struct moebius {
    complex a, b, c, d;

#ifdef __cplusplus
    moebius() = default;
    moebius(complex a, complex b, complex c, complex d) {
        moebius_new(this, a, b, c, d);
    }

    quaternion apply(quaternion p) const {
        return moebius_apply(this, p);
    }

    quaternion deriv(quaternion p, quaternion v) const {
        return moebius_deriv(this, p, v);
    }

    moebius operator*(const moebius &other) const {
        moebius r;
        moebius_chain(&r, this, &other);
        return r;
    }

    complex det() const {
        return moebius_det(this);
    }
    moebius inverse() const {
        moebius r;
        moebius_inverse(&r, this);
        return r;
    }

    static moebius zshift(real l) {
        moebius r;
        moebius_new_zshift(&r, l);
        return r;
    }
    static moebius zrotate(real phi) {
        moebius r;
        moebius_new_zrotate(&r, phi);
        return r;
    }
    static moebius xcoil(real theta) {
        moebius r;
        moebius_new_xcoil(&r, theta);
        return r;
    }
#endif // __cplusplus
};

#ifdef __cplusplus
std::ostream &operator<<(std::ostream &s, const moebius &m) {
    return s << "[" << m.a << ", " << m.b << ", " << m.c << ", " << m.d << "]";
}
#endif // __cplusplus

void moebius_new(moebius *o, complex a, complex b, complex c, complex d) {
    o->a = a;
    o->b = b;
    o->c = c;
    o->d = d;
}

quaternion moebius_apply(const moebius *m, quaternion p) {
    return qq_div(
        qc_add(cq_mul(m->a, p), m->b),
        qc_add(cq_mul(m->c, p), m->d)
    );
}

quaternion moebius_deriv(const moebius *m, quaternion p, quaternion v) {
    quaternion u = qc_add(cq_mul(m->a, p), m->b);
    quaternion d = qc_add(cq_mul(m->c, p), m->d);
    real d2 = q_abs2(d);
    quaternion s1 = qq_div(cq_mul(m->a, v), d);
    quaternion s21 = q_conj(cq_mul(m->c, v));
    quaternion s22 = rq_mul((real)2*qq_dot(d, cq_mul(m->c, v))/d2, q_conj(d));
    quaternion s2 = qq_mul(u, qr_div(qq_sub(s21, s22), d2));
    return qq_add(s1, s2);
}

complex moebius_det(const moebius *m) {
    return cc_sub(cc_mul(m->a, m->d), cc_mul(m->b, m->c));
}

void moebius_inverse(moebius *o, const moebius *m) {
    o->a = m->d;
    o->b = c_neg(m->b);
    o->c = c_neg(m->c);
    o->d = m->a;
}

void moebius_chain(moebius *o, const moebius *k, const moebius *l) {
    o->a = cc_add(cc_mul(k->a, l->a), cc_mul(k->b, l->c));
    o->b = cc_add(cc_mul(k->a, l->b), cc_mul(k->b, l->d));
    o->c = cc_add(cc_mul(k->c, l->a), cc_mul(k->d, l->c));
    o->d = cc_add(cc_mul(k->c, l->b), cc_mul(k->d, l->d));
}

void moebius_new_zshift(moebius *o, real l) {
    moebius_new(o,
        c_new_r(exp(l/(real)2)),
        c_new_r((real)0),
        c_new_r((real)0),
        c_new_r(exp(-l/(real)2))
    );
}

void moebius_new_zrotate(moebius *o, real phi) {
    real c = cos(phi/(real)2), s = sin(phi/(real)2);
    moebius_new(o,
        c_new(c, s),
        c_new_r((real)0),
        c_new_r((real)0),
        c_new(c, -s)
    );
}

void moebius_new_xcoil(moebius *o, real theta) {
    real c = cos(theta/(real)2), s = sin(theta/(real)2);
    moebius_new(o,
        c_new_r(c),
        c_new_r(-s),
        c_new_r(s),
        c_new_r(c)
    );
}


#ifdef TEST
#ifdef __cplusplus
#include <catch.hpp>


moebius random_moebius(Rng &rng) {
    return moebius(
        rand_c_normal(rng),
        rand_c_normal(rng),
        rand_c_normal(rng),
        rand_c_normal(rng)
    );
}

TEST_CASE("Moebius transformation", "[moebius.h]") {
    Rng rng;

    SECTION("Chaining") {
        for (int i = 0; i < ATTEMPTS; ++i) {
            moebius a = random_moebius(rng), b = random_moebius(rng);
            quaternion c = rand_q_normal(rng);
            REQUIRE((a*b).apply(c) == q_approx(a.apply(b.apply(c))));
        }
    }

    SECTION("Derivation") {
        for (int i = 0; i < ATTEMPTS; ++i) {
            moebius a = random_moebius(rng);
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

            moebius a, b, c;
            moebius_new_zrotate(&a, phi);
            moebius_new_xcoil(&b, theta);
            moebius_chain(&c, &b, &a);
            REQUIRE(c.deriv(quaternion::j, q) == q_approx(quaternion::j));
        }
    }
};

#endif // __cplusplus
#endif // TEST
