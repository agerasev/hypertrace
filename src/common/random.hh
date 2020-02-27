#pragma once

#include <types.hh>
#include <algebra/real.hh>


typedef struct Rng {
    uint state;
} Rng;


void rand_init(Rng *rng, uint seed);

uint rand_int(Rng *rng);
real rand_uniform(Rng *rng);
real3 rand_sphere(Rng *rng);
real3 rand_hemisphere(Rng *rng);
real3 rand_hemisphere_cosine(Rng *rng);
real3 rand_sphere_cap(Rng *rng, float cos_alpha);
