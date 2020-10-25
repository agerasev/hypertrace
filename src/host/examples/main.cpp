#include <iostream>
#include <vector>
#include <string>
#include <cassert>
#include <cstdint>
#include <cmath>
#include <chrono>

#include <algebra/quaternion.hh>
#include <algebra/moebius.hh>
#include <view.hh>
#include <object.hh>

#include <opencl/search.hpp>
#include <sdl/viewer.hpp>
#include <sdl/controller.hpp>
#include <renderer.hpp>
#include <color.hpp>
#include <sdl/image.hpp>

#include "scene.hpp"

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

    int width = 1600, height = 400;
    Renderer renderer(device, width, height, Renderer::Config {
        .path_max_depth = 3,
        .blur = { .lens = true, .motion = true }
    });
    renderer.store_objects(create_scene());

    Viewer viewer(1600, 400);
    Controller controller;
    controller.grab_mouse(true);

    controller.view.position = mo_new(
        c_new(0.110226, 0.314233),
        c_new(2.6954, -0.721198),
        c_new(-0.0271873, -0.0361756),
        c_new(0.674479, -2.62953)
    );
    controller.view.lens_radius = 1e-2;
    controller.view.focal_length = 2.0;

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

    sdl::save_image("output/image.png", width, height, [&](uint8_t *data) {
        renderer.load_image(data);
    });

    return 0;
}
