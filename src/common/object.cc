#include "object.hh"


real object_hit(
    const Object *object, ObjectHit *cache,
    Rng *rng, PathInfo *path,
    HyRay ray
) {
    HyRay r = hyray_map(mo_inverse(object->map), ray);

    quaternion hp;
    bool h = false;
    if (object->type == OBJECT_PLANE) {
        h = hyplane_hit(
            &object->plane, &cache->plane,
            path, r, &hp
        );
    } else if (object->type == OBJECT_HOROSPHERE) {
        h = horosphere_hit(
            &object->horosphere, &cache->horosphere,
            path, r, &hp
        );
    }

    if (h) {
        cache->hit_pos = hp;
        return hy_distance(hp, r.start);
    } else {
        return (real)(-1);
    }
}

bool object_bounce(
    const Object *object, const ObjectHit *cache,
    Rng *rng, PathInfo *path,
    HyRay *ray,
    float3 *light, float3 *emission
) {
    Material material;
    quaternion hit_dir, normal;
    
    if (object->type == OBJECT_PLANE) {
        hyplane_bounce(
            &object->plane, &cache->plane,
            &hit_dir, &normal,
            &material
        );
    } else if (object->type == OBJECT_HOROSPHERE) {
        horosphere_bounce(
            &object->horosphere, &cache->horosphere,
            &hit_dir, &normal,
            &material
        );
    }

    real3 bounce_dir;
        material_bounce(
            &material,
            rng, path,
            hit_dir.xyz, normal.xyz, &bounce_dir,
            light, emission
        );

    ray->start = cache->hit_pos;
    ray->direction = q_new(bounce_dir, (real)0);

    *ray = hyray_map(object->map, *ray);

    return true;
}

#ifdef OPENCL_INTEROP

void pack_object(ObjectPk *dst, const Object *src) {
    dst->type = src->type;
    dst->map = mo_pack(src->map);

    if (src->type == OBJECT_PLANE) {
        HyPlanePk tmp;
        pack_hyplane(&tmp, &src->plane);
        dst->plane = tmp;
    } else if (src->type == OBJECT_HOROSPHERE) {
        HorospherePk tmp;
        pack_horosphere(&tmp, &src->horosphere);
        dst->horosphere = tmp;
    }
}

void unpack_object(Object *dst, const ObjectPk *src) {
    dst->type = (ObjectType)src->type;
    dst->map = mo_unpack(src->map);

    if ((ObjectType)src->type == OBJECT_PLANE) {
        HyPlanePk tmp = src->plane;
        unpack_hyplane(&dst->plane, &tmp);
    } else if ((ObjectType)src->type == OBJECT_HOROSPHERE) {
        HorospherePk tmp = src->horosphere;
        unpack_horosphere(&dst->horosphere, &tmp);
    }
}

#endif // OPENCL_INTEROP
