#include <macros.hh>

#if \
    !defined($Geo) || !defined($geo) || \
    !defined($Map) || !defined($map)
#error "$Geo and $geo macro parameters must be defined."
#endif

#ifndef $postfix
#define $postfix_ $2(_,$map)
#else
#define $postfix_ $postfix
#endif

_ALLOW_MULTIPLE_DEFINITIONS_
$2(Ray,$Geo) $4(ray_,$geo,_map,$postfix_)($2(Ray,$Geo) ray, $Map map) {
    $2(Ray,$Geo) r;
    r.start = $2($map,_apply_pos)(map, ray.start);
    r.direction = normalize($2($map,_apply_dir)(map, ray.start, ray.direction));
    return r;
}

#undef $postfix_
