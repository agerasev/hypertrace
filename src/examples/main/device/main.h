#include <types.h>
#include <real.h>
#include <memory.hpp>
#include <algebra/vector.hpp>
#include <view.hpp>


extern "C" {

void trace(
    global_ptr<float3> screen,
    global_ptr<uchar4> image,
    int width, int height,
    int sample_no,
    global_ptr<uint> seed,
    View view
)
#ifdef DEVICE
;
#else
{}
#endif

};
