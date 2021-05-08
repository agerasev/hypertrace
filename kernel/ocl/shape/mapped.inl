#include <macros.hh>
#include <types.hh>
#include <render/context.hh>

#if \
    !defined($Self)  || !defined($self) || \
    !defined($Geo)   || !defined($geo) || \
    !defined($Map)   || !defined($map) || \
    !defined($Shape) || !defined($shape)
#error "All required macro parameters must be defined."
#endif

real $2($self,_detect)(__global const $Self *self, Context *context, $2($Geo,Dir) *normal, $2(Ray,$Geo) *ray) {
    $Map inv = $2($map,_inverse)(self->map);
    *ray = $4(ray_,$geo,_map_,$map)(*ray, inv);
    real dist = $2($shape,_detect)($2($self,__shape__gc)(self), context, normal, ray);
    *normal = $2($map,_apply_normal)(self->map, ray->start, *normal);
    *ray = $4(ray_,$geo,_map_,$map)(*ray, self->map);
    return dist;
}
