#pragma once

#include <algebra/quaternion.hh>
#include <algebra/moebius.hh>


real hy_distance(quaternion a, quaternion b);

// Returns the direction of the line at point `dst_pos`
// when we know that the line at the point `src_pos` has direction of `src_dir`.
quaternion hy_dir_at(quaternion src_pos, quaternion src_dir, quaternion dst_pos);

Moebius hy_zshift(real l);
Moebius hy_xshift(real l);
Moebius hy_yshift(real l);
Moebius hy_zrotate(real phi);
Moebius hy_xrotate(real theta);
Moebius hy_yrotate(real theta);

// Move to position at the horosphere.
Moebius hy_horosphere(complex pos);

// Turns direction `dir` to *j*.
Moebius hy_look_to(quaternion dir);

// Rotatates point `pos` around the origin to make it lay on the z axis.
Moebius hy_look_at(quaternion pos);

// Translates point `pos` to the origin preserving orientation
// relative to the line that connects `pos` to the origin.
Moebius hy_move_at(quaternion pos);
Moebius hy_move_to(quaternion dir, real dist);
