#include <macros.hh>

#if \
    !defined($Self) || !defined($self) || \
    !defined($Base) || !defined($base) || \
    !defined($Geo) || !defined($geo) || \
    !defined($Map) || !defined($map) || \
    !defined($View) || !defined($view)
#error "All required macro parameters must be defined."
#endif

typedef $Base $Self;

#include "interface.inl"

_ALLOW_MULTIPLE_DEFINITIONS_
$2(Ray,$Geo) $2($self,_sample)(__global const $Self *view, Context *context, real2 pixel_pos, real2 pixel_size) {
    $2(Ray,$Geo) ray = $2($view,_sample)(&view->inner, context, pixel_pos, pixel_size);
    ray.direction = $2($map,_apply_dir)(view->map, ray.start, ray.direction);
    ray.start = $2($map,_apply_pos)(view->map, ray.start);
    return ray;
}
