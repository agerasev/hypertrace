#include <chrono>

#include <rstd.hpp>

#include <common/geometry/hyperbolic.hh>
#include <common/object/hyperbolic/plane.hh>

#include <host/opencl/opencl.hpp>
#include <host/sdl/viewer.hpp>
#include <host/sdl/controller.hpp>
#include <host/renderer.hpp>

using namespace rstd;

using duration = std::chrono::duration<double>;


int main(int argc, const char *argv[]) {
    int platform_no = 0;
    int device_no = 0;
    if (argc >= 2) {
        platform_no = std::stoi(argv[1]);
        if (argc >= 3) {
            device_no = std::stoi(argv[2]);
        }
    }

    println_("Using platform {}, device {}", platform_no, device_no);
    cl_device_id device = cl::search_device(platform_no, device_no);

    int width = 800, height = 600;

    Renderer<Hyperbolic> renderer(
        device,
        rs::Rc(cl::Context::create(device).expect("Context create error")),
        width, height,
        Box(Plane<Hy>())
    );

    Viewer viewer(width, height);
    Controller<Hyperbolic> controller;
    controller.view = View<Hy> { mo_identity(), 1, 1, 1 };

    duration time_counter(0);
    int sample_counter = 0;
    int refresh = 1;
    for(;;) {
        duration elapsed;
        auto start = std::chrono::system_clock::now();

        if (refresh) {
            renderer.set_view(controller.view);
        }
        sample_counter += renderer.render_for(0.04);

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
            //println_("Current position: {}", controller.view.position);
            println_("Samples per second: {}", sample_counter/time_counter.count());
            time_counter = duration(0.0);
            sample_counter = 0;
        }
    }

    return 0;
}
