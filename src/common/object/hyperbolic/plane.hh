#pragma once

#include <common/algebra/quaternion.hh>
#include <common/geometry/hyperbolic.hh>
#include <common/render/light.hh>

#include <generated/trace_context.hh>

//#include <container/array.hpp>
//#include <object/shape.hpp>
//#include <object/object.hpp>
//#include <object/material.hpp>


real plane_hy_detect(TraceContext *context, HyDir *normal, Light<Hy> *light);

struct PlaneHyCache {
    HyDir normal;
};

#define tiled_plane_hy_detect plane_hy_detect

bool tiled_plane_hy_interact(
    TraceContext *context, const Cache *cache,
    LightHy *light, color3 *luminance
);

#define TILED_PLANE_HY_TILING_NONE        0
#define TILED_PLANE_HY_TILING_PENTAGONAL  1
#define TILED_PLANE_HY_TILING_PENTASTAR   2


#ifdef HOST

/*
class PlaneHy : public SurfaceShape<Hy> {
public:
    typedef Hy Geo;
    static const bool repeated = false;
};

template <> struct IsEmpty<PlaneHy> { static const bool value = true; };

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
    struct Cache : public Object<Hy>::Cache
        quat normal;
    };
    typedef Hy Geo;
    static const bool repeated = false;

    typedef PlaneTiling::type Tiling;

    static_assert(is_empty<Plane>(), "Surface is not empty");
    //Plane surface;

    Array<Mat, N> materials;
    Mat border_material;

    Tiling tiling;
    real border_width;

public:
    TiledPlane() {}
    TiledPlane(
        Tiling ti,
        const Array<Mat, N> &ms,
        const Mat &bm,
        real bw
    ) :
        materials(ms),
        border_material(bm),
        tiling(ti),
        border_width(bw)
    {}
};


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
        //device_type<hy::Plane> surface;
        Array<device_type<Mat>, N> materials;
        device_type<Mat> border_material;
        uint tiling;
        device::real border_width;
    };
    static type to_device(const hy::TiledPlane<Mat, N> &tp) {
        type out{
            //.surface = ::to_device(tp.surface),
            .border_material = ::to_device(tp.border_material),
            .tiling = uint(tp.tiling),
            .border_width = device::real(tp.border_width),
        };
        out.materials.size() = tp.materials.size();
        for (int i = 0; i < N; ++i) {
            out.materials[i] = ::to_device(tp.materials[i]);
        }
        return out;
    }
};
*/

#endif // HOST
