#pragma once

#include <ray.hh>
#include <algebra/quaternion.hh>
#include <geometry/hyperbolic.hh>


void specular_emit(
    float3 color,
    const Ray *ray, Ray *new_ray,
    quaternion pos, quaternion dir, quaternion norm
) {
    new_ray->start = pos;
    new_ray->direction = dir - ((real)2*dot(dir, norm))*norm;
    new_ray->intensity = ray->intensity*color;
}
