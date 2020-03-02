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

void mo_eigen(Moebius m, Moebius *l, Moebius *v) {
    if (c_fabs(m.b) < EPS && c_fabs(m.c) < EPS) {
        *l = mo_new(m.a, C0, C0, m.d);
        *v = mo_new(C1, C0, C0, C1);
    } else {
        complex ad = (m.a + m.d)/2;
        complex D = c_sqrt(c_mul(ad, ad) -
#ifdef MOEBIUS_DENORMALIZED
            mo_det(m)
#else // MOEBIUS_DENORMALIZED
            C1
#endif // MOEBIUS_DENORMALIZED
        );
        if (c_fabs(D) > EPS) {
            *l = mo_new(ad + D, C0, C0, ad - D);
            if (c_fabs(m.c) > EPS) {
                *v = mo_new(l->a - m.d, l->d - m.d, m.c, m.c);
            } else {
                *v = mo_new(m.b, m.b, l->a - m.a, l->d - m.a);
            }
        } else {
            *l = mo_new(ad, C1, C0, ad);
            *v = mo_new(C1, C0, C0, C1);
        }
#ifndef MOEBIUS_DENORMALIZED
        *v = mo_normalize(*v);
#endif // MOEBIUS_DENORMALIZED
    }
}

Moebius mo_pow(Moebius m, real p) {
    Moebius x = mo_sub(m, mo_identity());
    real y = mo_fabs(x);
    // FIXME: Why 1e4?
    if (y*y > 1e4*EPS) {
        Moebius j, v;
        mo_eigen(m, &j, &v);
        Moebius k;
        if (c_fabs(j.b) < EPS) {
            k = mo_new(c_powr(j.a, p), C0, C0, c_powr(j.d, p));
        } else {
            // Assume j.a == j.d and j.b == 1
            k = mo_new(
                c_powr(j.a, p),
                p*c_powr(j.a, p - 1),
                C0,
                c_powr(j.d, p)
            );
        }
        return mo_chain(mo_chain(v, k), mo_inverse(v));
    } else {
        return mo_normalize(mo_add(mo_identity(), mo_mul(x, p)));
    }
}

Moebius mo_add(Moebius a, Moebius b) {
    return mo_new(
        a.a + b.a,
        a.b + b.b,
        a.c + b.c,
        a.d + b.d
    );
}

Moebius mo_sub(Moebius a, Moebius b) {
    return mo_new(
        a.a - b.a,
        a.b - b.b,
        a.c - b.c,
        a.d - b.d
    );
}

Moebius mo_mul(Moebius a, real b) {
    return mo_new(
        a.a*b,
        a.b*b,
        a.c*b,
        a.d*b
    );
}

Moebius mo_div(Moebius a, real b) {
    return mo_new(
        a.a/b,
        a.b/b,
        a.c/b,
        a.d/b
    );
}

real mo_fabs(Moebius m) {
    real d = R0;
    d += c_fabs(m.a);
    d += c_fabs(m.b);
    d += c_fabs(m.c);
    d += c_fabs(m.d);
    return d;
}

real mo_diff(Moebius a, Moebius b) {
    return mo_fabs(mo_sub(a, b));
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

class MoebiusApprox {
    public:
    Moebius m;
    MoebiusApprox(Moebius c) : m(c) {}
    friend bool operator==(Moebius a, MoebiusApprox b) {
        return (
            a.a == ApproxV(b.m.a) &&
            a.b == ApproxV(b.m.b) &&
            a.c == ApproxV(b.m.c) &&
            a.d == ApproxV(b.m.d)
        );
    }
    friend bool operator==(MoebiusApprox a, Moebius b){
        return b == a;
    }
    friend std::ostream &operator<<(std::ostream &s, MoebiusApprox a) {
        return s << a.m;
    }
};
MoebiusApprox ApproxMo(Moebius m) {
    return MoebiusApprox(m);
}

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
            Moebius l, v;
            mo_eigen(m, &l, &v);

            REQUIRE(c_mul(m.a, v.a) + c_mul(m.b, v.c) == ApproxV(c_mul(l.a, v.a)));
            REQUIRE(c_mul(m.c, v.a) + c_mul(m.d, v.c) == ApproxV(c_mul(l.a, v.c)));
            REQUIRE(c_mul(m.a, v.b) + c_mul(m.b, v.d) == ApproxV(c_mul(l.d, v.b)));
            REQUIRE(c_mul(m.c, v.b) + c_mul(m.d, v.d) == ApproxV(c_mul(l.d, v.d)));

            Moebius o = mo_chain(mo_chain(v, l), mo_inverse(v));
            REQUIRE(o == ApproxMo(m));
        }
    }

    SECTION("Non-diagonalizable matrix") {
        Moebius m = mo_new(C1, C1, C0, C1);
        Moebius l, v;
        mo_eigen(m, &l, &v);

        Moebius o = mo_chain(mo_chain(v, l), mo_inverse(v));
        REQUIRE(o == ApproxMo(m));
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

            REQUIRE(o == ApproxMo(m));
        }
    }

    SECTION("Non-diagonalizable power") {
        Moebius m = mo_new(C1, C1, C0, C1);

        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            int p = (int)floor(8*rng.uniform()) + 2;
            int q = (int)floor(8*rng.uniform()) + 2;

            Moebius n = mo_identity();
            for (int i = 0; i < p; ++i) {
                n = mo_chain(n, m);
            }
            Moebius l = mo_pow(m, real(p)/q);
            Moebius o = mo_identity();
            for (int i = 0; i < q; ++i) {
                o = mo_chain(o, l);
            }

            REQUIRE(o == ApproxMo(n));
        }
    }

    SECTION("Power of almost identity transformation") {
        Moebius m = mo_new(
            c_new(1, 8.2331e-07),
            c_new(4.75378e-06, -7.47005e-07),
            c_new(9.3315e-08, -2.74772e-08),
            c_new(1, 9.82438e-07)
        );

        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            int n = int(floor(8*rng.uniform())) + 2;

            Moebius p = mo_pow(m, 1.0/n);
            Moebius o = mo_identity();
            for(int i = 0; i < n; ++i) {
                o = mo_chain(o, p);
            }

            REQUIRE(o == ApproxMo(m));
        }
    }
};
#endif // UNIT_TEST
