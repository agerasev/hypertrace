#pragma once

#include <types.h>
#include <real.h>

#include <parameters.h>


#ifdef __cplusplus
extern "C" {
#endif

void trace(
    float *out_color, // output color
    const real *pix_pos, real pix_size, // pixel position and size
    uint *rng_seed, // random number generator seed
    const void __attribute__((aligned(PARAMS_ALIGN))) *params // parameters
);

#ifdef __cplusplus
};
#endif
