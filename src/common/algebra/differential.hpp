#pragma once

#include <types.h>
#include "vector.hpp"


template <typename F>
struct Gradient {
    static real3 apply(F f, real3 p) {
        return real3(
            f(p + real3(EPS, 0, 0)) - f(p - real3(EPS, 0, 0)),
            f(p + real3(0, EPS, 0)) - f(p - real3(0, EPS, 0)),
            f(p + real3(0, 0, EPS)) - f(p - real3(0, 0, EPS))
        )/(2*EPS);
    }
};
template <typename F>
real3 gradient(F f, real3 p) {
    return Gradient<F>::apply(f, p);
}
