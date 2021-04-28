#pragma once

#include <algebra/complex.hh>
#include <algebra/quaternion.hh>
#include <algebra/matrix.hh>

typedef struct {
    comp2x2 v;
} Moebius;

comp mo_apply_c(Moebius m, comp p);
quat mo_apply_q(Moebius m, quat p);

comp mo_deriv_c(Moebius m, comp p);
quat mo_deriv_q(Moebius m, quat p, quat v);

#define $Map Moebius
#define $map mo
#define $elem quat
#include "interface.inl"
#undef $Map
#undef $map
#undef $elem

Moebius mo_interpolate(Moebius a, Moebius b, real t);

real mo_distance_l1(Moebius a, Moebius b);

#ifdef HOST
template <>
struct Distance<Moebius> {
    static real distance(Moebius a, Moebius b) {
        return ::distance(a.v, b.v);
    }
};
#endif // HOST

#ifdef UNITTEST

class TestRngMoebius {
private:
    TestRng<comp2x2> rng;
public:
    inline TestRngMoebius() = default;
    inline explicit TestRngMoebius(uint32_t seed) : rng(seed) {}
    Moebius normal();
};

#endif // UNITTEST


#ifndef HOST
#include "moebius.cc"
#endif // !HOST
