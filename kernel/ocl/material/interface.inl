#include <macros.hh>
#include <render/context.hh>
#include <render/light/base.hh>

#if !defined($Self)  || !defined($self)
#error "All required macro parameters must be defined."
#endif

bool $2($self,_interact)(
    __global const $Self *self,
    Context *context,
    real3 normal,
    LightLocal *light,
    float3 *emission
);
