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
    $2($Geo,Dir) *normal,
    $2(Light,$Geo) *light
);
