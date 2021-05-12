#include <macros.hh>
#include <types.hh>

#if !defined($Geo) || !defined($geo)
#error "All required macro parameters must be defined."
#endif

#define $Self $2(ConstBg,$Geo)
#define $self $2(const_bg_,$geo)

typedef struct $Self {
    color3 color;
} $Self;

#include "interface.inl"

_ALLOW_MULTIPLE_DEFINITIONS_
void $2($self,_interact)(__global const $Self *self, Context *context, const $2(Light,$Geo) *light, color3 *emission) {
    *emission += self->color * light->base.intensity;
}

#undef $Self
#undef $self
