#pragma once

#include <types.hh>
#include <random.hh>


typedef struct Context {
    Rng *rng;
    bool repeat;
} Context;
