#pragma once

#include <memory>
#include <string>
#include <vector>
#include <list>
#include <map>
#include <cassert>

#include <CL/cl.h>


namespace cl {
    class Context {
    private:
        cl_context context;

    public:
        Context(cl_device_id device);
        ~Context();

        Context(const Context &other) = delete;
        Context &operator=(const Context &other) = delete;

        operator cl_context() const;
    };

    class Queue {
    private:
        cl_command_queue queue;

    public:
        Queue(cl_context context, cl_device_id device);
        ~Queue();

        Queue(const Queue &other) = delete;
        Queue &operator=(const Queue &other) = delete;

        operator cl_command_queue() const;
    };

    class Program {
    private:
        cl_program program;
        cl_device_id device;

    public:
        Program(
            cl_context context,
            cl_device_id device,
            const std::string &source
        );
        ~Program();

        Program(const Program &other) = delete;
        Program &operator=(const Program &other) = delete;

        operator cl_program() const;

        std::string log();
    };

    class Buffer {
    private:
        cl_mem buffer;
        size_t _size;

        void init(cl_context context, size_t size);
        void release();

    public:
        Buffer();
        Buffer(cl_context context, size_t size);
        ~Buffer();

        Buffer(const Buffer &other) = delete;
        Buffer &operator=(const Buffer &other) = delete;

        cl_mem &raw();
        const cl_mem &raw() const;
        operator cl_mem() const;
        size_t size() const;

        void load(cl_command_queue queue, void *data);
        void load(cl_command_queue queue, void *data, size_t size);
        void store(cl_command_queue queue, const void *data);
        void store(cl_command_queue queue, const void *data, size_t size);
    };

    class Kernel {
    private:
        cl_kernel kernel;

    public:
        Kernel(cl_program program, const char *name);
        ~Kernel();

        Kernel(const Kernel &other) = delete;
        Kernel &operator=(const Kernel &other) = delete;

        operator cl_kernel() const;
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
        void set_arg(size_t n, const Buffer &buf);

        void run(cl_command_queue queue, size_t work_size);

        template <typename ... Args>
        void operator()(cl_command_queue queue, size_t work_size, const Args &... args) {
            unwind_args(0, args...);
            run(queue, work_size);
        }
    };
}
