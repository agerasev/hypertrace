#include <macros.hh>
#include "geometry.hh"

#if !defined($Geo) || !defined($geo)
#error "$Geo and $geo macros must be defined."
#endif

typedef struct {
    $2($Geo,Pos) start;
    $2($Geo,Dir) direction;
} $2(Ray,$Geo);

_ALLOW_MULTIPLE_DEFINITIONS_
$2(Ray,$Geo) $3(ray_,$geo,_map)($2(Ray,$Geo) ray, $2($Geo,Map) map) {
    $2(Ray,$Geo) r;
    r.start = $2($geo,_apply_pos)(map, ray.start);
    r.direction = normalize($2($geo,_apply_dir)(map, ray.start, ray.direction));
    return r;
}

_ALLOW_MULTIPLE_DEFINITIONS_
$2(Ray,$Geo) $3(ray_,$geo,_advance)($2(Ray,$Geo) ray, real l) {
    $2($Geo,Map) map = $2($geo,_move_to)(ray.direction, -l);
    return $3(ray_,$geo,_map)(ray, map);
}
