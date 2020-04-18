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
        Ray<G> ray = Ray<G>(G::origin(), normalize(quat(pix_pos, 1, 0))).map(view.position);

        Context context{rng};
        typename O::Cache cache;
        float3 luminanace(0);
        Ray<G> new_ray;
        if (obj.hit(context, cache, ray) > 0_r) {
            obj.bounce(context, cache, luminanace, new_ray);
        }
        return luminanace;
    }
};
