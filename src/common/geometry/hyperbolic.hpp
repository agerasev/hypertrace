#pragma once

#include <algebra/complex.hpp>
#include <algebra/moebius.hpp>

namespace hyperbolic {

real distance(quat a, quat b);

// Returns the direction of the line at point `dst_pos`
// when we know that the line at the point `src_pos` has direction of `src_dir`.
quat dir_at(quat src_pos, quat src_dir, quat dst_pos);

Moebius zshift(real l);
Moebius xshift(real l);
Moebius yshift(real l);
Moebius zrotate(real phi);
Moebius xrotate(real theta);
Moebius yrotate(real theta);

// Move to position at the horosphere.
Moebius horosphere(comp pos);

// Turns direction `dir` to *j*.
Moebius look_to(quat dir);

// Rotatates point `pos` around the origin to make it lay on the z axis.
Moebius look_at(quat pos);

// Translates point `pos` to the origin preserving orientation
// relative to the line that connects `pos` to the origin.
Moebius move_at(quat pos);
Moebius move_to(quat dir, real dist);

} // namespace hyperbolic
