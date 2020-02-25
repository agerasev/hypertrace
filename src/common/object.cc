#include "object.hh"


real object_hit(
    const Object *object, ObjectHit *cache,
    Rng *rng,
    HyRay ray
) {
    HyRay r = hyray_map(mo_inverse(object->map), ray);

    quaternion hp;
    if (object->type == OBJECT_PLANE) {
        if (!hyplane_hit(
            &object->plane, &cache->plane,
            r, &hp
        )) {
            return (real)(-1);
        }
    } else {
        return (real)(-1);
    }

    return hy_distance(hp, r.start);
}

bool object_bounce(
    const Object *object, const ObjectHit *cache,
    Rng *rng,
    HyRay *ray,
    float3 light_in, float3 *light_out, float3 *emission
) {
    bool b = false;

    if (object->type == OBJECT_PLANE) {
        b = hyplane_bounce(
            &object->plane, &cache->plane,
            rng,
            ray,
            light_in, light_out, emission
        );
    } else {
        b = false;
    }

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
    }
}

void unpack_object(Object *dst, const ObjectPk *src) {
    dst->type = (ObjectType)src->type;
    dst->map = mo_unpack(src->map);

    if ((ObjectType)src->type == OBJECT_PLANE) {
        HyPlanePk tmp = src->plane;
        unpack_hyplane(&dst->plane, &tmp);
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
