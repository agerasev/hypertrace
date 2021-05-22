#include <macros.hh>
#include <types.hh>
#include "constant.hh"

#if !defined($Geo) || !defined($geo)
#error "All required macro parameters must be defined."
#endif

#define $Self ConstBg
#define $self $2(const_bg_,$geo)

#include "interface.inl"

_ALLOW_MULTIPLE_DEFINITIONS_
void $2($self,_interact)(__global const $Self *self, Context *context, const LightLocal *light, color3 *emission) {
    *emission += self->color * light->base.intensity;
}

#undef $Self
#undef $self
