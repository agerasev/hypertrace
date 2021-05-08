#include <macros.hh>
#include <algebra/color.hh>
#include <render/context.hh>

#if \
    !defined($Self)  || !defined($self) || \
    !defined($Geo)   || !defined($geo)
#error "All required macro parameters must be defined."
#endif

void $2($self,_interact)(__global const $Self *background, Context *context, const $2(Light,$Geo) *light, color3 *emission);
