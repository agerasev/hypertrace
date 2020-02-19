#pragma once

#include <types.h>
#include <color.h>
#include <ray.h>

#include <algebra/moebius.h>

#include <geometry/plane.h>
#include <geometry/horosphere.h>

#ifndef OPENCL_DEVICE
#include <assert.h>
#endif // OPENCL_DEVICE


typedef struct Object Object;
typedef struct HitInfo HitInfo;

#ifdef OPENCL_INTEROP
typedef struct ObjectPacked ObjectPacked;
#endif // OPENCL_INTEROP


#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

real object_hit(
    const Object *obj, HitInfo *info,
    const Ray *ray
);
bool object_emit(
    const Object *obj, const HitInfo *info,
    const Ray *ray, Ray *new_ray
);

#ifdef OPENCL_INTEROP
void object_pack(ObjectPacked *dst, const Object *src);
void object_unpack(Object *dst, const ObjectPacked *src);
ObjectPacked object_pack_copy(Object src);
Object object_unpack_copy(ObjectPacked src);
#endif // OPENCL_INTEROP

#ifdef __cplusplus
};
#endif // __cplusplus


typedef enum ObjectType {
    OBJECT_NONE = 0,
    OBJECT_PLANE,
    OBJECT_HOROSPHERE
} ObjectType;


struct Object {
    ObjectType type;
    Moebius map;
    color surface_color;
};

struct HitInfo {
    quaternion pos;
    quaternion norm;
    quaternion local_pos;
};

#ifdef OPENCL_INTEROP

struct __attribute__ ((packed)) ObjectPacked {
    uint32_packed type;
    MoebiusPacked map;
    color_packed surface_color;
};

#endif // OPENCL_INTEROP


real object_hit(
    const Object *obj, HitInfo *info,
    const Ray *ray
) {
    real dist = (real)(-1);

    Moebius inv_map;
    moebius_inverse(&inv_map, &obj->map);
    quaternion rp = moebius_apply(&inv_map, ray->start);
    quaternion rd = q_norm(moebius_deriv(&inv_map, ray->start, ray->direction));

    quaternion hp, hn;
    if (obj->type == OBJECT_PLANE) {
        if (plane_hit(rp, rd, &hp)) {
            hn = hp;
        } else {
            return (real)(-1);
        }
    } else if (obj->type == OBJECT_HOROSPHERE) {
        if (horosphere_hit(rp, rd, &hp)) {
            hn = q_new((real)0, (real)0, (real)(-1), (real)0);
        } else {
            return (real)(-1);
        }
    } else {
        return (real)(-1);
    }

    info->local_pos = hp;
    info->pos = moebius_apply(&obj->map, hp);
    info->norm = q_norm(moebius_deriv(&obj->map, hp, hn));
    dist = qq_dist(hp, rp);

    return dist;
}

bool object_emit(
    const Object *obj, const HitInfo *info,
    const Ray *ray, Ray *new_ray
) {
    color surface_color = color_new(0.0f, 0.0f, 0.0f);

    if (obj->type == OBJECT_HOROSPHERE) {
        quaternion k, f;
#ifdef OPENCL_DEVICE
        f = fract(4.0f*info->local_pos, &k);
#else // OPENCL_DEVICE
        assert(info != NULL && false);
#endif // OPENCL_DEVICE
		int hs = (int)k.x + (int)k.y;

		const float br = 0.05f;
		if (f.x < br || f.x > 1.0f - br || f.y < br || f.y > 1.0f - br) {
			surface_color = color_new(0.1f, 0.1f, 0.1f);
		} else {
			surface_color = color_add(
                color_new_gray(0.3f),
                color_mulfc(0.2f, color_new(hs & 1, (hs>>1) & 1, (hs>>2) & 1))
            );
		}
    } else {
        surface_color = obj->surface_color;
    }

    new_ray->start = info->pos;
    
    Moebius a, b, c;
    moebius_new_move_to(&a, ray->start);
    moebius_new_move_to(&b, info->pos);
    moebius_inverse(&c, &b);
    moebius_chain(&b, &c, &a);

    quaternion hit_dir = q_norm(moebius_deriv(&b, ray->start, ray->direction));

    new_ray->direction = qq_sub(
        hit_dir,
        rq_mul((real)2*qq_dot(hit_dir, info->norm), info->norm)
    );
    new_ray->intensity = color_mulcc(ray->intensity, surface_color);

    return true;
}

#ifdef OPENCL_INTEROP

void object_pack(ObjectPacked *dst, const Object *src) {
    dst->type = src->type;
    moebius_pack(&dst->map, &src->map);
    dst->surface_color = color_pack(src->surface_color);
}

void object_unpack(Object *dst, const ObjectPacked *src) {
    dst->type = (ObjectType)src->type;
    moebius_unpack(&dst->map, &src->map);
    dst->surface_color = color_unpack(src->surface_color);
}

ObjectPacked object_pack_copy(Object src) {
    ObjectPacked dst;
    object_pack(&dst, &src);
    return dst;
}
Object object_unpack_copy(ObjectPacked src) {
    Object dst;
    object_unpack(&dst, &src);
    return dst;
}

#endif // OPENCL_INTEROP