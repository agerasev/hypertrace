#include <macros.hh>
#include <render/light.hh>
#include <render/context.hh>

#if !defined($Material) || !defined($material)
#error "$Material macro must be defined"
#endif


typedef struct {
#if $Material != void
    $Material inner;
#endif
    color3 color;
} $(Colored,$Material);


typedef struct {
#if $Material != void
    $Material inner;
#endif
    color3 emission;
} $(Emissive,$Material);


_ALLOW_MULTIPLE_DEFINITIONS_
bool $(colored_,$material,_interact)(__global const $(Colored,$Material) *material, Context *context, real3 normal, LightLocal *light, float3 *emission) {
    
    light->base.intensity *= material->color;

    return $($material,_interact)(
#if $Material != void
        &material->inner,
#else
        NULL,
#endif
        context, normal, light, emission);
}


_ALLOW_MULTIPLE_DEFINITIONS_
bool $(emissive_,$material,_interact)(__global const $(Emissive,$Material) *material, Context *context, real3 normal, LightLocal *light, float3 *emission) {
    
    *emission += light->base.intensity * material->emission;

    return $($material,_interact)(
#if $Material != void
        &material->inner,
#else
        NULL,
#endif
        context, normal, light, emission);
}
