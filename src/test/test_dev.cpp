#include <host/opencl/search.hpp>
#include <host/opencl/opencl.hpp>

#include <common/algebra/vector.hh>

#define RUN_TEST_DEV(name) { \
    std::cout << "    " << #name << " "; \
    std::cout.flush(); \
    test_dev_##name(device, queue); \
    std::cout << "... ok" << std::endl; \
}

int main(int, char *[]) {
    for (cl_platform_id platform : cl::get_platforms()) {
        cl::print_platform_info(platform);
        for (cl_device_id device : cl::get_devices(platform)) {
            cl::print_device_info(device, "  ");
            auto context = rstd::Rc<cl::Context>(cl::Context::create(device).unwrap());
            auto queue = rstd::Rc<cl::Queue>(cl::Queue::create(context, device).unwrap());

            RUN_TEST_DEV(real);
            //RUN_TEST_DEV(vector);

            std::cout << std::endl;

            queue.try_take().unwrap();
            context.try_take().unwrap();
        }
    }
    return 0;
}
