#pragma once

#include <fstream>
#include <rstd/prelude.hpp>

#include <host/opencl/search.hpp>
#include <host/opencl/opencl.hpp>


namespace devtest {

class Device {
private:
    cl_device_id id_;

public:
    Device() = delete;
    Device(const Device &) = delete;
    Device &operator=(const Device &) = delete;

    Device(Device &&) = default;
    Device &operator=(Device &&) = default;

    explicit Device(cl_device_id id);

    cl_device_id id() const;

    void print_info() const;
};

class Platform {
private:
    cl_platform_id id_;
    std::vector<Device> devices_;

public:
    Platform() = delete;
    Platform(const Platform &) = delete;
    Platform &operator=(const Platform &) = delete;

    Platform(Platform &&) = default;
    Platform &operator=(Platform &&) = default;

    explicit Platform(cl_platform_id id);

    cl_platform_id id() const;
    const std::vector<Device> &devices() const;

    void print_info() const;
};

class Target {
private:
    cl_device_id device_id_;
    rstd::Rc<cl::Context> context_;
public:
    Target() = delete;
    Target(const Target &) = delete;
    Target &operator=(const Target &) = delete;

    Target(Target &&) = default;
    Target &operator=(Target &&) = default;

    explicit Target(cl_device_id device_id);
    ~Target();

    //const Device &device() const;
    cl_device_id device_id() const;
    const rstd::Rc<cl::Context> &context() const;
    rstd::Rc<cl::Queue> make_queue() const;
};

class Selector {
public:
    class TargetIter {
    private:
        const Selector &parent;
        size_t plpos, devpos;
    public:
        explicit TargetIter(const Selector &parent, size_t plpos=0, size_t devpos=0);
        bool operator==(const TargetIter &other) const;
        bool operator!=(const TargetIter &other) const;
        void operator++();
        Target operator*() const;
    };

private:
    std::vector<Platform> platforms_;

public:
    Selector();
    Selector(const Selector &) = delete;
    Selector &operator=(const Selector &) = delete;
    Selector(Selector &&) = default;
    Selector &operator=(Selector &&) = default;

    const std::vector<Platform> &platforms() const;
    void print_info() const;

    TargetIter begin() const;
    TargetIter end() const;
};

class KernelBuilder {
private:
    cl_device_id device_id;
    rstd::Rc<cl::Queue> queue;
    rstd::Option<std::pair<std::string, std::string>> source_;
public:
    KernelBuilder() = default;
    KernelBuilder(const KernelBuilder &) = delete;
    KernelBuilder &operator=(const KernelBuilder &) = delete;
    KernelBuilder(KernelBuilder &&) = default;
    KernelBuilder &operator=(KernelBuilder &&) = default;

    KernelBuilder(cl_device_id device_id, rstd::Rc<cl::Queue> queue);
    KernelBuilder source(const std::string &filename, const std::string content);
    rstd::Result<cl::Kernel, std::string> build(const std::string &kernel_name);
};

} // namespace devtest
