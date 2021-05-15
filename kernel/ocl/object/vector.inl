#include <macros.hh>
#include <types.hh>
#include <algebra/color.hh>
#include <render/context.hh>
#include <render/light/base.hh>

#if \
    !defined($Self) || !defined($self) || \
    !defined($Geo) || !defined($geo) || \
    !defined($Object) || !defined($object)
#error "All required macro parameters must be defined."
#endif

typedef struct $2($Self,Cache) {
    $2($Object,Cache) inner;
    usize index;
} $2($Self,Cache);

_ALLOW_MULTIPLE_DEFINITIONS_
real $2($self,_detect)(
    __global const $Self *self,
    Context *context,
    $2($Self,Cache) *cache,
    $2(Light,$Geo) *light
) {
    Hasher orig_hasher = context->hasher;

    real min_dist = -R1;
    usize min_index;
    $2(Light,$Geo) min_light;
    $2($Object,Cache) min_cache;
    Hasher min_hasher;
    for (usize index = 0; index < self->objects.size; ++index) {
        __global const $Object *object = $3(vector__,$object,__element__gc)(&self->objects, index);
        $2(Light,$Geo) cur_light = *light;
        $2($Object,Cache) cur_cache;
        hash_usize(&context->hasher, index);
        real dist = $2($object,_detect)(object, context, &cur_cache, &cur_light);
        if (dist > R0 && (min_dist < R0 || dist < min_dist)) {
            min_dist = dist;
            min_index = index;
            min_light = cur_light;
            min_cache = cur_cache;
            min_hasher = context->hasher;
        }
        context->hasher = orig_hasher;
    }
    if (min_dist > R0) {
        cache->inner = min_cache;
        cache->index = min_index;
        *light = min_light;
        context->hasher = min_hasher;
        return min_dist;
    } else {
        return -R1;
    }
}

_ALLOW_MULTIPLE_DEFINITIONS_
bool $2($self,_interact)(
    __global const $Self *self,
    Context *context,
    $2($Self,Cache) *cache,
    $2(Light,$Geo) *light,
    color3 *emission
) {
    __global const $Object *object = $3(vector__,$object,__element__gc)(&self->objects, cache->index);
    return $2($object,_interact)(object, context, &cache->inner, light, emission);
}
