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
Moebius mo_normalize(Moebius m) {
    complex d = c_sqrt(mo_det(m));
    m.a = c_div(m.a, d);
    m.b = c_div(m.b, d);
    m.c = c_div(m.c, d);
    m.d = c_div(m.d, d);
    return m;
}

Moebius mo_inverse(Moebius m) {
#ifdef MOEBIUS_DENORMALIZED
    complex det = mo_det(m);
    return mo_new(
        c_div(m.d, det), c_div(-m.b, det),
        c_div(-m.c, det), c_div(m.a, det)
    );
#else  // MOEBIUS_DENORMALIZED
    return mo_new(m.d, -m.b, -m.c, m.a);
#endif // MOEBIUS_DENORMALIZED
}

Moebius mo_chain(Moebius k, Moebius l) {
    return mo_new(
        c_mul(k.a, l.a) + c_mul(k.b, l.c),
        c_mul(k.a, l.b) + c_mul(k.b, l.d),
        c_mul(k.c, l.a) + c_mul(k.d, l.c),
        c_mul(k.c, l.b) + c_mul(k.d, l.d)
    );
}

void mo_eigen(Moebius m, complex *l, Moebius *v) {
    if (c_abs2(m.b) < EPS && c_abs2(m.c) < EPS) {
        l[0] = m.a;
        l[1] = m.d;
        *v = mo_new(
            C1, C0,
            C0, C1
        );
    } else {
        complex ad = (m.a + m.d)/2;
        complex D = c_sqrt(c_mul(ad, ad) - mo_det(m));
        l[0] = ad + D;
        l[1] = ad - D;

        if (c_abs2(m.c) > EPS) {
            *v = mo_new(
                l[0] - m.d, l[1] - m.d,
                m.c, m.c
            );
        } else {
            *v = mo_new(
                m.b, m.b,
                l[0] - m.a, l[1] - m.a
            );
        }
#ifndef MOEBIUS_DENORMALIZED
        *v = mo_normalize(*v);
#endif // MOEBIUS_DENORMALIZED
    }
}

Moebius mo_pow(Moebius m, real p) {
    complex l[2];
    Moebius v;
    mo_eigen(m, l, &v);
    return mo_chain(mo_chain(v, mo_new(
        c_powr(l[0], p), C0, C0, c_powr(l[1], p)
    )), mo_inverse(v));
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
    Moebius m = mo_new(
        rand_c_normal(rng),
        rand_c_normal(rng),
        rand_c_normal(rng),
        rand_c_normal(rng)
    );
#ifndef MOEBIUS_DENORMALIZED
    m = mo_normalize(m);
#endif // MOEBIUS_DENORMALIZED
    return m;
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

    SECTION("Eigenvalues and eigenvectors") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius m = random_moebius(rng);
            complex l[2];
            Moebius v;
            mo_eigen(m, l, &v);

            REQUIRE(c_mul(m.a, v.a) + c_mul(m.b, v.c) == ApproxV(c_mul(l[0], v.a)));
            REQUIRE(c_mul(m.c, v.a) + c_mul(m.d, v.c) == ApproxV(c_mul(l[0], v.c)));
            REQUIRE(c_mul(m.a, v.b) + c_mul(m.b, v.d) == ApproxV(c_mul(l[1], v.b)));
            REQUIRE(c_mul(m.c, v.b) + c_mul(m.d, v.d) == ApproxV(c_mul(l[1], v.d)));

            Moebius o = mo_chain(mo_chain(v, mo_new(l[0], C0, C0, l[1])), mo_inverse(v));
            REQUIRE(o.a == ApproxV(m.a));
            REQUIRE(o.b == ApproxV(m.b));
            REQUIRE(o.c == ApproxV(m.c));
            REQUIRE(o.d == ApproxV(m.d));
        }
    }

    SECTION("Power") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius m = random_moebius(rng);
            int n = int(floor(8*rng.uniform())) + 2;

            Moebius p = mo_pow(m, 1.0/n);
            Moebius o = mo_identity();

            for(int i = 0; i < n; ++i) {
                o = mo_chain(o, p);
            }

            REQUIRE(o.a == ApproxV(m.a));
            REQUIRE(o.b == ApproxV(m.b));
            REQUIRE(o.c == ApproxV(m.c));
            REQUIRE(o.d == ApproxV(m.d));
        }
    }
};
#endif // UNIT_TEST
