#pragma once

#include "euclidean.hh"
#include "hyperbolic.hh"


struct RayEu {
    EuPos start;
    EuDir direction;
};

RayEu rayeu_map(RayEu ray, EuMap map);
RayEu rayeu_advance(RayEu ray, real l);


struct RayHy {
    HyPos start;
    HyPos direction;
};

RayHy rayhy_map(RayHy ray, HyMap map);
RayHy rayhy_advance(RayHy ray, real l);
