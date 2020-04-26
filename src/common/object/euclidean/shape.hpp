#pragma once

#include <algebra/vector.hpp>
#include <geometry/euclidean.hpp>
#include <render/light.hpp>


namespace euclidean {

class Sphere {
// : public Shape<Eu>
public:
    typedef Eu Geo;
    static const bool repeated = true;
    Sphere() = default;
    template <typename Context>
    static real detect(Context &context, real3 &normal, Light<Eu> &light) {
        real3 pos = light.ray.start;
        real3 dir = light.ray.direction;
        real b = -dot(dir, pos);
        real c = dot(pos, pos) - 1_r;
        real d = b*b - c;
        if (d < 0_r) {
            return -1_r;
        }
        d = math::sqrt(d);
        real e = b - d;
        if (e < EPS*context.repeat) {
            return -1_r;
        }
        real3 h = pos + dir*e;
        normal = h;
        light.ray.start = h;
        return e;
    }
};

class Cube {
// : public Shape<Eu>
public:
    typedef Eu Geo;
    static const bool repeated = true;
private:
    static real detect_nearest(real3 near, real3 &normal) {
        bool xy = near[0] > near[1];
        bool yz = near[1] > near[2];
        bool xz = near[0] > near[2];
        float dist = 0_r;
        if (xy && xz) {
            dist = near[0];
            normal[0] = 1_r;
        } else if (yz) {
            dist = near[1];
            normal[1] = 1_r;
        } else {
            dist = near[2];
            normal[2] = 1_r;
        }
        return dist;
    }
public:
    Cube() = default;
    template <typename Context>
    static real detect(Context &context, real3 &normal, Light<Eu> &light) {
        const real3 cmax = real3(1_r);
        const real3 cmin = real3(-1_r);

        real3 inv_dir = 1.0f/light.ray.direction;

        real3 vmin = (cmin - light.ray.start)*inv_dir;
        real3 vmax = (cmax - light.ray.start)*inv_dir;

        real3 near = math::min(vmin, vmax);
        real3 far = math::max(vmin, vmax);

        real3 normal_in = real3(0_r);
        float dist_in = detect_nearest(near, normal_in);
        normal_in *= -math::sign(light.ray.direction);

        real3 norm_out = real3(0_r);
        float dist_out = -detect_nearest(-far, norm_out);
        norm_out *= math::sign(light.ray.direction);

        if (dist_in < EPS*context.repeat || dist_in > dist_out) {
            return -1_r;
        }

        light.ray.start += dist_in*light.ray.direction;
        normal = normal_in;
        return dist_in;
    }
};

}


#ifdef HOST

template <>
struct ToDevice<eu::Sphere> {
    struct type {};
    static type to_device(const eu::Sphere &) {
        return type{};
    }
};
template <>
struct ToDevice<eu::Cube> {
    struct type {};
    static type to_device(const eu::Cube &) {
        return type{};
    }
};

#endif
