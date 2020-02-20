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
#include <viewer.hpp>

#include <algebra/quaternion.h>
#include <object.h>


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
        {OBJECT_HOROSPHERE, Moebius::identity(), color(0.8, 0.2, 0.1)},
        {OBJECT_HOROSPHERE, Moebius::yrotate(M_PI), color(0.8, 0.2, 0.1)},
        {OBJECT_PLANE, Moebius::identity(), color(0.7, 0.8, 0.2)}
    };
    cl::Buffer object_buffer(context, sizeof(ObjectPacked)*objects.size());
    {
        std::vector<ObjectPacked> package(objects.size());
        for (size_t i = 0; i < objects.size(); ++i) {
            object_pack(&package[i], &objects[i]);
        }
        object_buffer.store(queue, package.data());
    }

    Viewer viewer(width, height);

    duration time_counter;
    int sample_counter = 0;
    const duration frame_time(0.04);
    int monte_carlo_counter = 0;
    for(;;) {
        duration elapsed;
        auto start = std::chrono::system_clock::now();
        do {
            kernel(
                queue, width*height,
                screen, image,
                width, height,
                monte_carlo_counter,
                seeds,
                viewer.controller().map().pack(),
                object_buffer, (cl_int)object_buffer.size()
            );
            sample_counter += 1;
            monte_carlo_counter += 1;
            elapsed = std::chrono::system_clock::now() - start;
        } while(elapsed < frame_time);

        if (!viewer.display([&](uint8_t *data) {
            image.load(queue, data);
        })) {
            break;
        }

        elapsed = std::chrono::system_clock::now() - start;
        if (viewer.controller().step(elapsed.count())) {
            monte_carlo_counter = 0;
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

