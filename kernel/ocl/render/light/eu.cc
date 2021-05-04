#include "eu.hh"

LightLocal light_eu_to_local(LightEu light) {
    LightLocal ll;
    ll.direction = light.ray.direction;
    ll.base = light.base;
    return ll;
}

void light_eu_update_local(LightEu *light, LightLocal ll) {
    light->ray.direction = ll.direction;
    light->base = ll.base;
}
