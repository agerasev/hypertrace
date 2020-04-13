#pragma once

#include <types.h>
#include <algebra/moebius.hpp>


class View {
public:
    Moebius position = Moebius::identity();
    real field_of_view = 1.0_r;
    real lens_radius = 1e-2_r;
    real focal_length = 5.0_r;
};

#ifdef HOST

namespace device {
struct View {
    matrix<complex<real>, 2, 2> pos;
    real fov;
    real lr;
    real fl;
};
}

template <>
struct ToDevice<View> {
    typedef device::View type;
    static type to_device(View v) {
        return type {
            .pos = ::to_device(v.position.mat()),
            .fov = ::to_device(v.field_of_view),
            .lr = ::to_device(v.lens_radius),
            .fl = ::to_device(v.focal_length),
        };
    }
};

#endif
