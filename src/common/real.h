#pragma once


#ifdef OPENCL_DEVICE

typedef float real;

#ifdef OPENCL_INTEROP

typedef real real_packed;

#endif // OPENCL_INTEROP

#else // OPENCL_DEVICE

typedef double real;

#ifdef OPENCL_INTEROP

#include <CL/cl.h>

typedef cl_float real_packed;

#endif // OPENCL_INTEROP

#endif // OPENCL_DEVICE


#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#ifdef OPENCL_INTEROP

real_packed r_pack(real a);
real r_unpack(real_packed a);

#endif // OPENCL_INTEROP

#ifdef __cplusplus
};
#endif // __cplusplus



#ifdef UNIT_TEST
#ifdef __cplusplus

#include <random>

#define TEST_EPS 1e-8
#define TEST_PI 3.14159265358979323846
#define TEST_ATTEMPTS 16


class Rng {
private:
    std::minstd_rand rng;
    std::uniform_real_distribution<> unif;
    std::normal_distribution<> norm;

public:
    Rng(uint32_t seed) : rng(seed) {}
    Rng() : Rng(0xdeadbeef) {}

    real uniform() {
        return unif(rng);
    }
    real normal() {
        return norm(rng);
    }
};

#endif // __cplusplus
#endif // UNIT_TEST
