#include <iostream>
#include <vector>
#include <string>
#include <cassert>
#include <cstdint>
#include <cmath>
#include <chrono>

#include <algebra/quaternion.hh>
#include <algebra/moebius.hh>
#include <geometry/hyperbolic.hh>
#include <geometry/hyperbolic/plane.hh>
#include <geometry/hyperbolic/horosphere.hh>
#include <view.hh>
#include <object.hh>

#define SDL_MAIN_HANDLED

#include <opencl/search.hpp>
#include <sdl/viewer.hpp>
#include <sdl/controller.hpp>
#include <renderer.hpp>
#include <color.hpp>


using duration = std::chrono::duration<double>;


int main(int argc, const char *argv[]) {
    int platform_no = 0;
    int device_no = 0;
    try {
        if (argc >= 2) {
            platform_no = std::stoi(argv[1]);
            if (argc >= 3) {
                device_no = std::stoi(argv[2]);
            }
        }
    } catch(...) {
        std::cerr << "Invalid argument" << std::endl;
        return 1;
    }

    std::cout << "Using platform " << platform_no << ", device " << device_no << std::endl;
    cl_device_id device = cl::search_device(platform_no, device_no);

    int width = 800, height = 600;
    Renderer renderer(device, width, height);

    color3 border_color = make_color(color3(0.9));
    std::vector<Object> objects = {
        Object{
            .type = OBJECT_HOROSPHERE,
            .map = mo_identity(),
            .horosphere = Horosphere {
                .materials = {
                    Material {make_color(0xfe0000), 0.1, 0.1, float3(0)},
                    Material {make_color(0xffaa01), 0.1, 0.1, float3(0)},
                    Material {make_color(0x35adae), 0.1, 0.1, float3(0)},
                },
                .material_count = 3,
                .tiling = HorosphereTiling {
                    .type = HOROSPHERE_TILING_HEXAGONAL,
                    .cell_size = 0.5,
                    .border = HorosphereTilingBorder {
                        .width = 0.02,
                        .material = Material {border_color, 0.0, 0, float3(1.0)},
                    },
                },
            },
        },
        Object{
            .type = OBJECT_HOROSPHERE,
            .map = mo_chain(mo_new(C1, sqrt(2)*C1, C0, C1), hy_yrotate(PI)),
            .horosphere = Horosphere {
                .materials = {
                    Material {make_color(0xfe7401), 0.1, 0.1, float3(0)},
                    Material {make_color(0xfe0000), 0.1, 0.1, float3(0)},
                    Material {make_color(0xffaa01), 0.1, 0.1, float3(0)},
                    Material {make_color(0xfed601), 0.1, 0.1, float3(0)},
                },
                .material_count = 4,
                .tiling = HorosphereTiling {
                    .type = HOROSPHERE_TILING_SQUARE,
                    .cell_size = 0.5,
                    .border = HorosphereTilingBorder {
                        .width = 0.02,
                        .material = Material {border_color, 0.0, 0, float3(1.0)},
                    },
                },
            },
        },
        Object{
            .type = OBJECT_PLANE,
            .map = mo_identity(),
            .plane = HyPlane {
                .materials = {
                    Material {make_color(0xfe7401), 0.1, 0.0, float3(0)},
                    Material {make_color(0x35adae), 0.1, 0.0, float3(0)},
                },
                .material_count = 2,
                .tiling = HyPlaneTiling {
                    .type = HYPLANE_TILING_PENTASTAR,
                    .border = HyPlaneTilingBorder {
                        .width = 0.01,
                        .material = Material {border_color, 0.0, 0, float3(1.0)},
                    },
                },
            },
        },
        Object{
            .type = OBJECT_PLANE,
            .map = mo_new(C1, 2*CI, C0, C1),
            .plane = HyPlane {
                .materials = {
                    Material {make_color(0xfe0000), 0.1, 0.0, float3(0)},
                    Material {make_color(0xfed601), 0.1, 0.0, float3(0)},
                },
                .material_count = 2,
                .tiling = HyPlaneTiling {
                    .type = HYPLANE_TILING_PENTAGONAL,
                    .border = HyPlaneTilingBorder {
                        .width = 0.02,
                        .material = Material {border_color, 0.0, 0, float3(1.0)},
                    },
                },
            },
        },
    };
    renderer.store_objects(objects);

    Viewer viewer(width, height);
    Controller controller;
    controller.grab_mouse(true);

    controller.view.position = mo_new(
        c_new(0.114543, 0.285363),
        c_new(2.9287, -0.678274),
        c_new(-0.0461927, -0.0460196),
        c_new(0.697521, -2.64087)
    );
    //controller.view.lens_radius = 1e-1;

    duration time_counter;
    int sample_counter = 0;
    int refresh = 1;
    for(;;) {
        duration elapsed;
        auto start = std::chrono::system_clock::now();

        renderer.set_view(controller.view, controller.view_prev);
        sample_counter += renderer.render_for(0.04, refresh > 0);

        viewer.display([&](uint8_t *data) {
            renderer.load_image(data);
        });
        if (!controller.handle()) {
            break;
        }

        elapsed = std::chrono::system_clock::now() - start;
        if (controller.step(elapsed.count())) {
            refresh = 2;
        } else if (refresh > 0) {
            refresh -= 1;
        }
        
        time_counter += elapsed;
        if (time_counter.count() > 1.0) {
            std::cout << "Current position: " <<
                controller.view.position << std::endl;

            std::cout << "Samples per second: " <<
                sample_counter/time_counter.count() << std::endl;
            time_counter = duration(0.0);
            sample_counter = 0;
        }
    }

    return 0;
}

