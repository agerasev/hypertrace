#include "vector.hh"

#ifdef TEST

#include <gtest/gtest.h>

#ifdef TEST_UNIT

TEST(VectorTest, approx) {
    EXPECT_EQ(
        MAKE(real2)(R1 + EPS/2, R0 - EPS/2),
        approx(MAKE(real2)(R1, R0))
    );
}
TEST(VectorTest, contruction) {
    auto a = MAKE(int3)(-1);
    for (int i = 0; i < a.SIZE; ++i) {
        EXPECT_EQ(a[i], -1);
    }
    auto b = MAKE(int4)(0, 1, 2, 3);
    for (int i = 0; i < b.SIZE; ++i) {
        EXPECT_EQ(b[i], i);
    }
    auto c = int4(int3(0, int2(1, 2)), 3);
    for (int i = 0; i < c.SIZE; ++i) {
        EXPECT_EQ(c[i], i);
    }
}
TEST(VectorTest, field_alignment) {
    vec<int, 2> a2;
    a2[0] = 123456;
    EXPECT_EQ(a2.x, 123456);
    a2.y = 654321;
    EXPECT_EQ(a2[1], 654321);

    vec<int, 4> a4;
    a4.yz = vec<int, 2>(1, 2);
    EXPECT_EQ(a4[1], 1);
    EXPECT_EQ(a4[2], 2);

    vec<int, 8> a8(0,1,2,3,4,5,6,7);
    EXPECT_EQ(a8.s0123, (vec<int, 4>(0,1,2,3)));
    EXPECT_EQ(a8.s4567, (vec<int, 4>(4,5,6,7)));

    vec<int, 16> a16(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15);
    EXPECT_EQ(a16.s01234567, (vec<int, 8>(0,1,2,3,4,5,6,7)));
    EXPECT_EQ(a16.s89abcdef, (vec<int, 8>(8,9,10,11,12,13,14,15)));
    EXPECT_EQ(a16.s0123, (vec<int, 4>(0,1,2,3)));
    EXPECT_EQ(a16.s4567, (vec<int, 4>(4,5,6,7)));
    EXPECT_EQ(a16.s89ab, (vec<int, 4>(8,9,10,11)));
    EXPECT_EQ(a16.scdef, (vec<int, 4>(12,13,14,15)));
}
TEST(VectorTest, compare) {
    EXPECT_EQ(real4(1, 2, 3, 4), approx(real4(1, 2, 3, 4)));
}
TEST(VectorTest, norm) {
    EXPECT_EQ(length(real2(3, -4)), approx(5));
}
TEST(VectorTest, abs) {
    EXPECT_EQ(fabs(real4(1, -2, 3, -4)), approx(real4(1, 2, 3, 4)));
}
TEST(VectorTest, fract) {
    real4 i;
    real4 p = fract(real4(1.1, -1.8, 3.3, -3.6), &i);
    EXPECT_EQ(p, approx(real4(0.1, 0.2, 0.3, 0.4)));
    EXPECT_EQ(i, approx(real4(1, -2, 3, -4)));
}

#endif // TEST_UNIT

#ifdef TEST_DEV

#include <vector>

#include <test/devtest.hpp>

extern devtest::Target devtest_make_target();

rtest_module_(vector) {
    rtest_(mul_dot) {
        TestRng<real3> vrng(0xdead);

        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("vector.cl", std::string(
            "#include <algebra/vector.hh>\n"
            "__kernel void mul_dot(__global const real3 *x, __global const real3 *y, __global real3 *m, __global real *d) {\n"
            "    int i = get_global_id(0);\n"
            "    m[i] = x[i]*y[i];\n"
            "    d[i] = dot(x[i], y[i]);\n"
            "}\n"
        ))
        .build("mul_dot").expect("Kernel build error");

        const int n = TEST_ATTEMPTS;
        std::vector<real3> xbuf(n), ybuf(n), mbuf(n);
        std::vector<real> dbuf(n);
        for (size_t i = 0; i < n; ++i) {
            xbuf[i] = vrng.normal();
            ybuf[i] = vrng.normal();
        }

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, xbuf, ybuf, mbuf, dbuf).expect("Kernel run error");

        for(size_t i = 0; i < n; ++i) {
            real3 m = xbuf[i]*ybuf[i];
            assert_eq_(dev_approx(m), mbuf[i]);

            real d = dot(xbuf[i], ybuf[i]);
            assert_eq_(dev_approx(d), dbuf[i]);
        }
    }
}

#endif // TEST_DEV

#endif // TEST
