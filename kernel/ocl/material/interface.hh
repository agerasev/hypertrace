#include <macros.hh>
#include <render/context.hh>
#include <render/light/base.hh>

#define define_material_interface($Self, $self) \
    bool $2($self,_interact)( \
        __global const $Self *self, \
        Context *context, \
        real3 normal, \
        LightLocal *light, \
        float3 *emission \
    );
