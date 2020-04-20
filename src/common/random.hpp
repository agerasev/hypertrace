#pragma once

#include <types.h>
#include <math.hpp>
#include <algebra/vector.hpp>


namespace xrand {

class LCRng {
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

template <typename T, typename Rng>
T uniform(Rng &rng);
template <typename T>
struct Uniform {
    template <typename Rng>
    static T sample(Rng &rng) {
        return T(rng.next()/0x100000000_r);
    }
};
template <typename T, int N>
struct Uniform<vector<T, N>> {
    template <typename Rng>
    static vector<T, N> sample(Rng &rng) {
        vector<T, N> v;
        for (int i = 0; i < N; ++i) {
            v[i] = uniform<T>(rng);
        }
        return v;
    }
};
template <typename T, typename Rng>
T uniform(Rng &rng) {
    return Uniform<T>::sample(rng);
}

inline real3 _sphere_point(real cos_theta, real sin_theta, real phi) {
    return real3(math::cos(phi)*sin_theta, math::sin(phi)*sin_theta, cos_theta);
}

template <typename Rng>
real3 sphere(Rng &rng) {
    real phi = 2*PI*uniform<real>(rng);
    real cos_theta = 1 - 2*uniform<real>(rng);
    real sin_theta = math::sqrt(1 - cos_theta*cos_theta);
    return _sphere_point(cos_theta, sin_theta, phi);
}

template <typename Rng>
real3 hemisphere(Rng &rng) {
    real phi = 2*PI*uniform<real>(rng);
    real cos_theta = uniform<real>(rng);
    real sin_theta = math::sqrt(1 - cos_theta*cos_theta);
    return _sphere_point(cos_theta, sin_theta, phi);
}

template <typename Rng>
real3 hemisphere_cosine(Rng &rng) {
    real phi = 2*PI*uniform<real>(rng);
    real sqr_cos_theta = uniform<real>(rng);
    real cos_theta = math::sqrt(sqr_cos_theta);
    real sin_theta = math::sqrt(1 - sqr_cos_theta);
    return _sphere_point(cos_theta, sin_theta, phi);
}

template <typename Rng>
real3 sphere_cap(Rng &rng, real cos_alpha) {
    real phi = 2*PI*uniform<real>(rng);
    real cos_theta = 1 - (1 - cos_alpha)*uniform<real>(rng);
    real sin_theta = math::sqrt(1 - cos_theta*cos_theta);
    return _sphere_point(cos_theta, sin_theta, phi);
}

}
