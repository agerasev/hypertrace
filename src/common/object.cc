#include "object.hh"


real object_intersect(
    Rng *rng,
    const Object *obj, HitInfo *info,
    const Ray *ray
) {
    real dist = (real)(-1);

    Moebius inv_map = mo_inverse(obj->map);
    quaternion rp = mo_apply(inv_map, ray->start);
    quaternion rd = normalize(mo_deriv(inv_map, ray->start, ray->direction));

    quaternion hp, hn;
    if (obj->type == OBJECT_PLANE) {
        if (!plane_intersect(rp, rd, &hp, &hn)) {
            return (real)(-1);
        }
    } else if (obj->type == OBJECT_HOROSPHERE) {
        if (!horosphere_intersect(rp, rd, &hp, &hn)) {
            return (real)(-1);
        }
    } else {
        return (real)(-1);
    }

    info->local_pos = hp;
    info->pos = mo_apply(obj->map, hp);
    info->norm = normalize(mo_deriv(obj->map, hp, hn));
    dist = hy_distance(hp, rp);

    return dist;
}

bool object_interact(
    Rng *rng,
    const Object *obj, const HitInfo *info,
    const Ray *ray, Ray *new_ray
) {
    float3 color = make_float3(0.0f, 0.0f, 0.0f);

    if (obj->type == OBJECT_HOROSPHERE) {
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
            color = obj->color*make_float3(0.0f);
        } else {
		    color = obj->color*(0.2f + 0.8f*make_float3(hs & 1, (hs>>1) & 1, (hs>>2) & 1));
        }
    } else if (obj->type == OBJECT_PLANE) {
        quaternion p = info->local_pos;
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
        const real br = 0.03f;
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
            color = obj->color*(0.2f + 0.8f*make_float3((n>>0)&3, (n>>2)&3, (n>>4)&3)/3);
        } else {
            color = obj->color*make_float3(0.0f);
        }
    }

    quaternion hit_dir = hy_dir_at(ray->start, ray->direction, info->pos);
    if (rand_uniform(rng) < obj->gloss) {
        specular_interact(
            make_float3(1.0f),
            ray, new_ray,
            info->pos, hit_dir, info->norm
        );
    } else {
        lambert_interact(
            rng, color,
            ray, new_ray,
            info->pos, hit_dir, info->norm
        );
    }

    return true;
}

#ifdef OPENCL_INTEROP

void pack_object(ObjectPk *dst, const Object *src) {
    dst->type = src->type;
    dst->map = mo_pack(src->map);
    dst->color = pack_float3(src->color);
    dst->gloss = src->gloss;
}

void unpack_object(Object *dst, const ObjectPk *src) {
    dst->type = (ObjectType)src->type;
    dst->map = mo_unpack(src->map);
    dst->color = unpack_float3(src->color);
    dst->gloss = src->gloss;
}

ObjectPk pack_copy_object(Object src) {
    ObjectPk dst;
    pack_object(&dst, &src);
    return dst;
}
Object unpack_copy_object(ObjectPk src) {
    Object dst;
    unpack_object(&dst, &src);
    return dst;
}

#endif // OPENCL_INTEROP
