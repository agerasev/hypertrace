#include "vector.hh"

#ifdef UNITTEST

#include <gtest/gtest.h>

TEST(VectorTest, approx) {
    ASSERT_EQ(
        make(real2)(R1 + EPS/2, R0 - EPS/2),
        approx(make(real2)(R1, R0))
    );
}
TEST(VectorTest, contruction) {
    auto a = make(int3)(-1);
    for (int i = 0; i < a.SIZE; ++i) {
        ASSERT_EQ(a[i], -1);
    }
    auto b = make(int4)(0, 1, 2, 3);
    for (int i = 0; i < b.SIZE; ++i) {
        ASSERT_EQ(b[i], i);
    }
    auto c = int4(int3(0, int2(1, 2)), 3);
    for (int i = 0; i < c.SIZE; ++i) {
        ASSERT_EQ(c[i], i);
    }
}
TEST(VectorTest, field_alignment) {
    vec<int, 2> a2;
    a2[0] = 123456;
    ASSERT_EQ(a2.x, 123456);
    a2.y = 654321;
    ASSERT_EQ(a2[1], 654321);

    vec<int, 4> a4;
    a4.yz = vec<int, 2>(1, 2);
    ASSERT_EQ(a4[1], 1);
    ASSERT_EQ(a4[2], 2);

    vec<int, 8> a8(0,1,2,3,4,5,6,7);
    ASSERT_EQ(a8.s0123, (vec<int, 4>(0,1,2,3)));
    ASSERT_EQ(a8.s4567, (vec<int, 4>(4,5,6,7)));

    vec<int, 16> a16(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15);
    ASSERT_EQ(a16.s01234567, (vec<int, 8>(0,1,2,3,4,5,6,7)));
    ASSERT_EQ(a16.s89abcdef, (vec<int, 8>(8,9,10,11,12,13,14,15)));
    ASSERT_EQ(a16.s0123, (vec<int, 4>(0,1,2,3)));
    ASSERT_EQ(a16.s4567, (vec<int, 4>(4,5,6,7)));
    ASSERT_EQ(a16.s89ab, (vec<int, 4>(8,9,10,11)));
    ASSERT_EQ(a16.scdef, (vec<int, 4>(12,13,14,15)));
}
TEST(VectorTest, compare) {
    ASSERT_EQ(real4(1, 2, 3, 4), approx(real4(1, 2, 3, 4)));
}
TEST(VectorTest, norm) {
    ASSERT_EQ(length(real2(3, -4)), approx(5));
}
TEST(VectorTest, abs) {
    ASSERT_EQ(fabs(real4(1, -2, 3, -4)), approx(real4(1, 2, 3, 4)));
}
TEST(VectorTest, fract) {
    real4 i;
    real4 p = fract(real4(1.1, -1.8, 3.3, -3.6), &i);
    ASSERT_EQ(p, approx(real4(0.1, 0.2, 0.3, 0.4)));
    ASSERT_EQ(i, approx(real4(1, -2, 3, -4)));
}

#endif // UNITTEST
