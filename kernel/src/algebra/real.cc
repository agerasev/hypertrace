#include "real.hh"

#ifdef UNITTEST

#include <gtest/gtest.h>

TEST(RealTest, approx) {
    EXPECT_EQ(R1 + EPS/2, approx(R1));
}

#endif // UNITTEST
