#pragma once

#include <algebra/differential.hpp>
#include <geometry/euclidean.hpp>
#include <object/shape.hpp>


namespace euclidean {

// Shape defined by Signed Distance Function (SDF)
template <typename F>
class DistanceFunction {
// : public Shape<Eu>
public:
    static constexpr const real EPSILON = 1e-4;

    typedef Eu Geo;
    static const bool repeated = true;

    F function;
    real radius;
    real step = 1_r;
    int max_steps = 1000;
    real eps = 1e-4;

    DistanceFunction() = default;
    explicit DistanceFunction(F f, real r, real s=1_r) :
        function(f), radius(r), step(s) {}

    template <typename Context>
    real detect(Context &context, real3 &normal, Light<Eu> &light) const {
        real3 pos = light.ray.start;
        real3 dir = light.ray.direction;

        real rad2 = radius*radius;
        real len2 = length2(pos);
        if (len2 > rad2) {
            real dlen = -dot(pos, dir);
            real nrad2 = len2 - dlen*dlen;
            if (dlen < 0_r || nrad2 > rad2) {
                return -1_r;
            }
            pos += (dlen - math::sqrt(rad2 - nrad2))*dir;
        }
        
        bool found = false;//true;
        real depth = 0_r;
        real3 npos = pos;
        //real sign = function(pos) > 0_r ? 1_r : -1_r;
        for (int i = 0; i < max_steps; ++i) {
            real3 npos = pos + depth*dir;
            if (length2(npos) > rad2 + eps) {
                //found = false;
                break;
            }
            real dist = function(npos);
            if (dist > eps) {
                depth += dist*step;
            } else {
                found = true;
                break;
            }
        }
        pos = npos;

        if (found) {
            real dist = dot(pos - light.ray.start, dir);
            light.ray.start = pos;
            normal = gradient(function, pos);
            return dist;
        } else {
            return -1_r;
        }
    }
};

}


#ifdef HOST

template <typename F>
struct ToDevice<eu::DistanceFunction<F>> {
    struct type {
        device_type<F> function;
        device::real radius;
        device::real step;
        int max_steps;
        device::real eps;
    };
    static type to_device(const eu::DistanceFunction<F> &df) {
        return type{
            .function = ::to_device(df.function),
            .radius = ::to_device(df.radius),
            .step = ::to_device(df.step),
            .max_steps = ::to_device(df.max_steps),
            .eps = ::to_device(df.eps),
        };
    }
};

#endif
