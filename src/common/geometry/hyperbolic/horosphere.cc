#include "horosphere.hh"


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

/*
real2 k, f;
real2 g = info->local_pos.xy;
real2 bx = make_real2(2/sqrt(3.0f), (real)0);
real2 by = make_real2(-1/sqrt(3.0f), (real)1);
real2 h = 4*c_new(dot(bx, g), dot(by, g));
int hx = (int)floor((floor(h.x) - floor(h.y))/3);
int hy = (int)floor((floor(h.x + h.y) - hx)/2);
int hs = hx - hy;

h -= hx*make_real2(2.0f, -1.0f) + hy*make_real2(1.0f, 1.0f);
real br = 0.05f;
if (fabs(h.x - 1) > 1 - br || fabs(h.y) > 1 - br || fabs(h.x + h.y - 1.0f) > 1 - br) {
    color = obj_color*make_float3(0.0f);
} else {
    color = obj_color*(0.2f + 0.8f*make_float3(hs & 1, (hs>>1) & 1, (hs>>2) & 1));
}
*/
