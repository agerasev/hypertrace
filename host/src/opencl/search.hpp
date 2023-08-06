#pragma once

#include <iostream>
#include <vector>

#include <CL/cl.h>


namespace cl {

std::vector<cl_platform_id> get_platforms();
std::vector<cl_device_id> get_devices(cl_platform_id platform);

void print_platform_info(cl_platform_id platform, const char *prefix = "", std::ostream &ostream = std::cout);
void print_device_info(cl_device_id device, const char *prefix = "", std::ostream &ostream = std::cout);

cl_device_id search_device(int platform_no = 0, int device_no = 0);

}; // namespace cl
