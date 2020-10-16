#include "material.hh"

#include <common/algebra/linear.hh>


_ALLOW_UNUSED_PARAMETERS_
bool black_interact(Context *context, real3 normal, LightLocal *light, float3 *emission) {
    return false;
}

_ALLOW_UNUSED_PARAMETERS_
bool transparent_interact(Context *context, real3 normal, LightLocal *light, float3 *emission) {
    //light.face = !light.face;
    return true;
}

_ALLOW_UNUSED_PARAMETERS_
bool specular_interact(Context *context, real3 normal, LightLocal *light, float3 *emission) {
    light->direction -= (2*dot(light->direction, normal))*normal;
    return true;
}

_ALLOW_UNUSED_PARAMETERS_
bool lambertian_interact(Context *context, real3 normal, LightLocal *light, float3 *emission) {
    if (dot(normal, light->direction) > 0) {
        normal = -normal;
    }
    real3 rand = random_hemisphere_cosine(context->rng);
    // FIXME: Use Rotation3
    Linear3 rot = lin3_look_to(normal);
    light->direction = lin3_apply(rot, rand);
    //light->diffuse = true;
    return true;
}
