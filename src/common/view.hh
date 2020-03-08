#pragma once

#include <types.hh>

#include <algebra/real.hh>
#include <algebra/moebius.hh>


typedef struct {
    Moebius position;
    real field_of_view;
    real lens_radius;
    real focal_length;
} View;

#ifdef OPENCL_INTEROP

typedef struct _PACKED_STRUCT_ATTRIBUTE_ {
    MoebiusPk position;
    real_pk field_of_view;
    real_pk lens_radius;
    real_pk focal_length;
} ViewPk;

#endif // OPENCL_INTEROP


View view_init();
View view_position(Moebius m);
View view_interpolate(View a, View b, real t);

#ifdef OPENCL_INTEROP

ViewPk view_pack(View v);
View view_unpack(ViewPk v);

#endif // OPENCL_INTEROP
