#include <memory>
#include <iostream>
#include <sstream>
#include <cassert>

#include "opencl.hpp"


static cl_context get_context(cl_command_queue queue) {
    cl_context context;
    assert(clGetCommandQueueInfo(
        queue, CL_QUEUE_CONTEXT,
        sizeof(cl_context), &context, nullptr
    ) == CL_SUCCESS);
    return context;
}

cl::Context::Context(cl_device_id device) {
    context = clCreateContext(nullptr, 1, &device, nullptr, nullptr, nullptr);
    assert(context != nullptr);
}
cl::Context::~Context() {
    assert(clReleaseContext(context) == CL_SUCCESS);
}

cl::Context::operator cl_context() const {
    return context;
}

cl::Queue::Queue(cl_context context, cl_device_id device) {
    queue = clCreateCommandQueue(context, device, 0, nullptr);
    assert(queue != nullptr);
}
cl::Queue::~Queue() {
    assert(clReleaseCommandQueue(queue) == CL_SUCCESS);
}
cl::Queue::operator cl_command_queue() const {
    return queue;
}

cl::Program::Program(
    cl_context context,
    cl_device_id device,
    std::shared_ptr<c_includer> includer
) : includer(includer) {
    this->device = device;

    std::string src = includer->data();
    std::fstream("gen_kernel.cl", std::ios::out) << src << std::endl;
    
    const char *src_data = src.c_str();
    const size_t src_len = src.size();

    program = clCreateProgramWithSource(context, 1, &src_data, &src_len, nullptr);
    assert(program != nullptr);

    cl_uint status = clBuildProgram(program, 1, &device, nullptr, nullptr, nullptr);
    std::cout << log() << std::endl;
    assert(status == CL_SUCCESS);
}
cl::Program::~Program() {
    assert(clReleaseProgram(program) == CL_SUCCESS);
}

cl::Program::operator cl_program() const {
    return program;
}

std::string cl::Program::log() {
    size_t log_len = 0;
    assert(clGetProgramBuildInfo(
        program, device, CL_PROGRAM_BUILD_LOG,
        0, nullptr, &log_len
    ) == CL_SUCCESS);
    assert(log_len > 0);

    std::vector<char> buffer(log_len, '\0');
    assert(clGetProgramBuildInfo(
        program, device, CL_PROGRAM_BUILD_LOG,
        buffer.size(), buffer.data(), NULL
    ) == CL_SUCCESS);
    assert(buffer[buffer.size() - 1] == '\0');

    return includer->convert(std::string(buffer.data()));
}

void cl::Buffer::init(cl_command_queue queue, size_t size, bool zeroed) {
    _size = size;
    if (size > 0) {
        assert(queue != nullptr);
        buffer = clCreateBuffer(
            get_context(queue), CL_MEM_READ_WRITE,
            size, nullptr, nullptr
        );
        assert(buffer != nullptr);
        if (zeroed) {
            uint8_t z = 0;
            assert(CL_SUCCESS == clEnqueueFillBuffer(
                queue, buffer,
                &z, 1, 0, size,
                0, nullptr, nullptr
            ));
        }
    } else {
        buffer = nullptr;
    }
}
void cl::Buffer::release() {
    if (buffer != nullptr) {
        assert(clReleaseMemObject(buffer) == CL_SUCCESS);
        buffer = nullptr;
        _size = 0;
    }
}
cl::Buffer::Buffer() {
    init(nullptr, 0);
}
cl::Buffer::Buffer(cl_command_queue queue, size_t size, bool zeroed) {
    init(queue, size, zeroed);
}
cl::Buffer::~Buffer() {
    release();
}

cl_mem &cl::Buffer::raw() {
    return buffer;
}
const cl_mem &cl::Buffer::raw() const {
    return buffer;
}
cl::Buffer::operator cl_mem() const {
    return raw();
}
size_t cl::Buffer::size() const {
    return _size;
}

void cl::Buffer::load(cl_command_queue queue, void *data) {
    load(queue, data, _size);
}
void cl::Buffer::load(cl_command_queue queue, void *data, size_t size) {
    assert(size <= _size);
    if (size <= 0) {
        return;
    }
    assert(clEnqueueReadBuffer(
        queue, buffer, CL_TRUE,
        0, size, data,
        0, nullptr, nullptr
    ) == CL_SUCCESS);
}
void cl::Buffer::store(cl_command_queue queue, const void *data) {
    store(queue, data, _size);
}
void cl::Buffer::store(cl_command_queue queue, const void *data, size_t size) {
    if (size > _size) {
        release();
        
        init(queue, size);
    }
    if (size <= 0) {
        return;
    }
    assert(clEnqueueWriteBuffer(
        queue, buffer, CL_TRUE,
        0, size, data,
        0, nullptr, nullptr
    ) == CL_SUCCESS);
}

cl::Kernel::Kernel(cl_program program, const char *name) {
    cl_int errcode;
    kernel = clCreateKernel(program, name, &errcode);
    assert(kernel != nullptr);
}
cl::Kernel::~Kernel() {
    assert(clReleaseKernel(kernel) == CL_SUCCESS);
}

cl::Kernel::operator cl_kernel() const {
    return kernel;
}

void cl::Kernel::set_arg(size_t n, const cl::Buffer &buf) {
    set_arg(n, buf.raw());
};

void cl::Kernel::run(cl_command_queue queue, size_t work_size) {
    size_t global_work_size[1] = {work_size};
    assert(clEnqueueNDRangeKernel(
        queue, kernel,
        1, NULL, global_work_size, NULL,
        0, NULL, NULL
    ) == CL_SUCCESS);
    assert(clFlush(queue) == CL_SUCCESS);
    assert(clFinish(queue) == CL_SUCCESS);
}
