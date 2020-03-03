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
    int material_no = 0;

	const real br = horosphere->tiling.border.width/horosphere->tiling.cell_size;
    if (horosphere->tiling.type == HOROSPHERE_TILING_SQUARE) {
        complex k, f;
        complex g = cache->hit_pos.xy;
        f = fract(g/horosphere->tiling.cell_size, &k);

		if (f.x < br || f.x > 1 - br || f.y < br || f.y > 1 - br) {
            material_no = -1;
		} else {
            int hx = mod((int)k.x, 2);
            int hy = mod((int)k.y, 2);
            material_no = 3*hy - 2*hx*hy + hx;
		}
    } else if (horosphere->tiling.type == HOROSPHERE_TILING_HEXAGONAL) {
        real2 g = cache->hit_pos.xy;
        real2 bx = make_real2(2/sqrt(3.0f), (real)0);
        real2 by = make_real2(-1/sqrt(3.0f), (real)1);
        real2 h = make_real2(dot(bx, g), dot(by, g))/horosphere->tiling.cell_size;
        int hx = (int)floor((floor(h.x) - floor(h.y))/3);
        int hy = (int)floor((floor(h.x + h.y) - hx)/2);

        h -= hx*make_real2(2.0f, -1.0f) + hy*make_real2(1.0f, 1.0f);
        if (fabs(h.x - 1) > 1 - br || fabs(h.y) > 1 - br || fabs(h.x + h.y - 1) > 1 - br) {
            material_no = -1;
        } else {
            material_no = 2*hx + hy;
        }
    } else {
        material_no = 0;
    }

    quaternion normal = -QJ;
    real3 bounce_dir;

    Material material;
    if (material_no < 0) {
        material = horosphere->tiling.border.material;
    } else {
        material = horosphere->materials[mod(material_no, horosphere->material_count)];
    }

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
    for (int i = 0; i < HOROSPHERE_MATERIAL_COUNT_MAX; ++i) {
        MaterialPk tmp;
        pack_material(&tmp, &src->materials[i]);
        dst->materials[i] = tmp;
    }
    dst->material_count = src->material_count;

    dst->tiling_type = (uint_pk)src->tiling.type;
    dst->tiling_cell_size = (real_pk)src->tiling.cell_size;

    dst->tiling_border_width = (real_pk)src->tiling.border.width;
    MaterialPk tmp_mat;
    pack_material(&tmp_mat, &src->tiling.border.material);
    dst->tiling_border_material = tmp_mat;
}

void unpack_horosphere(Horosphere *dst, const HorospherePk *src) {
    for (int i = 0; i < HOROSPHERE_MATERIAL_COUNT_MAX; ++i) {
        MaterialPk tmp = src->materials[i];
        unpack_material(&dst->materials[i], &tmp);
    }
    dst->material_count = (int)src->material_count;

    dst->tiling.type = (HorosphereTilingType)src->tiling_type;
    dst->tiling.cell_size = (real)src->tiling_cell_size;
    
    dst->tiling.border.width = (real)src->tiling_border_width;
    MaterialPk tmp_mat = src->tiling_border_material;
    unpack_material(&dst->tiling.border.material, &tmp_mat);
}
#endif // OPENCL_INTEROP
