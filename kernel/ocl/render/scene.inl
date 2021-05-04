#include <macros.hh>
#include <geometry/geometry.hh>
#include <render/light.hh>
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

    RayEu ray;
    ray.start = (real3)((real)0, (real)0, (real)-4);
    ray.direction = normalize((real3)(pix_pos, (real)1));

    real dist = $2($object,_detect)($2($self,__object__gc)(self), context, &normal, &ray);

    color4 color = (color4)(0.0f, 0.0f, 0.0f, 1.0f);
    if (dist > (real)-0.5f) {
        color = (color4)(1.0f, 1.0f, 1.0f, 1.0f);
    }

    return color;
}
