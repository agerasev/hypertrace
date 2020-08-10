#pragma once

#include <common/algebra/quaternion.hh>
#include <common/algebra/moebius.hh>

#include "euclidean.hh"


typedef quat HyPos;
typedef quat HyDir;
typedef Moebius HyMap;


HyDir hy_origin();

EuDir hy_dir_to_local(HyDir pos, HyDir dir);
HyDir hy_dir_from_local(HyDir pos, EuDir ldir);

real hy_length(HyDir a);
real hy_distance(HyDir a, HyDir b);

HyPos hy_apply_pos(HyMap m, HyPos p);
HyDir hy_apply_dir(HyMap m, HyPos p, HyDir d);
#define hy_chain mo_chain
#define hy_inverse mo_inverse

// Returns the direction of the line at point `dst_pos`
// when we know that the line at the point `src_pos` has direction of `src_dir`.
HyDir hy_dir_at(HyDir src_pos, HyDir src_dir, HyDir dst_pos);

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
HyMap hy_look_at(HyDir pos);

// Translates point `pos` to the origin preserving orientation
// relative to the line that connects `pos` to the origin.
HyMap hy_move_at(HyDir pos);
HyMap hy_move_to(HyDir dir, real dist);


#ifdef HOST

#include "geometry.hpp"

class Hyperbolic : public Geometry {
public:
    typedef HyPos Pos;
    typedef HyDir Dir;
    typedef HyMap Map;

    inline static Pos origin() { return  hy_origin(); }
    inline static Eu::Dir dir_to_local(Pos pos, Dir dir) { return hy_dir_to_local(pos, dir); }
    inline static Dir dir_from_local(Pos pos, Eu::Dir ldir) { return hy_dir_from_local(pos, ldir); }

    inline static real length(Pos a) { return hy_length(a); }
    inline static real distance(Pos a, Pos b) { return hy_distance(a, b); }

    inline static Dir dir_at(Pos src_pos, Dir src_dir, Pos dst_pos) { return hy_dir_at(src_pos, src_dir, dst_pos); }

    inline static Map zshift(real l) { return hy_zshift(l); }
    inline static Map xshift(real l) { return hy_xshift(l); }
    inline static Map yshift(real l) { return hy_yshift(l); }
    inline static Map zrotate(real phi) { return hy_zrotate(phi); }
    inline static Map xrotate(real theta) { return hy_xrotate(theta); }
    inline static Map yrotate(real theta) { return hy_yrotate(theta); }

    inline static Map horosphere(comp pos) { return hy_horosphere(pos); }

    inline static Map look_to(Dir dir) { return hy_look_to(dir); }

    inline static Map look_at(Pos pos) { return hy_look_at(pos); }

    inline static Map move_at(Pos pos) { return hy_move_at(pos); }
    inline static Map move_to(Dir dir, real dist) { return hy_move_to(dir, dist); }
};

typedef Hyperbolic Hy;

#endif // HOST

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
