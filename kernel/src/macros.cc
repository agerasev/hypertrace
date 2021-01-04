#include "macros.hh"

#ifdef UNITTEST

#include <gtest/gtest.h>
#include <string>

#define _STRINGIFY(a) #a
#define STRINGIFY(a) _STRINGIFY(a)

TEST(MacrosTest, va_count) {
    ASSERT_EQ(1, VA_COUNT(A));
    ASSERT_EQ(2, VA_COUNT(A,B));
    ASSERT_EQ(3, VA_COUNT(A,B,C));
    ASSERT_EQ(4, VA_COUNT(A,B,C,D));
    ASSERT_EQ(5, VA_COUNT(A,B,C,D,E));
    ASSERT_EQ(6, VA_COUNT(A,B,C,D,E,F));
    ASSERT_EQ(7, VA_COUNT(A,B,C,D,E,F,G));
    ASSERT_EQ(8, VA_COUNT(A,B,C,D,E,F,G,H));
}

#define A a

TEST(MacrosTest, concat) {
    ASSERT_EQ(std::string("ab"), std::string(STRINGIFY(_CONCAT_2(a, b))));
    ASSERT_EQ(std::string("Ab"), std::string(STRINGIFY(_CONCAT_2(A, b))));
    ASSERT_EQ(std::string("ab"), std::string(STRINGIFY(CONCAT_2(A, b))));
    ASSERT_EQ(std::string("abc"), std::string(STRINGIFY(CONCAT_3(a, b, c))));

    //ASSERT_EQ(std::string("ab"), std::string(STRINGIFY(CONCAT(a, b))));
}

#define C2(x, y) x##y
#define C3(x, y, z) x##y##z

#define GET_MACRO(_1,_2,_3,NAME,...) NAME
#define C(...) GET_MACRO(__VA_ARGS__, C3, C2)(__VA_ARGS__)

#define X a

TEST(MacrosTest, test) {
    ASSERT_EQ(std::string("abc"), std::string(STRINGIFY(C(X, b, c))));
}

#endif // UNITTEST
