#include <macros.hh>
#include <types.hh>
#include <algebra/color.hh>
#include <render/context.hh>
#include <render/light/base.hh>

#if \
    !defined($Self) || !defined($self) || \
    !defined($Geo) || !defined($geo) || \
    !defined($Shape) || !defined($shape) || \
    !defined($Material) || !defined($material) || \
    !defined($Cache) || !defined($cache)
#error "All required macro parameters must be defined."
#endif

#include "interface.inl"

_ALLOW_MULTIPLE_DEFINITIONS_
real $2($self,_detect)(
    __global const $Self *self,
    Context *context,
    $Cache *cache,
    $2(Light,$Geo) *light
) {
    $2($Geo,Dir) normal;
    real dist = $2($shape,_detect)(
        $2($self,__shape__gc)(self),
        context,
        &normal,
        light
    );
    if (dist > -(real)0.5f) {
        cache->normal = $2($geo,_dir_to_local)(light->ray.start, normal);
    }
    return dist;
}

_ALLOW_MULTIPLE_DEFINITIONS_
bool $2($self,_interact)(
    __global const $Self *self,
    Context *context,
    $Cache *cache,
    LightLocal *light,
    color3 *emission
) {
    return $2($material,_interact)(
        $2($self,__material__gc)(self),
        context,
        cache->normal,
        light,
        emission
    );
}
