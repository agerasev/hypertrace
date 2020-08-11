#pragma once

#include <common/types.hh>

struct TraceRng {
    uint state;
};

struct TraceContext {
    TraceRng rng;
    uchar repeated;
};
