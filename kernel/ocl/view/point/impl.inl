#include <macros.hh>
#include <types.hh>

#if !defined($Geo) || !defined($geo)
#error "$Geo and $geo macros must be defined."
#endif

typedef struct {
    real fov;
} $2(PointView,$Geo);

#define $Self $2(PointView,$Geo)
#define $self $2(point_view_,$geo)
#include <view/interface.inl>
#undef $Self
#undef $self

_ALLOW_MULTIPLE_DEFINITIONS_
$2(Ray,$Geo) $3(point_view_,$geo,_sample)(__global const $2(PointView,$Geo) *view, Context *context, real2 pixel_pos, real2 pixel_size) {
    $2(Ray,$Geo) ray;
    real2 sampled_pos = pixel_pos + pixel_size * (random_uniform2(&context->rng) - (real)0.5f);
    ray.start = $2($geo,_origin)();
    ray.direction = $2($geo,_dir_from_local)(ray.start, normalize(MAKE(real3)(sampled_pos, -R1/view->fov)));
    return ray;
}
