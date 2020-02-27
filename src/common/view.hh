#pragma once

#include <algebra/real.hh>
#include <algebra/moebius.hh>


typedef struct {
    Moebius position;
    Moebius motion;
    real fov;
} View;

#ifdef OPENCL_INTEROP

typedef struct __attribute__((packed)) {
    MoebiusPk position;
    MoebiusPk motion;
    real_pk fov;
} ViewPk;

#endif // OPENCL_INTEROP


void init_view(View *view);

#ifdef OPENCL_INTEROP

void pack_view(ViewPk *dst, const View *src);
void unpack_view(View *dst, const ViewPk *src);

#endif // OPENCL_INTEROP
