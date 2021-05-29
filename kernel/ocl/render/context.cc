#include "context.hh"

Context context_init() {
    Context context;
    context.rng.state = 0;
    context.hasher = hash_init();
    context.prev_hash = hash_never();
    return context;
}

bool context_is_repeat(const Context *context) {
    return hash_finish(&context->hasher) == context->prev_hash;
}
