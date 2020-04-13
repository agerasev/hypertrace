#pragma once

#include "types.h"

// TODO: Generate this automatically
#define PARAMS_ALIGN 2*sizeof(real) // real2 alignment
#define PARAMS_SIZE \
2*( /* 2 x Views */ \
    4*2*sizeof(real) + /* Moebius of comps */ \
    3*sizeof(real) /* fov, lr, fl */ + \
    sizeof(real) /* padding */ \
) \

#ifndef __cplusplus

typedef struct __attribute__((aligned(PARAMS_ALIGN))) {
    uchar data[PARAMS_SIZE];
} Parameters;

#else // __cplusplus

#include <view.hpp>

class Parameters {
public:
    View view, view_prev;
};

#ifdef HOST

namespace device {
struct Parameters {
    device::View view, view_prev;
};
#endif // HOST
static_assert(sizeof(Parameters) == PARAMS_SIZE, "Parameters size mismatch");
static_assert(alignof(Parameters) == PARAMS_ALIGN, "Parameters alignment mismatch");
#ifdef HOST
}

template <>
struct ToDevice<Parameters> {
    typedef device::Parameters type;
    static type to_device(const Parameters &v) {
        return type {
            .view = ::to_device(v.view),
            .view_prev = ::to_device(v.view_prev),
        };
    }
};

#endif // HOST
#endif // __cplusplus