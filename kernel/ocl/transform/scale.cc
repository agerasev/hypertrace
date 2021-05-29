#include "scale.hh"

// Scale2

Scale2 scl2_new(real v) {
    Scale2 m;
    m.v = v;
    return m;
}

Scale2 scl2_identity() {
    return scl2_new(R1);
}

real2 scl2_apply_pos(Scale2 m, real2 v) {
    return m.v * v;
}
real2 scl2_apply_dir(Scale2 m, real2 p, real2 d) {
    return d;
}
real2 scl2_apply_normal(Scale2 m, real2 p, real2 d) {
    return d;
}

Scale2 scl2_chain(Scale2 a, Scale2 b) {
    return scl2_new(a.v * b.v);
}
Scale2 scl2_inverse(Scale2 m) {
    return scl2_new(R1 / m.v);
}

void scl2_shf2_reorder(Scale2 *a, Shift2 *b) {
    b->v = scl2_apply_pos(*a, b->v);
}
void shf2_scl2_reorder(Shift2 *a, Scale2 *b) {
    a->v = scl2_apply_pos(scl2_inverse(*b), a->v);
}

// Scale3

Scale3 scl3_new(real v) {
    Scale3 m;
    m.v = v;
    return m;
}

Scale3 scl3_identity() {
    return scl3_new(R1);
}

real3 scl3_apply_pos(Scale3 m, real3 v) {
    return m.v * v;
}
real3 scl3_apply_dir(Scale3 m, real3 p, real3 d) {
    return d;
}
real3 scl3_apply_normal(Scale3 m, real3 p, real3 d) {
    return d;
}

Scale3 scl3_chain(Scale3 a, Scale3 b) {
    return scl3_new(a.v * b.v);
}
Scale3 scl3_inverse(Scale3 m) {
    return scl3_new(R1 / m.v);
}

void scl3_shf3_reorder(Scale3 *a, Shift3 *b) {
    b->v = scl3_apply_pos(*a, b->v);
}
void shf3_scl3_reorder(Shift3 *a, Scale3 *b) {
    a->v = scl3_apply_pos(scl3_inverse(*b), a->v);
}

#ifdef HOST

std::ostream &operator<<(std::ostream &o, Scale m) {
    return o << "Scale { " << m.v << " }";
}

#endif // HOST
