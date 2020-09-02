#include <memory>
#include <iostream>
#include <sstream>
#include <cassert>

#include "opencl.hpp"


using namespace cl;


// Context

void Context::free_raw(cl_context raw) {
    assert(clReleaseContext(raw) == CL_SUCCESS);
}
Option<Context> Context::create(cl_device_id device) {
    cl_context raw = clCreateContext(nullptr, 1, &device, nullptr, nullptr, nullptr);
    if (raw != nullptr) {
        return Option<Context>::Some(Context(raw, device));
    } else {
        return Option<Context>::None();
    }
}


// Queue

void Queue::free_raw(cl_command_queue raw) {
    assert(clReleaseCommandQueue(raw) == CL_SUCCESS);
}
Option<Queue> Queue::create(Rc<Context> context, cl_device_id device) {
    cl_command_queue raw = clCreateCommandQueue(*context, device, 0, nullptr);
    if (raw != nullptr) {
        return Option<Queue>::Some(Queue(raw, std::move(context)));
    } else {
        return Option<Queue>::None();
    }
}
void Queue::flush() {
    assert(clFlush(raw) == CL_SUCCESS);
}
void Queue::finish() {
    flush();
    assert(clFinish(raw) == CL_SUCCESS);
}


// Program

void Program::free_raw(cl_program raw) {
    assert(clReleaseProgram(raw) == CL_SUCCESS);
}
std::string Program::log(cl_program program, cl_device_id device) {
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

    return std::string(buffer.data());
}
Tuple<Option<Program>, std::string> Program::create(
    Rc<Context> context,
    Device device,
    const std::string &source
) {
    //std::fstream("gen_kernel.cl", std::ios::out) << src << std::endl;
    
    const char *src_data = source.c_str();
    const size_t src_len = source.size();

    cl_program raw = clCreateProgramWithSource(
        *context, 1, &src_data, &src_len, nullptr
    );
    if (raw == nullptr) {
        return Tuple<Option<Program>, std::string>(Option<Program>::None(), "");
    }

    cl_uint status = clBuildProgram(raw, 1, &device, nullptr, nullptr, nullptr);
    std::string log_ = log(raw, device);
    Option<Program> program;
    if (status != CL_SUCCESS) {
        program = Option<Program>::Some(Program(
            raw, device, std::move(context)
        ));
    } else {
        program = Option<Program>::None();
    }
    return Tuple<Option<Program>, std::string>(std::move(program), std::move(log_));
}
Tuple<Option<Program>, std::string> Program::create(
    Rc<Context> context,
    Device device,
    const c_includer &includer
) {
    auto ret = create(std::move(context), device, includer.data());
    ret.get<1>() = includer.convert(ret.get<1>());
    return ret;
}


// Buffer

void Buffer::free_raw(cl_mem raw) {
    assert(clReleaseMemObject(raw) == CL_SUCCESS);
}
Option<Buffer> Buffer::create(Queue &queue, size_t size, bool zeroed) {
    assert(bool(queue));
    if (size > 0) {
        cl_mem raw = clCreateBuffer(
            queue.context(), CL_MEM_READ_WRITE,
            size, nullptr, nullptr
        );
        if (raw == nullptr) {
            return Option<Buffer>::None();
        }
        if (zeroed) {
            uint8_t z = 0;
            if (clEnqueueFillBuffer(
                queue, raw,
                &z, 1, 0, size,
                0, nullptr, nullptr
            ) != CL_SUCCESS) {
                return Option<Buffer>::None();
            }
        }
        return Option<Buffer>::Some(Buffer(raw, size, queue.context_ref()));
    } else {
        return Option<Buffer>::Some(Buffer());
    }
}
Result<> Buffer::load(Queue &queue, void *data) {
    return load(queue, data, size_);
}
Result<> Buffer::load(Queue &queue, void *data, size_t size) {
    if (size <= size_) {
        return Result<>::Err(Tuple<>());
    }
    if (size != 0 && clEnqueueReadBuffer(
        queue, raw, CL_TRUE,
        0, size, data,
        0, nullptr, nullptr
    ) != CL_SUCCESS) {
        return Result<>::Err(Tuple<>());
    }
    return Result<>::Ok(Tuple<>());
}
Result<> Buffer::store(Queue &queue, const void *data) {
    return store(queue, data, size_);
}
Result<> Buffer::store(Queue &queue, const void *data, size_t size) {
    if (size > size_) {
        auto new_this = create(queue, size, false);
        if (new_this.is_some()) {
            *this = new_this.unwrap();
        } else {
            return Result<>::Err(Tuple<>());
        }
    }
    if (size != 0 && clEnqueueWriteBuffer(
        queue, raw, CL_TRUE,
        0, size, data,
        0, nullptr, nullptr
    ) != CL_SUCCESS) {
        return Result<>::Err(Tuple<>());
    }
    return Result<>::Ok(Tuple<>());
}


// Kernel

void Kernel::free_raw(cl_kernel raw) {
    assert(clReleaseKernel(raw) == CL_SUCCESS);
}
Option<Kernel> Kernel::create(Rc<Program> program, const std::string &name) {
    cl_int errcode;
    cl_kernel raw = clCreateKernel(*program, name.c_str(), &errcode);
    if (raw != nullptr) {
        return Option<Kernel>::Some(Kernel(raw, program, name));
    } else {
        return Option<Kernel>::None();
    }
}

Result<> Kernel::run(Queue &queue, size_t work_size) {
    size_t global_work_size[1] = {work_size};
    if (clEnqueueNDRangeKernel(
        queue, raw,
        1, NULL, global_work_size, NULL,
        0, NULL, NULL
    ) != CL_SUCCESS) {
        return Result<>::Err(Tuple<>());
    }
    queue.flush();
    return Result<>::Ok(Tuple<>());
}
