#include <macros.hh>
#include <render/context.hh>

#if \
    !defined($Self)  || !defined($self) || \
    !defined($Geo)   || !defined($geo)
#error "All required macro parameters must be defined."
#endif

real $2($self,_detect)(
    __global const $Self *self,
    Context *context,
    $2($Self,Cache) *cache,
    $2(Light,$Geo) *light
);

bool $2($self,_interact)(
    __global const $Self *self,
    Context *context,
    $2($Self,Cache) *cache,
    $2(Light,$Geo) *light,
    color3 *emission
);
