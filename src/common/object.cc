#include "object.hh"


real object_hit(
    const Object *object, ObjectHit *cache,
    Rng *rng,
    HyRay ray
) {
    HyRay r = hyray_map(mo_inverse(object->map), ray);

    quaternion hp;
    bool h = false;
    if (object->type == OBJECT_PLANE) {
        h = hyplane_hit(
            &object->plane, &cache->plane,
            r, &hp
        );
    } else if (object->type == OBJECT_HOROSPHERE) {
        h = horosphere_hit(
            &object->horosphere, &cache->horosphere,
            r, &hp
        );
    }

    if (h) {
        return hy_distance(hp, r.start);
    } else {
        return (real)(-1);
    }
}

bool object_bounce(
    const Object *object, const ObjectHit *cache,
    Rng *rng,
    HyRay *ray,
    float3 *light, float3 *emission
) {
    bool b = false;

    if (object->type == OBJECT_PLANE) {
        b = hyplane_bounce(
            &object->plane, &cache->plane,
            rng,
            ray,
            light, emission
        );
    } else if (object->type == OBJECT_HOROSPHERE) {
        b = horosphere_bounce(
            &object->horosphere, &cache->horosphere,
            rng,
            ray,
            light, emission
        );
    }

    *ray = hyray_map(object->map, *ray);
    return b;
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

ObjectPk pack_copy_object(Object src) {
    ObjectPk dst;
    pack_object(&dst, &src);
    return dst;
}
Object unpack_copy_object(ObjectPk src) {
    Object dst = {};
    unpack_object(&dst, &src);
    return dst;
}

#endif // OPENCL_INTEROP
