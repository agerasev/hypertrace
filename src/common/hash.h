#pragma once

void hash(int *state, int value) {
    *state = value + (*state << 6) + (*state << 16) - *state;
}
