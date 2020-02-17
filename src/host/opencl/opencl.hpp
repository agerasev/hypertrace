#pragma once

#include <memory>
#include <iostream>
#include <sstream>
#include <vector>
#include <list>
#include <cassert>

#include <CL/cl.h>

#include "include.hpp"


namespace cl {
    std::vector<cl_platform_id> get_platforms() {
        cl_uint platform_count;
        assert(clGetPlatformIDs(0, nullptr, &platform_count) == CL_SUCCESS);
        assert(platform_count > 0);

        std::vector<cl_platform_id> platforms(platform_count, 0);
        assert(clGetPlatformIDs(platforms.size(), platforms.data(), nullptr) == CL_SUCCESS);

        return platforms;
    }

    std::vector<cl_device_id> get_devices(cl_platform_id platform) {
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

    void print_platform_info(cl_platform_id platform, const char *prefix = "") {
        cl_platform_info params[] = {CL_PLATFORM_VERSION, CL_PLATFORM_NAME, CL_PLATFORM_VENDOR};
        for (size_t i = 0; i < sizeof(params)/sizeof(cl_platform_info); ++i) {
            const cl_platform_info &p = params[i];
            size_t ret_size = 0;
            assert(clGetPlatformInfo(platform, p, 0, nullptr, &ret_size) == CL_SUCCESS);
            assert(ret_size > 0);

            std::vector<char> text(ret_size, '\0');
            assert(clGetPlatformInfo(platform, p, text.size(), text.data(), nullptr) == CL_SUCCESS);

            assert(text[text.size() - 1] == '\0');
            std::cout << prefix << text.data() << std::endl;
        }
    }

    void print_device_info(cl_device_id device, const char *prefix = "") {
        cl_device_info params[] = {CL_DEVICE_VERSION, CL_DEVICE_NAME, CL_DEVICE_VENDOR};
        for (size_t i = 0; i < sizeof(params)/sizeof(cl_device_info); ++i) {
            const cl_device_info &p = params[i];
            size_t ret_size = 0;
            assert(clGetDeviceInfo(device, p, 0, nullptr, &ret_size) == CL_SUCCESS);
            assert(ret_size > 0);

            std::vector<char> text(ret_size, '\0');
            assert(clGetDeviceInfo(device, p, text.size(), text.data(), nullptr) == CL_SUCCESS);

            assert(text[text.size() - 1] == '\0');
            std::cout << prefix << text.data() << std::endl;
        }
    }

    class Context {
    private:
        cl_context context;

    public:
        Context(cl_device_id device) {
            context = clCreateContext(nullptr, 1, &device, nullptr, nullptr, nullptr);
            assert(context != nullptr);
        }
        ~Context() {
            assert(clReleaseContext(context) == CL_SUCCESS);
        }

        Context(const Context &other) = delete;
        Context &operator=(const Context &other) = delete;

        operator cl_context() const {
            return context;
        }
    };

    class Queue {
    private:
        cl_command_queue queue;

    public:
        Queue(cl_context context, cl_device_id device) {
            queue = clCreateCommandQueue(context, device, 0, nullptr);
            assert(queue != nullptr);
        }
        ~Queue() {
            assert(clReleaseCommandQueue(queue) == CL_SUCCESS);
        }

        Queue(const Queue &other) = delete;
        Queue &operator=(const Queue &other) = delete;

        operator cl_command_queue() const {
            return queue;
        }
    };

    class Program {
    private:
        cl_program program;
        cl_device_id device;
        std::unique_ptr<c_includer> includer;

    public:
        Program(
            cl_context context,
            cl_device_id device,
            const char *path,
            const std::vector<const char *> &bases
        ) {
            this->device = device;

            std::list<std::string> dirs;
            for (const char *base : bases) {
                dirs.push_back(std::string(base));
            }
            includer = std::make_unique<c_includer>(path, dirs);
            
            bool include_status = includer->include();
            std::cout << includer->log() << std::endl;
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
        Program(cl_context context, cl_device_id device, const char *path, const char *base=".") :
            Program(context, device, path, std::vector<const char *>({base})) {}
        ~Program() {
            assert(clReleaseProgram(program) == CL_SUCCESS);
        }

        Program(const Program &other) = delete;
        Program &operator=(const Program &other) = delete;

        operator cl_program() const {
            return program;
        }

        std::string log() {
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
    };

    class Buffer {
    private:
        cl_mem buffer;
        size_t _size;

    public:
        Buffer(cl_context context, size_t size) {
            _size = size;
            buffer = clCreateBuffer(context, CL_MEM_READ_WRITE, size, nullptr, nullptr);
            assert(buffer != nullptr);
        }
        ~Buffer() {
            assert(clReleaseMemObject(buffer) == CL_SUCCESS);
        }

        Buffer(const Buffer &other) = delete;
        Buffer &operator=(const Buffer &other) = delete;

        cl_mem &raw() {
            return buffer;
        }
        const cl_mem &raw() const {
            return buffer;
        }
        operator cl_mem() const {
            return raw();
        }
        size_t size() const {
            return _size;
        }

        void load(cl_command_queue queue, void *data) {
            assert(clEnqueueReadBuffer(
                queue, buffer, CL_TRUE,
                0, size(), data,
                0, nullptr, nullptr
            ) == CL_SUCCESS);
        }
        void store(cl_command_queue queue, const void *data) {
            assert(clEnqueueWriteBuffer(
                queue, buffer, CL_TRUE,
                0, size(), data,
                0, nullptr, nullptr
            ) == CL_SUCCESS);
        }
    };

    class Kernel {
    private:
        cl_kernel kernel;

    public:
        Kernel(cl_program program, const char *name) {
            cl_int errcode;
            kernel = clCreateKernel(program, name, &errcode);
            assert(kernel != nullptr);
        }
        ~Kernel() {
            assert(clReleaseKernel(kernel) == CL_SUCCESS);
        }

        Kernel(const Kernel &other) = delete;
        Kernel &operator=(const Kernel &other) = delete;

        operator cl_kernel() const {
            return kernel;
        }
    private:
        template <typename T, typename ... Args>
        void unwind_args(size_t n, const T &arg, const Args &... args) {
            set_arg(n, arg);
            unwind_args(n + 1, args...);
        }

        template <typename T>
        void unwind_args(size_t n, const T &arg) {
            set_arg(n, arg);
        }
    public:
        template <typename T>
        void set_arg(size_t n, const T &arg) {
            assert(clSetKernelArg(kernel, n, sizeof(T), (void *)&arg) == CL_SUCCESS);
        }
        void set_arg(size_t n, const Buffer &buf) {
            set_arg(n, buf.raw());
        };

        void run(cl_command_queue queue, size_t work_size) {
            size_t global_work_size[1] = {work_size};
            assert(clEnqueueNDRangeKernel(
                queue, kernel,
                1, NULL, global_work_size, NULL,
                0, NULL, NULL
            ) == CL_SUCCESS);
            assert(clFlush(queue) == CL_SUCCESS);
            assert(clFinish(queue) == CL_SUCCESS);
        }

        template <typename ... Args>
        void operator()(cl_command_queue queue, size_t work_size, const Args &... args) {
            unwind_args(0, args...);
            run(queue, work_size);
        }
    };
}
