#include <real.h>
#include <trace.h>

#include <parameters.h>


__kernel void render(
    __global float *screen,
    __global uchar *image,
    int width, int height,
    int sample_no,
    __global uint *seed,
    
    Parameters params
) {
    int idx = get_global_id(0);

    real2 pos = 2.0f*(real2)(
        (real)(idx % width) - 0.5f*(width - 1),
        (real)(idx / width) - 0.5f*(height - 1)
    )/height;

    float3 color = (float3)(0.0f);

    uint lseed = seed[idx];


    // Enter single ray tracing computation
    trace(
        (float*)&color,
        (const real*)&pos, 2.0f/height,
        &lseed,
        &params
    );

    seed[idx] = lseed;

    float3 avg_color = (color + vload3(idx, screen)*sample_no)/(sample_no + 1);
    vstore3(avg_color, idx, screen);

    float3 out_color = clamp(avg_color, 0.0f, 1.0f);
    uchar4 pix = (uchar4)(convert_uchar3(0xff*out_color), 0xff);
    vstore4(pix, idx, image);
}
