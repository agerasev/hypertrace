#pragma once

#include <common/algebra/vector.hh>
#include <common/algebra/affine.hh>


typedef real3 eu_pos;
typedef real3 eu_dir;
typedef Affine3 EuMap;


eu_pos eu_origin();

real eu_length(eu_pos a);
real eu_distance(eu_pos a, eu_pos b);

eu_pos eu_apply_pos(EuMap m, eu_pos p);
eu_dir eu_apply_dir(EuMap m, eu_pos p, eu_dir d);
#define eu_chain aff3_chain
#define eu_inverse aff3_inverse

// Returns the direction of the line at point `dst_pos`
// when we know that the line at the point `src_pos` has direction of `src_dir`.
eu_dir eu_dir_at(eu_pos src_pos, eu_dir src_dir, eu_pos dst_pos);

EuMap eu_shift(eu_dir pos);
EuMap eu_rotate(eu_dir axis, real phi);

// Turns direction `dir` to *z*.
EuMap eu_look_to(eu_dir dir);

// Rotates point `pos` around the origin to make it lay on the z axis.
EuMap eu_look_at(eu_pos pos);

// Translates point `pos` to the origin preserving orientation
// relative to the line that connects `pos` to the origin.
EuMap eu_move_at(eu_pos pos);
EuMap eu_move_to(eu_dir dir, real dist);
