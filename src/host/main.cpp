#include <iostream>
#include <vector>
#include <string>
#include <cassert>
#include <cstdint>
#include <cstring>
#include <cmath>

#include <opencl/opencl.hpp>
#include <viewer.hpp>

#include <algebra/quaternion.h>
#include <object.h>


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
    cl::Buffer screen(context, width*height*4);
    
    std::vector<Object> objects = {
        {OBJECT_HOROSPHERE, Moebius::yrotate(0.5*M_PI), color(0.7, 0.8, 0.2)},
        {OBJECT_HOROSPHERE, Moebius::yrotate(-0.5*M_PI), color(0.8, 0.2, 0.1)},
        //{OBJECT_PLANE, Moebius::identity(), color(0.8, 0.2, 0.1)}
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


    double time = 0.0;
    for(;;) {
        kernel(
            queue, width*height,
            screen,
            width, height,
            viewer.controller().map().pack(),
            object_buffer, (int)object_buffer.size()
        );
        if (!viewer.display([&](uint8_t *data) {
            screen.load(queue, data);
        })) {
            break;
        }

        time += 1e-2;
        //p.z = 3.0 + sin(4*time);
    }

    return 0;
}

