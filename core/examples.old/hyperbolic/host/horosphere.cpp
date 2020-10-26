#include <iostream>
#include <iomanip>
#include <sstream>
#include <vector>
#include <string>
#include <cassert>
#include <cstdint>
#include <cmath>
#include <chrono>

#include <math.hpp>
#include <geometry/hyperbolic.hpp>
#include <algebra/color.hpp>

#include <opencl/search.hpp>
#include <sdl/viewer.hpp>
#include <sdl/controller.hpp>
#include <sdl/image.hpp>
#include <renderer.hpp>
#include <scenario.hpp>

#include <common.hpp>

#include <device/kernel.gen.cl.h>


class MyScenario : public PathScenario<MyObject> {
    private:
    std::vector<MyObject> objects;
    std::vector<double> ts; // timestamps

    template <typename Tr>
    void add_any(const Tr t) {
        add_transition(std::make_unique<Tr>(t));
    }

    public:
    MyScenario() {
        objects = {
            make_horosphere(
                Moebius<comp>::identity(),
                MyHorosphere::Tiling::SQUARE,
                MyHorosphereMaterials(
                    make_material(make_color(0x6ec3c1)),
                    make_material(make_color(0x335120)),
                    make_material(make_color(0x9dcc5f)),
                    make_material(make_color(0x0d5f8a))
                ),
                0.25, 0.03,
                make_material(float3(0))
            ),
        };
        
        ts = {
            0.0, // Move
            5.0, // Become transparent
            7.5, // Resize tiling
            10.0, // Enter horosphere
            15.0, // Move inside
            20.0, // End
        };

        std::vector<View<Hy>> points {
            View<Hy>{Hy::horosphere(comp(2, 2))*Hy::zshift(-1.0)},
            View<Hy>{Hy::horosphere(0.5*comp(0.25, 0.25))*Hy::zshift(-1.0)},
            View<Hy>{Hy::horosphere(0.5*comp(0.75, 0.75))*Hy::zshift(-1.0)},
            View<Hy>{Hy::horosphere(0.5*comp(0.75, 0.75))*Hy::zshift(0.1)},
            View<Hy>{Hy::horosphere(comp(-4, -4))*Hy::zshift(0.1)},
        };

        add_any(SquareTransition<Hy>(ts[1] - ts[0], points[0], points[1], 0.0, 1.0));
        add_any(DelayTransition<Hy>(ts[2] - ts[1], points[1]));
        add_any(LinearTransition<Hy>(ts[3] - ts[2], points[1], points[2]));
        add_any(SquareTransition<Hy>(ts[4] - ts[3], points[2], points[3], 0.0, 1.0));
        add_any(SquareTransition<Hy>(ts[5] - ts[4], points[3], points[4], 0.0, 1.0));
    }
    virtual std::vector<MyObject> get_objects(double t) const {
        std::vector<MyObject> objs(objects);
        MyHorosphere &hor = objs[0].inner.as_variant().as_union().template elem<1>();

        real tr = 0.5*(1.0 - math::clamp((ts[2] - t)/(ts[2] - ts[1]), 0.0, 1.0));
        for (int i = 0; i < hor.materials.size(); ++i) {
            set_transparency(hor.materials[i], tr);
        }

        real cs = 0.25 + 0.5*math::clamp((t - ts[2])/(ts[3] - ts[2]), 0.0, 1.0);
        hor.cell_size = cs;
        hor.border_width = 0.1*cs;

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
    std::string src(
        (const char *)kernel_gen_cl,
        size_t(kernel_gen_cl_len)
    );
    Renderer<MyObject> renderer(device, src, width, height);
    MyScenario scenario;
    
    Viewer viewer(width, height);
    Controller<Hy> controller;
    //controller.view.focal_length = 2.0;
    

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
