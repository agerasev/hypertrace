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
    return $2($shape,_detect)(
        $2($self,__shape__gc)(self),
        context,
        &cache->normal,
        light
    );
}

_ALLOW_MULTIPLE_DEFINITIONS_
bool $2($self,_interact)(
    __global const $Self *self,
    Context *context,
    $Cache *cache,
    $2(Light,$Geo) *light,
    color3 *emission
) {
    LightLocal ll = $3(light_,$geo,_to_local)(light);
    bool ret = $2($material,_interact)(
        $2($self,__material__gc)(self),
        context,
        $2($geo,_dir_to_local)(light->ray.start, cache->normal),
        &ll,
        emission
    );
    $3(light_,$geo,_update_local)(light, ll);
    return ret;
}
