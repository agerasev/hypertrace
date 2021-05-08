#include "gradient.hh"

void grad_bg_interact(__global const GradBg *background, Context *context, const LightEu *light, color3 *emission) {
    float factor = 0.5f * ((float)dot(light->ray.direction, background->direction) + 1.0f);
    color3 color = factor * background->colors[0] + (1.0f - factor) * background->colors[1];
    *emission += color * light->base.intensity;
}
