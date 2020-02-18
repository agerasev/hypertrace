#pragma once

#include <algebra/quaternion.h>


bool horosphere_hit(
    quaternion src_pos, quaternion src_dir,
    quaternion *hit_pos
) {
    quaternion p = src_pos, d = src_dir;
    real dxy = sqrt(d.x*d.x + d.y*d.y);
    // FIXME: check (dxy < EPS)

    if (p.z < dxy) {
        return false;
    }
    
    real dt = sqrt(p.z*p.z - dxy*dxy);
    real t = p.z*d.z - dt;
    if (t < (real)0) {
        t += (real)2*dt;
    }
    if (t < (real)0) {
        return false;
    }

    t /= dxy*dxy;
    quaternion h = q_new(
        p.x + d.x*t,
        p.y + d.y*t,
        (real)1, (real)0
    );

    *hit_pos = h;
    return true;
}
