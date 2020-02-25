#include "ray.hh"


HyRay hyray_map(Moebius map, HyRay src) {
    HyRay dst;
    dst.start = mo_apply(map, src.start);
    dst.direction = normalize(mo_deriv(map, src.start, src.direction));
    return dst;
}
