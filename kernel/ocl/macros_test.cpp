#ifdef UNITTEST

#include "macros.hh"

#include <gtest/gtest.h>
#include <string>

#define $A a

TEST(MacrosTest, concat) {
    ASSERT_EQ(std::string("a"), std::string(STRINGIFY(CONCAT_1(a))));
    ASSERT_EQ(std::string("ab"), std::string(STRINGIFY(CONCAT_2(a, b))));
    ASSERT_EQ(std::string("ab"), std::string(STRINGIFY(CONCAT_2($A, b))));
    ASSERT_EQ(std::string("abc"), std::string(STRINGIFY(CONCAT_3(a, b, c))));
    ASSERT_EQ(std::string("abc"), std::string(STRINGIFY($3($A, b, c))));
}

#ifdef VARIADIC_MACROS
TEST(MacrosTest, concat_variadic) {
    ASSERT_EQ(std::string("a"), std::string(STRINGIFY(CONCAT(a))));
    ASSERT_EQ(std::string("ab"), std::string(STRINGIFY(CONCAT(a, b))));
    ASSERT_EQ(std::string("ab"), std::string(STRINGIFY(CONCAT($A, b))));
    ASSERT_EQ(std::string("abc"), std::string(STRINGIFY(CONCAT(a, b, c))));
    ASSERT_EQ(std::string("abc"), std::string(STRINGIFY($($A, b, c))));
}
#endif

#endif // UNITTEST
