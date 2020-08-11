#include "light.hh"


LightLocal light_eu_to_local(LightEu light) {
    return LightLocal {
        .direction = light.ray.direction,
        .base = light.base,
    };
}

void light_eu_update_local(LightEu *light, LightLocal ll) {
    light->ray.direction = ll.direction;
    light->base = ll.base;
}

LightLocal light_hy_to_local(LightHy light) {
    return LightLocal {
        .direction = light.ray.direction.xyz,
        .base = light.base,
    };
}

void light_hy_update_local(LightHy *light, LightLocal ll) {
    light->ray.direction = q_new(ll.direction, R0);
    light->base = ll.base;
}
