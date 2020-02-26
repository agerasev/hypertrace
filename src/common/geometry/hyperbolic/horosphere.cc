#include "horosphere.hh"

#include <geometry/hyperbolic.hh>


bool horosphere_hit(
    const Horosphere *plane, HorosphereHit *cache,
    HyRay ray, quaternion *hit_pos
) {
    quaternion p = ray.start, d = ray.direction;
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

    cache->hit_pos = h;
    cache->hit_dir = hy_dir_at(p, d, h);

    *hit_pos = h;
    return true;
}

bool horosphere_bounce(
    const Horosphere *horosphere, const HorosphereHit *cache,
    Rng *rng,
    HyRay *ray,
    float3 *light, float3 *emission
) {
    float3 color = make_float3(1.0f);

	const real br = horosphere->border/horosphere->size;
    if (horosphere->tiling == HOROSPHERE_TILING_SQUARE) {
        complex k, f;
        complex g = cache->hit_pos.xy;
        f = fract(g/horosphere->size, &k);
        int hs = (int)k.x + (int)k.y;

		if (f.x < br || f.x > 1 - br || f.y < br || f.y > 1 - br) {
			color = make_float3(0.0f);
		} else {
			color = make_float3(hs & 1, (hs>>1) & 1, (hs>>2) & 1);
		}
    } else if (horosphere->tiling == HOROSPHERE_TILING_HEXAGONAL) {
        real2 g = cache->hit_pos.xy;
        real2 bx = make_real2(2/sqrt(3.0f), (real)0);
        real2 by = make_real2(-1/sqrt(3.0f), (real)1);
        real2 h = make_real2(dot(bx, g), dot(by, g))/horosphere->size;
        int hx = (int)floor((floor(h.x) - floor(h.y))/3);
        int hy = (int)floor((floor(h.x + h.y) - hx)/2);
        int hs = hx - hy;

        h -= hx*make_real2(2.0f, -1.0f) + hy*make_real2(1.0f, 1.0f);
        if (fabs(h.x - 1) > 1 - br || fabs(h.y) > 1 - br || fabs(h.x + h.y - 1) > 1 - br) {
            color = make_float3(0.0f);
        } else {
            color = make_float3(hs & 1, (hs>>1) & 1, (hs>>2) & 1);
        }
    }

    quaternion normal = -QJ;
    real3 bounce_dir;

    Material material = horosphere->material;
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
void pack_horosphere(HorospherePk *dst, const Horosphere *src) {
    dst->tiling = (uint)src->tiling;

    MaterialPk tmp_mat;
    pack_material(&tmp_mat, &src->material);
    dst->material = tmp_mat;

    dst->border = (real_pk)src->border;
    dst->size = (real_pk)src->size;
}

void unpack_horosphere(Horosphere *dst, const HorospherePk *src) {
    dst->tiling = (HorosphereTiling)src->tiling;

    MaterialPk tmp_mat = src->material;
    unpack_material(&dst->material, &tmp_mat);

    dst->border = (real)src->border;
    dst->size = (real)src->size;
}
#endif // OPENCL_INTEROP
