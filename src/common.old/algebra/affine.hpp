#pragma once

#include "traits.hpp"
#include "vector.hpp"
#include "matrix.hpp"
#include "linear.hpp"


template <typename T, int N>
class Affine {
private:
    Linear<T, N> m;
    vector<T, N> v;
public:
    Affine() = default;
    explicit Affine(
        const matrix<T, N, N> &m,
        const vector<T, N> &v=zero<vector<T, N>>()
    ) : m(m), v(v) {}
    explicit Affine(
        const Linear<T, N> &m,
        const vector<T, N> &v=zero<vector<T, N>>()
    ) : m(m), v(v) {}

    static Affine identity() {
        return Affine(
            Linear<T, N>::identity(),
            zero<vector<T, N>>()
        );
    }

    matrix<T, N, N> &mat() { return m.mat(); }
    const matrix<T, N, N> &mat() const { return m.mat(); }
    Linear<T, N> &linear() { return m; }
    const Linear<T, N> &linear() const { return m; }
    vector<T, N> &vec() { return v; }
    const vector<T, N> &vec() const { return v; }
    vector<T, N> &shift() { return v; }
    const vector<T, N> &shift() const { return v; }

    vector<T, N> apply(vector<T, N> p) const {
        return m.apply(p) + v;
    }
    Affine &operator*=(Affine a) {
        return *this = chain(*this, a);
    }
};

template <typename T, int N>
Affine<T, N> chain(Affine<T, N> a, Affine<T, N> b) {
    return Affine<T, N>(
        a.linear()*b.linear(),
        a.vec() + a.linear().apply(b.vec())
    );
}
template <typename T, int N>
Affine<T, N> operator*(Affine<T, N> a, Affine<T, N> b) {
    return chain(a, b);
}

template <typename T, int N>
Affine<T, N> inverse(Affine<T, N> a) {
    Linear<T, N> m = inverse(a.linear());
    return Affine<T, N>(m, -m.apply(a.vec()));
}
template <typename T, int N>
Affine<T, N> operator!(Affine<T, N> a) {
    return inverse(a);
}

template <typename T, int N>
Affine<T, N> interpolate(Affine<T, N> a, Affine<T, N> b, T t) {
    // FIXME: Use matrix power operation
    return Affine<T, N>(
        (1 - t)*a.mat() + t*b.mat(),
        (1 - t)*a.vec() + t*b.vec()
    );
}

#ifdef HOST

template <typename T, int N>
struct ToDevice<Affine<T, N>> {
    struct type {
        device_type<Linear<T, N>> m;
        device_type<vector<T, N>> v;
    };
    static type to_device(Affine<T, N> a) {
        return type {
            .m = ::to_device(a.linear()),
            .v = ::to_device(a.shift()),
        };
    }
};

#endif

#ifdef TEST_UNIT
#include <catch.hpp>
#include "test.hpp"

namespace test {

template <typename T, int N>
class Distrib<Affine<T, N>> : public Rng {
public:
    Affine<T, N> some() {
        return Affine<T, N>(
            d<matrix<T, N, N>>().invertible(),
            d<vector<T, N>>().normal()
        );
    }
};

} // namespace test

#endif
