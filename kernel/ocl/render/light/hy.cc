#include "hy.hh"

LightLocal light_hy_to_local(LightHy *light) {
    LightLocal ll;
    ll.direction = light->ray.direction.xyz;
    ll.base = light->base;
    return ll;
}

void light_hy_update_local(LightHy *light, LightLocal ll) {
    light->ray.direction = q_new(ll.direction, R0);
    light->base = ll.base;
}
