#pragma once

#include <types.h>
#include <math.hpp>
#include <algebra/vector.hpp>


namespace xrand {

template <typename T>
class RngBase {
public:
    T next();
    T state() const;
};

class LCRng : public RngBase<uint> {
private:
    uint state_;
public:
    inline LCRng(uint seed) : state_(seed) {}
    inline uint next() {
        return (state_ = 1103515245*state_ + 12345);
    }
    inline uint state() const {
        return state_;
    }
};

typedef LCRng Rng;

template <typename T>
T uniform(Rng &rng);
template <typename T>
struct Uniform {
    static T sample(Rng &rng) {
        return T(rng.next()/0x100000000_r);
    }
};
template <typename T, int N>
struct Uniform<vector<T, N>> {
    static vector<T, N> sample(Rng &rng) {
        vector<T, N> v;
        for (int i = 0; i < N; ++i) {
            v[i] = uniform<T>(rng);
        }
        return v;
    }
};
template <typename T>
T uniform(Rng &rng) {
    return Uniform<T>::sample(rng);
}

real3 sphere(Rng &rng);
real3 hemisphere(Rng &rng);
real3 hemisphere_cosine(Rng &rng);
real3 sphere_cap(Rng &rng, real cos_alpha);

}

using Rng = xrand::Rng;
