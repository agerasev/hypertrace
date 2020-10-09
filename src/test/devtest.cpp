#include "devtest.hpp"

using namespace rstd;
using namespace devtest;


Device::Device(cl_device_id id) {
    id_ = id;
}

cl_device_id Device::id() const {
    return id_;
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

Target::Target(cl_device_id device_id) :
    device_id_(device_id)
{
    context_ = Rc(cl::Context::create(device_id_).unwrap());
}
Target::~Target() {
    context_.try_take().expect("Context is referenced somewhere else");
}

//const Device &Target::device() const {
//    return device_;
//}
cl_device_id Target::device_id() const {
    return device_id_;
}
const Rc<cl::Context> &Target::context() const {
    return context_;
}
Rc<cl::Queue> Target::make_queue() const {
    return Rc(cl::Queue::create(context_, device_id_).unwrap());
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
    return Target(parent.platforms()[plpos].devices()[devpos].id());
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
            std::make_pair("TEST_UNIT", false),
            std::make_pair("TEST_DEV", false)
        }
    );

    bool ires = includer.include();
    std::fstream log(format_("build/src/test/{}.log", name), std::ios::out);
    log << includer.log() << std::endl;
    if (!ires) {
        return Result<cl::Kernel, std::string>::Err("Include error:\n" + includer.log());
    }
    std::fstream(format_("build/src/test/{}", name), std::ios::out) << includer.data();

    auto prog_and_log = cl::Program::create(queue->context_ref(), device_id, includer);
    log << prog_and_log.get<1>() << std::endl;
    
    return prog_and_log.get<0>().map_err([&](auto) {
        return prog_and_log.get<1>();
    }).and_then([&](auto program) {
        return cl::Kernel::create(Rc(std::move(program)), kernel_name).map_err([&](auto) {
            return format_("Kernel '{}' create error", kernel_name);
        });
    });
}
