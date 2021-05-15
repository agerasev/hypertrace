#include "hash.hh"

// sdbm algorithm implementation

Hasher hash_init() {
    Hasher h;
    h.state = 0;
    return h;
}

void hash_uint(Hasher *hasher, uint x) {
    uint h = hasher->state;
    hasher->state = x + (h << 6) + (h << 16) - h;
}
void hash_usize(Hasher *hasher, usize x) {
    hash_uint(hasher, (uint)x);
}

uint hash_finish(const Hasher *hasher) {
    return hasher->state;
}
uint hash_never() {
    return ~(uint)0;
}
