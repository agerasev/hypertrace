#pragma once

#include <algebra/quaternion.h>


bool plane_hit(
    quaternion src_pos, quaternion src_dir,
    quaternion *hit_pos
) {
    quaternion p = src_pos, d = src_dir;
    real dxy = sqrt(d.x*d.x + d.y*d.y);
    // FIXME: check (dxy < EPS)

    real pd = qq_dot(p, d);
    if (fabs(pd) < EPS) {
        return false;
    }
    real t = ((real)1 - q_abs2(p))/((real)2*pd);
    if (t < (real)0) {
        return false;
    }
    quaternion h = q_new(
        p.x + d.x*t,
        p.y + d.y*t,
        (real)0, (real)0
    );
    
    real pxy2 = h.x*h.x + h.y*h.y;
    if (pxy2 > (real)1) {
        return false;
    }
    h.z = sqrt((real)1 - pxy2);

    *hit_pos = h;
    return true;
}
