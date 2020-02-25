#include "material.hh"

#include <algebra/rotation.hh>


void specular_interact(
    float3 color,
    const Ray *ray, Ray *new_ray,
    quaternion pos, quaternion dir, quaternion norm
) {
    new_ray->start = pos;
    new_ray->direction = dir - ((real)2*dot(dir, norm))*norm;
    new_ray->intensity = ray->intensity*color;
}

void lambert_interact(
    Rng *rng, float3 color,
    const Ray *ray, Ray *new_ray,
    quaternion pos, quaternion dir, quaternion norm
) {
    new_ray->start = pos;

    if (dot(norm, dir) > 0) {
        norm = -norm;
    }

    real3 rand = rand_hemisphere_cosine(rng);
    rotation3 rot = rot3_look_at(norm.xyz);
    new_ray->direction = q_new(rot3_apply(rot, rand), (real)0);
    
    new_ray->intensity = ray->intensity*color;
}
