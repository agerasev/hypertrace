#include "devtest.hpp"

#include <fstream>
#include <sstream>
#include <regex>

#include <host/opencl/search.hpp>

using namespace rstd;
using namespace devtest;


Device::Device(cl_device_id id):
    id_(id),
    mutex_(Tuple<>())
{}

cl_device_id Device::id() const {
    return id_;
}
Mutex<Tuple<>> &Device::mutex() const {
    return mutex_;
}

void Device::print_info() const {
    cl::print_device_info(id(), "    ");
    println_();
}

// Platform

Platform::Platform(cl_platform_id id) {
    id_ = id;
    for (cl_device_id devid : cl::get_devices(id_)) {
        devices_.push_back(Device(devid));
    }
}

cl_platform_id Platform::id() const {
    return id_;
}
const std::vector<Device> &Platform::devices() const {
    return devices_;
}

void Platform::print_info() const {
    cl::print_platform_info(id());
    println_();
    for (const Device &device : devices()) {
        device.print_info();
    }
}

// Target

Target::Target(const Device *device) :
    device_(device)
{
    auto _ = device_->mutex().lock();
    context_ = Rc(cl::Context::create(device_->id()).unwrap());
}
Target::~Target() {
    auto _ = device_->mutex().lock();
    context_.try_take().expect("Context is referenced somewhere else");
}

//const Device &Target::device() const {
//    return device_;
//}
cl_device_id Target::device_id() const {
    return device_->id();
}
const Rc<cl::Context> &Target::context() const {
    return context_;
}
Rc<cl::Queue> Target::make_queue() const {
    return Rc(cl::Queue::create(context_, device_->id()).unwrap());
}

// Selector::TargetIter


Selector::TargetIter::TargetIter(const Selector &parent, size_t plpos, size_t devpos) :
    parent(parent), plpos(plpos), devpos(devpos)
{}
bool Selector::TargetIter::operator==(const TargetIter &other) const {
    return
        &parent == &other.parent &&
        plpos == other.plpos &&
        devpos == other.devpos;
}
bool Selector::TargetIter::operator!=(const TargetIter &other) const {
    return !(*this == other);
}
void Selector::TargetIter::operator++() {
    if (devpos + 1 < parent.platforms()[plpos].devices().size()) {
        devpos += 1;
    } else {
        plpos += 1;
        devpos = 0;
    }
}
Target Selector::TargetIter::operator*() const {
    return Target(&(parent.platforms()[plpos].devices()[devpos]));
}

// Selector

Selector::Selector() {
    for (cl_platform_id plid : cl::get_platforms()) {
        platforms_.push_back(Platform(plid));
    }
}

const std::vector<Platform> &Selector::platforms() const {
    return platforms_;
}
void Selector::print_info() const {
    for (const Platform &platform : platforms()) {
        platform.print_info();
    }
}

Selector::TargetIter Selector::begin() const {
    return TargetIter(*this);
}
Selector::TargetIter Selector::end() const {
    return TargetIter(*this, platforms().size());
}

class DropLogger final {
private:
    const std::stringstream &stream;
    std::string filename;
public:
    DropLogger(const std::stringstream &stream, const std::string &filename) :
        stream(stream), filename(filename)
    {}
    DropLogger(const DropLogger &) = delete;
    DropLogger &operator=(const DropLogger &) = delete;
    ~DropLogger() {
        std::string log = stream.str();
        bool empty = std::all_of(log.begin(), log.end(), [](char c) {
            return c == ' ' || c == '\n' || c == '\r';
        });
        if (!empty) {
            std::fstream output(filename, std::ios::out);
            assert_(output.good());
            write_(output, log);
            output.close();
        }
    }
};

KernelBuilder::KernelBuilder(cl_device_id device_id, Rc<cl::Queue> queue) :
    device_id(device_id),
    queue(queue)
{}

KernelBuilder KernelBuilder::source(const std::string &filename, const std::string content) {
    assert_(source_.is_none());
    source_ = Option(std::make_pair(filename, content));
    return std::move(*this);
}

Result<cl::Kernel, std::string> KernelBuilder::build(const std::string &kernel_name) {
    auto src = this->source_.unwrap();
    std::string name = src.first;
    includer includer(
        name,
        std::list<std::string>{"src"},
        std::map<std::string, std::string>{src},
        std::map<std::string, bool>{
            std::make_pair("HOST", false),
            std::make_pair("TEST", false),
            std::make_pair("TEST_UNIT", false),
            std::make_pair("TEST_DEV", false)
        }
    );

    std::stringstream log;
    DropLogger dl(log, format_("build/src/test/output/{}.{}.log", name, device_id));

    bool ires = includer.include();
    writeln_(log, includer.log());
    if (!ires) {
        return Err(format_("Include error:\n{}", log.str()));
    }
    std::fstream(format_("build/src/test/output/{}", name), std::ios::out)
    << std::regex_replace(includer.data(), std::regex("\n\n[\n]+"), "\n\n\n");

    auto prog_and_log = cl::Program::create(queue->context_ref(), device_id, includer);
    writeln_(log, prog_and_log.get<1>());
    
    cl::Program program;
    try_assign_(program, prog_and_log.get<0>().map_err([&](auto) {
        return format_("Program build error:\n{}", log.str());
    }));
    return cl::Kernel::create(Rc(std::move(program)), kernel_name).map_err([&](auto) {
        return format_("Kernel '{}' create error:\n{}", kernel_name, log.str());
    });
}
