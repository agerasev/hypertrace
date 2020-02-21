#include "hyperbolic.hh"


bool plane_hit(
    quaternion src_pos, quaternion src_dir,
    quaternion *hit_pos, quaternion *hit_norm
) {
    quaternion p = src_pos, d = src_dir;
    //real dxy = sqrt(d.x*d.x + d.y*d.y);
    // FIXME: check (dxy < EPS)

    real pd = dot(p, d);
    if (fabs(pd) < EPS) {
        return false;
    }
    real t = ((real)1 - q_abs2(p))/((real)2*pd);
    if (t < (real)0) {
        return false;
    }
    quaternion h = q_new(
        p.xy + d.xy*t,
        (real)0, (real)0
    );
    
    real pxy2 = c_abs2(h.xy);
    if (pxy2 > (real)1) {
        return false;
    }
    h.z = sqrt((real)1 - pxy2);

    *hit_pos = h;
    *hit_norm = h;
    return true;
}

bool horosphere_hit(
    quaternion src_pos, quaternion src_dir,
    quaternion *hit_pos, quaternion *hit_norm
) {
    quaternion p = src_pos, d = src_dir;
    real dxy = length(d.xy);
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
        p.xy + d.xy*t,
        (real)1, (real)0
    );

    *hit_pos = h;
    *hit_norm = -QJ;
    return true;
}
