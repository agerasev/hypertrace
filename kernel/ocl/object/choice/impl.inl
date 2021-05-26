#include <macros.hh>
#include <types.hh>
#include <algebra/color.hh>
#include <render/context.hh>
#include <object/interface.hh>

#if \
    !defined($Self) || !defined($self) || \
    !defined($Geo) || !defined($geo) || \
    !defined($Cache) || !defined($cache) || \
    !defined($object_choice_detect_list) || \
    !defined($object_choice_interact_list)
#error "All required macro parameters must be defined."
#endif

define_object_interface(
    $Self, $self,
    $Cache, $cache,
    $Geo, $geo
)

__allow_multiple__
real $2($self,_detect)(
    __global const $Self *self,
    Context *context,
    $Cache *cache,
    $2(Light,$Geo) *light
) {
    switch (self->index) {
    $object_choice_detect_list
    default:
        // unreachable
        return -R1;
    }
}

__allow_multiple__
bool $2($self,_interact)(
    __global const $Self *self,
    Context *context,
    $Cache *cache,
    $2(Light,$Geo) *light,
    color3 *emission
) {
    switch (self->index) {
    $object_choice_interact_list
    default:
        // unreachable
        return false;
    }
}

