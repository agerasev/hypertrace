#include "rotation.hh"

#ifndef OPENCL
#include <math.h>
#endif // OPENCL


rotation2 rot2_new(real angle) {
    return c_new(cos(angle), sin(angle));
}
real2 rot2_apply(rotation2 rot, real2 pos) {
    return c_mul(rot, pos);
}
rotation2 rot2_chain(rotation2 a, rotation2 b) {
    return c_mul(a, b);
}

rotation3 rot3_new(real3 axis, real angle) {
    return q_new(cos(angle/2), sin(angle/2)*axis);
}
real3 rot3_apply(rotation3 rot, real3 pos) {
    return q_div(q_mul(rot, q_new(R0, pos)), rot).yzw;
}
rotation3 rot3_chain(rotation3 a, rotation3 b) {
    return q_mul(a, b);
}

rotation2 rot2_look_at(real2 pos) {
    return c_new(pos.x, pos.y)/length(pos);
}
rotation3 rot3_look_at(real3 pos) {
    real3 dir = normalize(pos);
    real3 axis = make_real3(-dir.y, dir.x, (real)0);
    real axis_length = length(axis);
    if (axis_length > EPS) {
        axis /= axis_length;
        real angle = atan2(axis_length, dir.z);
        return rot3_new(axis, angle);
    } else {
        if (dir.z > (real)0) {
            return q_new(R1, R0, R0, R0);
        } else {
            return q_new(R0, R1, R0, R0);
        }
    }
}

