#include <types.h>
#include <real.h>
#include <memory.hpp>
#include <algebra/vector.hpp>
#include <view.hpp>
#include <work.hpp>


extern "C" {
void __kernel_trace(
    __global float3 *screen_,
    __global uchar4 *image_,
    int width, int height,
    int sample_no,
    __global uint *seed_,
    View view
) {
    global_ptr<float3> screen(screen_);
    global_ptr<uchar4> image(image_);
    global_ptr<uint> seed(seed_);

    int idx = work::get_global_id(0);

    real2 pos = 2*real2(
        (real)(idx % width) - 0.5_r*(width - 1),
        (real)(idx / width) - 0.5_r*(height - 1)
    )/height;

    float3 color(0);

    uint lseed = seed.load(idx);


    seed.store(lseed, idx);

    float3 avg_color = (color + screen.load(idx)*sample_no)/(sample_no + 1);
    screen.store(avg_color, idx);

    float3 out_color = clamp(avg_color, float3(0), float3(1));
    uchar4 pix = uchar4(convert<uchar3>(0xff*out_color), 0xff);
    image.store(pix, idx);
}
};
