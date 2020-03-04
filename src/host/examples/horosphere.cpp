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
    std::vector<Object> objects;
    std::vector<double> ts; // timestamps

    template <typename Tr>
    void add_any(const Tr t) {
        add_transition(std::make_unique<Tr>(t));
    }

    public:
    MyScenario() {
        objects = {
            Object{
                .type = OBJECT_HOROSPHERE,
                .map = mo_identity(),
                .horosphere = Horosphere {
                    .materials = {
                        Material {make_color(0x6ec3c1), 0.0, 0.0, float3(0)},
                        Material {make_color(0x335120), 0.0, 0.0, float3(0)},
                        Material {make_color(0x9dcc5f), 0.0, 0.0, float3(0)},
                        Material {make_color(0x0d5f8a), 0.0, 0.0, float3(0)},
                    },
                    .material_count = 4,
                    .tiling = HorosphereTiling {
                        .type = HOROSPHERE_TILING_SQUARE,
                        .cell_size = 0.5,
                        .border = HorosphereTilingBorder {
                            .width = 0.03,
                            .material = Material {float3(0.0), 0.0, 0, float3(0)},
                        },
                    },
                },
            }
        };
        
        ts = {
            0.0, // Start move
            5.0, // Start becoming transparent
            7.5, // Become transparent
            10.0, // Entered horosphere
            15.0, // Moved inside
        };

        std::vector<View> points {
            view_position(mo_chain(hy_horosphere(c_new(4, 4)), hy_zshift(-1.0))),
            view_position(mo_chain(hy_horosphere(c_new(0.25, 0.25)), hy_zshift(-1.0))),
            view_position(mo_chain(hy_horosphere(c_new(0.25, 0.25)), hy_zshift(0.1))),
            view_position(mo_chain(hy_horosphere(c_new(-4, -4)), hy_zshift(0.1))),
        };

        add_any(SquareSpeedTransition(ts[1] - ts[0], points[0], points[1], 0.0, 1.0));
        add_any(DelayTransition(ts[2] - ts[1], points[1]));
        add_any(SquareSpeedTransition(ts[3] - ts[2], points[1], points[2], 0.0, 1.0));
        add_any(SquareSpeedTransition(ts[4] - ts[3], points[2], points[3], 0.0, 1.0));
    }
    virtual std::vector<Object> get_objects(double t) const {
        std::vector<Object> objs(objects);

        real tr = (t - ts[1])/(ts[2] - ts[1]);
        tr = min(max(tr, 0.0), 1.0);
        for (int i = 0; i < objs[0].horosphere.material_count; ++i) {
            objs[0].horosphere.materials[i].transparency = tr;
        }
        
        return objs;
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
    //int width = 1920, height = 1080;
    Renderer renderer(device, width, height);
    MyScenario scenario;
    
    Viewer viewer(width, height);
    Controller controller;
    controller.view.focal_length = 2.0;
    

    int counter = 0;
    double time = 0.0;
    double frame_time = 0.04;//0.05/3;
    duration time_counter;
    int sample_counter = 0;
    for(;;) {
        auto start = std::chrono::system_clock::now();

        renderer.set_view(
            scenario.get_view(time),
            scenario.get_view(time - frame_time)
        );
        renderer.store_objects(
            scenario.get_objects(time),
            scenario.get_objects(time - frame_time),
            std::vector<bool>{true}
        );
        sample_counter += renderer.render_for(frame_time, true);
        //sample_counter += renderer.render_n(200, true);
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
