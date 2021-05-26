#include "plane.hh"

_ALLOW_UNUSED_PARAMETERS_
real plane_eu_detect(__global const PlaneEu *shape, Context *context, real3 *normal, LightEu *light) {
    // Line cannot intersect plane twice
    if (context_is_repeat(context)) {
        return -R1;
    }
    RayEu *ray = &light->ray;

    real3 pos = ray->start;
    real3 dir = ray->direction;
    if (pos.z * dir.z > R0) {
        return -R1;
    }

    real t = -pos.z / dir.z;
    ray->start = make(real3)(pos.xy + dir.xy * t, R0);
    *normal = make(real3)(0, 0, -1);
    return t;
}
