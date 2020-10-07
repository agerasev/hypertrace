#pragma once

#include "vector.hh"
#include "complex.hh"
#include "quaternion.hh"
#include "linear.hh"


typedef comp Rotation2;
typedef quat Rotation3;


Rotation2 rot2_identity();
Rotation2 rot2_from_angle(real angle);
Rotation2 rot2_look_at(real2 pos);

real2 rot2_apply(Rotation2 m, real2 pos);
Rotation2 rot2_chain(Rotation2 a, Rotation2 b);
Rotation2 rot2_inverse(Rotation2 m);
Linear2 rot2_to_linear(Rotation2 m);


Rotation3 rot3_identity();
Rotation3 rot3_from_axis(real3 axis, real angle);

real3 rot3_apply(Rotation3 m, real3 pos);
Rotation3 rot3_chain(Rotation3 a, Rotation3 b);
Rotation3 rot3_inverse(Rotation3 m);
Linear3 rot3_to_linear(Rotation3 m);

#ifdef TEST_UNIT

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

#endif // TEST_UNIT
