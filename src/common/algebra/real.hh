#pragma once

#include <common/types.hh>

#if defined(HOST) || defined(DEV_F64)

typedef double real;

#define PI 3.14159265358979323846
#define EPS 1e-8
#define R0 0.0
#define R1 1.0

#else // HOST || DEV_F64

typedef float real;

#define PI 3.14159265358979323846f
#define EPS 1e-6f
#define R0 0.0f
#define R1 1.0f

#endif // HOST || DEV_F64


#ifdef UNIT_TEST

#include <catch.hpp>

#include <random>

class TestRng {
private:
    std::minstd_rand rng;
    std::uniform_real_distribution<> unif;
    std::normal_distribution<> norm;

public:
    inline TestRng(uint32_t seed) : rng(seed) {}
    inline TestRng() : TestRng(0xdeadbeef) {}

    inline real uniform() {
        return unif(rng);
    }
    inline real normal() {
        return norm(rng);
    }
};

inline Approx approx(real v) {
    return Approx(v);
}

#define TEST_ATTEMPTS 16

#endif // UNIT_TEST
