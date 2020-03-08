#include "plane.hh"

#include <geometry/hyperbolic.hh>


bool hyplane_hit(
    const Object *plane, ObjectHit *cache,
    PathInfo *path, HyRay ray
) {
    // Line cannot intersect plane twice
    if (path->repeat) {
        return false;
    }

    quaternion p = ray.start, d = ray.direction;
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

    cache->pos = h;
    cache->dir = hy_dir_at(p, d, h);

    return true;
}

void hyplane_bounce(
    const Object *plane, const ObjectHit *cache,
    quaternion *hit_dir, quaternion *normal,
    Material *material
) {
    if (
        plane->tiling.type == HYPLANE_TILING_PENTAGONAL ||
        plane->tiling.type == HYPLANE_TILING_PENTASTAR
    ) {
        quaternion p = cache->pos;

        bool w = false;
        //uint n = 0, b = 1;
        real Q = sqrt(cos(PI/4 + PI/5)/cos(PI/4 - PI/5));
        real T = sqrt(cos(PI/4 + PI/5)*cos(PI/4 - PI/5));
        real S = (cos(PI/4) - sin(PI/5))/T;
        real L = T/cos(PI/4);
        real K = L*(2*cos(PI/5) - 1/cos(PI/5));
        Q = log((1 + Q)/(1 - Q));
        S = log((1 + S)/(1 - S));

        if (p.x < (real)0) {
            p.x = -p.x;
            //n |= 1;
            w = !w;
        }
        if (p.y < (real)0) {
            p.y = -p.y;
            //n |= 2;
            w = !w;
        }
        //b *= 4;

        p = mo_apply(mo_chain(
            hy_xshift(-Q),
            hy_zrotate(-PI/4)
        ), p);
        bool e = false;
        for (int j = 0; j < 5; ++j) {
            bool a[3] = {false};
            for (int i = 0; i < 3 - e; ++i) {
                real o = 2*PI*(i - 1)/5;
                real2 d = make_real2(cos(o), sin(o));
                a[i] = (dot(d, p.xy) < L);
            }
            a[2] = a[2] || e;
            int s = (int)a[0] + (int)a[1] + (int)a[2];
            //uint bb = b*(6 - 2*e);
            if (s == 3) {
                break;
            } else if (s == 2) {
                int i = (!a[1]) + 2*(!a[2]);
                real o = 2*PI*(i - 1)/5;
                p = mo_apply(mo_chain(
                    hy_zrotate(-PI/5),
                    mo_chain(
                        hy_xshift(-2*S),
                        hy_zrotate(-o)
                    )
                ), p);
                //n += b*(2*i + 1);
                e = true;
                w = !w;
            } else {
                int i = a[0];
                real o = PI*(2*i - 1)/5;
                p = mo_apply(mo_chain(
                    hy_xshift(-2*Q),
                    hy_zrotate(-o)
                ), p);
                //n += b*(2*(i + 1));
                e = false;
            }
            //b = bb;
        }
        
        const real br = plane->tiling.border_width;
        bool bh = 0;
        for (int i = 0; i < 5; ++i) {
            real o = 2*PI*i/5;
            real2 d = make_real2(cos(o), sin(o));
            bh = bh || (dot(d, p.xy) > (L - br*p.z));
            if (plane->tiling.type == HYPLANE_TILING_PENTASTAR) {
                real ps = K + dot(d, p.xy);
                w = w^(ps < 0);
                bh = bh || (fabs(ps) < br*p.z);
            }
        }
        if (bh == 0) {
            if (w || plane->material_count < 2) {
                *material = plane->materials[0];
            } else {
                *material = plane->materials[1];
            }
        } else {
            *material = plane->tiling.border_material;
        }
    } else {
        *material = plane->materials[0];
    }

    *hit_dir = cache->dir;
    *normal = cache->pos;
}
