#include <macros.hh>
#include <render/context.hh>
#include <algebra/color.hh>

#if \
    !defined($Self)  || !defined($self) || \
    !defined($Geo)   || !defined($geo) || \
    !defined($View)   || !defined($view) || \
    !defined($Object) || !defined($object)
#error "All required macro parameters must be defined."
#endif

color3 $2($self,_sample)(__global const $Self *self, Context *context, real2 pix_pos, real2 pix_size) {
    real3 normal = (real3)((real)0);

    $2(Light,$Geo) light;
    light.base.intensity = MAKE(color3)(1.0f);
    light.ray = $2($view,_sample)($2($self,__view__gc)(self), context, pix_pos, pix_size);

    color3 emission = MAKE(color3)(0.0f);

    for (uint i = 0; i < 2; ++i) {
        $2($Object,Cache) cache;
        real dist = $2($object,_detect)($2($self,__object__gc)(self), context, &cache, &light.ray);

        if (dist > (real)-0.5f) {
            $2($object,_interact)($2($self,__object__gc)(self), context, &cache, &light, &emission);
        } else {
            emission += light.base.intensity * light.ray.direction;
        }
    }

    return emission;
}
