#include <types.hh>
#include <algebra/real.hh>
#include <algebra/vector.hh>
#include "context.hh"

#ifdef HOST
#include <placeholder.hpp>
#endif // HOST


__kernel void render(
    const uint2 shape,
    __global const Scene *scene,
    __global float4 *canvas
) {
    uint2 ipos = (uint2)(get_global_id(0), get_global_id(1));
    uint idx = ipos.x + shape.x * ipos.y;
    real2 pos = ((real)2 * convert_real2(ipos) / convert_real2(shape) - (real)1);

    Context context;
    context.repeat = false;
    context.rng = NULL;
    
    color4 color = scene_sample(scene, &context, pos, (real2)(R0, R0));

    canvas[idx] += color;
}
