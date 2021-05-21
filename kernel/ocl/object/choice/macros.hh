#pragma once

#define object_choice_detect_variant($base, $index, $object, $Variant) \
    case $index: \
    return $2($object,_detect)( \
        $4($base,__,$Variant,__field_0__gc)($4($base,__,$Variant,__gc)(self)), \
        context, \
        $3($cache,__,$Variant)(cache), \
        light \
    );

#define object_choice_interact_variant($base, $index, $object, $Variant) \
    case $index: \
    return $2($object,_interact)( \
        $4($base,__,$Variant,__field_0__gc)($4($base,__,$Variant,__gc)(self)), \
        context, \
        $3($cache,__,$Variant)(cache), \
        light, \
        emission \
    );
