#pragma once

#include <types.h>
#include <math.hpp>
#include "traits.hpp"
#include "vector.hpp"
#include "complex.hpp"
#include "linear.hpp"


template <typename T, int N>
class Rotation {};

template <typename T>
class Rotation<T, 2> {
private:
    complex<T> c;
public:
    Rotation() = default;
    explicit Rotation(T angle) : c(math::cos(angle), math::sin(angle)) {}
    explicit Rotation(complex<T> c) : c(c) {}
    static Rotation identity() {
        return Rotation(complex<T>(1, 0));
    }
    static Rotation look_at(vector<T, 2> pos) {
        return Rotation(normalize(complex<T>(pos.x, pos.y)));
    }

    complex<T> &comp() {
        return c;
    }
    const complex<T> &comp() const {
        return c;
    }

    vector<T, 2> apply(vector<T, 2> pos) const {
        return (c*complex<T>(pos)).vec();
    }
    friend Rotation chain(Rotation a, Rotation b) {
        return Rotation(a.c*b.c);
    }

    friend Rotation inverse(Rotation a) {
        return Rotation(~a.c);
    }

    Linear<T, 2> to_linear() const {
        return Linear<T, 2>((~c).to_matrix());
    }
};

template <typename T>
class Rotation<T, 3> {
private:
    quaternion<T> c;
public:
    Rotation() = default;
    Rotation(vector<T, 3> axis, T angle) :
        c(math::cos(angle/2), math::sin(angle/2)*axis)
    {}
    explicit Rotation(quaternion<T> c) : c(c) {}
    static Rotation identity() {
        return Rotation(quaternion<T>(1, 0, 0, 0));
    }

    quaternion<T> &quat() {
        return c;
    }
    const quaternion<T> &quat() const {
        return c;
    }

    vector<T, 3> apply(vector<T, 3> pos) const {
        quaternion<T> q(0, pos);
        return (c*q*~c).vec().shuffle(1,2,3);
    }
    friend Rotation chain(Rotation a, Rotation b) {
        return Rotation(a.c*b.c);
    }

    friend Rotation inverse(Rotation a) {
        return Rotation(~a.c);
    }
    
    Linear<T, 3> to_linear() const {
        return Linear<T, 3>(
            1 - 2*c[2]*c[2] - 2*c[3]*c[3],
            2*c[1]*c[2] - 2*c[3]*c[0],
            2*c[1]*c[3] + 2*c[2]*c[0],

            2*c[1]*c[2] + 2*c[3]*c[0],
            1 - 2*c[1]*c[1] - 2*c[3]*c[3],
            2*c[2]*c[3] - 2*c[1]*c[0],

            2*c[1]*c[3] - 2*c[2]*c[0],
            2*c[2]*c[3] + 2*c[1]*c[0],
            1 - 2*c[1]*c[1] - 2*c[2]*c[2]
        );
    }
};

template <typename T, int N>
Rotation<T, N> operator*(Rotation<T, N> a, Rotation<T, N> b) {
    return chain(a, b);
}
template <typename T, int N>
Rotation<T, N> operator!(Rotation<T, N> a) {
    return inverse(a);
}


typedef Rotation<real, 2> Rotation2;
typedef Rotation<real, 3> Rotation3;


#ifdef TEST_UNIT
#include <catch.hpp>
#include "test.hpp"

namespace test {
template <typename T>
class Distrib<Rotation<T, 2>> : Rng {
public:
    Rotation<T, 2> some() {
        return Rotation<T, 2>(2*PI*d<T>().uniform());
    }
};
template <typename T>
class Distrib<Rotation<T, 3>> : Rng {
public:
    Rotation<T, 3> some() {
        return Rotation<T, 3>(
            d<vector<T, 3>>().unit(),
            2*PI*d<T>().uniform()
        );
    }
};
} // namespace test
#endif
