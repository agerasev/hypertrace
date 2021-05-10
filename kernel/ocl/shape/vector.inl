#include <macros.hh>
#include <types.hh>
#include <algebra/color.hh>
#include <render/context.hh>
#include <render/light/base.hh>

#if \
    !defined($Self) || !defined($self) || \
    !defined($Geo) || !defined($geo) || \
    !defined($Shape) || !defined($shape)
#error "All required macro parameters must be defined."
#endif


_ALLOW_MULTIPLE_DEFINITIONS_
real $2($self,_detect)(__global const $Self *self, Context *context, $2($Geo,Dir) *normal, $2(Light,$Geo) *light) {
    real min_dist = -R1;
    $2($Geo,Dir) min_normal;
    $2(Light,$Geo) min_light;
    for (usize index = 0; index < self->shapes.size; ++index) {
        __global const $Shape *shape = $3(vector__,$shape,__element__gc)(&self->shapes, index);
        $2(Light,$Geo) cur_light = *light;
        $2($Geo,Dir) cur_normal;
        real dist = $2($shape,_detect)(shape, context, &cur_normal, &cur_light);
        if (dist > -(real)0.5f && (min_dist < -(real)0.5f || dist < min_dist)) {
            min_dist = dist;
            min_normal = cur_normal;
            min_light = cur_light;
        }
    }
    if (min_dist > -(real)0.5f) {
        *normal = min_normal;
        *light = min_light;
        return min_dist;
    } else {
        return -R1;
    }
}
