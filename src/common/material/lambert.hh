#pragma once

#include <ray.hh>
#include <random.hh>
#include <algebra/quaternion.hh>


void lambert_emit(
    Rng *rng, float3 color,
    const Ray *ray, Ray *new_ray,
    quaternion pos, quaternion dir, quaternion norm
) {
    new_ray->start = pos;

    if (dot(norm, dir) > 0) {
        norm = -norm;
    }

    quaternion a = cross(QJ, norm);
    real al = length(a);
    a /= al;

    real theta = atan2(al, norm.z)/(real)2;
    quaternion q = q_new(cos(theta), sin(theta)*a.xyz);
    real3 r = rand_hemisphere_cosine(rng);

    new_ray->direction = q_new(q_div(q_mul(q, q_new(R0, r)), q).yzw, R0);
    
    new_ray->intensity = ray->intensity*color;
}
