#include "object.hh"


real object_hit(
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
        if (!plane_hit(rp, rd, &hp, &hn)) {
            return (real)(-1);
        }
    } else if (obj->type == OBJECT_HOROSPHERE) {
        if (!horosphere_hit(rp, rd, &hp, &hn)) {
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

bool object_emit(
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
    } else {
        color = obj->color;
    }

    quaternion hit_dir = hy_dir_at(ray->start, ray->direction, info->pos);
    if (rand_uniform(rng) < obj->gloss) {
        specular_emit(
            make_float3(1.0f),
            ray, new_ray,
            info->pos, hit_dir, info->norm
        );
    } else {
        lambert_emit(
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
