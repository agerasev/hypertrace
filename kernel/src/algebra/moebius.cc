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

#ifdef TEST

Moebius TestRngMoebius::normal() {
    return c22_normalize(rng.normal());
}

#include <gtest/gtest.h>

#ifdef TEST_UNIT

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
        EXPECT_EQ(
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
        real dabs = c_abs(deriv);
        EXPECT_EQ(
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
        EXPECT_EQ(
            (mo_apply_q(a, p + DEPS*v) - mo_apply_q(a, p))/DEPS,
            approx(deriv).epsilon(sqrt(DEPS)*dabs)
        );
    }
}

#endif // TEST_UNIT

#ifdef TEST_DEV

#include <rtest.hpp>

#include <vector>

#include <test/devtest.hpp>

extern devtest::Target devtest_make_target();

rtest_module_(moebius) {
    rtest_(chain) {
        TestRngMoebius morng(0xdead);

        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("moebius.cl", std::string(
            "#include <algebra/moebius.hh>\n"
            "__kernel void chain(__global const Moebius *x, __global const Moebius *y, __global Moebius *z) {\n"
            "    int i = get_global_id(0);\n"
            "    z[i] = mo_chain(x[i], y[i]);\n"
            "}\n"
        ))
        .build("chain").expect("Kernel build error");

        const int n = TEST_ATTEMPTS;
        std::vector<Moebius> xbuf(n), ybuf(n), zbuf(n);
        for (size_t i = 0; i < n; ++i) {
            xbuf[i] = morng.normal();
            ybuf[i] = morng.normal();
        }

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, xbuf, ybuf, zbuf).expect("Kernel run error");

        for(size_t i = 0; i < n; ++i) {
            Moebius z = mo_chain(xbuf[i], ybuf[i]);
            assert_eq_(dev_approx(z), zbuf[i]);
        }
    }
}

#endif // TEST_DEV

#endif // TEST
