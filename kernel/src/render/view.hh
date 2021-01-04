#pragma once

#include <types.hh>
#include <geometry/euclidean.hh>
#include <geometry/hyperbolic.hh>


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
View<G> view_interpolate(View<G> a, View<G> b, real t) {
    return _ViewTrait<G>::interpolate(a, b, t);
}

#ifdef INTEROP

template <typename G>
struct _ViewInterop {
    typedef View<G> Host;
    struct Dev {
        dev_type<typename G::Map> pos;
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
        dev_store(&dst->pos, &src->position);
        dst->fov = src->field_of_view;
        dst->lr = src->lens_radius;
        dst->fl = src->focal_length;
    }
};

template <>
struct Interop<ViewEu> : public _ViewInterop<Eu> {};
template <>
struct Interop<ViewHy> : public _ViewInterop<Hy> {};

#endif // INTEROP

#endif // HOST


#ifndef HOST
#include "view.cc"
#endif // !HOST
