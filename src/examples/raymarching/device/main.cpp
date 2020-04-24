#include "iface.h"

#include <types.h>
#include <real.h>
#include <memory.hpp>

#include <algebra/vector.hpp>

#include <view.hpp>
#include <work.hpp>

#include <geometry/euclidean.hpp>
#include <render/renderer.hpp>
#include <render/filter.hpp>

#include <common.hpp>


#define ASSERT_DUMMY(Dummy, Orig) \
static_assert(sizeof(Dummy) == sizeof(Orig), "Dummy structure size mismatch"); \
static_assert(alignof(Dummy) == alignof(Orig), "Dummy structure alignment mismatch"); \

#define ASSERT_DUMMY_ALIGN(Dummy, Orig) \
static_assert( \
    (alignof(Dummy) % alignof(Orig)) == 0, \
    "Dummy structure alignment is not multiple of original structure alignment" \
); \


ASSERT_DUMMY(_View, View<Eu>);
ASSERT_DUMMY_ALIGN(_MyObject, MyObject);


typedef xrand::LCRng Rng;

void trace(
    global_ptr<float> screen,
    global_ptr<uchar> image,
    int width, int height,
    int sample_no,
    global_ptr<uint> seed,
    const View<Eu> &view,
    global_const_ptr<MyObject> objects,
    int object_count
) {
    int idx = work::get_global_id(0);

    Rng rng(seed.load(idx));

    real2 pos = 2*real2(
        (real)(idx % width) - 0.5_r*(width) + xrand::uniform<real>(rng),
        (real)(idx / width) - 0.5_r*(height) + xrand::uniform<real>(rng)
    )/height;

    GradientBackground bg(real3(0,0,1), float3(1), float3(0));
    Renderer<Eu, Rng> renderer(rng);

    float3 color = renderer.trace(view, pos, 2, bg, objects, object_count);

    seed.store(rng.state(), idx);

    float3 avg_color = color;
    if (sample_no != 0) {
        avg_color = (color + screen.vload<3>(idx)*sample_no)/(sample_no + 1);
    }
    screen.vstore(avg_color, idx);

    GammaFilter filter;
    float3 out_color = filter.apply(avg_color);
    out_color = math::clamp(out_color, float3(0), float3(1));
    uchar4 pix = uchar4(convert<uchar3>(0xff*out_color), 0xff);
    image.vstore(pix, idx);
}

void trace_iface(
	__global float *screen,
    __global uchar *image,
    int width, int height,
    int sample_no,
    __global uint *seed,
    _View *view,
    __global const _MyObject *objects,
    int object_count
) {
    trace(
        global_ptr<float>(screen),
        global_ptr<uchar>(image),
        width, height,
        sample_no,
        global_ptr<uint>(seed),
        *(View<Eu>*)view,
        global_const_ptr<_MyObject>(objects).reinterpret<MyObject>(),
        object_count
    );
}
