#pragma once

#include <algebra/quaternion.h>
#include <algebra/moebius.h>


typedef struct Surface Surface;

#ifdef OPENCL_INTEROP
typedef struct SurfacePacked SurfacePacked;
#endif // OPENCL_INTEROP


#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

real surface_hit(
    Surface *surface,
    Moebius *src,
    quaternion *hit_pos,
    quaternion *hit_norm
);

#ifdef OPENCL_INTEROP
void surface_pack(SurfacePacked *p, const Surface *u);
void surface_unpack(Surface *u, const SurfacePacked *p);
#endif // OPENCL_INTEROP

#ifdef __cplusplus
};
#endif // __cplusplus


// Surface with a constant curvature.
struct Surface {
    quaternion point;
    quaternion normal;
    real curvature;
};


#ifdef OPENCL_INTEROP
struct __attribute__ ((packed)) SurfacePacked {
    quaternion_packed point;
    quaternion_packed normal;
    real_packed curvature;
};

void surface_pack(SurfacePacked *p, const Surface *u) {
    p->point = q_pack(u->point);
    p->normal = q_pack(u->normal);
    p->curvature = r_pack(u->curvature);
}

void surface_unpack(Surface *u, const SurfacePacked *p) {
    u->point = q_unpack(p->point);
    u->normal = q_unpack(p->normal);
    u->curvature = r_unpack(p->curvature);
}
#endif // OPENCL_INTEROP


real surface_hit(
    Surface *surface,
    Moebius *src,
    quaternion *hit_pos,
    quaternion *hit_norm
) {
    quaternion d = q_norm(moebius_deriv(src, surface->point, surface->normal));
	quaternion p = moebius_apply(src, surface->point);
    real mu = surface->curvature;

	real h = (real)0;
    quaternion t, n;
	if (fabs(mu + d.z) > EPS) {
		real r = p.z/(mu + d.z);
		quaternion c = p - r*d;
		real rd = r*r - c.x*c.x - c.y*c.y;
		if (rd > (real)0) {
			rd = sqrt(rd);
			h = c.z - rd;
			if (h < (real)1) {
				h = c.z + rd;
			}
            t = q_new((real)0, (real)0, h, (real)0);
            n = q_norm(t - c);
		} else {
            return (real)(-1);
        }
	} else if(fabs(d.z) > EPS) {
		h = dot(p, d)/d.z;
        t = q_new((real)0, (real)0, h, (real)0);
        n = d;
	}

    if (h > (real)1) {
        Moebius rsrc;
		moebius_inverse(&rsrc, src);
		*hit_pos = moebius_apply(&rsrc, t);
        *hit_norm = moebius_deriv(&rsrc, t, n);
        return log(h);
    } else {
        return (real)(-1);
    }
}
