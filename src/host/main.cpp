#include <iostream>
#include <vector>
#include <string>
#include <cassert>
#include <cstdint>
#include <cstring>
#include <cmath>

#include <algebra/quaternion.h>
#include <geometry/surface.h>

#include <opencl/opencl.hpp>
#include <viewer.hpp>


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
    
    std::vector<Surface> surfaces = {
        {q_new(0, 0, 3, 0), q_new(0, 0, -1, 0), 1}, // horosphere
        {q_new(0, 0, 0.9, 0), q_new(0, 0, 1, 0), 0} // plane
    };
    cl::Buffer objects(context, sizeof(SurfacePacked)*surfaces.size());
    std::vector<SurfacePacked> pack(surfaces.size());
    for (size_t i = 0; i < surfaces.size(); ++i) {
        surface_pack(&pack[i], &surfaces[i]);
    }
    objects.store(queue, pack.data());

    Viewer viewer(width, height);


    double time = 0.0;
    for(;;) {
        // Horosphere
        Surface s = {
            .point = q_new(1.0, 0.0, 3.0, 0.0),
            .normal = q_new(0.0, 0.0, -1.0, 0.0),
            .curvature = 1.0
        };
        SurfacePacked sp;
        surface_pack(&sp, &s);
        kernel(
            queue, width*height,
            screen,
            width, height,
            viewer.controller().map().pack(),
            objects, (int)surfaces.size()
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

