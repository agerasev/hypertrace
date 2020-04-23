#pragma once

#include <random.hpp>
#include "light.hpp"
#include "background.hpp"
#include <view.hpp>


template <typename G, typename Rng>
class Renderer {
public:
    struct Context {
        xrand::LCRng &rng;
        bool repeat = false;
    };

    Rng &rng;

public:
    Renderer(Rng &rng) : rng(rng) {}

    template <typename Obj, typename Bg>
    float3 trace(
        const View<G> &view, real2 pix_pos, Bg background,
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
        int prev_i = -1;

        for (int k = 0; k < 3; ++k) {
            int nearest_i = -1;
            real nearest_dist = -1_r;
            typename Obj::Cache nearest_cache;
            Light<G> nearest_light;

            for (int i = 0; i < object_count; ++i) {
                context.repeat = (prev_i == i);
                if (!Obj::repeated) {
                    continue;
                }
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
                if (obj.interact(context, nearest_cache, nearest_light, luminance)) {
                    light = nearest_light;
                    prev_i = nearest_i;
                } else {
                    break;
                }
            } else {
                background.interact(context, light, luminance);
                break;
            }
        }

        return luminance;
    }
};
