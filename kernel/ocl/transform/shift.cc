#include "shift.hh"

// Shift2

Shift2 shf2_new(real2 v) {
    Shift2 m;
    m.v = v;
    return m;
}

Shift2 shf2_identity() {
    return shf2_new(MAKE(real2)(R0));
}

real2 shf2_apply_pos(Shift2 m, real2 p) {
    return p + m.v;
}
real2 shf2_apply_dir(Shift2 m, real2 p, real2 d) {
    return d;
}
real2 shf2_apply_normal(Shift2 m, real2 p, real2 d) {
    return d;
}

Shift2 shf2_chain(Shift2 a, Shift2 b) {
    return shf2_new(a.v + b.v);
}
Shift2 shf2_inverse(Shift2 m) {
    return shf2_new(-m.v);
}

// Shift3

Shift3 shf3_new(real3 v) {
    Shift3 m;
    m.v = v;
    return m;
}

Shift3 shf3_identity() {
    return shf3_new(MAKE(real3)(R0));
}

real3 shf3_apply_pos(Shift3 m, real3 p) {
    return p + m.v;
}
real3 shf3_apply_dir(Shift3 m, real3 p, real3 d) {
    return d;
}
real3 shf3_apply_normal(Shift3 m, real3 p, real3 d) {
    return d;
}

Shift3 shf3_chain(Shift3 a, Shift3 b) {
    return shf3_new(a.v + b.v);
}
Shift3 shf3_inverse(Shift3 m) {
    return shf3_new(-m.v);
}

#ifdef HOST

std::ostream &operator<<(std::ostream &o, Shift2 m) {
    return o << "Shift2 { " << m.v << " }";
}

std::ostream &operator<<(std::ostream &o, Shift3 m) {
    return o << "Shift3 { " << m.v << " }";
}

#endif // HOST
