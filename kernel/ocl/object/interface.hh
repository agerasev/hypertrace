#include <macros.hh>
#include <render/context.hh>

#define define_object_interface( \
    $Self, $self, \
    $Cache, $cache, \
    $Geo, $geo \
) \
    real $2($self,_detect)( \
        __global const $Self *self, \
        Context *context, \
        $Cache *cache, \
        $2(Light,$Geo) *light \
    ); \
 \
    bool $2($self,_interact)( \
        __global const $Self *self, \
        Context *context, \
        $Cache *cache, \
        $2(Light,$Geo) *light, \
        color3 *emission \
    );
