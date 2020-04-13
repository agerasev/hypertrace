#pragma once

#include <algebra/complex.hpp>
#include <algebra/moebius.hpp>


namespace hyperbolic {

class Ray {
public:
    quat start;
    quat direction;

    inline Ray() : Ray(1_j, 1_j) {}
    inline Ray(quat s, quat d) : start(s), direction(d) {}

    inline Ray map(Moebius m) const {
        return Ray(m.apply(start), normalize(m.deriv(start, direction)));
    }
};

} // namespace hyperbolic
