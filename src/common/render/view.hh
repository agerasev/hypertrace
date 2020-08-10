#pragma once

#include <common/geometry/euclidean.hh>
#include <common/geometry/hyperbolic.hh>


struct ViewEu {
    EuMap position;
    real field_of_view;
    real lens_radius;
    real focal_length;
};

ViewEu vieweu_interpolate(ViewEu a, ViewEu b, real t) {
    return ViewEu {
        .position = aff3_interpolate(a.position, b.position, t),
        .field_of_view = a.field_of_view*(R1 - t) + b.field_of_view*t,
        .lens_radius = a.lens_radius*(R1 - t) + b.lens_radius*t,
        .focal_length = a.focal_length*(R1 - t) + b.focal_length*t,
    };
}

struct ViewHy {
    HyMap position;
    real field_of_view;
    real lens_radius;
    real focal_length;
};

ViewHy viewhy_interpolate(ViewHy a, ViewHy b, real t) {
    return ViewHy {
        .position = mo_interpolate(a.position, b.position, t),
        .field_of_view = a.field_of_view*(R1 - t) + b.field_of_view*t,
        .lens_radius = a.lens_radius*(R1 - t) + b.lens_radius*t,
        .focal_length = a.focal_length*(R1 - t) + b.focal_length*t,
    };
}

#ifdef HOST

template <typename G>
struct _ViewTrait;
template <>
struct _ViewTrait<Euclidean> {
    typedef ViewEu T;
    static ViewEu interpolate(ViewEu a, ViewEu b, real t) {
        return vieweu_interpolate(a, b, t);
    }
};
template <>
struct _ViewTrait<Hyperbolic> {
    typedef ViewHy T;
    static ViewHy interpolate(ViewHy a, ViewHy b, real t) {
        return viewhy_interpolate(a, b, t);
    }
};

template <typename G>
using View = typename _ViewTrait<G>::T;

template <typename G>
View<G> interpolate(View<G> a, View<G> b, real t) {
    return _ViewTrait<G>::interpolate(a, b, t);
}

template <typename G>
struct Interop<View<G>> {
    typedef View<G> Host;
    struct Dev {
        dev_type<G::Map> pos;
        dev_type<real> fov;
        dev_type<real> lr;
        dev_type<real> fl;
    };
    static void load(Host *dst, const Dev *src) {
        dev_load(&dst->position, &src->pos);
        dst->field_of_view = src->fov;
        dst->lens_radius = src->lr;
        dst->focal_length = src->fl;
    }
    static void store(Dev *dst, const Host *src) {
        dev_store(&src->pos, &dst->position);
        src->fov = dst->field_of_view;
        src->lr = dst->lens_radius;
        src->fl = dst->focal_length;
    }
};

#endif // HOST
