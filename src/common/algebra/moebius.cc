#include "moebius.hh"


comp mo_apply_c(Moebius m, comp p) {
    return c_div(
        c_mul(m.s01, p) + m.s23,
        c_mul(m.s45, p) + m.s67
    );
}

quat mo_apply_q(Moebius m, quat p) {
    return q_div(
        cq_mul(m.s01, p) + q_new(m.s23, C0),
        cq_mul(m.s45, p) + q_new(m.s67, C0)
    );
}

comp mo_deriv_c(Moebius m, comp p) {
    comp u = c_mul(m.s01, p) + m.s23;
    comp d = c_mul(m.s45, p) + m.s67;
    return c_div(c_mul(m.s01, d) - c_mul(u, m.s45), c_mul(d, d));
}

quat mo_deriv_q(Moebius m, quat p, quat v) {
    quat u = cq_mul(m.s01, p) + q_new(m.s23, C0);
    quat d = cq_mul(m.s45, p) + q_new(m.s67, C0);
    real d2 = q_abs2(d);
    quat g1 = q_div(cq_mul(m.s01, v), d);
    quat g21 = q_conj(cq_mul(m.s45, v));
    quat g22 = (2*dot(d, cq_mul(m.s45, v))/d2)*q_conj(d);
    quat g2 = q_mul(u, (g21 - g22)/d2);
    return g1 + g2;
}

Moebius mo_interpolate(Moebius a, Moebius b, real t) {
    return mo_chain(a, mo_pow(mo_chain(mo_inverse(a), b), t));
}

real mo_diff(Moebius a, Moebius b) {
    return mo_norm_l1(a - b);
}

#ifdef UNIT_TEST

#include <catch.hpp>

class TestRngMoebius {
private:
    TestRng<Moebius> rng;
public:
    inline TestRngMoebius() = default;
    inline explicit TestRngMoebius(uint32_t seed) : rng(seed) {}

    Moebius random() {
        return mo_normalize(rng.normal());
    }
};

TEST_CASE("Moebius transformation", "[moebius]") {
    TestRng<comp> crng;
    TestRng<quat> qrng;
    TestRngMoebius trng;

    SECTION("Chaining") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius a = trng.random(), b = trng.random();
            quat c = qrng.normal();
            REQUIRE(
                mo_apply_q(mo_chain(a, b), c) == 
                approx(mo_apply_q(a, mo_apply_q(b, c)))
            );
        }
    }

    SECTION("Complex derivation") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius a = trng.random();
            comp p = crng.normal();
            comp v = crng.nonzero();
            
            REQUIRE(
                approx(mo_deriv_c(a, p)) ==
                c_div(mo_apply_c(a, p + EPS*v) - mo_apply_c(a, p), EPS*v)
            );
        }
    }
    SECTION("Quaternion directional derivation") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius a = trng.random();
            quat p = qrng.normal();
            quat v = qrng.nonzero();
            
            REQUIRE(
                approx(mo_deriv_q(a, p, v)) ==
                (mo_apply_q(a, p + EPS*v) - mo_apply_q(a, p))/EPS
            );
        }
    }
};

#endif // UNIT_TEST
