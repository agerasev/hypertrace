#include "quaternion.h"


quaternion::quaternion() = default;
quaternion::quaternion(real r, real i, real j, real k) {
    *this = q_new(r, i, j, k);
}
quaternion::quaternion(real r) {
    *this = q_new_r(r);
}
quaternion::quaternion(complex c) {
    *this = q_new_c(c);
}

quaternion quaternion::conj() const {
    return q_conj(*this);
}
real quaternion::abs2() const {
    return q_abs2(*this);
}
real quaternion::abs() const {
    return q_abs(*this);
}
quaternion quaternion::norm() const {
    return q_norm(*this);
}
quaternion quaternion::inv() const {
    return q_inv(*this);
}

quaternion &quaternion::operator+=(quaternion b) {
    return *this = qq_add(*this, b);
}
quaternion &quaternion::operator+=(complex b) {
    return *this = qc_add(*this, b);
}
quaternion &quaternion::operator+=(real b) {
    return *this = qr_add(*this, b);
}
quaternion &quaternion::operator-=(quaternion b) {
    return *this = qq_sub(*this, b);
}
quaternion &quaternion::operator-=(complex b) {
    return *this = qc_sub(*this, b);
}
quaternion &quaternion::operator-=(real b) {
    return *this = qr_sub(*this, b);
}
quaternion &quaternion::operator*=(quaternion b) {
    return *this = qq_mul(*this, b);
}
quaternion &quaternion::operator*=(complex b) {
    return *this = qc_mul(*this, b);
}
quaternion &quaternion::operator*=(real b) {
    return *this = qr_mul(*this, b);
}
quaternion &quaternion::operator/=(quaternion b) {
    return *this = qq_div(*this, b);
}
quaternion &quaternion::operator/=(complex b) {
    return *this = qc_div(*this, b);
}
quaternion &quaternion::operator/=(real b) {
    return *this = qr_div(*this, b);
}


const quaternion
    quaternion::i = quaternion(0,1,0,0),
    quaternion::j = quaternion(0,0,1,0),
    quaternion::k = quaternion(0,0,0,1);

quaternion operator+(quaternion a) {
    return a;
}
quaternion operator-(quaternion a) {
    return q_neg(a);
}

quaternion operator+(quaternion a, quaternion b) {
    return qq_add(a, b);
}
quaternion operator+(quaternion a, real b) {
    return qr_add(a, b);
}
quaternion operator+(real a, quaternion b) {
    return rq_add(a, b);
}
quaternion operator+(quaternion a, complex b) {
    return qc_add(a, b);
}
quaternion operator+(complex a, quaternion b) {
    return cq_add(a, b);
}

quaternion operator-(quaternion a, quaternion b) {
    return qq_sub(a, b);
}
quaternion operator-(quaternion a, real b) {
    return qr_sub(a, b);
}
quaternion operator-(real a, quaternion b) {
    return rq_sub(a, b);
}
quaternion operator-(quaternion a, complex b) {
    return qc_sub(a, b);
}
quaternion operator-(complex a, quaternion b) {
    return cq_sub(a, b);
}

quaternion operator*(quaternion a, quaternion b) {
    return qq_mul(a, b);
}
quaternion operator*(quaternion a, real b) {
    return qr_mul(a, b);
}
quaternion operator*(real a, quaternion b) {
    return rq_mul(a, b);
}
quaternion operator*(quaternion a, complex b) {
    return qc_mul(a, b);
}
quaternion operator*(complex a, quaternion b) {
    return cq_mul(a, b);
}

quaternion operator/(quaternion a, real b) {
    return qr_div(a, b);
}
quaternion operator/(quaternion a, quaternion b) {
    return qq_div(a, b);
}
quaternion operator/(real a, quaternion b) {
    return rq_div(a, b);
}
quaternion operator/(quaternion a, complex b) {
    return qc_div(a, b);
}
quaternion operator/(complex a, quaternion b) {
    return cq_div(a, b);
}

real dot(quaternion a, quaternion b) {
    return qq_dot(a, b);
}

std::ostream &operator<<(std::ostream &s, quaternion c) {
    return s << "(" << c.x << ", " << c.y << ", " << c.z << ", " << c.w << ")";
}

quaternion operator "" _j(long double v) {
    return real(v)*quaternion::j;
}
quaternion operator "" _j(unsigned long long v) {
    return real(v)*quaternion::j;
}
quaternion operator "" _k(long double v) {
    return real(v)*quaternion::k;
}
quaternion operator "" _k(unsigned long long v) {
    return real(v)*quaternion::k;
}


#ifdef UNIT_TEST
#include <catch.hpp>

#include <vector>
#include <utility>
#include <functional>

q_approx::q_approx(quaternion q) : v(q) {}

bool operator==(quaternion a, q_approx b) {
    return
        a.x == Approx(b.v.x) &&
        a.y == Approx(b.v.y) &&
        a.z == Approx(b.v.z) &&
        a.w == Approx(b.v.w);
}
bool operator==(q_approx a, quaternion b) {
    return b == a;
}
std::ostream &operator<<(std::ostream &s, q_approx a) {
    return s << a.v;
}

quaternion rand_q_normal(Rng &rng) {
    return quaternion(rng.normal(), rng.normal(), rng.normal(), rng.normal());
}
quaternion rand_q_nonzero(Rng &rng) {
    quaternion a;
    do {
        a = rand_q_normal(rng);
    } while(a.abs2() < TEST_EPS);
    return a;
}
quaternion rand_q_unit(Rng &rng) {
    quaternion q = rand_q_nonzero(rng);
    return q/q.abs();
}

TEST_CASE("Quaternions", "[quaternion]") {
    Rng rng(0xfeed);

    SECTION("Imaginary units") {
        REQUIRE(1_i*1_i == q_approx(-1));
        REQUIRE(1_j*1_j == q_approx(-1));
        REQUIRE(1_k*1_k == q_approx(-1));
        REQUIRE(1_i*1_j*1_k == q_approx(-1));

        REQUIRE(1_i*1_j == q_approx(1_k));
        REQUIRE(1_j*1_k == q_approx(1_i));
        REQUIRE(1_k*1_i == q_approx(1_j));
        
        REQUIRE(1_j*1_i == q_approx(-1_k));
        REQUIRE(1_k*1_j == q_approx(-1_i));
        REQUIRE(1_i*1_k == q_approx(-1_j));
    }
    SECTION("Inversion") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quaternion a = rand_q_nonzero(rng);
            REQUIRE(a/a == q_approx(1));
        }
    }
    SECTION("Law of cosines") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quaternion a = rand_q_normal(rng), b = rand_q_normal(rng);
            REQUIRE(a.abs2() + b.abs2() + 2*dot(a, b) == Approx((a + b).abs2()));
        }
    }
    SECTION("Derivation") {
        std::vector<std::pair<
            std::function<quaternion(quaternion)>,
            std::function<quaternion(quaternion, quaternion)>
        >> cases = {
            std::make_pair(
                [](quaternion p) { return p; },
                [](quaternion, quaternion v) { return v; }
            ),
            std::make_pair(
                [](quaternion p) { return p*p; },
                [](quaternion p, quaternion v) { return p*v + v*p; }
            ),
            std::make_pair(
                [](quaternion p) { return 1/p; },
                [](quaternion p, quaternion v) {
                    real p2 = p.abs2();
                    return (v.conj() - (2*dot(p, v)/p2)*p.conj())/p2;
                }
            )
        };

        for (auto p : cases) {
            auto f = p.first;
            auto dfdv = p.second;
            for (int i = 0; i < TEST_ATTEMPTS; ++i) {
                quaternion p = rand_q_normal(rng);
                quaternion v = rand_q_unit(rng);
                REQUIRE((f(p + TEST_EPS*v) - f(p))/TEST_EPS == q_approx(dfdv(p, v)));
            }
        }
    }
    SECTION("Literal") {
        REQUIRE(-2_i == q_approx(quaternion(0,-2,0,0)));
        REQUIRE(-3_j == q_approx(quaternion(0,0,-3,0)));
        REQUIRE(-4_k == q_approx(quaternion(0,0,0,-4)));
    }
};

#endif // UNIT_TEST
