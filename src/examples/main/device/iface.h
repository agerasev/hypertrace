#pragma once

#include <types.h>

#define DEFINE_DUMMY(Name, size, align) \
typedef struct __attribute__((aligned(align))) { \
    uchar data[size]; \
} Name; \


DEFINE_DUMMY(_View, 4*2*4 + 3*4, 2*4);


#ifdef __cplusplus
extern "C" {
#endif

void trace_iface(
    __global float *screen,
    __global uchar *image,
    int width, int height,
    int sample_no,
    __global uint *seed,
    _View *view
);

#ifdef __cplusplus
};
#endif