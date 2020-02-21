#include "random.hh"


void rand_init(Rng *rng, uint seed) {
    rng->state = seed;
}

uint rand_int(Rng *rng) {
    return (rng->state = 1103515245*rng->state + 12345);
}

real rand_uniform(Rng *rng) {
    return rand_int(rng)/(real)0x100000000;
}

real3 rand_sphere(Rng *rng) {
    float phi = (real)2*PI*rand_uniform(rng);
    float cos_theta = (real)1 - (real)2*rand_uniform(rng);
    float sin_theta = sqrt((real)1 - cos_theta*cos_theta);
    return make_real3(cos(phi)*sin_theta, sin(phi)*sin_theta, cos_theta);
}

real3 rand_hemisphere(Rng *rng) {
    float phi = (real)2*PI*rand_uniform(rng);
    float cos_theta = rand_uniform(rng);
    float sin_theta = sqrt((real)1 - cos_theta*cos_theta);
    return make_real3(cos(phi)*sin_theta, sin(phi)*sin_theta, cos_theta);
}

real3 rand_hemisphere_cosine(Rng *rng) {
    float phi = (real)2*PI*rand_uniform(rng);
    float sqr_cos_theta = rand_uniform(rng);
    float cos_theta = sqrt(sqr_cos_theta);
    float sin_theta = sqrt((real)1 - sqr_cos_theta);
    return make_real3(cos(phi)*sin_theta, sin(phi)*sin_theta, cos_theta);
}

real3 rand_sphere_cap(Rng *rng, float cos_alpha) {
    float phi = (real)2*PI*rand_uniform(rng);
    float cos_theta = (real)1 - ((real)1 - cos_alpha)*rand_uniform(rng);
    float sin_theta = sqrt((real)1 - cos_theta*cos_theta);
    return make_real3(cos(phi)*sin_theta, sin(phi)*sin_theta, cos_theta);
}
