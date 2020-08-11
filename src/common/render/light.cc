#include "light.hh"


LightLocal lighteu_to_local(LightEu light) {
    return LightLocal {
        .direction = light.ray.direction,
        .base = light.base,
    };
}
void lighteu_update_local(LightEu *light, LightLocal ll) {
    light->ray.direction = ll.direction;
    light->base = ll.base;
}

LightLocal lighthy_to_local(LightHy light) {
    return LightLocal {
        .direction = light.ray.direction.xyz,
        .base = light.base,
    };
}
void lighthy_update_local(LightHy *light, LightLocal ll) {
    light->ray.direction = q_new(ll.direction, R0);
    light->base = ll.base;
}
