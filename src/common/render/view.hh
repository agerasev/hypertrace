#pragma once

#include <common/types.hh>
#include <common/geometry/euclidean.hh>
#include <common/geometry/hyperbolic.hh>


typedef struct ViewEu {
    EuMap position;
    real field_of_view;
    real lens_radius;
    real focal_length;
} ViewEu;

ViewEu vieweu_interpolate(ViewEu a, ViewEu b, real t);

typedef struct ViewHy {
    HyMap position;
    real field_of_view;
    real lens_radius;
    real focal_length;
} ViewHy;

ViewHy viewhy_interpolate(ViewHy a, ViewHy b, real t);


#ifdef HOST

template <typename G>
struct _ViewTrait;
template <>
struct _ViewTrait<Euclidean> {
    typedef ViewEu T;
    inline static ViewEu interpolate(ViewEu a, ViewEu b, real t) {
        return vieweu_interpolate(a, b, t);
    }
};
template <>
struct _ViewTrait<Hyperbolic> {
    typedef ViewHy T;
    inline static ViewHy interpolate(ViewHy a, ViewHy b, real t) {
        return viewhy_interpolate(a, b, t);
    }
};

template <typename G>
using View = typename _ViewTrait<G>::T;

template <typename G>
View<G> interpolate(View<G> a, View<G> b, real t) {
    return _ViewTrait<G>::interpolate(a, b, t);
}

#ifdef INTEROP

template <>
struct Interop<ViewEu> {
    typedef ViewEu Host;
    struct Dev {
        dev_type<EuMap> pos;
        dev_type<real> fov;
        dev_type<real> lr;
        dev_type<real> fl;
    };
    static void load(Host *dst, const Dev *src);
    static void store(Dev *dst, const Host *src);
};

template <>
struct Interop<ViewHy> {
    typedef ViewHy Host;
    struct Dev {
        dev_type<HyMap> pos;
        dev_type<real> fov;
        dev_type<real> lr;
        dev_type<real> fl;
    };
    static void load(Host *dst, const Dev *src);
    static void store(Dev *dst, const Host *src);
};

#endif // INTEROP

#endif // HOST


#ifndef HOST
#include "view.cc"
#endif // !HOST
