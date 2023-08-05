#include <types.hpp>

__kernel void test(__global const uint *in, __global uint *out, uint val) {
    uint i = get_global_id(0);
    out[i] = in[i] + val;
}
