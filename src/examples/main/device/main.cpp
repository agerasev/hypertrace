#include "iface.h"

#include <types.h>
#include <real.h>
#include <memory.hpp>

#include <algebra/vector.hpp>
#include <algebra/complex.hpp>

#include <view.hpp>
#include <work.hpp>

#include <geometry/hyperbolic.hpp>
#include <geometry/ray.hpp>
#include <random.hpp>

#include <object/hyperbolic/horosphere.hpp>

#include "render/renderer.hpp"


#define ASSERT_DUMMY(Orig, Dummy) \
static_assert(sizeof(Orig) == sizeof(Dummy), "Dummy structure size mismatch"); \
static_assert(alignof(Orig) == alignof(Dummy), "Dummy structure alignment mismatch"); \

ASSERT_DUMMY(_View, View<Hy>);


using namespace hyperbolic;

void trace(
    global_ptr<float> screen,
    global_ptr<uchar> image,
    int width, int height,
    int sample_no,
    global_ptr<uint> seed,
    const View<Hy> &view
) {
    int idx = work::get_global_id(0);

    Rng rng(seed.load(idx));

    real2 pos = 2*real2(
        (real)(idx % width) - 0.5_r*(width) + xrand::uniform<real>(rng),
        (real)(idx / width) - 0.5_r*(height) + xrand::uniform<real>(rng)
    )/height;

    hy::Horosphere obj;
    float3 color = Renderer<Hy>(rng).trace(view, pos, obj);

    seed.store(rng.state(), idx);

    float3 avg_color = color;
    if (sample_no != 0) {
        avg_color = (color + screen.vload<3>(idx)*sample_no)/(sample_no + 1);
    }
    screen.vstore(avg_color, idx);

    float3 out_color = clamp(avg_color, float3(0), float3(1));
    uchar4 pix = uchar4(convert<uchar3>(0xff*out_color), 0xff);
    image.vstore(pix, idx);
}

void trace_iface(
	__global float *screen,
    __global uchar *image,
    int width, int height,
    int sample_no,
    __global uint *seed,
    _View *view
) {
    trace(
        global_ptr<float>(screen),
        global_ptr<uchar>(image),
        width, height,
        sample_no,
        global_ptr<uint>(seed),
        *(View<Hy>*)view
    );
}
