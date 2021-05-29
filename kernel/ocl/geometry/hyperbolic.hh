#pragma once

#include <algebra/quaternion.hh>
#include <transform/moebius.hh>

#include "euclidean.hh"

#define $Geo Hy
#define $geo hy
#define $Map Moebius
#define $map mo
#define $pos quat
#define $dir quat
#include "interface.inl"
#undef $Geo
#undef $geo
#undef $Map
#undef $map
#undef $pos
#undef $dir

// Move to position at the horosphere.
HyMap hy_horosphere(comp pos);

#ifdef UNITTEST

class TestRngHyPos {
private:
    TestRng<real> rng;
    TestRng<real2> rng2;

public:
    inline TestRngHyPos() = default;
    inline explicit TestRngHyPos(uint32_t seed) : rng(seed), rng2(seed) {}
    quat normal();
};

#endif // UNITTEST


#ifndef HOST
#include "hyperbolic.cc"
#endif // !HOST
