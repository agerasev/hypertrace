#include <memory>
#include <iostream>
#include <sstream>

#include "opencl.hpp"


using namespace rstd;
using namespace cl;


// Context

void Context::free_raw(cl_context raw) {
    assert_eq_(clReleaseContext(raw), CL_SUCCESS);
}
Result<Context> Context::create(cl_device_id device) {
    cl_context raw = clCreateContext(nullptr, 1, &device, nullptr, nullptr, nullptr);
    if (raw != nullptr) {
        return Ok(Context(raw, device));
    } else {
        return Err();
    }
}


// Queue

void Queue::free_raw(cl_command_queue raw) {
    assert_eq_(clReleaseCommandQueue(raw), CL_SUCCESS);
}
Result<Queue> Queue::create(Rc<Context> context, cl_device_id device) {
    cl_command_queue raw = clCreateCommandQueue(*context, device, 0, nullptr);
    if (raw != nullptr) {
        return Ok(Queue(raw, std::move(context)));
    } else {
        return Err();
    }
}
void Queue::flush() {
    assert_eq_(clFlush(raw_), CL_SUCCESS);
}
void Queue::finish() {
    flush();
    assert_eq_(clFinish(raw_), CL_SUCCESS);
}


// Program

void Program::free_raw(cl_program raw) {
    assert_eq_(clReleaseProgram(raw), CL_SUCCESS);
}
std::string Program::log(cl_program program, cl_device_id device) {
    size_t log_len = 0;
    assert_eq_(clGetProgramBuildInfo(
        program, device, CL_PROGRAM_BUILD_LOG,
        0, nullptr, &log_len
    ), CL_SUCCESS);
    assert_(log_len > 0);

    std::vector<char> buffer(log_len, '\0');
    assert_eq_(clGetProgramBuildInfo(
        program, device, CL_PROGRAM_BUILD_LOG,
        buffer.size(), buffer.data(), NULL
    ), CL_SUCCESS);
    assert_eq_(buffer[buffer.size() - 1], '\0');

    return std::string(buffer.data());
}
Tuple<Result<Program>, std::string> Program::create(
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
        return Tuple<Result<Program>, std::string>(
            Err(),
            "clCreateProgramWithSource returned NULL"
        );
    }

    cl_uint status = clBuildProgram(raw, 1, &device, nullptr, nullptr, nullptr);
    std::string log_ = log(raw, device);
    Result<Program> program;
    if (status == CL_SUCCESS) {
        program = Ok(Program(
            raw, device, std::move(context)
        ));
    } else {
        program = Err();
    }
    return Tuple<Result<Program>, std::string>(std::move(program), std::move(log_));
}
Tuple<Result<Program>, std::string> Program::create(
    Rc<Context> context,
    Device device,
    const includer &includer
) {
    auto ret = create(std::move(context), device, includer.data());
    ret.get<1>() = includer.convert(ret.get<1>());
    return ret;
}


// Buffer

void Buffer::free_raw(cl_mem raw) {
    assert_eq_(clReleaseMemObject(raw), CL_SUCCESS);
}
Result<Buffer> Buffer::create(Queue &queue, size_t size, bool zeroed) {
    assert_(bool(queue));
    if (size > 0) {
        cl_mem raw = clCreateBuffer(
            queue.context(), CL_MEM_READ_WRITE,
            size, nullptr, nullptr
        );
        if (raw == nullptr) {
            return Err();
        }
        if (zeroed) {
            uint8_t z = 0;
            if (clEnqueueFillBuffer(
                queue, raw,
                &z, 1, 0, size,
                0, nullptr, nullptr
            ) != CL_SUCCESS) {
                return Err();
            }
            queue.finish();
        }
        return Ok(Buffer(raw, size, queue.context_ref()));
    } else {
        return Ok(Buffer());
    }
}
typedef Result<rstd::Tuple<>, std::string> LsResult;
LsResult Buffer::load(Queue &queue, void *data) {
    return load(queue, data, size_);
}
LsResult Buffer::load(Queue &queue, void *data, size_t size) {
    if (size > size_) {
        return LsResult::Err(format_("Requested size ({}) is greater that buffer size ({})", size, size_));
    }
    if (size != 0 && clEnqueueReadBuffer(
        queue, raw_, CL_TRUE,
        0, size, data,
        0, nullptr, nullptr
    ) != CL_SUCCESS) {
        return LsResult::Err("clEnqueueReadBuffer error");
    }
    queue.finish();
    return LsResult::Ok();
}
LsResult Buffer::store(Queue &queue, const void *data) {
    return store(queue, data, size_);
}
LsResult Buffer::store(Queue &queue, const void *data, size_t size) {
    if (size > size_) {
        auto new_this = create(queue, size, false);
        if (new_this.is_some()) {
            *this = new_this.unwrap();
        } else {
            return LsResult::Err("Buffer reallocation error");
        }
    }
    if (size != 0 && clEnqueueWriteBuffer(
        queue, raw_, CL_TRUE,
        0, size, data,
        0, nullptr, nullptr
    ) != CL_SUCCESS) {
        return LsResult::Err("clEnqueueWriteBuffer error");
    }
    queue.finish();
    return LsResult::Ok();
}


// Kernel

void Kernel::free_raw(cl_kernel raw) {
    assert_eq_(clReleaseKernel(raw), CL_SUCCESS);
}
Result<Kernel> Kernel::create(Rc<Program> program, const std::string &name) {
    cl_int errcode;
    cl_kernel raw = clCreateKernel(*program, name.c_str(), &errcode);
    if (raw != nullptr) {
        return Ok(Kernel(raw, program, name));
    } else {
        return Err();
    }
}

Result<> Kernel::run(Queue &queue, size_t work_size) {
    size_t global_work_size[1] = {work_size};
    if (clEnqueueNDRangeKernel(
        queue, raw_,
        1, NULL, global_work_size, NULL,
        0, NULL, NULL
    ) != CL_SUCCESS) {
        return Err();
    }
    queue.finish();
    return Ok();
}
