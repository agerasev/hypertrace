#pragma once

#include <memory>
#include <string>
#include <vector>
#include <list>
#include <map>
#include <functional>
#include <cassert>

#include <CL/cl.h>

#include <core/prelude.hpp>

#include "search.hpp"
#include "include.hpp"


namespace cl {

template <typename T, void D(T)>
class _Guard {
private:
    T resource = 0;
public:
    _Guard() = default;
    _Guard(T r) : resource(r) {}
    void put(T r) {
        resource = r;
    }
    void destroy() {
        if (resource) {
            D(resource);
            resource = 0;
        }
    }
    ~_Guard() {
        destroy();
    }
    _Guard(const _Guard &g) = delete;
    _Guard &operator=(const _Guard &g) = delete;
    _Guard(_Guard &&g) : resource(g.resource) {
        g.resource = 0;
    }
    _Guard &operator=(_Guard &&g) {
        destroy();
        resource = g.resource;
        g.resource = 0;
    }
    T &operator*() { return resource; }
    const T &operator*() const { return resource; }
    T &operator->() { return resource; }
    const T &operator->() const { return resource; }
    operator T() const { return resource; }
    operator bool() const { return resource != 0; }
};

class Context {
private:
    static void free_raw(cl_context raw);

    Device device_;
    _Guard<cl_context, free_raw> raw;

    Context(cl_context raw, Device device);

public:
    Context() = default;
    ~Context() = default;

    Context(const Context &other) = delete;
    Context &operator=(const Context &other) = delete;
    Context(Context &&other) = default;
    Context &operator=(Context &&other) = default;

    static Option<Context> create(Device device);

    inline operator cl_context() const { return raw; }
    inline operator bool() const { return raw; }

    inline Device device() const { return device_; }
};

class Queue {
private:
    static void free_raw(cl_command_queue raw);

    Rc<Context> context_;
    _Guard<cl_command_queue, free_raw> raw;

    Queue(cl_command_queue raw, Rc<Context> context);

public:
    Queue() = default;
    ~Queue() = default;

    Queue(const Queue &other) = delete;
    Queue &operator=(const Queue &other) = delete;
    Queue(Queue &&other) = default;
    Queue &operator=(Queue &&other) = default;

    static Option<Queue> create(Rc<Context> context, Device device);
    void flush();
    void finish();

    inline operator cl_command_queue() const { return raw; }
    inline operator bool() const { return raw; }

    inline Context &context() { return *context_; }
    inline const Context &context() const { return *context_; }
    inline Rc<Context> context_ref() const { return context_; }
};

class Program {
private:
    static void free_raw(cl_program raw);

    Device device_;
    Rc<Context> context_;
    _Guard<cl_program, free_raw> raw;

    Program(
        cl_program raw,
        Device device,
        Rc<Context> context
    );
    static std::string log(cl_program program, cl_device_id device);

public:
    Program() = default;
    ~Program() = default;

    Program(const Program &other) = delete;
    Program &operator=(const Program &other) = delete;
    Program(Program &&other) = default;
    Program &operator=(Program &&other) = default;

    static Tuple<Option<Program>, std::string> create(
        Rc<Context> context,
        Device device,
        const std::string &source
    );
    static Tuple<Option<Program>, std::string> create(
        Rc<Context> context,
        Device device,
        const c_includer &includer
    );

    inline operator cl_program() const { return raw; }
    inline operator bool() const { return raw; }

    inline Context &context() { return *context_; }
    inline const Context &context() const { return *context_; }
    inline Rc<Context> context_ref() const { return context_; }
    inline Device device() const { return device_; }
};

class Buffer {
private:
    static void free_raw(cl_mem raw);
    
    Rc<Context> context_;
    _Guard<cl_mem, free_raw> raw;
    size_t size_ = 0;

    Buffer(cl_mem raw, size_t size);

public:
    Buffer() = default;
    ~Buffer() = default;

    Buffer(const Buffer &other) = delete;
    Buffer &operator=(const Buffer &other) = delete;
    Buffer(Buffer &&other) = default;
    Buffer &operator=(Buffer &&other) = default;

    static Option<Buffer> create(Queue &queue, size_t size, bool zeroed=false);

    inline operator cl_mem() const { return raw; }
    inline operator bool() const { return raw; }
    inline size_t size() const { return size_; }
    inline Context &context() { return *context_; }
    inline const Context &context() const { return *context_; }
    inline Rc<Context> context_ref() const { return context_; }

    Result<> load(Queue &queue, void *data);
    Result<> load(Queue &queue, void *data, size_t size);
    Result<> store(Queue &queue, const void *data);
    Result<> store(Queue &queue, const void *data, size_t size);
};

class Kernel {
private:
    static void free_raw(cl_kernel raw);

    Rc<Program> program_;
    _Guard<cl_kernel, free_raw> raw;
    std::string name_;

    Kernel(cl_kernel raw, Rc<Program> program, const std::string &name);

public:
    Kernel() = default;
    ~Kernel();

    Kernel(const Kernel &other) = delete;
    Kernel &operator=(const Kernel &other) = delete;
    Kernel(Kernel &&other) = default;
    Kernel &operator=(Kernel &&other) = default;

    static Option<Kernel> create(Rc<Program> program, const std::string &name);

    inline operator cl_kernel() const { return raw; }
    inline operator bool() const { return raw; }

    inline Program &program() { return *program_; }
    inline const Program &program() const { return *program_; }
    inline const Rc<Program> &program_ref() const { return program_; }
    inline const std::string &name() const { return name_; }

private:
    template <typename T, typename ... Args>
    void unwind_args(size_t n, const T &arg, Args &&...args) {
        set_arg(n, arg);
        unwind_args(n + 1, std::forward(args)...);
    }
    template <typename T>
    void unwind_args(size_t n, const T &arg) {
        set_arg(n, arg);
    }

public:
    template <typename T>
    void set_arg(size_t n, const T &arg) {
        assert(clSetKernelArg(raw, n, sizeof(T), (void *)&arg) == CL_SUCCESS);
    }
    inline void set_arg(size_t n, const Buffer &buf) {
        set_arg(n, cl_mem(buf));
    }
    Result<> run(Queue &queue, size_t work_size);

    template <typename ... Args>
    Result<> operator()(Queue &queue, size_t work_size, Args &&...args) {
        unwind_args(0, std::forward(args)...);
        return run(queue, work_size);
    }
};

}
