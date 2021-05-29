#pragma once

#include <macros.hh>
#include <algebra/color.hh>
#include <render/context.hh>

#define define_background_interface( \
    $Self, $self, \
    $Geo, $geo \
) \
    void $2($self,_interact)( \
        __global const $Self *self, \
        Context *context, \
        const $2(Light,$Geo) *light, \
        color3 *emission \
    );
