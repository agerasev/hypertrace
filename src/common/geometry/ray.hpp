#pragma once

#include <algebra/complex.hpp>
#include <geometry/hyperbolic.hpp>


template <typename G>
class Ray {};

template <>
class Ray<Hyperbolic> {
public:
    quat start;
    quat direction;

    inline Ray() : Ray(1_j, 1_j) {}
    inline Ray(quat s, quat d) : start(s), direction(d) {}

    inline Ray map(Moebius<comp> m) const {
        return Ray(m.apply(start), normalize(m.deriv(start, direction)));
    }
    //inline Ray advance(real l) const {}
};
