#include <macros.hh>
#include <render/context.hh>
#include <algebra/color.hh>

#if \
    !defined($Self)  || !defined($self) || \
    !defined($Geo)   || !defined($geo) || \
    !defined($View)   || !defined($view) || \
    !defined($Object) || !defined($object) || \
    !defined($Background) || !defined($background) || \
    !defined($light_hops)
#error "All required macro parameters must be defined."
#endif

_ALLOW_MULTIPLE_DEFINITIONS_
color3 $2($self,_sample)(__global const $Self *self, Context *context, real2 pix_pos, real2 pix_size) {
    $2(Light,$Geo) light;
    light.base.intensity = MAKE(color3)(1.0f);
    light.ray = $2($view,_sample)($2($self,__view__gc)(self), context, pix_pos, pix_size);

    color3 emission = MAKE(color3)(0.0f);

    for (uint i = 0; i < $light_hops; ++i) {
        $2($Object,Cache) cache;
        real dist = $2($object,_detect)($2($self,__object__gc)(self), context, &cache, &light);

        if (dist > (real)-0.5f) {
            bool ret = $2($object,_interact)($2($self,__object__gc)(self), context, &cache, &light, &emission);
            if (!ret) {
                break;
            }
        } else {
            $2($background,_interact)($2($self,__background__gc)(self), context, &light, &emission);
            break;
        }
    }

    return emission;
}
