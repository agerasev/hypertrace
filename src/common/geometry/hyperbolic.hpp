#pragma once

#include <geometry.hpp>

#include <algebra/complex.hpp>
#include <algebra/moebius.hpp>

class Hyperbolic : public Geometry {
public:
    typedef quat Position;
    typedef quat Direction;
    typedef Moebius Map;

    static real distance(Position a, Position b);

    // Returns the direction of the line at point `dst_pos`
    // when we know that the line at the point `src_pos` has direction of `src_dir`.
    static Direction dir_at(Position src_pos, Direction src_dir, Position dst_pos);

    static Map zshift(real l);
    static Map xshift(real l);
    static Map yshift(real l);
    static Map zrotate(real phi);
    static Map xrotate(real theta);
    static Map yrotate(real theta);

    // Move to position at the horosphere.
    static Map horosphere(comp pos);

    // Turns direction `dir` to *j*.
    static Map look_to(Direction dir);

    // Rotatates point `pos` around the origin to make it lay on the z axis.
    static Map look_at(Position pos);

    // Translates point `pos` to the origin preserving orientation
    // relative to the line that connects `pos` to the origin.
    static Map move_at(Position pos);
    static Map move_to(Direction dir, real dist);
};

typedef Hyperbolic Hy;
