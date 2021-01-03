#include "complex.hh"


comp c_conj(comp a) {
    return c_new(a.x, -a.y);
}
real c_abs2(comp a) {
    return dot(a, a);
}
real c_abs(comp a) {
    return length(a);
}
real c_norm_l1(comp a) {
    return fabs(a.x) + fabs(a.y);
}

comp c_mul(comp a, comp b) {
    return c_new(
        a.x*b.x - a.y*b.y,
        a.x*b.y + a.y*b.x
    );
}

comp c_inverse(comp a) {
    return c_conj(a)/c_abs2(a);
}

comp c_div(comp a, comp b) {
    return c_mul(a, c_inverse(b));
}

comp c_exp(comp p) {
    return exp(p.x)*c_new(cos(p.y), sin(p.y));
}
comp c_pow_r(comp a, real p) {
    real r = pow(c_abs2(a), p/2);
    real phi = p*atan2(a.y, a.x);
    return c_new(r*cos(phi), r*sin(phi));
}
comp c_sqrt(comp a) {
    real r = sqrt(c_abs(a));
    real phi = 0.5f*atan2(a.y, a.x);
    return c_new(r*cos(phi), r*sin(phi));
}


#ifdef TEST

#include <gtest/gtest.h>

#ifdef TEST_UNIT

class ComplexTest : public testing::Test {
protected:
    TestRng<real> rng = TestRng<real>(0xbeef);
    TestRng<comp> crng = TestRng<comp>(0xbeef);
};

TEST_F(ComplexTest, constructor) {
    comp a = c_new(R0, R1);
    EXPECT_EQ(a.x, Approx(R0));
    EXPECT_EQ(a.y, Approx(R1));
}
TEST_F(ComplexTest, inversion) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        comp a = crng.nonzero();
        EXPECT_EQ(c_div(a, a), approx(C1));
    }
}
TEST_F(ComplexTest, square_root) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        comp a = crng.normal();
        comp b = c_sqrt(a);
        EXPECT_EQ(c_mul(b, b), approx(a));
    }
}
TEST_F(ComplexTest, power) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        comp a = crng.normal();
        int n = int(floor(2 + 10*rng.uniform()));
        comp b = c_pow_r(a, 1.0/n);
        comp c = C1;
        for (int i = 0; i < n; ++i) {
            c = c_mul(c, b);
        }
        EXPECT_EQ(c, approx(a));
    }
}
TEST_F(ComplexTest, norm) {
    EXPECT_EQ(c_norm_l1(c_new(-1, 2)), approx(3));
    EXPECT_EQ(length(c_new(3, -4)), approx(5));
}

#endif // TEST_UNIT

#ifdef TEST_DEV

#include <rtest.hpp>

#include <vector>

#include <test/devtest.hpp>

extern devtest::Target devtest_make_target();

rtest_module_(complex) {
    rtest_(to_dev_and_back) {
        TestRng<comp> crng(0xbeef);

        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("complex.cl", std::string(
            "#include <algebra/complex.hh>\n"
            "__kernel void identity(__global const comp *ibuf, __global comp *obuf) {\n"
            "    int i = get_global_id(0);\n"
            "    obuf[i] = ibuf[i];\n"
            "}\n"
        ))
        .build("identity").expect("Kernel build error");

        const int n = TEST_ATTEMPTS;
        std::vector<comp> ibuf(n), obuf(n);
        for (size_t i = 0; i < n; ++i) {
            ibuf[i] = crng.normal();
        }

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, ibuf, obuf).expect("Kernel run error");

        for(size_t i = 0; i < obuf.size(); ++i) {
            assert_eq_(dev_approx(ibuf[i]), obuf[i]);
        }
    }
    rtest_(mul_div) {
        TestRng<comp> crng(0xbeef);

        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("complex.cl", std::string(
            "#include <algebra/complex.hh>\n"
            "__kernel void mul_div(__global const comp *x, __global const comp *y, __global comp *m, __global comp *d) {\n"
            "    int i = get_global_id(0);\n"
            "    m[i] = c_mul(x[i], y[i]);\n"
            "    d[i] = c_div(x[i], y[i]);\n"
            "}\n"
        ))
        .build("mul_div").expect("Kernel build error");

        const int n = TEST_ATTEMPTS;
        std::vector<comp> xbuf(n), ybuf(n), mbuf(n), dbuf(n);
        for (size_t i = 0; i < n; ++i) {
            xbuf[i] = crng.normal();
            ybuf[i] = crng.normal();
        }

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, xbuf, ybuf, mbuf, dbuf).expect("Kernel run error");

        for(size_t i = 0; i < n; ++i) {
            comp m = c_mul(xbuf[i], ybuf[i]);
            assert_eq_(dev_approx(m), mbuf[i]);

            comp d = c_div(xbuf[i], ybuf[i]);
            assert_eq_(dev_approx(d), dbuf[i]);
        }
    }
}

#endif // TEST_DEV

#endif // TEST
