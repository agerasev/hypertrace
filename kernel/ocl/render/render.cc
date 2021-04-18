#include <types.hh>
#include <algebra/real.hh>
#include <algebra/vector.hh>
#include "light.hh"
#include "context.hh"

#ifdef HOST

#include <placeholder.hpp>

struct Shape;

real shape_detect(
    __global const Shape *shape,
    Context *context,
    real3 *normal,
    RayEu *ray
);

#endif // HOST


__kernel void render(
    const uint2 shape,
    __global const Shape *object,
    __global float4 *canvas
) {
    uint2 ipos = (uint2)(get_global_id(0), get_global_id(1));
    uint idx = ipos.x + shape.x * ipos.y;
    real2 pos = ((real)2 * convert_real2(ipos) / convert_real2(shape) - (real)1);

    Context context;
    context.repeat = false;
    context.rng = NULL;
    
    real3 normal = (real3)((real)0);

    RayEu ray;
    ray.start = (real3)((real)0, (real)0, (real)-4);
    ray.direction = normalize((real3)(pos, (real)1));

    real dist = shape_detect(object, &context, &normal, &ray);

    float4 color = (float4)(0.0f, 0.0f, 0.0f, 1.0f);
    if (dist > (real)-0.5f) {
        color = (float4)(1.0f, 1.0f, 1.0f, 1.0f);
    }

    canvas[idx] += color;
}
