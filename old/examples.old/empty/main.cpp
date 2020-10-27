#include <iostream>
#include <vector>
#include <string>
#include <cassert>
#include <cstdint>
#include <cmath>
#include <chrono>

#include <common/geometry/hyperbolic.hh>
#include <common/render/view.hh>

#include <host/opencl/search.hpp>
#include <host/sdl/viewer.hpp>
#include <host/sdl/controller.hpp>
#include <host/renderer.hpp>


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

    Renderer renderer(device, width, height, Renderer::Config {
        .path_max_depth = 3,
        .blur = { .lens = true, .motion = true }
    });
    Renderer<MyObject> renderer(
        device, src,
        width, height
    );

    std::vector<MyObject> objects = create_scene();
    renderer.store_objects(objects);

    Viewer viewer(width, height);
    Controller<Hyperbolic> controller;
    controller.grab_mouse(true);

    controller.view.position = Moebius<comp>(
        comp(0.114543, 0.285363),
        comp(2.9287, -0.678274),
        comp(-0.0461927, -0.0460196),
        comp(0.697521, -2.64087)
    );
    //controller.view.lens_radius = 1e-1;

    duration time_counter(0);
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
            //std::cout << "Current position: " <<
            //    controller.view.position << std::endl;

            std::cout << "Samples per second: " <<
                sample_counter/time_counter.count() << std::endl;
            time_counter = duration(0.0);
            sample_counter = 0;
        }
    }

    return 0;
}
