#pragma once

#include <types.h>
#include <algebra/rotation.hpp>
#include <render/light.hpp>
#include <random.hpp>

/*
class Material {
    template <typename Context>
    bool interact(
        Context &context,
        real3 normal,
        LocalLight &light,
        float3 &emission
    ) const;
};

class SurfaceMaterial : public Material {};
class VolumeMaterial : public Material {};
*/

class Black {
// : public SurfaceMaterial
public:
    template <typename Context>
    static bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) {
        return false;
    }
};
template <> struct IsEmpty<Black> { static const bool value = true; };

class Transparent {
// : public SurfaceMaterial
public:
    template <typename Context>
    static bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) {
        //light.face = !light.face;
        return true;
    }
};
template <> struct IsEmpty<Transparent> { static const bool value = true; };

class Specular {
// : public SurfaceMaterial
public:
    template <typename Context>
    static bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) {
        light.direction -= (2*dot(light.direction, normal))*normal;
        return true;
    }
};
template <> struct IsEmpty<Specular> { static const bool value = true; };

class Lambertian {
// : public SurfaceMaterial
public:
    template <typename Context>
    static bool interact(
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
template <> struct IsEmpty<Lambertian> { static const bool value = true; };

#ifdef HOST

template <>
struct ToDevice<Black> {
    struct type {};
    static type to_device(const Black &) {
        return type {};
    }
};
template <>
struct ToDevice<Transparent> {
    struct type {};
    static type to_device(const Transparent &) {
        return type {};
    }
};
template <>
struct ToDevice<Specular> {
    struct type {};
    static type to_device(const Specular &) {
        return type {};
    }
};
template <>
struct ToDevice<Lambertian> {
    struct type {};
    static type to_device(const Lambertian &) {
        return type {};
    }
};

#endif


template <typename M, bool Z=is_empty<M>()>
class Colored {
// : public Material
public:
    M base;
    float3 color;

    Colored() = default;
    Colored(float3 v, const M &b) : base(b), color(v) {}

    template <typename Context>
    bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) const {
        light.intensity *= color;
        return base.interact(context, normal, light, emission);
    }
};
template <typename M>
class Colored<M, true> {
// : public Material
public:
    float3 color;

    Colored() = default;
    Colored(float3 v) : color(v) {}

    template <typename Context>
    bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) const {
        light.intensity *= color;
        return M::interact(context, normal, light, emission);
    }
};

template <typename M, bool Z=is_empty<M>()>
class Emissive {
// : public Material
public:
    M base;
    float3 intensity;

    Emissive() = default;
    Emissive(float3 v, const M &b) : base(b), intensity(v) {}

    template <typename Context>
    bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) const {
        emission += intensity*light.intensity;
        return base.interact(context, normal, light, emission);
    }
};
template <typename M>
class Emissive<M, true> {
// : public Material
public:
    float3 intensity;

    Emissive() = default;
    Emissive(float3 v) : intensity(v) {}

    template <typename Context>
    bool interact(
        Context &context, real3 normal,
        LocalLight &light, float3 &emission
    ) const {
        emission += intensity*light.intensity;
        return M::interact(context, normal, light, emission);
    }
};


#ifdef HOST

template <typename M>
struct ToDevice<Colored<M, false>> {
    struct type {
        device_type<M> base;
        device_type<float3> color;
    };
    static type to_device(const Colored<M> &mat) {
        return type {
            .base = ::to_device(mat.base),
            .color = ::to_device(mat.color),
        };
    }
};
template <typename M>
struct ToDevice<Colored<M, true>> {
    struct type {
        device_type<float3> color;
    };
    static type to_device(const Colored<M> &mat) {
        return type {
            .color = ::to_device(mat.color),
        };
    }
};
template <typename M>
struct ToDevice<Emissive<M, false>> {
    struct type {
        device_type<M> base;
        device_type<float3> intensity;
    };
    static type to_device(const Emissive<M> &mat) {
        return type {
            .base = ::to_device(mat.base),
            .intensity = ::to_device(mat.intensity),
        };
    }
};
template <typename M>
struct ToDevice<Emissive<M, true>> {
    struct type {
        device_type<float3> intensity;
    };
    static type to_device(const Emissive<M> &mat) {
        return type {
            .intensity = ::to_device(mat.intensity),
        };
    }
};

#endif
