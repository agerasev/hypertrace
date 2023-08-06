#include <memory>
#include <iostream>
#include <sstream>
#include <cassert>

#include "opencl.hpp"

static const char *error_string(int code);

static cl_context get_context(cl_command_queue queue) {
    cl_context context;
    assert(clGetCommandQueueInfo(queue, CL_QUEUE_CONTEXT, sizeof(cl_context), &context, nullptr) == CL_SUCCESS);
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

cl::Program::Program(cl_context context, cl_device_id device, const std::span<uint8_t> &source) {
    this->device = device;

    cl_int err;
    program = clCreateProgramWithIL(context, source.data(), source.size(), &err);
    if (err != CL_SUCCESS) {
        std::cout << "error: " << error_string(err) << std::endl;
    }
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
    assert(clGetProgramBuildInfo(program, device, CL_PROGRAM_BUILD_LOG, 0, nullptr, &log_len) == CL_SUCCESS);
    assert(log_len > 0);

    std::vector<char> buffer(log_len, '\0');
    assert(clGetProgramBuildInfo(program, device, CL_PROGRAM_BUILD_LOG, buffer.size(), buffer.data(), NULL) == CL_SUCCESS);
    assert(buffer[buffer.size() - 1] == '\0');

    return std::string(buffer.data());
}

void cl::Buffer::init(cl_command_queue queue, size_t size, bool zeroed) {
    _size = size;
    if (size > 0) {
        assert(queue != nullptr);
        buffer = clCreateBuffer(get_context(queue), CL_MEM_READ_WRITE, size, nullptr, nullptr);
        assert(buffer != nullptr);
        if (zeroed) {
            uint8_t z = 0;
            assert(CL_SUCCESS == clEnqueueFillBuffer(queue, buffer, &z, 1, 0, size, 0, nullptr, nullptr));
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
    assert(clEnqueueReadBuffer(queue, buffer, CL_TRUE, 0, size, data, 0, nullptr, nullptr) == CL_SUCCESS);
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
    assert(clEnqueueWriteBuffer(queue, buffer, CL_TRUE, 0, size, data, 0, nullptr, nullptr) == CL_SUCCESS);
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
    assert(clEnqueueNDRangeKernel(queue, kernel, 1, NULL, global_work_size, NULL, 0, NULL, NULL) == CL_SUCCESS);
    assert(clFlush(queue) == CL_SUCCESS);
    assert(clFinish(queue) == CL_SUCCESS);
}

static const char *error_string(int code) {
    switch (code) {
    case CL_SUCCESS:
        return "CL_SUCCESS";
    case CL_DEVICE_NOT_FOUND:
        return "CL_DEVICE_NOT_FOUND";
    case CL_DEVICE_NOT_AVAILABLE:
        return "CL_DEVICE_NOT_AVAILABLE";
    case CL_COMPILER_NOT_AVAILABLE:
        return "CL_COMPILER_NOT_AVAILABLE";
    case CL_MEM_OBJECT_ALLOCATION_FAILURE:
        return "CL_MEM_OBJECT_ALLOCATION_FAILURE";
    case CL_OUT_OF_RESOURCES:
        return "CL_OUT_OF_RESOURCES";
    case CL_OUT_OF_HOST_MEMORY:
        return "CL_OUT_OF_HOST_MEMORY";
    case CL_PROFILING_INFO_NOT_AVAILABLE:
        return "CL_PROFILING_INFO_NOT_AVAILABLE";
    case CL_MEM_COPY_OVERLAP:
        return "CL_MEM_COPY_OVERLAP";
    case CL_IMAGE_FORMAT_MISMATCH:
        return "CL_IMAGE_FORMAT_MISMATCH";
    case CL_IMAGE_FORMAT_NOT_SUPPORTED:
        return "CL_IMAGE_FORMAT_NOT_SUPPORTED";
    case CL_MAP_FAILURE:
        return "CL_MAP_FAILURE";
    case CL_MISALIGNED_SUB_BUFFER_OFFSET:
        return "CL_MISALIGNED_SUB_BUFFER_OFFSET";
    case CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST:
        return "CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST";
    case CL_COMPILE_PROGRAM_FAILURE:
        return "CL_COMPILE_PROGRAM_FAILURE";
    case CL_LINKER_NOT_AVAILABLE:
        return "CL_LINKER_NOT_AVAILABLE";
    case CL_LINK_PROGRAM_FAILURE:
        return "CL_LINK_PROGRAM_FAILURE";
    case CL_DEVICE_PARTITION_FAILED:
        return "CL_DEVICE_PARTITION_FAILED";
    case CL_KERNEL_ARG_INFO_NOT_AVAILABLE:
        return "CL_KERNEL_ARG_INFO_NOT_AVAILABLE";
    case CL_INVALID_VALUE:
        return "CL_INVALID_VALUE";
    case CL_INVALID_DEVICE_TYPE:
        return "CL_INVALID_DEVICE_TYPE";
    case CL_INVALID_PLATFORM:
        return "CL_INVALID_PLATFORM";
    case CL_INVALID_DEVICE:
        return "CL_INVALID_DEVICE";
    case CL_INVALID_CONTEXT:
        return "CL_INVALID_CONTEXT";
    case CL_INVALID_QUEUE_PROPERTIES:
        return "CL_INVALID_QUEUE_PROPERTIES";
    case CL_INVALID_COMMAND_QUEUE:
        return "CL_INVALID_COMMAND_QUEUE";
    case CL_INVALID_HOST_PTR:
        return "CL_INVALID_HOST_PTR";
    case CL_INVALID_MEM_OBJECT:
        return "CL_INVALID_MEM_OBJECT";
    case CL_INVALID_IMAGE_FORMAT_DESCRIPTOR:
        return "CL_INVALID_IMAGE_FORMAT_DESCRIPTOR";
    case CL_INVALID_IMAGE_SIZE:
        return "CL_INVALID_IMAGE_SIZE";
    case CL_INVALID_SAMPLER:
        return "CL_INVALID_SAMPLER";
    case CL_INVALID_BINARY:
        return "CL_INVALID_BINARY";
    case CL_INVALID_BUILD_OPTIONS:
        return "CL_INVALID_BUILD_OPTIONS";
    case CL_INVALID_PROGRAM:
        return "CL_INVALID_PROGRAM";
    case CL_INVALID_PROGRAM_EXECUTABLE:
        return "CL_INVALID_PROGRAM_EXECUTABLE";
    case CL_INVALID_KERNEL_NAME:
        return "CL_INVALID_KERNEL_NAME";
    case CL_INVALID_KERNEL_DEFINITION:
        return "CL_INVALID_KERNEL_DEFINITION";
    case CL_INVALID_KERNEL:
        return "CL_INVALID_KERNEL";
    case CL_INVALID_ARG_INDEX:
        return "CL_INVALID_ARG_INDEX";
    case CL_INVALID_ARG_VALUE:
        return "CL_INVALID_ARG_VALUE";
    case CL_INVALID_ARG_SIZE:
        return "CL_INVALID_ARG_SIZE";
    case CL_INVALID_KERNEL_ARGS:
        return "CL_INVALID_KERNEL_ARGS";
    case CL_INVALID_WORK_DIMENSION:
        return "CL_INVALID_WORK_DIMENSION";
    case CL_INVALID_WORK_GROUP_SIZE:
        return "CL_INVALID_WORK_GROUP_SIZE";
    case CL_INVALID_WORK_ITEM_SIZE:
        return "CL_INVALID_WORK_ITEM_SIZE";
    case CL_INVALID_GLOBAL_OFFSET:
        return "CL_INVALID_GLOBAL_OFFSET";
    case CL_INVALID_EVENT_WAIT_LIST:
        return "CL_INVALID_EVENT_WAIT_LIST";
    case CL_INVALID_EVENT:
        return "CL_INVALID_EVENT";
    case CL_INVALID_OPERATION:
        return "CL_INVALID_OPERATION";
    case CL_INVALID_GL_OBJECT:
        return "CL_INVALID_GL_OBJECT";
    case CL_INVALID_BUFFER_SIZE:
        return "CL_INVALID_BUFFER_SIZE";
    case CL_INVALID_MIP_LEVEL:
        return "CL_INVALID_MIP_LEVEL";
    case CL_INVALID_GLOBAL_WORK_SIZE:
        return "CL_INVALID_GLOBAL_WORK_SIZE";
    case CL_INVALID_PROPERTY:
        return "CL_INVALID_PROPERTY";
    case CL_INVALID_IMAGE_DESCRIPTOR:
        return "CL_INVALID_IMAGE_DESCRIPTOR";
    case CL_INVALID_COMPILER_OPTIONS:
        return "CL_INVALID_COMPILER_OPTIONS";
    case CL_INVALID_LINKER_OPTIONS:
        return "CL_INVALID_LINKER_OPTIONS";
    case CL_INVALID_DEVICE_PARTITION_COUNT:
        return "CL_INVALID_DEVICE_PARTITION_COUNT";
    case CL_INVALID_PIPE_SIZE:
        return "CL_INVALID_PIPE_SIZE";
    case CL_INVALID_DEVICE_QUEUE:
        return "CL_INVALID_DEVICE_QUEUE";
    case CL_INVALID_SPEC_ID:
        return "CL_INVALID_SPEC_ID";
    case CL_MAX_SIZE_RESTRICTION_EXCEEDED:
        return "CL_MAX_SIZE_RESTRICTION_EXCEEDED";
    default:
        return "CL_UNKNOWN_ERROR";
    }
}