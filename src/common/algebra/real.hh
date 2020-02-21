#pragma once

#include <types.hh>


#ifdef OPENCL

typedef float  real;
typedef float2 real2;
typedef float3 real3;
typedef float4 real4;
#define make_real2 make_float2
#define make_real3 make_float3
#define make_real4 make_float4

#ifdef OPENCL_INTEROP
typedef real  real_pk;
typedef real2 real2_pk;
typedef real3 real3_pk;
typedef real4 real4_pk;
#define pack_real2 pack_float2
#define pack_real3 pack_float3
#define pack_real4 pack_float4
#define unpack_real2 unpack_float2
#define unpack_real3 unpack_float3
#define unpack_real4 unpack_float4
#endif // OPENCL_INTEROP

#define PI 3.14159265358979323846f
#define EPS 1e-6f

#define R0 0.0f
#define R1 1.0f

#else // OPENCL

typedef double  real;
typedef double2 real2;
typedef double3 real3;
typedef double4 real4;
#define make_real2 make_double2
#define make_real3 make_double3
#define make_real4 make_double4


#ifdef OPENCL_INTEROP
typedef float_pk  real_pk;
typedef float2_pk real2_pk;
typedef float3_pk real3_pk;
typedef float4_pk real4_pk;
#define pack_real2 pack_double2
#define pack_real3 pack_double3
#define pack_real4 pack_double4
#define unpack_real2 unpack_double2
#define unpack_real3 unpack_double3
#define unpack_real4 unpack_double4
#endif // OPENCL_INTEROP

#define PI 3.14159265358979323846
#define EPS 1e-8
#define R0 0.0
#define R1 1.0

#endif // OPENCL


#ifdef UNIT_TEST
#ifdef __cplusplus

#include <random>

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
