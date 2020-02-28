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

#include <opencl/crawler.hpp>
#include <sdl/viewer.hpp>
#include <sdl/controller.hpp>
#include <renderer.hpp>
#include <scenario.hpp>


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

    std::vector<Object> objects = {
        Object{
            .type = OBJECT_HOROSPHERE,
            .map = mo_identity(),
            .horosphere = Horosphere{
                .tiling = HOROSPHERE_TILING_HEXAGONAL,
                .material = Material {float3(1.0, 0.5, 0.25), 0.1},
                .size = 0.25, .border = 0.02,
            },
        },
        Object{
            .type = OBJECT_HOROSPHERE,
            .map = mo_chain(mo_new(C1, sqrt(2)*C1, C0, C1), hy_yrotate(M_PI)),
            .horosphere = Horosphere{
                .tiling = HOROSPHERE_TILING_SQUARE,
                .material = Material {float3(0.25, 0.5, 1.0), 0.1},
                .size = 0.25, .border = 0.02,
            },
        },
        Object{
            .type = OBJECT_PLANE,
            .map = mo_identity(),
            .plane = HyPlane{
                .tiling = HYPLANE_TILING_NONE,
                .material = Material {float3(0.5, 1.0, 0.25), 0.1},
                .border = 0.02,
            },
        },
        Object{
            .type = OBJECT_PLANE,
            .map = mo_new(C1, 2*CI, C0, C1),
            .plane = HyPlane{
                .tiling = HYPLANE_TILING_PENTAGONAL,
                .material = Material {float3(1.0, 1.0, 0.25), 0.1},
                .border = 0.02,
            },
        },
    };
    renderer.store_objects(objects);

    Viewer viewer(width, height);
    Controller controller;

    double tdelay = 4.0;
    double pfl = 4.0;
    Scenario scenario(
        Scenario::Point {
            .position = mo_new(
                c_new(0.114543, 0.285363),
                c_new(2.9287, -0.678274),
                c_new(-0.0461927, -0.0460196),
                c_new(0.697521, -2.64087)
            ),
            .focal_length = pfl,
        },
        Scenario::Transition {
            .duration = tdelay,
        },
        Scenario::Point {
            .position = mo_new(
                c_new(0.0298572, 0.116725),
                c_new(2.99292, -1.557),
                c_new(-0.178148, -0.0703524),
                c_new(1.27215, -2.73547)
            ),
            .focal_length = pfl,
        },
        Scenario::Transition {
            .duration = tdelay,
        },
        Scenario::Point {
            mo_new(
                c_new(-0.209295, 0.379613),
                c_new(3.46353, -3.29636),
                c_new(-0.231689, 0.2415),
                c_new(2.12595, -3.78954)
            ),
            .focal_length = pfl,
        },
        Scenario::Transition {
            .duration = tdelay,
        },
        Scenario::Point {
            mo_new(
                c_new(0.63954, 0.498772),
                c_new(2.68254, -3.25001),
                c_new(0.413499, 0.355539),
                c_new(2.87836, -2.85482)
            ),
            .focal_length = pfl,
        },
        Scenario::Transition {
            .duration = tdelay,
        },
        Scenario::Point {
            mo_new(
                c_new(0.495165, -0.000707206),
                c_new(1.05431, -2.55384),
                c_new(0.229223, -0.103703),
                c_new(1.97474, -1.40022)
            ),
            .focal_length = pfl,
        },
        Scenario::Transition {
            .duration = tdelay,
        },
        Scenario::Point {
            mo_new(
                c_new(0.277396, -0.0671577),
                c_new(-0.79975, -2.37376),
                c_new(0.0404912, -0.170447),
                c_new(1.88414, 0.601065)
            ),
            .focal_length = pfl,
        },
        Scenario::Transition {
            .duration = tdelay,
        },
        Scenario::Point {
            mo_new(
                c_new(0.418802, -0.237497),
                c_new(-1.87922, -1.26276),
                c_new(-0.0360326, -0.245777),
                c_new(0.848506, 1.69266)
            ),
            .focal_length = pfl,
        },
        Scenario::Transition {
            .duration = tdelay,
        },
        Scenario::Point {
            mo_new(
                c_new(0.862866, -0.339343),
                c_new(-1.01238, -0.366878),
                c_new(0.207137, -0.72253),
                c_new(0.26842, 0.865222)
            ),
            .focal_length = pfl,
        },
        Scenario::Transition {
            .duration = tdelay,
        },
        Scenario::Point {
            mo_new(
                c_new(-0.134283, -0.25151),
                c_new(-3.06023, 0.93036),
                c_new(0.0554463, 0.0817881),
                c_new(-0.631135, 2.66185)
            ),
            .focal_length = pfl,
        }
    );

    //controller.view.lens_radius = 1e-1;

    double time = 0.0;
    double frame_time = 0.04;
    duration time_counter;
    int sample_counter = 0;
    for(;;) {
        auto start = std::chrono::system_clock::now();

        sample_counter += renderer.render_for(scenario.get_view(time, 0.0), frame_time, true);
        time += frame_time;

        viewer.display([&](uint8_t *data) {
            renderer.load_image(data);
        });
        if (!controller.handle() || time > scenario.duration()) {
            break;
        }

        time_counter += std::chrono::system_clock::now() - start;
        if (time_counter.count() > 1.0) {
            std::cout << "Elapsed: " << time << " / " << scenario.duration() << std::endl;
            std::cout << "Samples per second: " <<
                sample_counter/time_counter.count() << std::endl;
            time_counter = duration(0.0);
            sample_counter = 0;
        }
    }

    return 0;
}

