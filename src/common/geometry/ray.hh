#pragma once

#include "euclidean.hh"
#include "hyperbolic.hh"


typedef struct RayEu {
    EuPos start;
    EuDir direction;
} RayEu;

RayEu rayeu_map(RayEu ray, EuMap map);
RayEu rayeu_advance(RayEu ray, real l);


typedef struct RayHy {
    HyPos start;
    HyPos direction;
} RayHy;

RayHy rayhy_map(RayHy ray, HyMap map);
RayHy rayhy_advance(RayHy ray, real l);


#ifndef HOST
#include "ray.cc"
#endif // !HOST
