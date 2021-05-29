#pragma once

#define shape_choice_detect_variant($base, $index, $shape, $Variant) \
    case $index: \
    return $2($shape,_detect)( \
        $4($base,__,$Variant,__field_0__gc)($4($base,__,$Variant,__gc)(self)), \
        context, \
        normal, \
        light \
    );
