#pragma once

template <typename T>
struct Group {};

template <typename T>
T chain(T a, T b) {
    return Group<T>::chain(a, b);
}
