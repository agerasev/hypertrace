#include <opencl/search.hpp>
#include <opencl/opencl.hpp>

#include <kernel.h>

int main(int argc, const char *argv[]) {
    int platform_no = 0;
    int device_no = 0;
    if (argc >= 2) {
        platform_no = std::stoi(argv[1]);
        if (argc >= 3) {
            device_no = std::stoi(argv[2]);
        }
    }
    std::cout << "Using platform " << platform_no << ", device " << device_no << std::endl;
    cl_device_id device = cl::search_device(platform_no, device_no);

    cl::Context context(device);
    cl::Queue queue(context, device);

    std::span source(KERNEL_SPV, sizeof(KERNEL_SPV));
    cl::Program program(context, device, source);
    cl::Kernel kernel(program, "test");

    int width = 800, height = 600;
    cl::Buffer image(queue, width * height * 4);

    return 0;
}
