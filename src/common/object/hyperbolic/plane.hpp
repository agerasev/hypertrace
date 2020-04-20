#include <traits.hpp>
#include <algebra/complex.hpp>
#include <geometry/hyperbolic.hpp>
#include <object/shape.hpp>
#include <object/object.hpp>
#include <object/material.hpp>
#include <render/light.hpp>


namespace hyperbolic {

class Plane : public SurfaceShape<Hy> {
public:
    template <typename Context>
    real detect(Context &context, quat &normal, Light<Hy> &light) const {
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
        normal = h;

        return Hy::distance(p, h);
    }
};


struct PlaneTiling {
    enum type : uint {
        NONE = 0,
        PENTAGONAL = 1,
        PENTASTAR = 2,
    };
};

template <typename Mat, int N>
class TiledPlane : public Object<Hy> {
public:
    struct Cache : public Object<Hy>::Cache {
        quat normal;
    };

    typedef PlaneTiling::type Tiling;

    Plane surface;

    Mat materials[N];
    Mat border_material;

    Tiling tiling;
    real border_width;

public:
    TiledPlane() {}
    template <typename ...Args>
    TiledPlane(
        Tiling ti,
        real bw,
        const Mat &bm,
        Args ...args
    ) :
        materials{args...},
        border_material(bm),
        tiling(ti),
        border_width(bw)
    {}
    
    template <typename Context>
    real detect(Context &context, Cache &cache, Light<Hy> &light) const {
        return surface.detect(context, cache.normal, light);
    }

    template <typename Context>
    bool interact(
        Context &context, const Cache &cache,
        Light<Hy> &light, float3 &luminance
    ) const {
        const Mat *material;

        if (tiling == Tiling::PENTAGONAL || tiling == Tiling::PENTASTAR) {
            quat p = light.ray.start;

            bool w = false;
            //uint n = 0, b = 1;
            real Q = math::sqrt(math::cos(PI/4 + PI/5)/math::cos(PI/4 - PI/5));
            real T = math::sqrt(math::cos(PI/4 + PI/5)*math::cos(PI/4 - PI/5));
            real S = (math::cos(PI/4) - math::sin(PI/5))/T;
            real L = T/math::cos(PI/4);
            real K = L*(2*math::cos(PI/5) - 1/math::cos(PI/5));
            Q = math::log((1 + Q)/(1 - Q));
            S = math::log((1 + S)/(1 - S));

            if (p[0] < 0_r) {
                p[0] = -p[0];
                //n |= 1;
                w = !w;
            }
            if (p[1] < 0_r) {
                p[1] = -p[1];
                //n |= 2;
                w = !w;
            }
            //b *= 4;

            p = (Hy::xshift(-Q)*Hy::zrotate(-PI/4)).apply(p);
            bool e = false;
            for (int j = 0; j < 5; ++j) {
                bool a[3] = {false, false, false};
                for (int i = 0; i < 3 - e; ++i) {
                    real o = 2*PI*(i - 1)/5;
                    real2 d = real2(math::cos(o), math::sin(o));
                    a[i] = (dot(d, p.re().vec()) < L);
                }
                a[2] = a[2] || e;
                int s = (int)a[0] + (int)a[1] + (int)a[2];
                //uint bb = b*(6 - 2*e);
                if (s == 3) {
                    break;
                } else if (s == 2) {
                    int i = (!a[1]) + 2*(!a[2]);
                    real o = 2*PI*(i - 1)/5;
                    p = (Hy::zrotate(-PI/5)*Hy::xshift(-2*S)*Hy::zrotate(-o)).apply(p);
                    //n += b*(2*i + 1);
                    e = true;
                    w = !w;
                } else {
                    int i = a[0];
                    real o = PI*(2*i - 1)/5;
                    p = (Hy::xshift(-2*Q)*Hy::zrotate(-o)).apply(p);
                    //n += b*(2*(i + 1));
                    e = false;
                }
                //b = bb;
            }
            
            const real br = border_width;
            bool bh = 0;
            for (int i = 0; i < 5; ++i) {
                real o = 2*PI*i/5;
                real2 d = real2(math::cos(o), math::sin(o));
                bh = bh || (dot(d, p.re().vec()) > (L - br*p[2]));
                if (tiling == Tiling::PENTASTAR) {
                    real ps = K + dot(d, p.re().vec());
                    w = w^(ps < 0);
                    bh = bh || (math::abs(ps) < br*p[2]);
                }
            }
            if (bh == 0) {
                if (w || N < 2) {
                    material = &materials[0];
                } else {
                    material = &materials[1];
                }
            } else {
                material = &border_material;
            }
        } else {
            material = &materials[0];
        }
        
        real3 ln = Hy::dir_to_local(light.ray.start, cache.normal);
        LocalLight ll = light.get_local();
        bool bounce = material->interact(context, ln, ll, luminance);
        if (bounce) {
            light.set_local(ll);
        }
        return bounce;
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

template <typename Mat, int N>
struct ToDevice<hy::TiledPlane<Mat, N>> {
    struct type {
        device_type<hy::Plane> surface;
        device_type<Mat> materials[N];
        device_type<Mat> border_material;
        uint tiling;
        device::real border_width;
    };
    static type to_device(const hy::TiledPlane<Mat, N> &tp) {
        type out{
            .surface = ::to_device(tp.surface),
            .border_material = ::to_device(tp.border_material),
            .tiling = uint(tp.tiling),
            .border_width = device::real(tp.border_width),
        };
        for (int i = 0; i < N; ++i) {
            out.materials[i] = ::to_device(tp.materials[i]);
        }
        return out;
    }
};

#endif
