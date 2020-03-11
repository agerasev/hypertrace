#pragma once

// FIXME: Implement using C++ templates when migrated to OpenCL 2.1

#define mat_(T,N,M) T##N##x##M
#define matfn_(T,N,M,F) T##N##x##M##_##F


#define MAT_TYPE_DECL(T,M,N) \
typedef struct mat_(T,M,N) mat_(T,M,N); \


#define MAT_DECL(T,M,N) \
struct mat_(T,M,N) { \
    T s[M*N]; \
}; \
\
mat_(T,M,N) matfn_(T,M,N,from_data)( \
    const T *data, \
    const int pitch, const int stride \
); \
mat_(T,N,M) matfn_(T,M,N,transpose)(mat_(T,M,N) m); \


#define MAT_DEF(T,M,N) \
mat_(T,M,N) matfn_(T,M,N,from_data)( \
    const T *data, \
    const int pitch, const int stride \
) { \
    mat_(T,M,N) r; \
    for (int j = 0; j < M; ++j) { \
        for (int i = 0; i < N; ++i) { \
            r.s[j*N + i] = data[j*pitch + i*stride]; \
        } \
    } \
    return r; \
} \
\
mat_(T,N,M) matfn_(T,M,N,transpose)(mat_(T,M,N) m) { \
    return matfn_(T,N,M,from_data)(m.s, 1, 4); \
} \


MAT_TYPE_DECL(int, 2, 3)
MAT_TYPE_DECL(int, 3, 2)

MAT_DECL(int, 2, 3)
MAT_DECL(int, 3, 2)

MAT_DEF(int, 2, 3)
MAT_DEF(int, 3, 2)
