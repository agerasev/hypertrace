#include "plane.hh"

#include <geometry/hyperbolic.hh>


bool hyplane_hit(
    const HyPlane *plane, HyPlaneHit *cache,
    HyRay ray, quaternion *hit_pos
) {
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

bool hyplane_bounce(
    const HyPlane *plane, const HyPlaneHit *cache,
    Rng *rng,
    HyRay *ray,
    float3 *light, float3 *emission
) {
    float3 color = make_float3(1.0f);

    if (plane->tiling == HYPLANE_TILING_PENTAGONAL) {
        quaternion p = cache->hit_pos;

        uint n = 0, b = 1;
        real Q = sqrt(cos(PI/4 + PI/5)/cos(PI/4 - PI/5));
        real T = sqrt(cos(PI/4 + PI/5)*cos(PI/4 - PI/5));
        real S = (cos(PI/4) - sin(PI/5))/T;
        real L = T/cos(PI/4);
        Q = log((1 + Q)/(1 - Q));
        S = log((1 + S)/(1 - S));

        if (p.x < (real)0) {
            p.x = -p.x;
            n |= 1;
        }
        if (p.y < (real)0) {
            p.y = -p.y;
            n |= 2;
        }
        b *= 4;

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
            uint bb = b*(6 - 2*e);
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
                n += b*(2*i + 1);
                e = true;
            } else {
                int i = a[0];
                real o = PI*(2*i - 1)/5;
                p = mo_apply(mo_chain(
                    hy_xshift(-2*Q),
                    hy_zrotate(-o)
                ), p);
                n += b*(2*(i + 1));
                e = false;
            }
            b = bb;
        }
        const real br = plane->border;
        int bh = 0;
        for (int i = 0; i < 5; ++i) {
            real o = 2*PI*i/5;
            real2 d = make_real2(cos(o), sin(o));
            bh += (dot(d, p.xy) > (L - br*p.z));
        }
        if (bh == 0) {
            n = (n>>16)^n;
            n = (n>>8)^n;
            n = (n>>4)^n;
            n = (n>>2)^n;
            color = (0.2f + 0.8f*make_float3((n>>0)&3, (n>>2)&3, (n>>4)&3)/3);
        } else {
            color = make_float3(0.0f);
        }
    }

    quaternion normal = cache->hit_pos;
    real3 bounce_dir;

    Material material = plane->material;
    material.diffuse_color *= color;

    if (!material_bounce(
        &material,
        rng,
        cache->hit_dir.xyz, normal.xyz, &bounce_dir,
        light, emission
    )) {
        return false;
    }

    ray->start = cache->hit_pos;
    ray->direction = q_new(bounce_dir, (real)0);
    return true;
}


#ifdef OPENCL_INTEROP
void pack_hyplane(HyPlanePk *dst, const HyPlane *src) {
    dst->tiling = (uint)src->tiling;

    MaterialPk tmp_mat;
    pack_material(&tmp_mat, &src->material);
    dst->material = tmp_mat;

    dst->border = (real_pk)src->border;
}
void unpack_hyplane(HyPlane *dst, const HyPlanePk *src) {
    dst->tiling = (HyPlaneTiling)src->tiling;

    MaterialPk tmp_mat = src->material;
    unpack_material(&dst->material, &tmp_mat);

    dst->border = (real)src->border;
}
#endif // OPENCL_INTEROP
