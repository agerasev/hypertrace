#pragma once

#include <rstd/prelude.hpp>

#include <common/types.hh>
#include <host/opencl/opencl.hpp>
#include <host/storage.hpp>


namespace devtest {

class Device;

class Target {
public:
    friend class Device;

private:
    const Device *device_;
    rstd::Rc<cl::Context> context_;

    explicit Target(const Device *device);

public:
    Target() = delete;
    Target(const Target &) = delete;
    Target &operator=(const Target &) = delete;

    Target(Target &&) = default;
    Target &operator=(Target &&) = default;

    ~Target();

    //const Device &device() const;
    cl_device_id device_id() const;
    const rstd::Rc<cl::Context> &context() const;
    rstd::Rc<cl::Queue> make_queue() const;
};

class Device {
public:
    friend class Target;

private:
    cl_device_id id_;
    mutable rstd::Mutex<rstd::Tuple<>> mutex_;

public:
    Device() = delete;
    Device(const Device &) = delete;
    Device &operator=(const Device &) = delete;

    Device(Device &&) = default;
    Device &operator=(Device &&) = default;

    explicit Device(cl_device_id id);

    cl_device_id id() const;
    std::string info() const;

    Target make_target() const;
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

    std::string info() const;
};

class Machine {
private:
    std::vector<Platform> platforms_;

public:
    Machine();
    Machine(const Machine &) = delete;
    Machine &operator=(const Machine &) = delete;
    Machine(Machine &&) = default;
    Machine &operator=(Machine &&) = default;

    const std::vector<Platform> &platforms() const;
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

class KernelRunner {
private:
    class KernelArg {
    public:
        virtual ~KernelArg() = default;
        virtual void load() = 0;
    };

    template <typename T>
    class ValueArg : public KernelArg {
    private:
        dev_type<T> dv;
    public:
        ValueArg(const T &v) {
            dev_store(&dv, &v);
        }
        const dev_type<T> &arg() const {
            return dv;
        }
        void load() override {}
    };

    template <typename T>
    class BufferArg : public KernelArg {
    private:
        rstd::Rc<cl::Queue> queue;
        std::vector<T> &hostbuf;
        cl::Buffer devbuf;
    public:
        BufferArg(const rstd::Rc<cl::Queue> &q, std::vector<T> &hb) :
            queue(q), hostbuf(hb)
        {
            aligned_vector<dev_type<T>> tmpbuf(hostbuf.size());
            for (size_t i = 0; i < hostbuf.size(); ++i) {
                dev_store(&tmpbuf[i], &hostbuf[i]);
            }
            devbuf = cl::Buffer::create(*queue, hb.size()*sizeof(dev_type<T>)).expect("Buffer create error");
            devbuf.store(*queue, tmpbuf.data()).expect("Buffer store error");
        }
        const cl::Buffer &arg() const {
            return devbuf;
        }
        void load() override {
            aligned_vector<dev_type<T>> tmpbuf(hostbuf.size());
            devbuf.load(*queue, tmpbuf.data()).expect("Buffer load error");
            for (size_t i = 0; i < hostbuf.size(); ++i) {
                dev_load(&hostbuf[i], &tmpbuf[i]);
            }
        }
    };

private:
    rstd::Rc<cl::Queue> queue;
    rstd::Rc<cl::Kernel> kernel;
    std::vector<std::unique_ptr<KernelArg>> args;

public:
    KernelRunner() = default;
    KernelRunner(const KernelRunner &) = delete;
    KernelRunner &operator=(const KernelRunner &) = delete;
    KernelRunner(KernelRunner &&) = default;
    KernelRunner &operator=(KernelRunner &&) = default;

    inline KernelRunner(const rstd::Rc<cl::Queue> &q, const rstd::Rc<cl::Kernel> &k) :
        queue(q), kernel(k)
    {}
    inline KernelRunner(const rstd::Rc<cl::Queue> &q, cl::Kernel &&k) :
        queue(q), kernel(rstd::Rc(std::move(k)))
    {}

    template <typename T>
    void push_arg(const T &val) {
        auto varg = std::make_unique<ValueArg<T>>(val);
        kernel->set_arg(args.size(), varg->arg());
        args.push_back(std::move(varg));
    }
    template <typename T>
    void push_arg(std::vector<T> &buf) {
        auto barg = std::make_unique<BufferArg<T>>(queue, buf);
        kernel->set_arg(args.size(), barg->arg());
        args.push_back(std::move(barg));
    }
    inline void load_args() {
        for (auto &arg : args) {
            arg->load();
        }
    }

private:
    template <typename T, typename ... Args>
    void unwind_args(const T &arg, Args &&...args) {
        push_arg(arg);
        unwind_args(std::forward<Args>(args)...);
    }
    template <typename T, typename ... Args>
    void unwind_args(T &arg, Args &&...args) {
        push_arg(arg);
        unwind_args(std::forward<Args>(args)...);
    }
    void unwind_args() {}

public:
    template <typename ... Args>
    rstd::Result<> run(size_t work_size, Args &&...args) {
        unwind_args(std::forward<Args>(args)...);
        return kernel->run(*queue, work_size)
        .and_then([&](auto) {
            this->load_args();
            return rstd::Result<>::Ok();
        });
    }
};

} // namespace devtest
