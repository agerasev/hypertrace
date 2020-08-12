#pragma once

#include "container/variant.hpp"


template <typename T>
class Option : public Variant<T, void> {
public:
    static Option<T> Some(T &&x) {
        
    }
    static Option<T> Some(T x) {
        Some(std::move(x));
    }
};


template <typename T, typename E>
class Result : public Variant<T, E> {

};
