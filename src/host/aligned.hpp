#pragma once
#pragma GCC diagnostic ignored "-Wignored-attributes"

#include <cstdlib>
#include <memory>
#include <common/math.hh>


void *aligned_alloc_(size_t align, size_t size, void **orig);

template <typename T=void>
class AlignedMem;

template <>
class AlignedMem<void> {
private:
    size_t align_, size_;
    void *orig_ = nullptr;
    void *data_ = nullptr;
    
public:
    AlignedMem() = default;
    AlignedMem(size_t align, size_t size) {
        align_ = align;
        size_ = size;
        data_ = aligned_alloc_(align, size, &orig_);
    }
    ~AlignedMem() {
        if (orig_ != nullptr) {
            std::free(orig_);
        }
    }
    
    AlignedMem(const AlignedMem &) = delete;
    AlignedMem &operator=(const AlignedMem &) = delete;

    AlignedMem(AlignedMem &&other) {
        data_ = other.data_;
        other.data_ = nullptr;
        orig_ = other.orig_;
        other.orig_ = nullptr;
    }
    AlignedMem &operator=(AlignedMem &&other) {
        this->~AlignedMem();
        new(this) AlignedMem(std::move(other));
        return *this;
    }

    size_t size() const { return size_; }
    size_t align() const { return align_; }

    void *data() { return data_; }
    const void *data() const { return data_; }
};

template <typename T>
class AlignedMem : protected AlignedMem<void> {
public:
    AlignedMem() = default;
    AlignedMem(size_t len) :
        AlignedMem<void>(alignof(T), len*sizeof(T))
    {}
    ~AlignedMem() = default;
    
    size_t size() const {
        return AlignedMem<void>::size()/sizeof(T);
    }
    size_t align() const {
        return AlignedMem<void>::align();
    }

    T *data() {
        return (T *)AlignedMem<void>::data();
    }
    const T *data() const {
        return (const T *)AlignedMem<void>::data();
    }

    T &operator[](size_t i) {
        return data()[i];
    }
    const T &operator[](size_t i) const {
        return data()[i];
    }
};
