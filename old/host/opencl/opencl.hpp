#pragma once

#include <memory>
#include <string>
#include <vector>
#include <list>
#include <map>
#include <functional>

#include <CL/cl.h>

#include <rstd.hpp>

#include "search.hpp"
#include <includer.hpp>


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
        return *this;
    }
    T &operator*() { return resource; }
    const T &operator*() const { return resource; }
    T &operator->() { return resource; }
    const T &operator->() const { return resource; }
    operator T() const { return resource; }
    explicit operator bool() const { return resource != 0; }
};

class Context {
private:
    static void free_raw(cl_context raw);

    Device device_;
    _Guard<cl_context, free_raw> raw_;

    inline Context(cl_context raw, Device device) :
        device_(device), raw_(raw) {}

public:
    Context() = default;
    ~Context() = default;

    Context(const Context &other) = delete;
    Context &operator=(const Context &other) = delete;
    Context(Context &&other) = default;
    Context &operator=(Context &&other) = default;

    static rs::Result<Context> create(Device device);

    inline cl_context raw() const { return raw_; }
    inline operator cl_context() const { return raw(); }
    inline explicit operator bool() const { return raw(); }

    inline Device device() const { return device_; }
};

class Queue {
private:
    static void free_raw(cl_command_queue raw);

    rs::Rc<Context> context_;
    _Guard<cl_command_queue, free_raw> raw_;

    inline Queue(cl_command_queue raw, rs::Rc<Context> context) :
        context_(std::move(context)), raw_(raw) {}

public:
    Queue() = default;
    ~Queue() = default;

    Queue(const Queue &other) = delete;
    Queue &operator=(const Queue &other) = delete;
    Queue(Queue &&other) = default;
    Queue &operator=(Queue &&other) = default;

    static rs::Result<Queue> create(rs::Rc<Context> context, Device device);
    void flush();
    void finish();

    inline cl_command_queue raw() const { return raw_; }
    inline operator cl_command_queue() const { return raw(); }
    inline explicit operator bool() const { return raw(); }

    inline Context &context() { return *context_; }
    inline const Context &context() const { return *context_; }
    inline rs::Rc<Context> context_ref() const { return context_; }
};

class Program {
private:
    static void free_raw(cl_program raw);

    Device device_;
    rs::Rc<Context> context_;
    _Guard<cl_program, free_raw> raw_;

    inline Program(cl_program raw, Device device, rs::Rc<Context> context) :
        device_(device), context_(std::move(context)), raw_(raw) {}
    static std::string log(cl_program program, cl_device_id device);

public:
    Program() = default;
    ~Program() = default;

    Program(const Program &other) = delete;
    Program &operator=(const Program &other) = delete;
    Program(Program &&other) = default;
    Program &operator=(Program &&other) = default;

    static rs::Tuple<rs::Result<Program>, std::string> create(
        rs::Rc<Context> context,
        Device device,
        const std::string &source
    );
    static rs::Tuple<rs::Result<Program>, std::string> create(
        rs::Rc<Context> context,
        Device device,
        const includer &includer
    );

    inline cl_program raw() const { return raw_; }
    inline operator cl_program() const { return raw(); }
    inline explicit operator bool() const { return raw(); }

    inline Context &context() { return *context_; }
    inline const Context &context() const { return *context_; }
    inline rs::Rc<Context> context_ref() const { return context_; }
    inline Device device() const { return device_; }
};

class Buffer {
private:
    static void free_raw(cl_mem raw);
    
    rs::Rc<Context> context_;
    _Guard<cl_mem, free_raw> raw_;
    size_t size_ = 0;

    inline Buffer(cl_mem raw, size_t size, rs::Rc<Context> context) :
        context_(std::move(context)), raw_(raw), size_(size) {}

public:
    Buffer() = default;
    ~Buffer() = default;

    Buffer(const Buffer &other) = delete;
    Buffer &operator=(const Buffer &other) = delete;
    Buffer(Buffer &&other) = default;
    Buffer &operator=(Buffer &&other) = default;

    static rs::Result<Buffer> create(Queue &queue, size_t size, bool zeroed=false);

    rs::Result<rs::Tuple<>, std::string> load(Queue &queue, void *data);
    rs::Result<rs::Tuple<>, std::string> load(Queue &queue, void *data, size_t size);
    rs::Result<rs::Tuple<>, std::string> store(Queue &queue, const void *data);
    rs::Result<rs::Tuple<>, std::string> store(Queue &queue, const void *data, size_t size);

    inline cl_mem raw() const { return raw_; }
    inline operator cl_mem() const { return raw(); }
    inline explicit operator bool() const { return raw(); }
    inline size_t size() const { return size_; }
    inline Context &context() { return *context_; }
    inline const Context &context() const { return *context_; }
    inline rs::Rc<Context> context_ref() const { return context_; }
};

class Kernel {
private:
    static void free_raw(cl_kernel raw);

    rs::Rc<Program> program_;
    _Guard<cl_kernel, free_raw> raw_;
    std::string name_;

    inline Kernel(cl_kernel raw, rs::Rc<Program> program, const std::string &name) :
        program_(std::move(program)), raw_(raw), name_(name) {}

public:
    Kernel() = default;
    ~Kernel() = default;

    Kernel(const Kernel &other) = delete;
    Kernel &operator=(const Kernel &other) = delete;
    Kernel(Kernel &&other) = default;
    Kernel &operator=(Kernel &&other) = default;

    static rs::Result<Kernel> create(rs::Rc<Program> program, const std::string &name);

    inline cl_kernel raw() const { return raw_; }
    inline operator cl_kernel() const { return raw(); }
    inline explicit operator bool() const { return raw(); }

    inline Program &program() { return *program_; }
    inline const Program &program() const { return *program_; }
    inline const rs::Rc<Program> &program_ref() const { return program_; }
    inline const std::string &name() const { return name_; }

private:
    template <typename T, typename ... Args>
    void unwind_args(size_t n, const T &arg, Args &&...args) {
        set_arg(n, arg);
        unwind_args(n + 1, std::forward<Args>(args)...);
    }
    template <typename T>
    void unwind_args(size_t n, const T &arg) {
        set_arg(n, arg);
    }

public:
    template <typename T>
    void set_arg(size_t n, const T &arg) {
        assert_eq_(clSetKernelArg(raw_, n, sizeof(T), (void *)&arg), CL_SUCCESS);
    }
    inline void set_arg(size_t n, const Buffer &buf) {
        set_arg(n, cl_mem(buf));
    }
    rs::Result<> run(Queue &queue, size_t work_size);

    template <typename ... Args>
    rs::Result<> operator()(Queue &queue, size_t work_size, Args &&...args) {
        unwind_args(0, std::forward<Args>(args)...);
        return run(queue, work_size);
    }
};

} // namespace cl