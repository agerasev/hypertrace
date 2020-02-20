#pragma once

#include <types.h>

#define _RAND_PI 3.14159265358979323846f


typedef struct Rng {
    uint32_t state;
} Rng;


void rand_init(Rng *rng, uint32_t seed) {
    rng->state = seed;
}

uint32_t rand_int(Rng *rng) {
    return (rng->state = 1103515245*rng->state + 12345);
}

float rand_uniform(Rng *rng) {
    return rand_int(rng)/(float)0x100000000;
}
/*
quaternion rand_sphere(Rng *rng) {
    float phi = (real)2*_RAND_PI*rand_uniform(rng);
    float cos_theta = (real)1 - (real)2*rand_uniform(rng);
    float sin_theta = sqrt((real)1 - cos_theta*cos_theta);
    return q_new(cos(phi)*sin_theta, sin(phi)*sin_theta, cos_theta, (real)0);
}

quaternion rand_hemisphere(Rng *rng) {
    float phi = (real)2*_RAND_PI*rand_uniform(rng);
    float cos_theta = rand_uniform(rng);
    float sin_theta = sqrt((real)1 - cos_theta*cos_theta);
    return q_new(cos(phi)*sin_theta, sin(phi)*sin_theta, cos_theta, (real)0);
}

quaternion rand_hemisphere_cosine(Rng *rng) {
    float phi = (real)2*_RAND_PI*rand_uniform(rng);
    float sqr_cos_theta = rand_uniform(rng);
    float cos_theta = sqrt(sqr_cos_theta);
    float sin_theta = sqrt((real)1 - sqr_cos_theta);
    return q_new(cos(phi)*sin_theta, sin(phi)*sin_theta, cos_theta, (real)0);
}

quaternion rand_sphere_cap(Rng *rng, float cos_alpha) {
    float phi = (real)2*_RAND_PI*rand_uniform(rng);
    float cos_theta = (real)1 - ((real)1 - cos_alpha)*rand_uniform(rng);
    float sin_theta = sqrt((real)1 - cos_theta*cos_theta);
    return q_new(cos(phi)*sin_theta, sin(phi)*sin_theta, cos_theta, (real)0);
}
*/
