kernel void test(global const uint *in, global uint *out, uint val) {
    uint i = get_global_id(0);
    out[i] = in[i] + val;
}
