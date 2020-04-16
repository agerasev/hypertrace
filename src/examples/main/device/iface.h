#pragma once

#include <types.h>

#define DEFINE_DUMMY(Name, size, align) \
typedef struct __attribute__((aligned(align))) { \
    uchar data[size]; \
} Name; \


DEFINE_DUMMY(_float3, 3*4, 4*4);
DEFINE_DUMMY(_uchar4, 4, 4);
DEFINE_DUMMY(_View, 4*2*4 + 3*4, 2*4);


#ifdef __cplusplus
extern "C" {
#endif

void trace_iface(
    __global _float3 *screen,
    __global _uchar4 *image,
    int width, int height,
    int sample_no,
    __global uint *seed,
    _View *view
);

#ifdef __cplusplus
};
#endif