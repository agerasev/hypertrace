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

template <typename T>
class aligned_buffer {
private:
    T *data_ = nullptr;
    size_t size_ = 0;
public:
    aligned_buffer() = default;
    explicit aligned_buffer(size_t n) {
        if (n > 0) {
            size_ = n;
            data_ = aligned_allocator<T>().allocate(size_);
            for (size_t i = 0; i < size_; ++i) {
                new(&data_[i]) T();
            }
        }
    }
    ~aligned_buffer() {
        if (data_ != nullptr) {
            for (size_t i = 0; i < size_; ++i) {
                data_[i].~T();
            }
            aligned_allocator<T>().deallocate(data_, size_);
            data_ = nullptr;
            size_ = 0;
        }
    }

    aligned_buffer(const aligned_buffer &other) = delete;
    aligned_buffer &operator=(const aligned_buffer &other) = delete;
    aligned_buffer(aligned_buffer &&other) = delete;
    aligned_buffer &operator=(aligned_buffer &&other) = delete;

    T *data() {
        return data_;
    }
    const T *data() const {
        return data_;
    }
    size_t size() const {
        return size_;
    }

    T &operator[](size_t i) {
        return data_[i];
    }
    const T &operator[](size_t i) const {
        return data_[i];
    }
};
