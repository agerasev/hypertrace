#pragma once

#include <algebra/vector.hh>
#include <algebra/complex.hh>
#include <algebra/quaternion.hh>
#include "linear.hh"

// Rotation2

typedef struct {
    comp v;
} Rotation2;

Rotation2 rot2_from_angle(real angle);
Rotation2 rot2_look_at(real2 pos);

Linear2 rot2_to_linear(Rotation2 m);

#define $Map Rotation2
#define $map rot2
#define $elem real2
#include "derive.inl"
#undef $Map
#undef $map
#undef $elem

// Rotation3

typedef struct {
    quat v;
} Rotation3;

#define $Map Rotation3
#define $map rot3
#define $elem real3
#include "derive.inl"
#undef $Map
#undef $map
#undef $elem

Rotation3 rot3_from_axis(real3 axis, real angle);

Linear3 rot3_to_linear(Rotation3 m);
//Rotation3 rot3_look_to(real3 m);

// Tests

#ifdef UNITTEST

class TestRngRotation2 {
private:
    TestRng<real> rng;

public:
    inline TestRngRotation2() = default;
    inline explicit TestRngRotation2(uint32_t seed) : rng(seed) {}
    Rotation2 uniform();
};

class TestRngRotation3 {
private:
    TestRng<real> rng;
    TestRng<real3> vrng;

public:
    inline TestRngRotation3() = default;
    inline explicit TestRngRotation3(uint32_t seed) : rng(seed), vrng(seed) {}
    Rotation3 uniform();
};

#endif // UNITTEST

#ifndef HOST
#include "rotation.cc"
#endif // !HOST
