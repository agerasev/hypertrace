#pragma once

#include "euclidean.hh"
#include "hyperbolic.hh"


struct RayEu {
    EuPos start;
    EuDir direction;
};

RayEu ray_eu_map(RayEu ray, EuMap map);
RayEu ray_eu_advance(RayEu ray, real l);


struct RayHy {
    HyPos start;
    HyPos direction;
};

RayHy ray_hy_map(RayHy ray, HyMap map);
RayHy ray_hy_advance(RayHy ray, real l);
