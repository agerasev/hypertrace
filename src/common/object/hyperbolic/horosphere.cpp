#include "horosphere.hpp"

namespace hyperbolic {

real Horosphere::hit(Context &context, Cache &cache, Ray ray) const {
    bool face = true;
    quat p = ray.start, d = ray.direction;
    real dxy = length(d.re());
    // FIXME: check (dxy < EPS)

    if (p[2] < dxy) {
        return -1_r;
    }
    
    real dt = math::sqrt(p[2]*p[2] - dxy*dxy);
    real t = p[2]*d[2] - dt;
    if (t < (real)0 || (context.path.repeat && context.path.face != face)) {
        t += 2*dt;
        face = false;
    }
    if (t < (real)0 || (context.path.repeat && context.path.face != face)) {
        return -1_r;
    }

    t /= dxy*dxy;
    quat h(p.re() + d.re()*t, 1, 0);

    cache.pos = h;
    cache.dir = Hy::dir_at(p, d, h);

    context.path.face = face;

    return Hy::length(h);
}

bool Horosphere::bounce(
    Context &context, const Cache &cache,
    float3 &luminance, Ray &new_ray) const {
    /*
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
    */
    real2 fr = math::fract(cache.pos.re().vec()).first;
    luminance += float3(fr[0], fr[1], 0);
    return false;
}

}
