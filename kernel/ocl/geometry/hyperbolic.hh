#pragma once

#include <algebra/quaternion.hh>
#include <transform/moebius.hh>

#include "euclidean.hh"


typedef quat HyPos;
typedef quat HyDir;
typedef Moebius HyMap;


HyDir hy_origin();

EuDir hy_dir_to_local(HyPos pos, HyDir dir);
HyDir hy_dir_from_local(HyPos pos, EuDir ldir);

real hy_length(HyPos a);
real hy_distance(HyPos a, HyPos b);

HyPos hy_apply_pos(HyMap m, HyPos p);
HyDir hy_apply_dir(HyMap m, HyPos p, HyDir d);
#define hy_chain mo_chain
#define hy_inverse mo_inverse

// Returns the direction of the line at point `dst_pos`
// when we know that the line at the point `src_pos` has direction of `src_dir`.
HyDir hy_dir_at(HyPos src_pos, HyDir src_dir, HyPos dst_pos);

HyMap hy_zshift(real l);
HyMap hy_xshift(real l);
HyMap hy_yshift(real l);
HyMap hy_zrotate(real phi);
HyMap hy_xrotate(real theta);
HyMap hy_yrotate(real theta);

// Move to position at the horosphere.
HyMap hy_horosphere(comp pos);

// Turns direction `dir` to *j*.
HyMap hy_look_to(HyDir dir);

// Rotatates point `pos` around the origin to make it lay on the z axis.
HyMap hy_look_at(HyPos pos);

// Translates point `pos` to the origin preserving orientation
// relative to the line that connects `pos` to the origin.
HyMap hy_move_at(HyPos pos);
HyMap hy_move_to(HyDir dir, real dist);


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
