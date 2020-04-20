#pragma once

#include <traits.hpp>
#include <algebra/complex.hpp>
#include <geometry/hyperbolic.hpp>
#include <object/shape.hpp>
#include <render/light.hpp>


namespace hyperbolic {

class Horosphere : public SurfaceShape<Hy> {
public:
    static const bool repeated = true;

public:
    template <typename Context>
    real detect(Context &context, quat &normal, Light<Hy> &light) const {
        quat p = light.ray.start, d = light.ray.direction;
        real dxy = length(d.re());
        // FIXME: check (dxy < EPS)

        if (p[2] < dxy) {
            return -1_r;
        }
        
        real dt = math::sqrt(p[2]*p[2] - dxy*dxy);
        real t = p[2]*d[2] - dt;
        if (t < 0_r + context.repeat*EPS) {
            t += 2*dt;
        }
        if (t < 0_r + context.repeat*EPS) {
            return -1_r;
        }

        t /= dxy*dxy;
        quat h(p.re() + d.re()*t, 1, 0);

        light.ray.start = h;
        light.ray.direction = Hy::dir_at(p, d, h);
        normal = quat(0, 0, 1, 0);

        return Hy::distance(p, h);
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
