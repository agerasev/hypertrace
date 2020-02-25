#include "moebius.hh"

#ifndef OPENCL
#include <math.h>
#endif // OPENCL


Moebius mo_new(complex a, complex b, complex c, complex d) {
    return (Moebius){ .a = a, .b = b, .c = c, .d = d };
}
Moebius mo_identity() {
    return mo_new(C1, C0, C0, C1);
}

quaternion mo_apply(Moebius m, quaternion p) {
    return q_div(
        cq_mul(m.a, p) + q_new(m.b, C0),
        cq_mul(m.c, p) + q_new(m.d, C0)
    );
}

quaternion mo_deriv(Moebius m, quaternion p, quaternion v) {
    quaternion u = cq_mul(m.a, p) + q_new(m.b, C0);
    quaternion d = cq_mul(m.c, p) + q_new(m.d, C0);
    real d2 = q_abs2(d);
    quaternion s1 = q_div(cq_mul(m.a, v), d);
    quaternion s21 = q_conj(cq_mul(m.c, v));
    quaternion s22 = ((real)2*dot(d, cq_mul(m.c, v))/d2)*q_conj(d);
    quaternion s2 = q_mul(u, (s21 - s22)/d2);
    return s1 + s2;
}

complex mo_det(Moebius m) {
    return c_mul(m.a, m.d) - c_mul(m.b, m.c);
}

Moebius mo_inverse(Moebius m) {
    return mo_new(m.d, -m.b, -m.c, m.a);
}

Moebius mo_chain(Moebius k, Moebius l) {
    return mo_new(
        c_mul(k.a, l.a) + c_mul(k.b, l.c),
        c_mul(k.a, l.b) + c_mul(k.b, l.d),
        c_mul(k.c, l.a) + c_mul(k.d, l.c),
        c_mul(k.c, l.b) + c_mul(k.d, l.d)
    );
}


#ifdef OPENCL_INTEROP

MoebiusPk pack_moebius(Moebius m) {
    return (MoebiusPk){
        .a = c_pack(m.a),
        .b = c_pack(m.b),
        .c = c_pack(m.c),
        .d = c_pack(m.d)
    };
}

Moebius unpack_moebius(MoebiusPk p) {
    return (Moebius){
        .a = c_unpack(p.a),
        .b = c_unpack(p.b),
        .c = c_unpack(p.c),
        .d = c_unpack(p.d)
    };
}

#endif // OPENCL_INTEROP


#ifdef __cplusplus
std::ostream &operator<<(std::ostream &s, const Moebius &m) {
    return s << "[" << m.a << ", " << m.b << ", " << m.c << ", " << m.d << "]";
}
#endif // __cplusplus


#ifdef UNIT_TEST
#include <catch.hpp>

Moebius random_moebius(TestRng &rng) {
    return mo_new(
        rand_c_normal(rng),
        rand_c_normal(rng),
        rand_c_normal(rng),
        rand_c_normal(rng)
    );
}

TEST_CASE("Moebius transformation", "[moebius]") {
    TestRng rng;

    SECTION("Chaining") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius a = random_moebius(rng), b = random_moebius(rng);
            quaternion c = rand_q_normal(rng);
            REQUIRE(
                mo_apply(mo_chain(a, b), c) == 
                ApproxV(mo_apply(a, mo_apply(b, c)))
            );
        }
    }

    SECTION("Derivation") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius a = random_moebius(rng);
            quaternion p = rand_q_normal(rng);
            quaternion v = rand_q_nonzero(rng);
            
            REQUIRE(
                mo_deriv(a, p, v) ==
                ApproxV((mo_apply(a, p + EPS*v) - mo_apply(a, p))/EPS)
            );
        }
    }
};
#endif // UNIT_TEST
