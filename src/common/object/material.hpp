#pragma once

#include <types.h>
#include <algebra/rotation.hpp>
#include <render/light.hpp>
#include <random.hpp>


class Material {};
class SurfaceMaterial : public Material {};
class VolumeMaterial : public Material {};

class Black : public SurfaceMaterial {
public:
    template <typename Context>
    bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) {
        return false;
    }
};
class Transparent : public SurfaceMaterial {
public:
    template <typename Context>
    bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) {
        //light.face = !light.face;
        return true;
    }
};

class Specular : public SurfaceMaterial {
public:
    template <typename Context>
    bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) {
        light.direction -= (2*dot(light.direction, normal))*normal;
        return true;
    }
};
class Lambertian : public SurfaceMaterial {
public:
    template <typename Context>
    bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) {
        if (dot(normal, light.direction) > 0) {
            normal = -normal;
        }
        real3 rand = xrand::hemisphere_cosine(context.rng);
        Rotation3 rot = Rotation3::look_at(normal);
        light.direction = rot.apply(rand);
        light.diffuse = true;
        return true;
    }
};


template <typename M>
class Colored : public SurfaceMaterial {
public:
    M base;
    float3 color;

    template <typename Context>
    bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) {
        light.intensity *= color;
        return base.interact(context, normal, light, emission);
    }
};
template <typename M>
class Emissive : public SurfaceMaterial {
public:
    M base;
    float3 color;

    template <typename Context>
    bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) {
        emission += color*light.intensity;
        return base.interact(context, normal, light, emission);
    }
};
