#pragma once

#include <common/algebra/quaternion.hh>
#include <common/geometry/hyperbolic.hh>
#include <common/render/light.hh>
#include <common/render/context.hh>


real horosphere_detect(__global const void *shape, Context *context, HyDir *normal, LightHy *light);


#ifdef HOST

#include <common/object/shape.hpp>

class Horosphere final : public dyn::ImplEmptyType<Horosphere, SurfaceShape<Hy>> {
public:
    inline virtual bool repeat_allowed() const override { return true; }
    inline virtual std::string prefix() const override { return "horosphere"; };
    inline virtual dyn::Source source() const override { return dyn::Source("common/object/hyperbolic/horosphere.hh"); };
};

#endif // HOST


/*
namespace hyperbolic {

class Horosphere {
// : public SurfaceShape<Hy>
public:
    typedef Hy Geo;
    static const bool repeated = true;
};

} // namespace hyperbolic

template <> struct IsEmpty<hy::Horosphere> { static const bool value = true; };

namespace hyperbolic {

struct HorosphereTiling {
    enum type : uint {
        NONE = 0,
        SQUARE = 1,
        HEXAGONAL = 2,
    };
};

template <typename Mat, int N>
class TiledHorosphere {
// : public Object<Hy>
public:
    struct Cache {
    // : public Object<Hy>::Cache
        quat normal;
    };
    typedef Hy Geo;
    static const bool repeated = true;

    typedef HorosphereTiling::type Tiling;

    static_assert(is_empty<Horosphere>(), "Surface is not empty");
    //Horosphere surface;

    Array<Mat, N> materials;
    Mat border_material;

    Tiling tiling;
    real cell_size;
    real border_width;

public:
    TiledHorosphere() {}
    TiledHorosphere(
        Tiling ti,
        const Array<Mat, N> &ms,
        real cs,
        const Mat &bm,
        real bw
    ) :
        materials(ms),
        border_material(bm),
        tiling(ti),
        cell_size(cs),
        border_width(bw)
    {}
    
    template <typename Context>
    real detect(Context &context, Cache &cache, Light<Hy> &light) const {
        return Horosphere::detect(context, cache.normal, light);
    }

    template <typename Context>
    bool interact(
        Context &context, const Cache &cache,
        Light<Hy> &light, float3 &luminance
    ) const {
        int material_no = 0;
        bool border = false;

        real br = border_width/cell_size;
        real2 g = light.ray.start.re().vec();
        if (tiling == Tiling::SQUARE) {
            auto fp = math::fract(g/cell_size);
            real2 f = fp.first, k = fp.second;

            if (f[0] < br || f[0] > 1 - br || f[1] < br || f[1] > 1 - br) {
                border = true;
            } else {
                int hx = math::rem((int)k[0], 2);
                int hy = math::rem((int)k[1], 2);
                material_no = 3*hy - 2*hx*hy + hx;
            }
        } else if (tiling == Tiling::HEXAGONAL) {
            real2 bx = real2(2/math::sqrt(3_r), 0);
            real2 by = real2(-1/math::sqrt(3_r), 1);
            real s = cell_size*math::sqrt(3_r)/2;
            real2 h = real2(dot(bx, g), dot(by, g))/s;
            int hx = (int)math::floor((math::floor(h[0]) - math::floor(h[1]))/3);
            int hy = (int)math::floor((math::floor(h[0] + h[1]) - hx)/2);

            h -= hx*real2(2, -1) + hy*real2(1, 1);
            if (
                math::abs(h[0] - 1) > 1 - br ||
                math::abs(h[1]) > 1 - br ||
                math::abs(h[0] + h[1] - 1) > 1 - br
            ) {
                border = true;
            } else {
                material_no = 2*hx + hy;
            }
        } else {
            material_no = 0;
        }

        const Mat *material;
        if (border) {
            material = &border_material;
        } else {
            material = &materials[math::rem(material_no, materials.size())];
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

} // namespace hyperbolic

#ifdef HOST

template <>
struct ToDevice<hy::Horosphere> {
    typedef struct{} type;
    static type to_device(const hy::Horosphere &) {
        return type{};
    }
};

template <typename Mat, int N>
struct ToDevice<hy::TiledHorosphere<Mat, N>> {
    struct type {
        //device_type<hy::Horosphere> surface;
        Array<device_type<Mat>, N> materials;
        device_type<Mat> border_material;
        uint tiling;
        device::real cell_size;
        device::real border_width;
    };
    static type to_device(const hy::TiledHorosphere<Mat, N> &th) {
        type out{
            //.surface = ::to_device(th.surface),
            .border_material = ::to_device(th.border_material),
            .tiling = uint(th.tiling),
            .cell_size = device::real(th.cell_size),
            .border_width = device::real(th.border_width),
        };
        out.materials.size() = th.materials.size();
        for (int i = 0; i < N; ++i) {
            out.materials[i] = ::to_device(th.materials[i]);
        }
        return out;
    }
};

#endif
*/

#ifndef HOST
#include "horosphere.cc"
#endif // !HOST
