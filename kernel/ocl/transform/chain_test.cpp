#include "scale.hh"
#include "shift.hh"

#define $Self ShiftScale3
#define $self ss3
#define $Outer Shift3
#define $outer shf3
#define $Inner Scale3
#define $inner scl3
#define $elem real3
#include "chain.inl"
#undef $Self
#undef $self
#undef $Outer
#undef $outer
#undef $Inner
#undef $inner
#undef $elem


#ifdef UNITTEST

#include <gtest/gtest.h>

class ChainTest : public testing::Test {
protected:
    TestRng<real> rng = TestRng<real>(0xABBA);
    TestRng<real3> vrng = TestRng<real3>(0xBAAB);
};

TEST_F(ChainTest, chaining) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        auto a = ss3_new(shf3_new(vrng.normal()), scl3_new(0.5f + rng.uniform()));
        auto b = ss3_new(shf3_new(vrng.normal()), scl3_new(0.5f + rng.uniform()));
        real3 c = vrng.normal();

        ASSERT_EQ(ss3_chain(a, ss3_identity()), approx(a));
        ASSERT_EQ(ss3_chain(ss3_identity(), b), approx(b));
        ASSERT_EQ(ss3_apply_pos(ss3_chain(a, b), c), approx(ss3_apply_pos(a, ss3_apply_pos(b, c))));
    }
}
TEST_F(ChainTest, inversion) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        auto a = ss3_new(shf3_new(vrng.normal()), scl3_new(0.5f + rng.uniform()));
        real3 x = vrng.normal();

        ASSERT_EQ(ss3_chain(a, ss3_inverse(a)), approx(ss3_identity()));
        ASSERT_EQ(ss3_chain(ss3_inverse(a), a), approx(ss3_identity()));
        ASSERT_EQ(ss3_apply_pos(ss3_inverse(a), ss3_apply_pos(a, x)), approx(x));
        ASSERT_EQ(ss3_apply_pos(a, ss3_apply_pos(ss3_inverse(a), x)), approx(x));
    }
}

#endif // UNITTEST
