#pragma once

#include <types.h>

#define DEFINE_DUMMY(Name, size, align) \
typedef struct __attribute__((aligned(align))) { \
    uchar data[size]; \
} Name; \

#define DEFINE_DUMMY_ALIGN(Name, align) \
typedef struct __attribute__((aligned(align))) { \
    uchar data[align]; \
} Name; \

DEFINE_DUMMY(_View, 12*4 + 4*4 + 3*4, 4*4);
DEFINE_DUMMY_ALIGN(_MyObject, 128); // Largest possible alignment


#ifdef __cplusplus
extern "C" {
#endif

void trace_iface(
    __global float *screen,
    __global uchar *image,
    int width, int height,
    int sample_no,
    __global uint *seed,
    _View *view,
    __global const _MyObject *objects,
    int object_count
);

#ifdef __cplusplus
};
#endif