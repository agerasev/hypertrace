#pragma once

#include <traits.hpp>
#include <algebra/complex.hpp>
#include <geometry/hyperbolic.hpp>
#include <object.hpp>
#include <render/light.hpp>


namespace hyperbolic {
class Horosphere : public Object<Hy> {
public:
    typedef Object<Hy>::Cache Cache;
    static const bool repeated = true;

public:
    template <typename Context>
    real detect(
        const Context &context, Cache &cache,
        Light<Hy> &light, bool repeat=false
    ) const {
        quat p = light.ray.start, d = light.ray.direction;
        real dxy = length(d.re());
        // FIXME: check (dxy < EPS)

        if (p[2] < dxy) {
            return -1_r;
        }
        
        real dt = math::sqrt(p[2]*p[2] - dxy*dxy);
        real t = p[2]*d[2] - dt;
        if (t < 0_r + repeat*EPS) {
            t += 2*dt;
        }
        if (t < 0_r + repeat*EPS) {
            return -1_r;
        }

        t /= dxy*dxy;
        quat h(p.re() + d.re()*t, 1, 0);

        light.ray.start = h;
        light.ray.direction = Hy::dir_at(p, d, h);

        return Hy::distance(p, h);
    }

    template <typename Context>
    bool interact(
        const Context &context, const Cache &cache,
        Light<Hy> &light, float3 &luminance
    ) const {
        real2 fr = math::fract(light.ray.start.re().vec()).first;
        if (fr[0] > 0.1_r && fr[1] > 0.1_r) {
            luminance += float3(0.5f, 0.5f, 1.0f);
        } else {
            luminance += float3(0, 0, 1);
        }
        return false;
    }
};

}

#ifdef HOST

template <>
struct ToDevice<hy::Horosphere> {
    typedef struct{} type;
    static type to_device(const hy::Horosphere &) {
        return type{};
    }
};

#endif
