#pragma once

#include <algebra/complex.hpp>
#include <object.hpp>
#include <geometry/hyperbolic.hpp>
#include <geometry/ray.hpp>
#include <render/context.hpp>

namespace hyperbolic {
class Horosphere : Object<Hy> {
public:
    struct Cache : Object<Hy>::Cache {
        quat pos;
        quat dir;
    };

    /*
    enum TilingType : uint {
        SQUARE = 1,
        HEXAGONAL = 2
    };
    */

    real hit(Context &context, Cache &cache, Ray ray) const;
    bool bounce(
        Context &context, const Cache &cache,
        float3 &luminance, Ray &new_ray
    ) const;
};
}
