#include <macros.hh>
#include <render/context.hh>

#if \
    !defined($Self)  || !defined($self) || \
    !defined($Geo)   || !defined($geo) || \
    !defined($View)   || !defined($view) || \
    !defined($Object) || !defined($object)
#error "All required macro parameters must be defined."
#endif

color4 $2($self,_sample)(__global const $Self *self, Context *context, real2 pix_pos, real2 pix_size) {
    real3 normal = (real3)((real)0);

    RayEu ray = $2($view,_sample)($2($self,__view__gc)(self), context, pix_pos, pix_size);

    real dist = $2($object,_detect)($2($self,__object__gc)(self), context, &normal, &ray);

    color4 color = (color4)(ray.direction, 1.0f);
    if (dist > (real)-0.5f) {
        color = (color4)(1.0f, 1.0f, 1.0f, 1.0f);
    }

    return color;
}
