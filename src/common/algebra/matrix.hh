#pragma once


// FIXME: Implement using C++ templates when migrated to OpenCL 2.1

#define mat_(T,N,M) T##N##x##M##_t
#define matfn_(T,N,M,F) T##N##x##M##_##F
#define tfn_(T,F) T##_##F


#define MAT_DECL_TYPE(T,M,N) \
typedef struct mat_(T,M,N) mat_(T,M,N); \


#define MAT_DECL_MAP(T,M,N,F,Op) \
mat_(T,M,N) matfn_(T,M,N,F)(mat_(T,M,N) m) \

#define MAT_DEF_MAP(T,M,N,F,Op) \
MAT_DECL_MAP(T,M,N,F,Op) { \
    mat_(T,M,N) r; \
    for (int i = 0; i < M*N; ++i) { \
        r.s[i] = Op(m.s[i]); \
    } \
    return r; \
} \

#define MAT_DECL_MAP2(T,M,N,F,Op) \
mat_(T,M,N) matfn_(T,M,N,F)(mat_(T,M,N) a, mat_(T,M,N) b) \

#define MAT_DEF_MAP2(T,M,N,F,Op) \
MAT_DECL_MAP2(T,M,N,F,Op) { \
    mat_(T,M,N) r; \
    for (int i = 0; i < M*N; ++i) { \
        r.s[i] = Op(a.s[i], b.s[i]); \
    } \
    return r; \
} \

#define MAT_DECL_MAPC(T,M,N,F,Op) \
mat_(T,M,N) matfn_(T,M,N,F)(mat_(T,M,N) a, T b) \

#define MAT_DEF_MAPC(T,M,N,F,Op) \
MAT_DECL_MAPC(T,M,N,F,Op) { \
    mat_(T,M,N) r; \
    for (int i = 0; i < M*N; ++i) { \
        r.s[i] = Op(a.s[i], b); \
    } \
    return r; \
} \

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
MAT_DECL_MAP(T,M,N,neg,tfn_(T,neg)); \
MAT_DECL_MAP2(T,M,N,add,tfn_(T,add)); \
MAT_DECL_MAP2(T,M,N,sub,tfn_(T,sub)); \
MAT_DECL_MAPC(T,M,N,mul,tfn_(T,mul)); \
MAT_DECL_MAPC(T,M,N,div,tfn_(T,div)); \


#define MAT_DEF(T,M,N) \
mat_(T,M,N) matfn_(T,M,N,from_data)( \
    const T *data, \
    const int pitch, const int stride \
) { \
    mat_(T,M,N) r; \
    for (int i = 0; i < M; ++i) { \
        for (int j = 0; j < N; ++j) { \
            r.s[i*N + j] = data[i*pitch + j*stride]; \
        } \
    } \
    return r; \
} \
\
mat_(T,N,M) matfn_(T,M,N,transpose)(mat_(T,M,N) m) { \
    return matfn_(T,N,M,from_data)(m.s, 1, N); \
} \
\
MAT_DEF_MAP(T,M,N,neg,tfn_(T,neg)) \
MAT_DEF_MAP2(T,M,N,add,tfn_(T,add)) \
MAT_DEF_MAP2(T,M,N,sub,tfn_(T,sub)) \
MAT_DEF_MAPC(T,M,N,mul,tfn_(T,mul)) \
MAT_DEF_MAPC(T,M,N,div,tfn_(T,div)) \

#define MAT_DECL_DOT(T,L,M,N) \
mat_(T,L,N) T##L##x##M##x##N##_dot(mat_(T,L,M) a, mat_(T,M,N) b)

#define MAT_DEF_DOT(T,L,M,N) \
MAT_DECL_DOT(T,L,M,N) { \
    mat_(T,L,N) r; \
    for (int i = 0; i < L; ++i) { \
        for (int j = 0; j < N; ++j) { \
            T c = (T)(0); \
            for (int k = 0; k < M; ++k) { \
                c = tfn_(T,add)(c, tfn_(T,mul)(a.s[i*M + k], b.s[k*N + j])); \
            } \
            r.s[i*N + j] = c; \
        } \
    } \
    return r; \
} \


#define MAT_IMPL234(T) \
\
MAT_DECL_TYPE(T,2,2) \
MAT_DECL_TYPE(T,2,3) \
MAT_DECL_TYPE(T,2,4) \
MAT_DECL_TYPE(T,3,2) \
MAT_DECL_TYPE(T,3,3) \
MAT_DECL_TYPE(T,3,4) \
MAT_DECL_TYPE(T,4,2) \
MAT_DECL_TYPE(T,4,3) \
MAT_DECL_TYPE(T,4,4) \
\
MAT_DECL(T,2,2) \
MAT_DECL(T,2,3) \
MAT_DECL(T,2,4) \
MAT_DECL(T,3,2) \
MAT_DECL(T,3,3) \
MAT_DECL(T,3,4) \
MAT_DECL(T,4,2) \
MAT_DECL(T,4,3) \
MAT_DECL(T,4,4) \
\
MAT_DECL_DOT(T,2,2,2); \
MAT_DECL_DOT(T,2,2,3); \
MAT_DECL_DOT(T,2,2,4); \
MAT_DECL_DOT(T,2,3,2); \
MAT_DECL_DOT(T,2,3,3); \
MAT_DECL_DOT(T,2,3,4); \
MAT_DECL_DOT(T,2,4,2); \
MAT_DECL_DOT(T,2,4,3); \
MAT_DECL_DOT(T,2,4,4); \
MAT_DECL_DOT(T,3,2,2); \
MAT_DECL_DOT(T,3,2,3); \
MAT_DECL_DOT(T,3,2,4); \
MAT_DECL_DOT(T,3,3,2); \
MAT_DECL_DOT(T,3,3,3); \
MAT_DECL_DOT(T,3,3,4); \
MAT_DECL_DOT(T,3,4,2); \
MAT_DECL_DOT(T,3,4,3); \
MAT_DECL_DOT(T,3,4,4); \
MAT_DECL_DOT(T,4,2,2); \
MAT_DECL_DOT(T,4,2,3); \
MAT_DECL_DOT(T,4,2,4); \
MAT_DECL_DOT(T,4,3,2); \
MAT_DECL_DOT(T,4,3,3); \
MAT_DECL_DOT(T,4,3,4); \
MAT_DECL_DOT(T,4,4,2); \
MAT_DECL_DOT(T,4,4,3); \
MAT_DECL_DOT(T,4,4,4); \
\
MAT_DEF(T,2,2) \
MAT_DEF(T,2,3) \
MAT_DEF(T,2,4) \
MAT_DEF(T,3,2) \
MAT_DEF(T,3,3) \
MAT_DEF(T,3,4) \
MAT_DEF(T,4,2) \
MAT_DEF(T,4,3) \
MAT_DEF(T,4,4) \
\
MAT_DEF_DOT(T,2,2,2) \
MAT_DEF_DOT(T,2,2,3) \
MAT_DEF_DOT(T,2,2,4) \
MAT_DEF_DOT(T,2,3,2) \
MAT_DEF_DOT(T,2,3,3) \
MAT_DEF_DOT(T,2,3,4) \
MAT_DEF_DOT(T,2,4,2) \
MAT_DEF_DOT(T,2,4,3) \
MAT_DEF_DOT(T,2,4,4) \
MAT_DEF_DOT(T,3,2,2) \
MAT_DEF_DOT(T,3,2,3) \
MAT_DEF_DOT(T,3,2,4) \
MAT_DEF_DOT(T,3,3,2) \
MAT_DEF_DOT(T,3,3,3) \
MAT_DEF_DOT(T,3,3,4) \
MAT_DEF_DOT(T,3,4,2) \
MAT_DEF_DOT(T,3,4,3) \
MAT_DEF_DOT(T,3,4,4) \
MAT_DEF_DOT(T,4,2,2) \
MAT_DEF_DOT(T,4,2,3) \
MAT_DEF_DOT(T,4,2,4) \
MAT_DEF_DOT(T,4,3,2) \
MAT_DEF_DOT(T,4,3,3) \
MAT_DEF_DOT(T,4,3,4) \
MAT_DEF_DOT(T,4,4,2) \
MAT_DEF_DOT(T,4,4,3) \
MAT_DEF_DOT(T,4,4,4) \


#define int_neg(x) (-(x))
#define int_add(x, y) ((x) + (y))
#define int_sub(x, y) ((x) - (y))
#define int_mul(x, y) ((x)*(y))
#define int_div(x, y) ((x)/(y))

MAT_IMPL234(int)
