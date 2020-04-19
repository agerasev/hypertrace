#pragma once

#include <random.hpp>
#include "light.hpp"
#include <view.hpp>

template <typename G>
class Renderer {
public:
    struct Context {
        xrand::Rng &rng;
        bool repeat = false;
    };

    xrand::Rng &rng;

public:
    Renderer(xrand::Rng &rng) :
        rng(rng)
    {}

    template <typename Obj>
    float3 trace(
        const View<G> &view, real2 pix_pos,
        global_const_ptr<Obj> objects, int object_count
    ) {
        Light<G> init_light;
        init_light.ray.start = G::origin();
        init_light.set_local(LocalLight(
            normalize(real3(pix_pos, 1)),
            BaseLight(float3(1))
        ));
        init_light.ray = init_light.ray.map(view.position);


        Context context{rng};
        float3 luminance(0);
        Light<G> light = init_light;

        for (int k = 0; k < 1; ++k) {
            int nearest_i = -1;
            real nearest_dist = -1_r;
            typename Obj::Cache nearest_cache;
            Light<G> nearest_light;

            for (int i = 0; i < object_count; ++i) {
                Obj obj = objects.load(i);
                Light<G> temp_light = light;
                typename Obj::Cache cache;
                real dist = obj.detect(context, cache, temp_light);
                if (dist > 0_r && (nearest_dist < 0_r || dist < nearest_dist)) {
                    nearest_i = i;
                    nearest_dist = dist;
                    nearest_cache = cache;
                    nearest_light = temp_light;
                }
            }

            if (nearest_i >= 0) {
                Obj obj = objects.load(nearest_i);
                obj.interact(context, nearest_cache, nearest_light, luminance);
            } else {
                luminance += light.color;
                break;
            }
        }

        return luminance;
    }
};
