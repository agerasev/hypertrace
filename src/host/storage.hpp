#pragma once

#include <memory>
#include <vector>


template <typename T>
struct aligned_allocator {
    static_assert(sizeof(T) % alignof(T) == 0);

    typedef T value_type;
    typedef std::size_t size_type;
    typedef std::ptrdiff_t difference_type;

    T *allocate(std::size_t n) {
        return (T *)std::aligned_alloc(alignof(T), n*sizeof(T));
    }
    void deallocate(T *p, std::size_t) {
        std::free(p);
    }
};

template <typename T>
using aligned_vector = std::vector<T, aligned_allocator<T>>;
