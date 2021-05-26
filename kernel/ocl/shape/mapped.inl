#include <macros.hh>
#include <types.hh>
#include <render/context.hh>
#include "interface.hh"

#if \
    !defined($Self)  || !defined($self) || \
    !defined($Base)  || !defined($base) || \
    !defined($Geo)   || !defined($geo) || \
    !defined($Map)   || !defined($map) || \
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
    $Map inv = $2($map,_inverse)(self->map);
    light->ray = $4(ray_,$geo,_map_,$map)(light->ray, inv);
    real dist = $2($shape,_detect)($2($base,__inner__gc)(self), context, normal, light);
    *normal = $2($map,_apply_normal)(self->map, light->ray.start, *normal);
    light->ray = $4(ray_,$geo,_map_,$map)(light->ray, self->map);
    return dist;
}
