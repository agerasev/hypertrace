#include "moebius.h"


Moebius::Moebius() = default;
Moebius::Moebius(complex a, complex b, complex c, complex d) {
    moebius_new(this, a, b, c, d);
}

quaternion Moebius::apply(quaternion p) const {
    return moebius_apply(this, p);
}

quaternion Moebius::deriv(quaternion p, quaternion v) const {
    return moebius_deriv(this, p, v);
}

Moebius Moebius::operator*(const Moebius &other) const {
    Moebius r;
    moebius_chain(&r, this, &other);
    return r;
}
Moebius &Moebius::operator*=(const Moebius &other) {
    Moebius t;
    moebius_chain(&t, this, &other);
    *this = t;
    return *this;
}

complex Moebius::det() const {
    return moebius_det(this);
}
Moebius Moebius::inverse() const {
    Moebius r;
    moebius_inverse(&r, this);
    return r;
}

Moebius Moebius::identity() {
    Moebius r;
    moebius_new_identity(&r);
    return r;
}
Moebius Moebius::zshift(real l) {
    Moebius r;
    moebius_new_zshift(&r, l);
    return r;
}
Moebius Moebius::xshift(real l) {
    Moebius r;
    moebius_new_xshift(&r, l);
    return r;
}
Moebius Moebius::yshift(real l) {
    Moebius r;
    moebius_new_yshift(&r, l);
    return r;
}
Moebius Moebius::zrotate(real phi) {
    Moebius r;
    moebius_new_zrotate(&r, phi);
    return r;
}
Moebius Moebius::xrotate(real theta) {
    Moebius r;
    moebius_new_xrotate(&r, theta);
    return r;
}
Moebius Moebius::yrotate(real theta) {
    Moebius r;
    moebius_new_yrotate(&r, theta);
    return r;
}
Moebius Moebius::lookat(quaternion dir) {
    Moebius r;
    moebius_new_lookat(&r, dir);
    return r;
}

#ifdef OPENCL_INTEROP

MoebiusPacked Moebius::pack() const {
    MoebiusPacked p;
    moebius_pack(&p, this);
    return p;
}

Moebius MoebiusPacked::unpack() const {
    Moebius u;
    moebius_unpack(&u, this);
    return u;
}

#endif // OPENCL_INTEROP

std::ostream &operator<<(std::ostream &s, const Moebius &m) {
    return s << "[" << m.a << ", " << m.b << ", " << m.c << ", " << m.d << "]";
}


#ifdef UNIT_TEST

#include <catch.hpp>


Moebius random_moebius(Rng &rng) {
    return Moebius(
        rand_c_normal(rng),
        rand_c_normal(rng),
        rand_c_normal(rng),
        rand_c_normal(rng)
    );
}

TEST_CASE("Moebius transformation", "[moebius]") {
    Rng rng;

    SECTION("Chaining") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius a = random_moebius(rng), b = random_moebius(rng);
            quaternion c = rand_q_normal(rng);
            REQUIRE((a*b).apply(c) == q_approx(a.apply(b.apply(c))));
        }
    }

    SECTION("Derivation") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius a = random_moebius(rng);
            quaternion p = rand_q_normal(rng);
            quaternion v = rand_q_nonzero(rng);
            
            REQUIRE(a.deriv(p, v) == q_approx((a.apply(p + TEST_EPS*v) - a.apply(p))/TEST_EPS));
        }
    }

    SECTION("Rotation of derivative") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quaternion q = q_norm(q_new(rng.normal(), rng.normal(), 1, 0));
            real phi = -atan2(q.y, q.x);
            real theta = -atan2(sqrt(q.x*q.x + q.y*q.y), q.z);

            Moebius a, b, c;
            moebius_new_zrotate(&a, phi);
            moebius_new_xrotate(&b, theta);
            moebius_chain(&c, &b, &a);
            REQUIRE(c.deriv(quaternion::j, q) == q_approx(quaternion::j));
        }
    }
};

#endif // UNIT_TEST
