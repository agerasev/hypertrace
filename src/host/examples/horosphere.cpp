#include <iostream>
#include <iomanip>
#include <sstream>
#include <vector>
#include <string>
#include <cassert>
#include <cstdint>
#include <cmath>
#include <chrono>

#include <algebra/quaternion.hh>
#include <algebra/moebius.hh>
#include <geometry/hyperbolic.hh>
#include <geometry/hyperbolic/horosphere.hh>
#include <view.hh>
#include <object.hh>

#include <opencl/search.hpp>
#include <sdl/viewer.hpp>
#include <sdl/controller.hpp>
#include <sdl/image.hpp>
#include <renderer.hpp>
#include <scenario.hpp>


class MyScenario : public PathScenario {
    private:
    mutable std::vector<Object> objects;

    public:
    MyScenario() {
        objects = {
            Object{
                .type = OBJECT_HOROSPHERE,
                .map = mo_identity(),
                .horosphere = Horosphere {
                    .materials = {
                        Material {float3(1.0, 1.0, 0.0), 0.0},
                        Material {float3(1.0, 0.0, 0.0), 0.0},
                        Material {float3(0.0, 1.0, 0.0), 0.0},
                        Material {float3(0.0, 0.0, 1.0), 0.0},
                    },
                    .material_count = 4,
                    .tiling = HorosphereTiling {
                        .type = HOROSPHERE_TILING_SQUARE,
                        .cell_size = 0.25,
                        .border = HorosphereTilingBorder {
                            .width = 0.02,
                            .material = Material {float3(0.0), 0.0},
                        },
                    },
                },
            },
        };
    }
    virtual const std::vector<Object> &get_objects(double t) const {
        return objects;
    }
};

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
    MyScenario scenario;
    renderer.store_objects(scenario.get_objects(0.0));
    
    Viewer viewer(width, height);
    Controller controller;
    controller.view.focal_length = 2.0;
    controller.grab_mouse(true);


    duration time_counter;
    int sample_counter = 0;
    int refresh = 1;
    for(;;) {
        duration elapsed;
        auto start = std::chrono::system_clock::now();

        sample_counter += renderer.render_for(controller.view, 0.04, refresh > 0);

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
