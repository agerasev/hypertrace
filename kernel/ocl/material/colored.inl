#include <macros.hh>
#include <render/light/base.hh>
#include <render/context.hh>
#include "interface.hh"

#if \
    !defined($Self) || !defined($self) || \
    !defined($Material) || !defined($material)
#error "All required macro parameters must be defined."
#endif

define_material_interface($Self, $self)

__allow_multiple__
bool $2($self,_interact)(__global const $Self *self, Context *context, real3 normal, LightLocal *light, float3 *emission) {
    
    light->base.intensity *= self->color;

    return $2($material,_interact)(
        $2($self,__inner__gc)(self),
        context, normal, light, emission
    );
}
