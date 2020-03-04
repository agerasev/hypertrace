#include <memory>
#include <iostream>
#include <sstream>
#include <vector>
#include <list>
#include <cassert>

#include "opencl.hpp"


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
    const char *path,
    const std::vector<const char *> &bases,
    bool include_warnings
) {
    this->device = device;

    std::list<std::string> dirs;
    for (const char *base : bases) {
        dirs.push_back(std::string(base));
    }
    includer = std::make_unique<c_includer>(path, dirs);
    
    bool include_status = includer->include();
    if (!include_status || include_warnings) {
        std::cout << includer->log() << std::endl;
    }
    assert(include_status);

    std::string src = includer->data();
    
    const char *src_data = src.c_str();
    const size_t src_len = src.size();

    program = clCreateProgramWithSource(context, 1, &src_data, &src_len, nullptr);
    assert(program != nullptr);

    cl_uint status = clBuildProgram(program, 1, &device, nullptr, nullptr, nullptr);
    std::cout << log() << std::endl;
    assert(status == CL_SUCCESS);
}
cl::Program::Program(
    cl_context context,
    cl_device_id device,
    const char *path,
    const char *base,
    bool include_warnings
) :
    Program(
        context,
        device,
        path,
        std::vector<const char *>({base}),
        include_warnings
    ) {}
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

void cl::Buffer::init(cl_context context, size_t size) {
    _size = size;
    if (size > 0) {
        assert(context != nullptr);
        buffer = clCreateBuffer(context, CL_MEM_READ_WRITE, size, nullptr, nullptr);
        assert(buffer != nullptr);
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
cl::Buffer::Buffer(cl_context context, size_t size) {
    init(context, size);
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

        cl_context context;
        assert(clGetCommandQueueInfo(
            queue, CL_QUEUE_CONTEXT,
            sizeof(cl_context), &context, nullptr
        ) == CL_SUCCESS);
        
        init(context, size);
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
