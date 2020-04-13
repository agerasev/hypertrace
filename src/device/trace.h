#pragma once

#include <types.h>
#include <real.h>


#ifdef __cplusplus
extern "C" {
#endif

void trace(
    float *out_color, // output color
    const real *pix_pos, real pix_size, // pixel position and size
    uint *rng_seed // random number generator seed
);

#ifdef __cplusplus
};
#endif
