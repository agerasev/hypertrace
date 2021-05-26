#include "refractive.hh"

_ALLOW_UNUSED_PARAMETERS_
bool refractive_interact(__global const Refractive *self, Context *context, real3 normal, LightLocal *light, float3 *emission) {
    real3 d = light->direction;
    real k = self->index;
    real a = dot(d, normal);
    if (a < -EPS) {
        k = R1 / k;
    }
    real x = (a * a - R1) * (k * k) + R1;
    if (x > EPS) {
        // Refraction
        real b = sign(a) * sqrt(x);
        d = d * k + (b - a * k) * normal; 
    } else {
        // Total internal reflection
        d -= (2 * a) * normal;
    }
    light->direction = d;
    return true;
}
