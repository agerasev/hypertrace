#include <iostream>
#include <vector>
#include <string>
#include <cassert>
#include <cstdint>
#include <cstring>

#include "cl.hpp"


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
        exit(1);
    }

    std::cout << "Using platform " << platform_no << ", device " << device_no << std::endl;
    
    assert(platform_no >= 0 && platform_no < int(platforms.size()));
    cl_platform_id platform = platforms[0];
    assert(device_no >= 0 && device_no < int(platform_devices[platform_no].size()));
    cl_device_id device = platform_devices[platform_no][device_no];

	cl::Context context(device);
	cl::Queue queue(context, device);

    cl::Program program(context, device, "device/kernel.cl", "src");
    cl::Kernel kernel(program, "_increment");

    const char in_data[7] = "ABCdef";
    char out_data[7] = {'\0'};
    size_t size = 6;

    cl::Buffer in_buffer(context, size);
    cl::Buffer out_buffer(context, size);

    in_buffer.store(queue, in_data);

    kernel(queue, size, in_buffer.raw(), out_buffer.raw());

    out_buffer.load(queue, out_data);

    std::cout << in_data << std::endl << out_data << std::endl;
    for (int i = 0; i < 6; ++i) {
        assert(in_data[i] + 1 == out_data[i]);
    }

    return 0;
}
