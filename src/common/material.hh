#pragma once

#include <ray.hh>
#include <random.hh>
#include <algebra/quaternion.hh>
#include <geometry/hyperbolic.hh>

#define DEF_MATERIAL_INTERACT()


void specular_interact(
    float3 color,
    const Ray *ray, Ray *new_ray,
    quaternion pos, quaternion dir, quaternion norm
);

void lambert_interact(
    Rng *rng, float3 color,
    const Ray *ray, Ray *new_ray,
    quaternion pos, quaternion dir, quaternion norm
);

#ifdef OPENCL
#include "material.cc"
#endif // OPENCL
