#include <macros.hh>
#include <types.hh>
#include <algebra/color.hh>
#include <render/context.hh>
#include <random.hh>
#include <material/interface.hh>

#if !defined($Self) || !defined($self) || \
    !defined($mixture_interact_list)
#error "All required macro parameters must be defined."
#endif

define_material_interface($Self, $self)

__allow_multiple__
bool $2($self,_interact)(
    __global const $Self *self,
    Context *context,
    real3 normal,
    LightLocal *light,
    float3 *emission
) {
    real alpha = random_uniform(&context->rng);
    $mixture_interact_list
    // unreachable
    return false;
}
