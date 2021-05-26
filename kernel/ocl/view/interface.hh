#include <macros.hh>
#include <types.hh>
#include <render/context.hh>

#define define_view_interface( \
    $Self, $self, \
    $Geo, $geo \
) \
    $2(Ray,$Geo) $2($self,_sample)( \
        __global const $Self *view, \
        Context *context, \
        real2 pixel_pos, \
        real2 pixel_size \
    );
