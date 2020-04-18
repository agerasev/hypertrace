#include "random.hpp"


namespace xrand {

static real3 _sphere_point(real cos_theta, real sin_theta, real phi) {
    return real3(math::cos(phi)*sin_theta, math::sin(phi)*sin_theta, cos_theta);
}

real3 sphere(Rng &rng) {
    real phi = 2*PI*uniform<real>(rng);
    real cos_theta = 1 - 2*uniform<real>(rng);
    real sin_theta = math::sqrt(1 - cos_theta*cos_theta);
    return _sphere_point(cos_theta, sin_theta, phi);
}

real3 hemisphere(Rng &rng) {
    real phi = 2*PI*uniform<real>(rng);
    real cos_theta = uniform<real>(rng);
    real sin_theta = math::sqrt(1 - cos_theta*cos_theta);
    return _sphere_point(cos_theta, sin_theta, phi);
}

real3 hemisphere_cosine(Rng &rng) {
    real phi = 2*PI*uniform<real>(rng);
    real sqr_cos_theta = uniform<real>(rng);
    real cos_theta = math::sqrt(sqr_cos_theta);
    real sin_theta = math::sqrt(1 - sqr_cos_theta);
    return _sphere_point(cos_theta, sin_theta, phi);
}

real3 sphere_cap(Rng &rng, real cos_alpha) {
    real phi = 2*PI*uniform<real>(rng);
    real cos_theta = 1 - (1 - cos_alpha)*uniform<real>(rng);
    real sin_theta = math::sqrt(1 - cos_theta*cos_theta);
    return _sphere_point(cos_theta, sin_theta, phi);
}

}
