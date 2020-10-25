#pragma once

#include <vector>

#include <object.hh>
#include <geometry/hyperbolic/horosphere.hh>
#include <geometry/hyperbolic/plane.hh>

#include <color.hpp>


std::vector<Object> create_scene() {
    Material border_material = Material {make_color(color3(0.9)), 0.1, 0, float3(0.0)};
    std::vector<Object> objects = {
        Object{
            .type = OBJECT_HOROSPHERE,
            .map = mo_identity(),
            .materials = {
                Material {make_color(0xfe0000), 0.13, 0.25, float3(0)},
                Material {make_color(0xffaa01), 0.13, 0.25, float3(0)},
                Material {make_color(0x35adae), 0.13, 0.25, float3(0)},
            },
            .material_count = 3,
            .tiling = {
                .type = HOROSPHERE_TILING_HEXAGONAL,
                .cell_size = 0.5,
                .border_width = 0.03,
                .border_material = border_material,
            },
        },
        Object{
            .type = OBJECT_HOROSPHERE,
            .map = mo_chain(mo_new(C1, sqrt(2)*C1, C0, C1), hy_yrotate(PI)),
            .materials = {
                Material {make_color(0xfe7401), 0.13, 0.25, float3(0)},
                Material {make_color(0xfe0000), 0.13, 0.25, float3(0)},
                Material {make_color(0xfed601), 0.13, 0.25, float3(0)},
                Material {make_color(0xffaa01), 0.13, 0.25, float3(0)},
            },
            .material_count = 4,
            .tiling = {
                .type = HOROSPHERE_TILING_SQUARE,
                .cell_size = 0.5,
                .border_width = 0.03,
                .border_material = border_material,
            },
        },
        Object{
            .type = OBJECT_HYPLANE,
            .map = mo_identity(),
            .materials = {
                Material {make_color(0xfe7401), 0.1, 0.0, float3(0)},
                Material {make_color(0x35adae), 0.1, 0.0, float3(0)},
            },
            .material_count = 2,
            .tiling = {
                .type = HYPLANE_TILING_PENTASTAR,
                .border_width = 0.02,
                .border_material = border_material,
            },
        },
        Object{
            .type = OBJECT_HYPLANE,
            .map = mo_new(C1, 2*CI, C0, C1),
            .materials = {
                Material {make_color(0xfe0000), 0.1, 0.0, float3(0)},
                Material {make_color(0xfed601), 0.1, 0.0, float3(0)},
            },
            .material_count = 2,
            .tiling = {
                .type = HYPLANE_TILING_PENTAGONAL,
                .border_width = 0.04,
                .border_material = border_material,
            },
        },
    };

    return objects;
}