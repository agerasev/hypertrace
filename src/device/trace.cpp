#include <types.h>
#include <real.h>
#include <trace.h>

#include <algebra/vector.hpp>


void trace(float *color_ptr, const real *pos_ptr) {
    float3 &color = *(float3*)color_ptr;
    const real2 &pos = *(const real2*)pos_ptr;

    color = float3(pos[0], pos[1], 1.0f);
}
