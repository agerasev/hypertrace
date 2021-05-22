#pragma once

#define mixture_interact_component($material, $component, $field) \
    alpha -= self->$field.portion; \
    if (alpha <= R0) { \
        return $2($material,_interact)( \
            $2($component,__material__gc)(&self->$field), \
            context, \
            normal, \
            light, \
            emission \
        ); \
    }
