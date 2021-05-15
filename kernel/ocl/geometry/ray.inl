#include <macros.hh>

#if !defined($Geo) || !defined($geo)
#error "$Geo and $geo macro parameters must be defined."
#endif

typedef struct {
    $2($Geo,Pos) start;
    $2($Geo,Dir) direction;
} $2(Ray,$Geo);

#define $Map $2($Geo,Map)
#define $map $geo
#define $postfix
#include "ray_map.inl"
#undef $Map
#undef $map
#undef $postfix

_ALLOW_MULTIPLE_DEFINITIONS_
$2(Ray,$Geo) $3(ray_,$geo,_advance)($2(Ray,$Geo) ray, real l) {
    $2($Geo,Map) map = $2($geo,_move_to)(ray.direction, l);
    return $3(ray_,$geo,_map)(ray, map);
}
