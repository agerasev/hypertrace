#include "macros.hh"

#ifdef UNITTEST

#include <gtest/gtest.h>
#include <string>

#define STRINGIFY(a) #a
#define STRINGIFY_EX(a) STRINGIFY(a)

#define ABC_ abc_

TEST(MacrosTest, concat) {
    ASSERT_EQ(std::string("abc_def"), std::string(STRINGIFY_EX(CONCAT(abc_, def))));
    ASSERT_EQ(std::string("abc_def"), std::string(STRINGIFY_EX(CONCAT_EX(ABC_, def))));
}

#endif // UNITTEST
