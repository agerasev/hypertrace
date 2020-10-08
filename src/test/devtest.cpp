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
