#include <iostream>
#include <sstream>
#include <cassert>

#include "crawler.hpp"


std::vector<cl_platform_id> cl::get_platforms() {
    cl_uint platform_count;
    assert(clGetPlatformIDs(0, nullptr, &platform_count) == CL_SUCCESS);
    assert(platform_count > 0);

    std::vector<cl_platform_id> platforms(platform_count, 0);
    assert(clGetPlatformIDs(platforms.size(), platforms.data(), nullptr) == CL_SUCCESS);

    return platforms;
}

std::vector<cl_device_id> cl::get_devices(cl_platform_id platform) {
    cl_uint device_count = 0;
    assert(clGetDeviceIDs(platform, CL_DEVICE_TYPE_ALL, 0, nullptr, &device_count) == CL_SUCCESS);
    assert(device_count > 0);

    std::vector<cl_device_id> devices(device_count, 0);
    assert(clGetDeviceIDs(
        platform, CL_DEVICE_TYPE_ALL,
        devices.size(), devices.data(), nullptr
    ) == CL_SUCCESS);

    return devices;
}

void cl::print_platform_info(
    cl_platform_id platform,
    const char *prefix,
    std::ostream &ostream
) {
    cl_platform_info params[] = {CL_PLATFORM_VERSION, CL_PLATFORM_NAME, CL_PLATFORM_VENDOR};
    for (size_t i = 0; i < sizeof(params)/sizeof(cl_platform_info); ++i) {
        const cl_platform_info &p = params[i];
        size_t ret_size = 0;
        assert(clGetPlatformInfo(platform, p, 0, nullptr, &ret_size) == CL_SUCCESS);
        assert(ret_size > 0);

        std::vector<char> text(ret_size, '\0');
        assert(clGetPlatformInfo(platform, p, text.size(), text.data(), nullptr) == CL_SUCCESS);

        assert(text[text.size() - 1] == '\0');
        ostream << prefix << text.data() << std::endl;
    }
}

void cl::print_device_info(
    cl_device_id device,
    const char *prefix,
    std::ostream &ostream
) {
    cl_device_info params[] = {CL_DEVICE_VERSION, CL_DEVICE_NAME, CL_DEVICE_VENDOR};
    for (size_t i = 0; i < sizeof(params)/sizeof(cl_device_info); ++i) {
        const cl_device_info &p = params[i];
        size_t ret_size = 0;
        assert(clGetDeviceInfo(device, p, 0, nullptr, &ret_size) == CL_SUCCESS);
        assert(ret_size > 0);

        std::vector<char> text(ret_size, '\0');
        assert(clGetDeviceInfo(device, p, text.size(), text.data(), nullptr) == CL_SUCCESS);

        assert(text[text.size() - 1] == '\0');
        ostream << prefix << text.data() << std::endl;
    }
}

cl_device_id cl::search_device(int platform_no, int device_no) {
    std::vector<cl_platform_id> platforms = cl::get_platforms();
    std::vector<std::vector<cl_device_id>> platform_devices(platforms.size());

    std::stringstream sstream;
    for (size_t i = 0; i < platforms.size(); ++i) {
        sstream << std::endl << "Platform " << i << ":" << std::endl;
        cl::print_platform_info(platforms[i], "  ", sstream);

        std::vector<cl_device_id> devices = cl::get_devices(platforms[i]);
        for (size_t j = 0; j < devices.size(); ++j) {
            sstream << std::endl << "  Device " << j << ":" << std::endl;
            cl::print_device_info(devices[j], "    ", sstream);
        }
        platform_devices[i] = std::move(devices);
    }

    if(
        (platform_no < 0 || platform_no >= int(platforms.size())) ||
        (device_no < 0 || device_no >= int(platform_devices[platform_no].size()))
    ) {
        std::cout << "Available platforms and devices:" << std::endl <<
            sstream.str() << std::endl;
        std::cerr << "Cannot find device " << device_no <<
            " at platform " << platform_no << std::endl;
        exit(1);
    }

    return platform_devices[platform_no][device_no];
}