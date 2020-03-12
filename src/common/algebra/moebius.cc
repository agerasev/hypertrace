#include "moebius.hh"

#ifndef OPENCL
#include <math.h>
#endif // OPENCL


quaternion mo_apply(Moebius m, quaternion p) {
    return q_div(
        cq_mul(m.s[0], p) + q_new(m.s[1], C0),
        cq_mul(m.s[2], p) + q_new(m.s[3], C0)
    );
}

quaternion mo_deriv(Moebius m, quaternion p, quaternion v) {
    quaternion u = cq_mul(m.s[0], p) + q_new(m.s[1], C0);
    quaternion d = cq_mul(m.s[2], p) + q_new(m.s[3], C0);
    real d2 = q_abs2(d);
    quaternion s1 = q_div(cq_mul(m.s[0], v), d);
    quaternion s21 = q_conj(cq_mul(m.s[2], v));
    quaternion s22 = ((real)2*dot(d, cq_mul(m.s[2], v))/d2)*q_conj(d);
    quaternion s2 = q_mul(u, (s21 - s22)/d2);
    return s1 + s2;
}

Moebius mo_interpolate(Moebius a, Moebius b, real t) {
    return mo_chain(a, mo_pow(mo_chain(mo_inverse(a), b), t));
}

real mo_diff(Moebius a, Moebius b) {
    return mo_fabs(mo_sub(a, b));
}


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
            Moebius l, v;
            mo_eigen(m, &l, &v);

            REQUIRE(c_mul(m.s[0], v.s[0]) + c_mul(m.s[1], v.s[2]) == ApproxV(c_mul(l.s[0], v.s[0])));
            REQUIRE(c_mul(m.s[2], v.s[0]) + c_mul(m.s[3], v.s[2]) == ApproxV(c_mul(l.s[0], v.s[2])));
            REQUIRE(c_mul(m.s[0], v.s[1]) + c_mul(m.s[1], v.s[3]) == ApproxV(c_mul(l.s[3], v.s[1])));
            REQUIRE(c_mul(m.s[2], v.s[1]) + c_mul(m.s[3], v.s[3]) == ApproxV(c_mul(l.s[3], v.s[3])));

            Moebius o = mo_chain(mo_chain(v, l), mo_inverse(v));
            REQUIRE(o == ApproxMo(m));
        }
    }

    SECTION("Non-diagonalizable matrix") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius m = mo_new(C1, C0, C0, C1);
            if (rng.uniform() > 0.5) {
                m.s[1] = rand_c_normal(rng);
            } else {
                m.s[2] = rand_c_normal(rng);
            }
            Moebius l, v;
            mo_eigen(m, &l, &v);

            Moebius o = mo_chain(mo_chain(v, l), mo_inverse(v));
            REQUIRE(o == ApproxMo(m));
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

            REQUIRE(o == ApproxMo(m));
        }
    }

    SECTION("Non-diagonalizable power") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius m = mo_new(C1, C0, C0, C1);
            if (rng.uniform() > 0.5) {
                m.s[1] = rand_c_normal(rng);
            } else {
                m.s[2] = rand_c_normal(rng);
            }
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
