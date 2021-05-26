#include <macros.hh>
#include <types.hh>
#include "constant.hh"
#include "interface.hh"

#if !defined($Geo) || !defined($geo)
#error "All required macro parameters must be defined."
#endif

#define $Self ConstBg
#define $self $2(const_bg_,$geo)

define_background_interface(
    $Self, $self,
    $Geo, $geo
)

_ALLOW_MULTIPLE_DEFINITIONS_
void $2($self,_interact)(__global const $Self *self, Context *context, const $2(Light,$Geo) *light, color3 *emission) {
    *emission += self->color * light->base.intensity;
}

#undef $Self
#undef $self
