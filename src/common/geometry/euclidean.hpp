#pragma once

#include "geometry.hpp"

#include <algebra/vector.hpp>
#include <algebra/affine.hpp>

class Euclidean {
// : public Geometry
public:
    typedef real3 Position;
    typedef real3 Direction;
    typedef Affine<real, 3> Map;

    static Position origin();
    static real3 dir_to_local(Position pos, Direction dir);
    static Direction dir_from_local(Position pos, real3 dir);

    static real length(Position a);
    static real distance(Position a, Position b);

    // Returns the direction of the line at point `dst_pos`
    // when we know that the line at the point `src_pos` has direction of `src_dir`.
    static Direction dir_at(Position src_pos, Direction src_dir, Position dst_pos);

    static Map shift(Direction pos);
    inline static Map xshift(real l) { return shift(Direction(l,0,0)); }
    inline static Map yshift(real l) { return shift(Direction(0,l,0)); }
    inline static Map zshift(real l) { return shift(Direction(0,0,l)); }

    static Map rotate(Direction axis, real phi);
    inline static Map xrotate(real angle) { return rotate(Direction(1,0,0), angle); }
    inline static Map yrotate(real angle) { return rotate(Direction(0,1,0), angle); }
    inline static Map zrotate(real angle) { return rotate(Direction(0,0,1), angle); }

    // Turns direction `dir` to *j*.
    static Map look_to(Direction dir);

    // Rotatates point `pos` around the origin to make it lay on the z axis.
    static Map look_at(Position pos);

    // Translates point `pos` to the origin preserving orientation
    // relative to the line that connects `pos` to the origin.
    static Map move_at(Position pos);
    static Map move_to(Direction dir, real dist);
};
typedef Euclidean Eu;

namespace euclidean {}
namespace eu = euclidean;
