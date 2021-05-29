#pragma once

#include <types.hh>
#include <random.hh>
#include <hash.hh>

typedef struct Context {
    Rng rng;
    Hasher hasher;
    uint prev_hash;
} Context;

Context context_init();

// Check if the ray hits the same object twice.
bool context_is_repeat(const Context *context);

#ifndef HOST
#include "context.cc"
#endif // !HOST
