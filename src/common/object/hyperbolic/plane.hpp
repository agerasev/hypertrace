#include <traits.hpp>
#include <algebra/complex.hpp>
#include <geometry/hyperbolic.hpp>
#include <object.hpp>
#include <render/light.hpp>


namespace hyperbolic {
class Plane : public Object<Hy> {
public:
    typedef Object<Hy>::Cache Cache;

public:
    template <typename Context>
    real detect(
        Context &context, Cache &cache,
        Light<Hy> &light
    ) const {
        // Line cannot intersect plane twice
        if (context.repeat) {
            return -1_r;
        }

        quat p = light.ray.start, d = light.ray.direction;
        //real dxy = sqrt(d.x*d.x + d.y*d.y);
        // FIXME: check (dxy < EPS)

        real pd = dot(p, d);
        if (math::abs(pd) < EPS) {
            return -1_r;
        }
        real t = (1_r - length2(p))/(2*pd);
        if (t < 0_r) {
            return -1_r;
        }
        quat h(p.re() + d.re()*t, 0, 0);
        
        real pxy2 = length2(h.re());
        if (pxy2 > 1_r) {
            return -1_r;
        }
        h[2] = math::sqrt(1 - pxy2);

        light.ray.start = h;
        light.ray.direction = Hy::dir_at(p, d, h);

        return Hy::distance(p, h);
    }

    template <typename Context>
    bool interact(
        Context &context, const Cache &cache,
        Light<Hy> &light, float3 &luminance
    ) const {
        luminance += float3(1.0f, 0.75f, 0.5f);
        return false;
    }
};

}

#ifdef HOST

template <>
struct ToDevice<hy::Plane> {
    typedef struct{} type;
    static type to_device(const hy::Plane &) {
        return type{};
    }
};

#endif
