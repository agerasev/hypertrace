#pragma once

#include <common/algebra/vector.hh>
#include <common/algebra/affine.hh>


typedef real3 EuPos;
typedef real3 EuDir;
typedef Affine3 EuMap;


EuPos eu_origin();

real eu_length(EuPos a);
real eu_distance(EuPos a, EuPos b);

EuPos eu_apply_pos(EuMap m, EuPos p);
EuDir eu_apply_dir(EuMap m, EuPos p, EuDir d);
#define eu_chain aff3_chain
#define eu_inverse aff3_inverse

// Returns the direction of the line at point `dst_pos`
// when we know that the line at the point `src_pos` has direction of `src_dir`.
EuDir eu_dir_at(EuPos src_pos, EuDir src_dir, EuPos dst_pos);

EuMap eu_shift(EuDir pos);
EuMap eu_rotate(EuDir axis, real phi);

// Turns direction `dir` to *z*.
EuMap eu_look_to(EuDir dir);

// Rotates point `pos` around the origin to make it lay on the z axis.
EuMap eu_look_at(EuPos pos);

// Translates point `pos` to the origin preserving orientation
// relative to the line that connects `pos` to the origin.
EuMap eu_move_at(EuPos pos);
EuMap eu_move_to(EuDir dir, real dist);


#ifdef HOST

#include "geometry.hpp"

class Euclidean : public Geometry {
public:
    typedef EuPos Pos;
    typedef EuDir Dir;
    typedef EuMap Map;

    inline static Pos origin() { return  eu_origin(); }
    inline static Dir dir_to_local(Pos, Dir dir) { return dir; }
    inline static Dir dir_from_local(Pos, Dir dir) { return dir; }

    inline static real length(Pos a) { return eu_length(a); }
    inline static real distance(Pos a, Pos b) { return eu_distance(a, b); }

    inline static Dir dir_at(Pos src_pos, Dir src_dir, Pos dst_pos) { return eu_dir_at(src_pos, src_dir, dst_pos); }

    inline static Map shift(Pos pos) { return eu_shift(pos); }
    inline static Map xshift(real l) { return shift(Dir(l,0,0)); }
    inline static Map yshift(real l) { return shift(Dir(0,l,0)); }
    inline static Map zshift(real l) { return shift(Dir(0,0,l)); }

    inline static Map rotate(Dir axis, real phi) { return eu_rotate(axis, phi); }
    inline static Map xrotate(real angle) { return rotate(Dir(1,0,0), angle); }
    inline static Map yrotate(real angle) { return rotate(Dir(0,1,0), angle); }
    inline static Map zrotate(real angle) { return rotate(Dir(0,0,1), angle); }

    inline static Map look_to(Dir dir) { return eu_look_to(dir); }

    inline static Map look_at(Pos pos) { return eu_look_at(pos); }

    inline static Map move_at(Pos pos) { return eu_move_at(pos); }
    inline static Map move_to(Dir dir, real dist) { return eu_move_to(dir, dist); }
};

typedef Euclidean Eu;

#endif // HOST
