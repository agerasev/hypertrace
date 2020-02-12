#pragma once

#include "complex.h"
#include "quaternion.h"


typedef struct moebius moebius;


#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void moebius_new(moebius *o, complex a, complex b, complex c, complex d);

complex    moebius_apply_c(const moebius *m, complex p);
quaternion moebius_apply_q(const moebius *m, quaternion p);

complex    moebius_deriv_c(const moebius *m, complex p,    complex v);
quaternion moebius_deriv_q(const moebius *m, quaternion p, quaternion v);

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
        return moebius_apply_q(this, p);
    }
    complex apply(complex p) const {
        return moebius_apply_c(this, p);
    }

    quaternion deriv(quaternion p, quaternion v) const {
        return moebius_deriv_q(this, p, v);
    }
    complex deriv(complex p, complex v) const {
        return moebius_deriv_c(this, p, v);
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
#endif // __cplusplus
};


void moebius_new(moebius *o, complex a, complex b, complex c, complex d) {
    o->a = a;
    o->b = b;
    o->c = c;
    o->d = d;
}

complex moebius_apply_c(const moebius *m, complex p) {
    return cc_div(
        cc_add(cc_mul(m->a, p), m->b),
        cc_add(cc_mul(m->c, p), m->d)
    );
}
quaternion moebius_apply_q(const moebius *m, quaternion p) {
    return qq_div(
        qc_add(cq_mul(m->a, p), m->b),
        qc_add(cq_mul(m->c, p), m->d)
    );
}
complex moebius_deriv_c(const moebius *m, complex p, complex v) {
    complex den = c_inv(cc_add(cc_mul(m->c, p), m->d));
    return cc_mul(cc_sub(
        cc_mul(m->a, v),
        cc_mul(cc_mul(cc_add(cc_mul(m->a, p), m->b), cc_mul(m->c, v)), den)
    ), den);
}
quaternion moebius_deriv_q(const moebius *m, quaternion p, quaternion v) {
    quaternion den = q_inv(qc_add(cq_mul(m->c, p), m->d));
    return qq_mul(qq_sub(
        cq_mul(m->a, v),
        qq_mul(qq_mul(qc_add(cq_mul(m->a, p), m->b), cq_mul(m->c, v)), den)
    ), den);
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

#ifdef TEST
#ifdef __cplusplus

#include <random>
#include <vector>
#include <utility>
#include <functional>

#include <catch.hpp>

/*
quaternion moebius_apply_dir(const moebius *m, quaternion p, quaternion v) {
   quaternion one = q_new(0.0f, 0.0f, 1.0f, 0.0f);
   quaternion nv = qq_add(one, rq_mul(1e-4f, v));
   nv = moebius_apply_pos(m, nv);
   return qq_sub(nv, one);
}
*/

class Rng {
private:
    std::minstd_rand rng;
    float second_normal;
    bool second_normal_stored = false;

public:
    Rng(uint32_t seed) : rng(seed) {}
    Rng() : Rng(0xdeadbeef) {}

    float uniform() {
        return float(rng() - rng.min())/(rng.max() - rng.min());
    }
    float normal() {
        if (second_normal_stored) {
            second_normal_stored = false;
            return second_normal;
        } else {
            double phi = 2.0*M_PI*uniform();
            double r = -log(0.5*(1.0 - uniform()));
            second_normal = r*sin(phi);
            second_normal_stored = true;
            return r*cos(phi);
        }
    }

    complex normal_complex() {
        return complex(normal(), normal());
    }
    quaternion normal_quaternion() {
        return quaternion(normal(), normal(), normal(), normal());
    }
    quaternion unit_quaternion() {
        quaternion q;
        do {
            q = normal_quaternion();
        } while (q.abs2() < 1e-2);
        return q/q.abs();
    }
    moebius random_moebius() {
        return moebius(
            normal_complex(),
            normal_complex(),
            normal_complex(),
            normal_complex()
        );
    }
};

TEST_CASE("Moebius transformation", "[moebius.h]") {
    Rng rng;

    SECTION("Chaining") {
        for (int i = 0; i < 16; ++i) {
            moebius a = rng.random_moebius(), b = rng.random_moebius();
            quaternion c = rng.normal_quaternion();
            REQUIRE((a*b).apply(c) == q_approx(a.apply(b.apply(c))));
        }
    }

    SECTION("Derivation") {
        std::vector<std::pair<
            std::function<quaternion(quaternion)>,
            std::function<quaternion(quaternion, quaternion)>
        >> cases = {
            std::make_pair(
                [](quaternion p) { return p; },
                [](quaternion p, quaternion v) { return v; }
            )
        };
        float eps = 1e-4;

        for (auto p : cases) {
            auto f = p.first;
            auto dfdv = p.second;
            for (int i = 0; i < 16; ++i) {
                quaternion p = rng.normal_quaternion();
                quaternion v = rng.unit_quaternion();
                REQUIRE((f(p + eps*v) - f(p))/eps == q_approx(dfdv(p, v)));
            }
        }

        for (int i = 0; i < 16; ++i) {
            moebius a = rng.random_moebius();
            quaternion p = rng.normal_quaternion();
            quaternion v = rng.normal_quaternion();
            
            quaternion d = (a.apply(p + eps*v) - a.apply(p))/eps;
            REQUIRE(a.deriv(p, v) == q_approx(d));
        }
    }
};

#endif // __cplusplus
#endif // TEST
