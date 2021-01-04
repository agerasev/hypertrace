#include "macros.hh"
#include "geometry.hh"

#if !defined($Geo) || !defined($geo)
#error "$Geo and $geo macros must be defined."
#endif

typedef struct {
    $($Geo,Pos) start;
    $($Geo,Dir) direction;
} $(Ray,$Geo);

$(Ray,$Geo) $(ray_,$geo,_map)($(Ray,$Geo) ray, $($Geo,Map) map) {
    $(Ray,$Geo) r;
    r.start = $($geo,_apply_pos)(map, ray.start);
    r.direction = normalize($($geo,_apply_dir)(map, ray.start, ray.direction));
    return r;
}

$(Ray,$Geo) $(ray_,$geo,_advance)($(Ray,$Geo) ray, real l) {
    $($Geo,Map) map = $($geo,_move_to)(ray.direction, -l);
    return $(ray_,$geo,_map)(ray, map);
}
