#include <macros.hh>
#include <types.hh>
#include <algebra/color.hh>
#include <render/context.hh>
#include <render/light/base.hh>
#include "interface.hh"

#if \
    !defined($Self) || !defined($self) || \
    !defined($Base) || !defined($base) || \
    !defined($Geo) || !defined($geo) || \
    !defined($Shape) || !defined($shape)
#error "All required macro parameters must be defined."
#endif

typedef $Base $Self;

define_shape_interface(
    $Self, $self,
    $Geo, $geo
)

_ALLOW_MULTIPLE_DEFINITIONS_
real $2($self,_detect)(__global const $Self *self, Context *context, $2($Geo,Dir) *normal, $2(Light,$Geo) *light) {
    Hasher orig_hasher = context->hasher;

    real min_dist = -R1;
    $2($Geo,Dir) min_normal;
    $2(Light,$Geo) min_light;
    Hasher min_hasher;
    for (usize index = 0; index < self->size; ++index) {
        __global const $Shape *shape = $2($base,__element__gc)(self, index);
        $2(Light,$Geo) cur_light = *light;
        $2($Geo,Dir) cur_normal;
        hash_usize(&context->hasher, index);
        real dist = $2($shape,_detect)(shape, context, &cur_normal, &cur_light);
        if (dist > -EPS && (min_dist <= -EPS || dist < min_dist)) {
            min_dist = dist;
            min_normal = cur_normal;
            min_light = cur_light;
            min_hasher = context->hasher;
        }
        context->hasher = orig_hasher;
    }
    if (min_dist > -EPS) {
        *normal = min_normal;
        *light = min_light;
        context->hasher = min_hasher;
        return min_dist;
    } else {
        return -R1;
    }
}
