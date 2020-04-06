#include "types.hh"

#ifdef OPENCL

#define DEF_VEC_PACK(type, n, size) \
type##n##_pk pack_##type##n(type##n v) { type##n##_pk p; vstore##n(v, 0, p.s); return p; } \
type##n unpack_##type##n(type##n##_pk p) { return vload##n(0, p.s); }

#define DEF_VEC_PACK_234(type) \
    DEF_VEC_PACK(type, 2, 2) \
    DEF_VEC_PACK(type, 3, 4) \
    DEF_VEC_PACK(type, 4, 4)

DEF_VEC_PACK_234(int)
DEF_VEC_PACK_234(uint)
DEF_VEC_PACK_234(float)

#endif // OPENCL
