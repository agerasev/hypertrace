#pragma once

#include <common/types.hh>
#include <common/algebra/real.hh>
#include <common/algebra/vector.hh>


typedef struct Rng {
    uint state;
} Rng;

Rng rng_init(uint seed);
uint rng_next(Rng *rng);

real random_uniform(Rng *rng);
real2 random_uniform2(Rng *rng);
real3 random_uniform3(Rng *rng);
real4 random_uniform4(Rng *rng);

real3 random_sphere(Rng *rng);
real3 random_hemisphere(Rng *rng);
real3 hemisphere_cosine(Rng *rng);
real3 sphere_cap(Rng *rng, real cos_alpha);

#ifndef HOST
#include "random.cc"
#endif // HOST
