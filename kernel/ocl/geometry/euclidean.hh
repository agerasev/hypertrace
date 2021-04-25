#pragma once

#include <algebra/vector.hh>
#include <transform/affine.hh>


typedef real3 EuPos;
typedef real3 EuDir;
typedef Affine3 EuMap;


EuPos eu_origin();

real eu_length(EuPos a);
real eu_distance(EuPos a, EuPos b);

EuPos eu_apply_pos(EuMap m, EuPos p);
EuDir eu_apply_dir(EuMap m, EuPos p, EuDir d);

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

#define eu_chain aff3_chain
#define eu_inverse aff3_inverse

#ifndef HOST
#include "euclidean.cc"
#endif // !HOST
