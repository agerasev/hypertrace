#include <macros.hh>
#include <render/light/base.hh>
#include <render/context.hh>

#if !defined($Material) || !defined($material)
#error "$Material macro must be defined"
#endif


typedef struct {
#if $Material != void
    $Material inner;
#endif
    color3 emission;
} $2(Emissive,$Material);


_ALLOW_MULTIPLE_DEFINITIONS_
bool $3(emissive_,$material,_interact)(__global const $2(Emissive,$Material) *material, Context *context, real3 normal, LightLocal *light, float3 *emission) {
    
    *emission += light->base.intensity * material->emission;

    return $2($material,_interact)(
#if $Material != void
        &material->inner,
#else
        NULL,
#endif
        context, normal, light, emission);
}
