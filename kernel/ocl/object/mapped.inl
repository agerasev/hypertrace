#include <macros.hh>
#include <types.hh>
#include <render/context.hh>
#include "interface.hh"

#if \
    !defined($Self)  || !defined($self) || \
    !defined($Base)  || !defined($base) || \
    !defined($Geo)   || !defined($geo) || \
    !defined($Map)   || !defined($map) || \
    !defined($Object) || !defined($object) || \
    !defined($Cache) || !defined($cache)
#error "All required macro parameters must be defined."
#endif

typedef $Base $Self;

define_object_interface(
    $Self, $self,
    $Cache, $cache,
    $Geo, $geo
)

__allow_multiple__
real $2($self,_detect)(
    __global const $Self *self,
    Context *context,
    $Cache *cache,
    $2(Light,$Geo) *light
) {
    $Map inv = $2($map,_inverse)(self->map);
    light->ray = $4(ray_,$geo,_map_,$map)(light->ray, inv);
    real dist = $2($object,_detect)($2($base,__inner__gc)(self), context, cache, light);
    light->ray = $4(ray_,$geo,_map_,$map)(light->ray, self->map);
    return dist;
}

__allow_multiple__
bool $2($self,_interact)(
    __global const $Self *self,
    Context *context,
    $Cache *cache,
    $2(Light,$Geo) *light,
    color3 *emission
) {
    $Map inv = $2($map,_inverse)(self->map);
    light->ray = $4(ray_,$geo,_map_,$map)(light->ray, inv);
    real dist = $2($object,_interact)($2($base,__inner__gc)(self), context, cache, light, emission);
    light->ray = $4(ray_,$geo,_map_,$map)(light->ray, self->map);
    return dist;
}
