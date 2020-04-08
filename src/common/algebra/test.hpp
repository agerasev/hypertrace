#pragma once

#include <random>

#include "real.hpp"


#define TEST_ATTEMPTS 16

class TestRng {
private:
    std::minstd_rand rng;
    std::uniform_real_distribution<> unif;
    std::normal_distribution<> norm;

public:
    TestRng(uint32_t seed) : rng(seed) {}
    TestRng() : TestRng(0xdeadbeef) {}

    real uniform() {
        return unif(rng);
    }
    real normal() {
        return norm(rng);
    }
};

template <typename T>
Approx approx(T x) {
    return Approx(x);
}

#define COMMA ,

#define TEST_DEFINE_CMP_OPS(Template, Self, V) \
Template \
bool operator==(V a, Self b) { \
    return b == a; \
} \
Template \
bool operator!=(Self a, V b) { \
    return !(a == b); \
} \
Template \
bool operator!=(V a, Self b) { \
    return b != a; \
} \

