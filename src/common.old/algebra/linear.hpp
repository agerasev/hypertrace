#pragma once

#include "traits.hpp"
#include "vector.hpp"
#include "matrix.hpp"


template <typename T, int N>
class Linear {
private:
    matrix<T, N, N> m;
public:
    Linear() = default;
    template <typename ...Args>
    explicit Linear(Args ...args) : m(args...) {}
    explicit Linear(const matrix<T, N, N> &m) : m(m) {}

    static Linear identity() {
        return Linear(one<matrix<T, N, N>>());
    }
    template <int K=N>
    static enable_if<K == 3, Linear<T, 3>> look_to(vector<T, 3> dir, vector<T, 3> up) {
        vector<T, N> right = normalize(cross(dir, up));
        vector<T, N> down = cross(dir, right);
        return Linear<T, 3>(right, down, dir);
    }
    template <int K=N>
    static enable_if<K == 3, Linear<T, 3>> look_to(vector<T, 3> dir) {
        if (2*math::abs(dir[2]) < 1) {
            return look_to(dir, vector<T, 3>(0,0,1));
        } else {
            return look_to(dir, vector<T, 3>(0,1,0));
        }
    }

    matrix<T, N, N> &mat() {
        return m;
    }
    const matrix<T, N, N> &mat() const {
        return m;
    }

    vector<T, N> apply(vector<T, N> p) const {
        return dot(m, p);
    }

    Linear &operator*=(Linear a) {
        return *this = chain(*this, a);
    }
};

template <typename T, int N>
Linear<T, N> chain(Linear<T, N> a, Linear<T, N> b) {
    return Linear<T, N>(dot(a.mat(), b.mat()));
}
template <typename T, int N>
Linear<T, N> operator*(Linear<T, N> a, Linear<T, N> b) {
    return chain(a, b);
}

template <typename T, int N>
Linear<T, N> inverse(Linear<T, N> a) {
    return Linear<T, N>(inverse(a.mat()));
}
template <typename T, int N>
Linear<T, N> operator!(Linear<T, N> a) {
    return inverse(a);
}

template <typename T, int N>
Linear<T, N> interpolate(Linear<T, N> a, Linear<T, N> b, T t) {
    // FIXME: Use matrix power operation
    return Linear<T, N>((1 - t)*a.mat() + t*b.mat());
}

#ifdef HOST

template <typename T, int N>
struct ToDevice<Linear<T, N>> {
    typedef device_type<matrix<T, N, N>> type;
    static type to_device(Linear<T, N> m) {
        return ::to_device(m.mat());
    }
};

#endif

#ifdef TEST_UNIT
#include <catch.hpp>
#include "test.hpp"

namespace test {

template <typename T, int N>
class Distrib<Linear<T, N>> : public Rng {
public:
    Linear<T, N> some() {
        return Linear<T, N>(d<matrix<T, N, N>>().invertible());
    }
};

} // namespace test

#endif
