#pragma once

#include <ray.hh>
#include <algebra/quaternion.hh>
#include <geometry/hyperbolic.hh>


void specular_emit(
    float3 color,
    const Ray *ray, Ray *new_ray,
    quaternion pos, quaternion norm    
) {
    new_ray->start = pos;
    quaternion hit_dir = hy_dir_at(ray->start, ray->direction, pos);
    new_ray->direction = hit_dir - ((real)2*dot(hit_dir, norm))*norm;
    new_ray->intensity = ray->intensity*color;
}
