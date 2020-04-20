#pragma once

#include <algebra/vector.hpp>
#include <geometry/ray.hpp>
#include <geometry/hyperbolic.hpp>

struct BaseLight {
    float3 intensity;
    bool diffuse = false;

    BaseLight() = default;
    BaseLight(float3 col, bool diff=false) :
        intensity(col), diffuse(diff)
    {}

    const BaseLight &base() const {
        return *this;
    }
    BaseLight &base() {
        return *this;
    }
};

struct LocalLight : public BaseLight {
    real3 direction;

    LocalLight() = default;
    LocalLight(real3 dir, const BaseLight &base) :
        BaseLight(base), direction(dir)
    {}
};

template <typename G>
struct Light : public BaseLight {
    Ray<G> ray;

    Light() = default;
    Light(Ray<G> ray, const BaseLight &base) :
        BaseLight(base), ray(ray)
    {}
};

template <>
struct Light<Hyperbolic> : public BaseLight {
    Ray<Hy> ray;

    Light() = default;
    Light(Ray<Hy> ray, const BaseLight &base) :
        BaseLight(base), ray(ray)
    {}

    LocalLight get_local() const {
        return LocalLight(
            ray.direction.vec().shuffle(0,1,2),
            base()
        );
    }
    void set_local(const LocalLight &ll) {
        ray.direction = quat(ll.direction, 0);
        base() = ll.base();
    }
};
