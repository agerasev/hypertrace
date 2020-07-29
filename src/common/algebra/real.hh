#pragma once

#include <common/types.hh>

#ifdef HOST

typedef double real;

#ifdef INTEROP
typedef double_dev   real_dev;
#define real_load    double_load
#define real_store   double_store
#endif // INTEROP

#define PI 3.14159265358979323846
#define EPS 1e-8
#define R0 0.0
#define R1 1.0

#else // HOST

#ifdef DEVICE_DOUBLE

typedef double real;

#define PI 3.14159265358979323846
#define EPS 1e-8
#define R0 0.0
#define R1 1.0

#else // DEVICE_DOUBLE

typedef float real;

#define PI 3.14159265358979323846f
#define EPS 1e-6f
#define R0 0.0f
#define R1 1.0f

#endif // DEVICE_DOUBLE

#endif // HOST


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

Approx approx(real v) {
    return Approx(v);
}

#endif // UNIT_TEST
