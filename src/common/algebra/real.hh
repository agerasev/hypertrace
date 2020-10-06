#pragma once

#include <common/types.hh>

#if defined(HOST) || defined(DEV_F64)

typedef double real;

#define PI 3.14159265358979323846
#define EPS 1e-8
#define R0 0.0
#define R1 1.0

#else // !(HOST || DEV_F64)

typedef float real;

#define PI 3.14159265358979323846f
#define EPS 1e-6f
#define R0 0.0f
#define R1 1.0f

#endif // HOST || DEV_F64


#ifdef TEST_UNIT

#include <catch.hpp>

#include <random>

template <typename T>
class TestRng;

template <>
class TestRng<real> {
private:
    std::minstd_rand rng;
    std::uniform_real_distribution<> unif;
    std::normal_distribution<> norm;

public:
    inline explicit TestRng(uint32_t seed=0xdeadbeef) : rng(seed) {}

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

#endif // TEST_UNIT

#ifdef TEST_DEV

#include <host/opencl/opencl.hpp>

void test_dev_real(
    cl_device_id device,
    rstd::Rc<cl::Queue> queue
);

#endif // TEST_DEV
