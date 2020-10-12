#pragma once

#include "euclidean.hh"
#include "hyperbolic.hh"


typedef struct RayEu {
    EuPos start;
    EuDir direction;
} RayEu;

RayEu ray_eu_map(RayEu ray, EuMap map);
RayEu ray_eu_advance(RayEu ray, real l);


typedef struct RayHy {
    HyPos start;
    HyPos direction;
} RayHy;

RayHy ray_hy_map(RayHy ray, HyMap map);
RayHy ray_hy_advance(RayHy ray, real l);


#ifndef HOST
#include "ray.cc"
#endif // !HOST
