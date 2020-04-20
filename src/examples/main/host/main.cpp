#include <iostream>
#include <vector>
#include <string>
#include <cassert>
#include <cstdint>
#include <cmath>
#include <chrono>

#include <geometry/hyperbolic.hpp>
#include <view.hpp>
//#include <object.hh>

#include <opencl/search.hpp>
#include <sdl/viewer.hpp>
#include <sdl/controller.hpp>
#include <renderer.hpp>

#include <myobject.hpp>

//#include "scene.hpp"

#include <device/kernel.gen.cl.h>


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
    std::string src(
        (const char *)kernel_gen_cl,
        size_t(kernel_gen_cl_len)
    );
    Viewer viewer(width, height);
    Renderer<Hyperbolic, MyObject> renderer(
        device, src,
        width, height
    );

    std::vector<MyObject> objects{
        MyObject(
            MyShape::init<0>(hy::Plane()),
            MyMaterial(
                make_pair(0.2, Specular()),
                make_pair(0.8, Colored<Lambertian>(
                    Lambertian(), float3(0.9f, 0.9f, 0.1f)
                ))
            )
        ),
        MyObject(
            MyShape::init<1>(hy::Horosphere()),
            MyMaterial(
                make_pair(0.1, Specular()),
                make_pair(0.9, Colored<Lambertian>(
                    Lambertian(), float3(0.1f, 0.1f, 0.9f)
                ))
            )
        ),
    };
    renderer.store_objects(objects);

    Controller<Hyperbolic> controller;
    controller.grab_mouse(true);

    //controller.view.position = mo_new(
    //    c_new(0.114543, 0.285363),
    //    c_new(2.9287, -0.678274),
    //    c_new(-0.0461927, -0.0460196),
    //    c_new(0.697521, -2.64087)
    //);
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

