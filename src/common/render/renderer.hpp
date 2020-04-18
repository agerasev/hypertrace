#pragma once

#include <random.hpp>
#include "context.hpp"


template <typename G>
class Renderer {
public:
    Rng &rng;

public:
    Renderer(Rng &rng) :
        rng(rng)
    {}

    template <typename O>
    float3 trace(const View<G> &view, real2 pix_pos, const O &obj) {
        Light<G> light{
            Ray<G>(
                G::origin(),
                normalize(quat(pix_pos, 1, 0))
            ).map(view.position),
            float3(1)
        };

        Context context{rng};
        float3 luminance(0);

        typename O::Cache cache;
        if (obj.detect(context, cache, light) > 0_r) {
            obj.interact(context, cache, light, luminance);
        }
        return luminance;
    }
};
