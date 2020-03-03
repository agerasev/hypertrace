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
#include <color.hpp>

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
                        Material {make_color(0x6ec3c1), 0.0, 0.5, float3(0)},
                        Material {make_color(0x335120), 0.0, 0.5, float3(0)},
                        Material {make_color(0x9dcc5f), 0.0, 0.5, float3(0)},
                        Material {make_color(0x0d5f8a), 0.0, 0.5, float3(0)},
                    },
                    .material_count = 4,
                    .tiling = HorosphereTiling {
                        .type = HOROSPHERE_TILING_SQUARE,
                        .cell_size = 0.4,
                        .border = HorosphereTilingBorder {
                            .width = 0.02,
                            .material = Material {float3(0.0), 0.0, 0, float3(0)},
                        },
                    },
                },
            }
        };

        std::vector<Point> points {
            Point(mo_chain(hy_horosphere(c_new(2, 2)), hy_zshift(-1.0))),
            Point(mo_chain(hy_horosphere(c_new(0, 0)), hy_zshift(-1.0))),
        };
        std::vector<SquareSpeedTransition> transitions {
            SquareSpeedTransition(4.0, points[0], points[1], 0.0, 1.0),
        };
        for (const auto &t : transitions) {
            add_transition(std::make_unique<SquareSpeedTransition>(t));
        }
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
    

    int counter = 0;
    double time = 0.0;
    double frame_time = 0.04;
    duration time_counter;
    int sample_counter = 0;
    for(;;) {
        auto start = std::chrono::system_clock::now();

        View v = scenario.get_view(time, frame_time);
        v.field_of_view = 0.8;
        sample_counter += renderer.render_for(v, frame_time, true);
        //sample_counter += renderer.render_n(v, 200, true);
        time += frame_time;

        auto store = [&](uint8_t *data) {
            renderer.load_image(data);
        };
        viewer.display(store);

        //std::stringstream ss;
        //ss << std::setfill('0') << std::setw(5) << "output/" << counter << ".png";
        //sdl::save_image(ss.str(), width, height, store);

        if (!controller.handle() || time > scenario.duration()) {
            break;
        }
        counter += 1;

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
