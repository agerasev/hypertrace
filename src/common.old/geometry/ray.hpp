#pragma once

#include <geometry/euclidean.hpp>
#include <geometry/hyperbolic.hpp>


template <typename G>
class Ray {};

template <>
class Ray<Euclidean> {
public:
    real3 start;
    real3 direction;

    inline Ray() : Ray(real3(0), real3(0,0,1)) {}
    inline Ray(real3 s, real3 d) : start(s), direction(d) {}

    inline Ray map(const Affine<real, 3> &m) const {
        return Ray(m.apply(start), normalize(m.linear().apply(direction)));
    }
    //inline Ray advance(real l) const {}
};

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
