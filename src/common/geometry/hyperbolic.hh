#pragma once

#include <common/algebra/quaternion.hh>
#include <common/algebra/moebius.hh>

#include "euclidean.hh"


typedef quat hy_pos;
typedef quat hy_dir;
typedef Moebius HyMap;


hy_dir hy_origin();

eu_dir hy_dir_to_local(hy_dir pos, hy_dir dir);
hy_dir hy_dir_from_local(hy_dir pos, eu_dir dir);

real hy_length(hy_dir a);
real hy_distance(hy_dir a, hy_dir b);

hy_pos hy_apply_pos(HyMap m, hy_pos p);
hy_dir hy_apply_dir(HyMap m, hy_pos p, hy_dir d);
#define hy_chain mo_chain
#define hy_inverse mo_inverse

// Returns the direction of the line at point `dst_pos`
// when we know that the line at the point `src_pos` has direction of `src_dir`.
hy_dir hy_dir_at(hy_dir src_pos, hy_dir src_dir, hy_dir dst_pos);

HyMap hy_zshift(real l);
HyMap hy_xshift(real l);
HyMap hy_yshift(real l);
HyMap hy_zrotate(real phi);
HyMap hy_xrotate(real theta);
HyMap hy_yrotate(real theta);

// Move to position at the horosphere.
HyMap hy_horosphere(comp pos);

// Turns direction `dir` to *j*.
HyMap hy_look_to(hy_dir dir);

// Rotatates point `pos` around the origin to make it lay on the z axis.
HyMap hy_look_at(hy_dir pos);

// Translates point `pos` to the origin preserving orientation
// relative to the line that connects `pos` to the origin.
HyMap hy_move_at(hy_dir pos);
HyMap hy_move_to(hy_dir dir, real dist);


#ifdef UNIT_TEST

class TestRngHyPos {
private:
    TestRng<real> rng;
    TestRng<real2> rng2;

public:
    inline TestRngHyPos() = default;
    inline explicit TestRngHyPos(uint32_t seed) : rng(seed), rng2(seed) {}
    quat normal();
};

#endif // UNIT_TEST
