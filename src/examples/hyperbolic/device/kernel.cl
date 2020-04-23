#include "iface.h"


__kernel void trace_kernel(
    __global float *screen,
    __global uchar *image,
    int width, int height,
    int sample_no,
    __global uint *seed,
    _View view,
    __global const _MyObject *objects,
    int object_count
) {
    trace_iface(
        screen,
        image,
        width, height,
        sample_no,
        seed,
        &view,
        objects,
        object_count
    );
}
