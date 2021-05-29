#pragma once

#include <macros.hh>
#include <render/context.hh>

#define define_shape_interface( \
    $Self, $self, \
    $Geo, $geo \
) \
    real $2($self,_detect)( \
        __global const $Self *self, \
        Context *context, \
        $2($Geo,Dir) *normal, \
        $2(Light,$Geo) *light \
    );
