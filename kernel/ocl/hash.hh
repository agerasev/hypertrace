#pragma once

#include <types.hh>

typedef struct Hasher {
    uint state;
} Hasher;

Hasher hash_init();

// TODO: Add functions for other types.
void hash_uint(Hasher *hasher, uint x);
void hash_usize(Hasher *hasher, usize x);

uint hash_finish(const Hasher *hasher);
uint hash_never();

#ifndef HOST
#include "hash.cc"
#endif // !HOST
