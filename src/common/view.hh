#pragma once

#include <algebra/real.hh>
#include <algebra/moebius.hh>


typedef struct {
    Moebius position;
    Moebius motion;
    real field_of_view;
    real lens_radius;
    real focal_length;
} View;

#ifdef OPENCL_INTEROP

typedef struct __attribute__((packed)) {
    MoebiusPk position;
    MoebiusPk motion;
    real_pk field_of_view;
    real_pk lens_radius;
    real_pk focal_length;
} ViewPk;

#endif // OPENCL_INTEROP


void init_view(View *view);

#ifdef OPENCL_INTEROP

void pack_view(ViewPk *dst, const View *src);
void unpack_view(View *dst, const ViewPk *src);

#endif // OPENCL_INTEROP
