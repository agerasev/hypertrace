#include "gradient.hh"

void grad_bg_interact(__global const GradBg *self, Context *context, const LightLocal *light, color3 *emission) {
    float factor = pow(
        clamp(
            0.5f * ((float)dot(light->direction, self->direction) + 1.0f),
            0.0f, 1.0f
        ),
        self->power
    );
    color3 color = factor * self->colors[0] + (1.0f - factor) * self->colors[1];
    *emission += color * light->base.intensity;
}
