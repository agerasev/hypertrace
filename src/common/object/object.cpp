#include "object.hpp"

#include <geometry/hyperbolic/plane.hpp>
#include <geometry/hyperbolic/horosphere.hpp>

using namespace random;
using namespace hyperbolic;

real Object::hit(
    ObjectHit *cache,
    Rng *rng, PathInfo *path,
    HyRay ray
) {
    
}

bool object_bounce(
    const Object *object, const ObjectHit *cache,
    Rng *rng, PathInfo *path,
    HyRay *ray,
    float3 *light, float3 *emission
) {
    Material material;
    quaternion hit_dir, normal;
    
    if (object->type == OBJECT_HYPLANE) {
        hyplane_bounce(
            object, cache,
            &hit_dir, &normal,
            &material
        );
    } else if (object->type == OBJECT_HOROSPHERE) {
        horosphere_bounce(
            object, cache,
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

    ray->start = cache->pos;
    ray->direction = q_new(bounce_dir, (real)0);

    *ray = hyray_map(object->map, *ray);

    return true;
}

void object_interpolate(
    Object *o,
    const Object *a, const Object *b,
    real t
) {
    o->type = b->type;
    o->map = mo_interpolate(a->map, b->map, t);

    o->material_count = b->material_count;
    for (int i = 0; i < MATERIAL_COUNT_MAX; ++i) {
        material_interpolate(
            &o->materials[i],
            &a->materials[i], &b->materials[i],
            t
        );
    }

    tiling_interpolate(&o->tiling, &a->tiling, &b->tiling, t);
}

void tiling_interpolate(
    Tiling *o,
    const Tiling *a, const Tiling *b,
    real t
) {
    o->type = b->type;
    INTERPOLATE_FIELD(*o, *a, *b, cell_size, t);
    INTERPOLATE_FIELD(*o, *a, *b, border_width, t);
    material_interpolate(
        &o->border_material,
        &a->border_material, &b->border_material,
        t
    );
}

#ifdef OPENCL_INTEROP

void pack_object(ObjectPk *dst, const Object *src) {
    dst->type = src->type;
    dst->map = mo_pack(src->map);

    for (int i = 0; i < MATERIAL_COUNT_MAX; ++i) {
        pack_material(&dst->materials[i], &src->materials[i]);
    }
    dst->material_count = src->material_count;

    pack_tiling(&dst->tiling, &src->tiling);
}

void unpack_object(Object *dst, const ObjectPk *src) {
    dst->type = (ObjectType)src->type;
    dst->map = mo_unpack(src->map);

    for (int i = 0; i < MATERIAL_COUNT_MAX; ++i) {
        unpack_material(&dst->materials[i], &src->materials[i]);
    }
    dst->material_count = (int)src->material_count;

    unpack_tiling(&dst->tiling, &src->tiling);
}

void pack_tiling(TilingPk *dst, const Tiling *src) {
    dst->type = (TilingTypePk)src->type;
    dst->cell_size = (real_pk)src->cell_size;

    dst->border_width = (real_pk)src->border_width;
    pack_material(&dst->border_material, &src->border_material);
}

void unpack_tiling(Tiling *dst, const TilingPk *src) {
    dst->type = (TilingType)src->type;
    dst->cell_size = (real)src->cell_size;
    
    dst->border_width = (real)src->border_width;
    unpack_material(&dst->border_material, &src->border_material);
}

#endif // OPENCL_INTEROP
