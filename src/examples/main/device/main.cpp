#include "iface.h"

#include <types.h>
#include <real.h>
#include <memory.hpp>

#include <algebra/vector.hpp>
#include <algebra/complex.hpp>

#include <view.hpp>
#include <work.hpp>

#include <geometry/hyperbolic/ray.hpp>
#include <random.hpp>


#define ASSERT_DUMMY(Orig, Dummy) \
static_assert(sizeof(Orig) == sizeof(Dummy), "Dummy structure size mismatch"); \
static_assert(alignof(Orig) == alignof(Dummy), "Dummy structure alignment mismatch"); \

ASSERT_DUMMY(_View, View);


using namespace hyperbolic;

bool hit_horosphere(Ray ray, quat &h) {
    quat p = ray.start, d = ray.direction;
    real dxy = length(d.re());

    if (p[2] < dxy) {
        return false;
    }
    
    real dt = math::sqrt(p[2]*p[2] - dxy*dxy);
    real t = p[2]*d[2] - dt;
    if (t < 0) {
        t += 2*dt;
    }
    if (t < 0) {
        return false;
    }

    t /= dxy*dxy;
    h = quat(
        p.re() + d.re()*t,
        1, 0
    );

    return true;
}

void trace(
    global_ptr<float> screen,
    global_ptr<uchar> image,
    int width, int height,
    int sample_no,
    global_ptr<uint> seed,
    const View &view
) {
    int idx = work::get_global_id(0);

    real2 pix_pos = 2*real2(
        (real)(idx % width) - 0.5_r*(width - 1),
        (real)(idx / width) - 0.5_r*(height - 1)
    )/height;
    real pix_size = 2_r/height;

    float3 color(0);

    random::LCRng rng(seed.load(idx));

    real2 new_pix_pos = pix_pos + pix_size*(random::uniform<real2>(rng) - real2(0.5_r));
    Ray ray = Ray(1_j, normalize(quat(new_pix_pos, 1, 0))).map(view.position);

    quat h;
    if (hit_horosphere(ray, h)) {
        color = float3(fract(h.re().vec()).first, 0);
    }

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
        *(View*)view
    );
}
