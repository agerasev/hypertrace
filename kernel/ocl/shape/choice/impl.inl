#include <macros.hh>
#include <types.hh>
#include <algebra/color.hh>
#include <render/context.hh>

#if \
    !defined($Self) || !defined($self) || \
    !defined($Geo) || !defined($geo) || \
    !defined($shape_choice_detect_list)
#error "All required macro parameters must be defined."
#endif

#include <shape/interface.inl>

_ALLOW_MULTIPLE_DEFINITIONS_
real $2($self,_detect)(
    __global const $Self *self,
    Context *context,
    $2($Geo,Dir) *normal,
    $2(Light,$Geo) *light
) {
    switch (self->index) {
    $shape_choice_detect_list
    default:
        // unreachable
        return -R1;
    }
}
