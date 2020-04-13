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
    MoebiusT<::complex<device::real>> pos;
    device::real fov;
    device::real lr;
    device::real fl;
};
}

template <>
struct ToDevice<View> {
    typedef device::View type;
    static type to_device(View v) {
        return type {
            .pos = MoebiusT<complex<device::real>>(
                convert<matrix<device::real, 2, 2>>(v.position.mat())
            ),
            .fov = v.field_of_view,
            .lr = v.lens_radius,
            .fl = v.focal_length
        };
    }
};

#endif
