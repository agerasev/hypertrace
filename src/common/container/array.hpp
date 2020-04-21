#pragma once

#include <types.h>
#include <traits.hpp>


template <typename T, int N>
class Array {
private:
    T elems[N];
    int count;
    
public:
    Array() : count(0) {}
    template <typename ...Args>
    explicit Array(Args ...args) : elems{args...}, count(sizeof...(Args)) {
        static_assert(sizeof...(Args) <= N, "Too many arguments");
    }

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
    int &size() {
        return count;
    }
    static constexpr int capacity() {
        return N;
    }
};
