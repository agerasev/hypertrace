#include "iface.h"


__kernel void trace_kernel(
    __global float3 *screen,
    __global uchar4 *image,
    int width, int height,
    int sample_no,
    __global uint *seed,
    _View view
) {
    trace_iface(
        (__global _float3 *)screen,
        (__global _uchar4 *)image,
        width, height,
        sample_no,
        seed,
        &view
    );
}
