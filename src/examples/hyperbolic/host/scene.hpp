#pragma once

#include <vector>
#include <algebra/color.hpp>
#include <common.hpp>


std::vector<MyObject> create_scene() {
    float3 border_color = make_color(float3(0.9));

    std::vector<MyObject> objects = {
        make_horosphere(
            Moebius<comp>::identity(),
            MyHorosphere::Tiling::HEXAGONAL,
            MyHorosphereMaterials(
                make_material(make_color(0xfe0000), 0.1, 0.1),
                make_material(make_color(0xffaa01), 0.1, 0.1),
                make_material(make_color(0x35adae), 0.1, 0.1)
            ),
            0.5, 0.02,
            make_material(border_color, 0.0, 0, float3(1.0))
        ),
        make_horosphere(
            Moebius<comp>(1_r, math::sqrt(2_r), 0_r, 1_r)*Hy::yrotate(PI),
            MyHorosphere::Tiling::SQUARE,
            MyHorosphereMaterials(
                make_material(make_color(0xfe7401), 0.1, 0.1),
                make_material(make_color(0xfe0000), 0.1, 0.1),
                make_material(make_color(0xffaa01), 0.1, 0.1),
                make_material(make_color(0xfed601), 0.1, 0.1)
            ),
            0.5, 0.02,
            make_material(border_color, 0.0, 0, float3(1.0))
        ),
        make_plane(
            Moebius<comp>::identity(),
            MyPlane::Tiling::PENTASTAR,
            MyPlaneMaterials(
                make_material(make_color(0xfe7401), 0.1),
                make_material(make_color(0x35adae), 0.1)
            ),
            0.01,
            make_material(border_color, 0.0, 0, float3(1.0))
        ),
        make_plane(
            Moebius<comp>(1_r, 2*1_i, 0_r, 1_r),
            MyPlane::Tiling::PENTAGONAL,
            MyPlaneMaterials(
                make_material(make_color(0xfe0000), 0.1),
                make_material(make_color(0xfed601), 0.1)
            ),
            0.02,
            make_material(border_color, 0.0, 0, float3(1.0))
        ),
    };

    return objects;
}