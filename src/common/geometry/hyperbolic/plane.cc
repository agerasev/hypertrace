#include "plane.hh"

#include <geometry/hyperbolic.hh>


bool hyplane_hit(
    const HyPlane *plane, HyPlaneHit *cache,
    PathInfo *path, HyRay ray, quaternion *hit_pos
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

    cache->hit_pos = h;
    cache->hit_dir = hy_dir_at(p, d, h);

    *hit_pos = h;
    return true;
}

void hyplane_bounce(
    const HyPlane *plane, const HyPlaneHit *cache,
    quaternion *hit_dir, quaternion *normal,
    Material *material
) {
    if (
        plane->tiling.type == HYPLANE_TILING_PENTAGONAL ||
        plane->tiling.type == HYPLANE_TILING_PENTASTAR
    ) {
        quaternion p = cache->hit_pos;

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
        
        const real br = plane->tiling.border.width;
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
            *material = plane->tiling.border.material;
        }
    } else {
        *material = plane->materials[0];
    }

    *hit_dir = cache->hit_dir;
    *normal = cache->hit_pos;
}

void hyplane_interpolate(
    HyPlane *o,
    const HyPlane *a, const HyPlane *b,
    real t
) {
    const HyPlane *c = t < 0.5f ? a : b;

    if (a->material_count == b->material_count) {
        o->material_count = a->material_count;
        for (int i = 0; i < a->material_count; ++i) {
            material_interpolate(
                &o->materials[i],
                &a->materials[i], &b->materials[i],
                t
            );
        }
    } else {
        o->material_count = c->material_count;
        for (int i = 0; i < c->material_count; ++i) {
            o->materials[i] = c->materials[i];
        }
    }

    if (a->tiling.type == b->tiling.type) {
        o->tiling.type = a->tiling.type;
        INTERPOLATE_FIELD(*o, *a, *b, tiling.border.width, t);
        material_interpolate(
            &o->tiling.border.material,
            &a->tiling.border.material, &b->tiling.border.material,
            t
        );
    } else {
        o->tiling = c->tiling;
    }
}

#ifdef OPENCL_INTEROP
void pack_hyplane(HyPlanePk *dst, const HyPlane *src) {
    for (int i = 0; i < HYPLANE_MATERIAL_COUNT_MAX; ++i) {
        MaterialPk tmp;
        pack_material(&tmp, &src->materials[i]);
        dst->materials[i] = tmp;
    }
    dst->material_count = src->material_count;

    dst->tiling_type = (uint_pk)src->tiling.type;
    dst->tiling_border_width = (real_pk)src->tiling.border.width;
    MaterialPk tmp_mat;
    pack_material(&tmp_mat, &src->tiling.border.material);
    dst->tiling_border_material = tmp_mat;
}

void unpack_hyplane(HyPlane *dst, const HyPlanePk *src) {
    for (int i = 0; i < HYPLANE_MATERIAL_COUNT_MAX; ++i) {
        MaterialPk tmp = src->materials[i];
        unpack_material(&dst->materials[i], &tmp);
    }
    dst->material_count = (int)src->material_count;

    dst->tiling.type = (HyPlaneTilingType)src->tiling_type;
    dst->tiling.border.width = (real)src->tiling_border_width;
    MaterialPk tmp_mat = src->tiling_border_material;
    unpack_material(&dst->tiling.border.material, &tmp_mat);
}
#endif // OPENCL_INTEROP
