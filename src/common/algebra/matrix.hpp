#pragma once


template <typename T, int M, int N>
class matrix {
private:
    T s[M*N];

    template <int P, int Q, typename ...Args>
    void unwind(T x, Args ...args) {
        static_assert(Q < N, "Out of row bounds (unreachable)");
        static_assert(P < M, "Too many elements in the constructor");
        s[P*N + Q] = x;
        unwind<
            Q + 1 < N ? P : P + 1,
            Q + 1 < N ? Q + 1 : 0
        >(args...);
    }
    template <int P, int Q, typename ...Args>
    void unwind(vector<T, N> x, Args ...args) {
        static_assert(Q == 0, "Vector must match row");
        static_assert(P < M, "Too many rows in the constructor");
        x.store(s + P*N);
        unwind<P + 1, 0>(args...);
    }
    template <int P, int Q>
    void unwind() {
        static_assert(P == M && Q == N, "Too few elements in the constructor");
    }
public:
    matrix() = default;
    matrix(T x) {
        for (int i = 0; i < M*N; ++i) {
            s[i] = x;
        }
    }
    template <typename ...Args>
    matrix(Args ...args) {
        unwind<0, 0>(args...);
    }

    template <typename T, int M, int N, typename F>
    friend matrix<T, M, N> map(F f, matrix<T, M, N> a) {
        matrix<T, M, N> r;
        for (int i = 0; i < M*N; ++i) {
            r.s[i] = f(a.s[i]);
        }
        return r;
    }
    template <typename T, int M, int N, typename F>
    friend matrix<T, M, N> map2(F f, matrix<T, M, N> a, matrix<T, M, N> b) {
        matrix<T, M, N> r;
        for (int i = 0; i < M*N; ++i) {
            r.s[i] = f(a.s[i], b.s[i]);
        }
        return r;
    }

    T &operator[](int i) {
        return s[i];
    }
    const T &operator[](int i) const {
        return s[i];
    }
    T &elem(int i, int j) {
        return s[i*N + j];
    }
    const T &elem(int i, int j) const {
        return s[i*N + j];
    }

    matrix<T, M, N> load(
        const T *data,
        const int pitch=N, const int stride=1
    ) {
        matrix<T, M, N> r;
        for (int i = 0; i < M; ++i) {
            for (int j = 0; j < N; ++j) {
                r.elem(i, j) = data[i*pitch + j*stride];
            }
        }
        return r;
    }
};

MAT_(T,N,M) transpose(matrix<T, M, N> m);

MAT_MAP_H(T,M,N,neg);
MAT_MAP2_H(T,M,N,add);
MAT_MAP2_H(T,M,N,sub);
MAT_MAPC_H(T,M,N,mul);
MAT_MAPC_H(T,M,N,div);

// Square matrix functionality

#define MAT_SQUARE_H(T,N)
MAT_(T,N,N) one();

#define MAT_SQUARE2_H(T)
MAT_SQUARE_H(T,2)

T det(MAT_(T,2,2) m);
MAT_(T,2,2) inverse(MAT_(T,2,2) m);

#define MAT_SQUARE3_H(T)
MAT_SQUARE_H(T,3)

#define MAT_SQUARE4_H(T)
MAT_SQUARE_H(T,4)


// Dot product

#define MAT_DOT_H(T,L,M,N)
MAT_(T,L,N) T##L##x##M##x##N##_dot(MAT_(T,L,M) a, matrix<T, M, N> b)


// All in one up to specified dimension

#define MAT2_H(T)

MAT_TYPE_H(T,1,1)
MAT_TYPE_H(T,1,2)
MAT_TYPE_H(T,2,1)
MAT_TYPE_H(T,2,2)

MAT_COMMON_H(T,1,1)
MAT_COMMON_H(T,1,2)
MAT_COMMON_H(T,2,1)
MAT_COMMON_H(T,2,2)

MAT_DOT_H(T,1,1,1);
MAT_DOT_H(T,1,1,2);
MAT_DOT_H(T,1,2,1);
MAT_DOT_H(T,1,2,2);
MAT_DOT_H(T,2,1,1);
MAT_DOT_H(T,2,1,2);
MAT_DOT_H(T,2,2,1);
MAT_DOT_H(T,2,2,2);

MAT_SQUARE2_H(T);

#define MAT3_H(T)

MAT2_H(T)

MAT_TYPE_H(T,1,3)
MAT_TYPE_H(T,2,3)
MAT_TYPE_H(T,3,1)
MAT_TYPE_H(T,3,2)
MAT_TYPE_H(T,3,3)

MAT_COMMON_H(T,1,3)
MAT_COMMON_H(T,2,3)
MAT_COMMON_H(T,3,1)
MAT_COMMON_H(T,3,2)
MAT_COMMON_H(T,3,3)

MAT_DOT_H(T,1,1,3);
MAT_DOT_H(T,1,2,3);
MAT_DOT_H(T,1,3,1);
MAT_DOT_H(T,1,3,2);
MAT_DOT_H(T,1,3,3);
MAT_DOT_H(T,2,1,3);
MAT_DOT_H(T,2,2,3);
MAT_DOT_H(T,2,3,1);
MAT_DOT_H(T,2,3,2);
MAT_DOT_H(T,2,3,3);
MAT_DOT_H(T,3,1,1);
MAT_DOT_H(T,3,1,2);
MAT_DOT_H(T,3,1,3);
MAT_DOT_H(T,3,2,1);
MAT_DOT_H(T,3,2,2);
MAT_DOT_H(T,3,2,3);
MAT_DOT_H(T,3,3,1);
MAT_DOT_H(T,3,3,2);
MAT_DOT_H(T,3,3,3);

MAT_SQUARE3_H(T);

#define MAT4_H(T)

MAT3_H(T)

MAT_TYPE_H(T,1,4)
MAT_TYPE_H(T,2,4)
MAT_TYPE_H(T,3,4)
MAT_TYPE_H(T,4,1)
MAT_TYPE_H(T,4,2)
MAT_TYPE_H(T,4,3)
MAT_TYPE_H(T,4,4)

MAT_COMMON_H(T,1,4)
MAT_COMMON_H(T,2,4)
MAT_COMMON_H(T,3,4)
MAT_COMMON_H(T,4,1)
MAT_COMMON_H(T,4,2)
MAT_COMMON_H(T,4,3)
MAT_COMMON_H(T,4,4)

MAT_DOT_H(T,1,1,4);
MAT_DOT_H(T,1,2,4);
MAT_DOT_H(T,1,3,4);
MAT_DOT_H(T,1,4,1);
MAT_DOT_H(T,1,4,2);
MAT_DOT_H(T,1,4,3);
MAT_DOT_H(T,1,4,4);
MAT_DOT_H(T,2,1,4);
MAT_DOT_H(T,2,2,4);
MAT_DOT_H(T,2,3,4);
MAT_DOT_H(T,2,4,1);
MAT_DOT_H(T,2,4,2);
MAT_DOT_H(T,2,4,3);
MAT_DOT_H(T,2,4,4);
MAT_DOT_H(T,3,1,4);
MAT_DOT_H(T,3,2,4);
MAT_DOT_H(T,3,3,4);
MAT_DOT_H(T,3,4,1);
MAT_DOT_H(T,3,4,2);
MAT_DOT_H(T,3,4,3);
MAT_DOT_H(T,3,4,4);
MAT_DOT_H(T,4,1,1);
MAT_DOT_H(T,4,1,2);
MAT_DOT_H(T,4,1,3);
MAT_DOT_H(T,4,1,4);
MAT_DOT_H(T,4,2,1);
MAT_DOT_H(T,4,2,2);
MAT_DOT_H(T,4,2,3);
MAT_DOT_H(T,4,2,4);
MAT_DOT_H(T,4,3,1);
MAT_DOT_H(T,4,3,2);
MAT_DOT_H(T,4,3,3);
MAT_DOT_H(T,4,3,4);
MAT_DOT_H(T,4,4,1);
MAT_DOT_H(T,4,4,2);
MAT_DOT_H(T,4,4,3);
MAT_DOT_H(T,4,4,4);

MAT_SQUARE4_H(T);

// Packing

#ifdef OPENCL_INTEROP

#define MAT_PK_(T,N,M) T##N##x##M##_pk
#define ,NM) F##_##T##N##x##M
#define TF_PK_(F,T) F##_##T

#define MAT_PK_H(T,M,N)
typedef struct __attribute__((packed, aligned(8))) MAT_PK_(T,M,N) {
    T##_pk s[M*N] __attribute__((aligned(8)));
} MAT_PK_(T,M,N);
MAT_PK_(T,M,N) kT,M,N)(matrix<T, M, N> m);
matrix<T, M, N> ackT,M,N)(MAT_PK_(T,M,N) m);

#define MAT2_PK_H(T)
MAT_PK_H(T,1,1)
MAT_PK_H(T,1,2)
MAT_PK_H(T,2,1)
MAT_PK_H(T,2,2)

#define MAT3_PK_H(T)
MAT2_PK_H(T)
MAT_PK_H(T,1,3)
MAT_PK_H(T,2,3)
MAT_PK_H(T,3,1)
MAT_PK_H(T,3,2)
MAT_PK_H(T,3,3)

#define MAT4_PK_H(T)
MAT3_PK_H(T)
MAT_PK_H(T,1,4)
MAT_PK_H(T,2,4)
MAT_PK_H(T,3,4)
MAT_PK_H(T,4,1)
MAT_PK_H(T,4,2)
MAT_PK_H(T,4,3)
MAT_PK_H(T,4,4)

#endif // OPENCL_INTEROP

// Instantiation

#include "real.hh"
#include "complex.hh"

MAT4_H(real)
MAT2_H(complex)

#ifdef OPENCL_INTEROP
MAT4_PK_H(real)
MAT2_PK_H(complex)
#endif // OPENCL_INTEROP

complex2x2 complex2x2_new(complex a, complex b, complex c, complex d);
real complex2x2_fabs(complex2x2 m);

complex2x2 complex2x2_normalize(complex2x2 m);
complex2x2 complex2x2_inverse_normalized(complex2x2 m);

void complex2x2_eigen(complex2x2 m, complex2x2 *l, complex2x2 *v);
void complex2x2_eigen_normalized(complex2x2 m, complex2x2 *l, complex2x2 *v);

complex2x2 complex2x2_pow(complex2x2 m, real p);
complex2x2 complex2x2_pow_normalized(complex2x2 m, real p);

#ifdef UNIT_TEST
MAT4_H(int)
#endif // UNIT_TEST

#ifndef OPENCL
#include <mat.hpp>
#endif // OPENCL