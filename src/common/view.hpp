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
