#include <iostream>
#include <vector>
#include <string>
#include <cassert>
#include <cstdint>
#include <cstring>
#include <cmath>
#include <chrono>
#include <thread>
#include <random>

#include <opencl/opencl.hpp>
#include <sdl/viewer.hpp>
#include <sdl/controller.hpp>

#include <algebra/quaternion.hh>
#include <algebra/moebius.hh>
#include <geometry/hyperbolic.hh>
#include <geometry/hyperbolic/plane.hh>
#include <geometry/hyperbolic/horosphere.hh>
#include <object.hh>


using duration = std::chrono::duration<double>;


int main(int argc, const char *argv[]) {
	std::vector<cl_platform_id> platforms = cl::get_platforms();
    std::vector<std::vector<cl_device_id>> platform_devices(platforms.size());

    for (size_t i = 0; i < platforms.size(); ++i) {
        std::cout << std::endl << "Platform " << i << ":" << std::endl;
        cl::print_platform_info(platforms[i], "  ");

        std::vector<cl_device_id> devices = cl::get_devices(platforms[i]);
        for (size_t j = 0; j < devices.size(); ++j) {
            std::cout << std::endl << "  Device " << j << ":" << std::endl;
            cl::print_device_info(devices[j], "    ");
        }
        platform_devices[i] = std::move(devices);
    }
    std::cout << std::endl;

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
    
    assert(platform_no >= 0 && platform_no < int(platforms.size()));
    assert(device_no >= 0 && device_no < int(platform_devices[platform_no].size()));
    cl_device_id device = platform_devices[platform_no][device_no];

	cl::Context context(device);
	cl::Queue queue(context, device);

    cl::Program program(context, device, "render.cl", {"src/device", "src/common"});
    cl::Kernel kernel(program, "render");

    int width = 800, height = 600;
    cl::Buffer image(context, width*height*4);
    cl::Buffer screen(context, width*height*3*sizeof(cl_float));

    cl::Buffer seeds(context, width*height*sizeof(cl_uint));
    {
        std::mt19937 rng(0xdeadbeef);
        std::vector<uint32_t> host_seeds(width*height);
        for (uint32_t &seed : host_seeds) {
            seed = rng();
        }
        seeds.store(queue, host_seeds.data());
    }

    std::vector<Object> objects = {
        Object {
            .type = OBJECT_HOROSPHERE,
            .map = mo_identity(),
            .horosphere = Horosphere {
                .tiling = HOROSPHERE_TILING_HEXAGONAL,
                .material = Material {float3(1.0, 0.5, 0.25), 0.1},
                .size = 0.25, .border = 0.02
            },
        },
        Object {
            .type = OBJECT_HOROSPHERE,
            .map = mo_chain(mo_new(C1, sqrt(2)*C1, C0, C1), hy_yrotate(M_PI)),
            .horosphere = Horosphere {
                .tiling = HOROSPHERE_TILING_SQUARE,
                .material = Material {float3(0.25, 0.5, 1.0), 0.1},
                .size = 0.25, .border = 0.02
            },
        },
        Object {
            .type = OBJECT_PLANE,
            .map = mo_identity(),
            .plane = HyPlane {
                .tiling = HYPLANE_TILING_NONE,
                .material = Material {float3(0.5, 1.0, 0.25), 0.1},
                .border = 0.02
            },
        },
        Object {
            .type = OBJECT_PLANE,
            .map = mo_new(C1, 2*CI, C0, C1),
            .plane = HyPlane {
                .tiling = HYPLANE_TILING_PENTAGONAL,
                .material = Material {float3(1.0, 1.0, 0.25), 0.1},
                .border = 0.02
            },
        },
    };
    cl::Buffer object_buffer(context, sizeof(ObjectPk)*objects.size());
    {
        std::vector<ObjectPk> package(objects.size());
        for (size_t i = 0; i < objects.size(); ++i) {
            pack_object(&package[i], &objects[i]);
        }
        object_buffer.store(queue, package.data());
    }

    Viewer viewer(width, height);
    Controller controller(height);
    controller.grab_mouse(true);

    duration time_counter;
    int sample_counter = 0;
    const duration frame_time(0.04);
    int monte_carlo_counter = 0;
    bool refresh = true;
    for(;;) {
        duration elapsed;

        ViewPk view;
        pack_view(&view, &controller.get_view());

        auto start = std::chrono::system_clock::now();
        do {
            kernel(
                queue, width*height,
                screen, image,
                width, height,
                monte_carlo_counter,
                seeds,
                view,
                object_buffer, (cl_int)object_buffer.size()
            );
            sample_counter += 1;
            monte_carlo_counter += 1;
            elapsed = std::chrono::system_clock::now() - start;
        } while(elapsed < frame_time);

        viewer.display([&](uint8_t *data) {
            image.load(queue, data);
        });
        if (!controller.handle()) {
            break;
        }

        elapsed = std::chrono::system_clock::now() - start;
        if (controller.step(elapsed.count())) {
            refresh = true;
            monte_carlo_counter = 0;
        } else {
            if (refresh) {
                monte_carlo_counter = 0;
            }
            refresh = false;
        }
        
        time_counter += elapsed;
        if (time_counter.count() > 1.0) {
            double seconds;
            time_counter = duration(modf(time_counter.count(), &seconds));
            std::cout << "SPS: " << sample_counter/seconds << std::endl;
            sample_counter = 0;
        }
    }

    return 0;
}

