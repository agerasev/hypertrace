#include "moebius.hh"

Moebius mo_new(comp2x2 v) {
    Moebius m;
    m.v = v;
    return m;
}

Moebius mo_identity() {
    return mo_new(c22_one());
}

comp mo_apply_c(Moebius m, comp p) {
    return c_div(
        c_mul(m.v.s01, p) + m.v.s23,
        c_mul(m.v.s45, p) + m.v.s67
    );
}

quat mo_apply_q(Moebius m, quat p) {
    return q_div(
        cq_mul(m.v.s01, p) + q_new(m.v.s23, C0),
        cq_mul(m.v.s45, p) + q_new(m.v.s67, C0)
    );
}

comp mo_deriv_c(Moebius m, comp p) {
    comp u = c_mul(m.v.s01, p) + m.v.s23;
    comp d = c_mul(m.v.s45, p) + m.v.s67;
    return c_div(c_mul(m.v.s01, d) - c_mul(u, m.v.s45), c_mul(d, d));
}

quat mo_deriv_q(Moebius m, quat p, quat v) {
    quat u = cq_mul(m.v.s01, p) + q_new(m.v.s23, C0);
    quat d = cq_mul(m.v.s45, p) + q_new(m.v.s67, C0);
    real d2 = q_abs2(d);
    quat g1 = q_div(cq_mul(m.v.s01, v), d);
    quat g21 = q_conj(cq_mul(m.v.s45, v));
    quat g22 = (2*dot(d, cq_mul(m.v.s45, v))/d2)*q_conj(d);
    quat g2 = q_mul(u, (g21 - g22)/d2);
    return g1 + g2;
}

quat mo_apply_pos(Moebius m, quat p) {
    return mo_apply_q(m, p);
}
quat mo_apply_dir(Moebius m, quat p, quat v) {
    return normalize(mo_deriv_q(m, p, v));
}
quat mo_apply_normal(Moebius m, quat p, quat v) {
    return mo_apply_dir(m, p, v);
}

Moebius mo_inverse(Moebius m) {
    return mo_new(c22_inverse_n(m.v));
}

Moebius mo_chain(Moebius a, Moebius b) {
    return mo_new(c22_dot(a.v, b.v));
}

Moebius mo_interpolate(Moebius a, Moebius b, real t) {
    return mo_chain(a, mo_new(c22_pow(mo_chain(mo_inverse(a), b).v, t)));
}

real mo_distance_l1(Moebius a, Moebius b) {
    return c22_norm_l1(a.v - b.v);
}

Moebius mo_pow(Moebius m, real p) {
    return mo_new(c22_pow_n(m.v, p));
}

#ifdef HOST

std::ostream &operator<<(std::ostream &o, Moebius m) {
    return o << "Moebius { " << m.v << " }";
}

#endif // HOST

#ifdef UNITTEST

Moebius TestRngMoebius::normal() {
    return mo_new(c22_normalize(rng.normal()));
}

#include <gtest/gtest.h>

class MoebiusTest : public testing::Test {
protected:
    TestRng<comp> crng = TestRng<comp>();
    TestRng<quat> qrng = TestRng<quat>();
    TestRngMoebius trng = TestRngMoebius();
};

TEST_F(MoebiusTest, chaining) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        Moebius a = trng.normal(), b = trng.normal();
        quat c = qrng.normal();
        ASSERT_EQ(
            mo_apply_q(mo_chain(a, b), c), 
            approx(mo_apply_q(a, mo_apply_q(b, c)))
        );
    }
}

TEST_F(MoebiusTest, complex_derivation) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        Moebius a = trng.normal();
        comp p = crng.normal();
        comp v = crng.unit();
        
        const real DEPS = sqrt(EPS);
        comp deriv = mo_deriv_c(a, p);
        real dabs = 4*c_abs(deriv);
        ASSERT_EQ(
            c_div(mo_apply_c(a, p + DEPS*v) - mo_apply_c(a, p), DEPS*v),
            approx(deriv).epsilon(sqrt(DEPS)*dabs)
        );
    }
}
TEST_F(MoebiusTest, quaternion_directional_derivation) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        Moebius a = trng.normal();
        quat p = qrng.normal();
        quat v = qrng.unit();
        
        const real DEPS = sqrt(EPS);
        quat deriv = mo_deriv_q(a, p, v);
        real dabs = q_abs(deriv);
        ASSERT_EQ(
            (mo_apply_q(a, p + DEPS*v) - mo_apply_q(a, p))/DEPS,
            approx(deriv).epsilon(sqrt(DEPS)*dabs)
        );
    }
}

#endif // UNITTEST
