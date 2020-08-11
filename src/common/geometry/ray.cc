#include "ray.hh"


RayEu ray_eu_map(RayEu ray, EuMap map) {
    return RayEu {
        .start = eu_apply_pos(map, ray.start),
        .direction = normalize(eu_apply_dir(map, ray.start, ray.direction)),
    };
}
RayEu ray_eu_advance(RayEu ray, real l) {
    return RayEu {
        .start = ray.start + ray.direction*l,
        .direction = ray.direction,
    };
}

RayHy ray_hy_map(RayHy ray, HyMap map) {
    return RayHy {
        .start = hy_apply_pos(map, ray.start),
        .direction = normalize(hy_apply_dir(map, ray.start, ray.direction)),
    };
}
RayHy ray_hy_advance(RayHy ray, real l) {
    HyMap map = hy_move_to(ray.direction, l);
    return ray_hy_map(ray, map);
}
