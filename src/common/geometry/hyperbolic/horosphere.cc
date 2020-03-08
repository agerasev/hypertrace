#include "horosphere.hh"

#include <geometry/hyperbolic.hh>


bool horosphere_hit(
    const Object *plane, ObjectHit *cache,
    PathInfo *path, HyRay ray
) {
    bool face = true;
    quaternion p = ray.start, d = ray.direction;
    real dxy = length(d.xy);
    // FIXME: check (dxy < EPS)

    if (p.z < dxy) {
        return false;
    }
    
    real dt = sqrt(p.z*p.z - dxy*dxy);
    real t = p.z*d.z - dt;
    if (t < (real)0 || (path->repeat && path->face != face)) {
        t += (real)2*dt;
        face = false;
    }
    if (t < (real)0 || (path->repeat && path->face != face)) {
        return false;
    }

    t /= dxy*dxy;
    quaternion h = q_new(
        p.xy + d.xy*t,
        (real)1, (real)0
    );

    cache->pos = h;
    cache->dir = hy_dir_at(p, d, h);

    path->face = face;

    return true;
}

void horosphere_bounce(
    const Object *horosphere, const ObjectHit *cache,
    quaternion *hit_dir, quaternion *normal,
    Material *material
) {
    int material_no = 0;
    bool border = false;

	real br = horosphere->tiling.border_width/horosphere->tiling.cell_size;
    if (horosphere->tiling.type == HOROSPHERE_TILING_SQUARE) {
        complex k, f;
        complex g = cache->pos.xy;
        f = fract(g/horosphere->tiling.cell_size, &k);

		if (f.x < br || f.x > 1 - br || f.y < br || f.y > 1 - br) {
            border = true;
		} else {
            int hx = mod((int)k.x, 2);
            int hy = mod((int)k.y, 2);
            material_no = 3*hy - 2*hx*hy + hx;
		}
    } else if (horosphere->tiling.type == HOROSPHERE_TILING_HEXAGONAL) {
        real2 g = cache->pos.xy;
        real2 bx = make_real2(2/sqrt(3.0f), (real)0);
        real2 by = make_real2(-1/sqrt(3.0f), (real)1);
        real s = horosphere->tiling.cell_size*sqrt(3.0f)/2;
        real2 h = make_real2(dot(bx, g), dot(by, g))/s;
        int hx = (int)floor((floor(h.x) - floor(h.y))/3);
        int hy = (int)floor((floor(h.x + h.y) - hx)/2);

        h -= hx*make_real2(2.0f, -1.0f) + hy*make_real2(1.0f, 1.0f);
        if (fabs(h.x - 1) > 1 - br || fabs(h.y) > 1 - br || fabs(h.x + h.y - 1) > 1 - br) {
            border = true;
        } else {
            material_no = 2*hx + hy;
        }
    } else {
        material_no = 0;
    }

    *hit_dir = cache->dir;
    *normal = -QJ;

    if (border) {
        *material = horosphere->tiling.border_material;
    } else {
        *material = horosphere->materials[mod(material_no, horosphere->material_count)];
    }
}
