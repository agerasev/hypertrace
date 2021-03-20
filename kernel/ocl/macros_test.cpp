#ifdef UNITTEST

#include "macros.hh"

#include <gtest/gtest.h>
#include <string>

#define _STRINGIFY(a) #a
#define STRINGIFY(a) _STRINGIFY(a)

#define $A a

TEST(MacrosTest, concat) {
    ASSERT_EQ(std::string("a"), std::string(STRINGIFY(CONCAT(a))));
    ASSERT_EQ(std::string("ab"), std::string(STRINGIFY(CONCAT(a, b))));
    ASSERT_EQ(std::string("ab"), std::string(STRINGIFY(CONCAT($A, b))));
    ASSERT_EQ(std::string("abc"), std::string(STRINGIFY(CONCAT(a, b, c))));
    ASSERT_EQ(std::string("abc"), std::string(STRINGIFY($($A, b, c))));
}

#endif // UNITTEST
