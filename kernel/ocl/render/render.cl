#include <types.hh>
#include <algebra/real.hh>
#include <algebra/vector.hh>
#include "context.hh"

__kernel void render(
    const uint2 shape,
    __global const Scene *scene,
    __global float4 *canvas,
    __global uint *seeds
) {
    uint2 ipos = (uint2)(get_global_id(0), get_global_id(1));
    uint idx = ipos.x + shape.x * ipos.y;
    real2 pos = (real2)(
        (real)(2 * ipos.x + 1) - (real)shape.x,
        (real)shape.y - (real)(2 * ipos.y + 1)
    ) / (real)shape.y;
    real2 size = (real2)((real)2 / (real)shape.y);

    Context context;
    context.rng.state = seeds[idx];
    
    color4 color = (color4)(scene_sample(scene, &context, pos, size), 1.0f);

    canvas[idx] += color;
    seeds[idx] = context.rng.state;
}
