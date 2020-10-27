#include "ray.hh"


RayEu rayeu_map(RayEu ray, EuMap map) {
    RayEu r;
    r.start = eu_apply_pos(map, ray.start);
    r.direction = normalize(eu_apply_dir(map, ray.start, ray.direction));
    return r;
}
RayEu rayeu_advance(RayEu ray, real l) {
    RayEu r;
    r.start = ray.start + ray.direction*l;
    r.direction = ray.direction;
    return r;
}

RayHy rayhy_map(RayHy ray, HyMap map) {
    RayHy r;
    r.start = hy_apply_pos(map, ray.start);
    r.direction = normalize(hy_apply_dir(map, ray.start, ray.direction));
    return r;
}
RayHy rayhy_advance(RayHy ray, real l) {
    HyMap map = hy_move_to(ray.direction, l);
    return rayhy_map(ray, map);
}
