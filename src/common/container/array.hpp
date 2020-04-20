#pragma once

#include <types.h>
#include <traits.hpp>


template <typename T, int N>
class array {
private:
    T elems[N];
    int count;
    
public:
    Array() = default;
    explicit Array(Args ...args) : elems{args...}, count(sizeof...(args)) {}

    T *data() {
        return elems;
    }
    const T *data() const {
        return elems;
    }
    T &operator[](int i) {
        return elems[i];
    }
    const T &operator[](int i) const {
        return elems[i];
    }
    int size() const {
        return count;
    }
    static constexpr int capacity() {
        return N;
    }
};
