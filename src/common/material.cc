#include "material.hh"


void specular_emit(
    float3 color,
    const Ray *ray, Ray *new_ray,
    quaternion pos, quaternion dir, quaternion norm
) {
    new_ray->start = pos;
    new_ray->direction = dir - ((real)2*dot(dir, norm))*norm;
    new_ray->intensity = ray->intensity*color;
}

void lambert_emit(
    Rng *rng, float3 color,
    const Ray *ray, Ray *new_ray,
    quaternion pos, quaternion dir, quaternion norm
) {
    new_ray->start = pos;

    if (dot(norm, dir) > 0) {
        norm = -norm;
    }

    real3 r = rand_hemisphere_cosine(rng);
    quaternion d;

    quaternion a = cross(QJ, norm);
    real al = length(a);
    if (al > EPS) {
        a /= al;
        real theta = atan2(al, norm.z)/(real)2;
        quaternion q = q_new(cos(theta), sin(theta)*a.xyz);
        d = q_new(q_div(q_mul(q, q_new(R0, r)), q).yzw, R0);
    } else {
        d = q_new(norm.z < 0 ? -r : r, R0);
    }

    new_ray->direction = d;
    
    new_ray->intensity = ray->intensity*color;
}
