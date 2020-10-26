#pragma once

#include <common/types.hh>
#include <common/random.hh>


typedef struct Context {
    Rng *rng;
    bool repeat;
} Context;
