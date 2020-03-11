#pragma once

#include <iostream>
#include <cassert>

#include <algebra/matrix.hh>


template <typename T, int M, int N> struct mat {};

template <typename T, int M, int N> mat<T,N,M> transpose(mat<T,M,N> m) { return mat<T,N,M>::not_implemented(); }

template <typename T, int M, int N> mat<T,M,N> operator+(mat<T,M,N> m) { return m; }
template <typename T, int M, int N> mat<T,M,N> operator-(mat<T,M,N> m) { return mat<T,M,N>::not_implemented(); }
template <typename T, int M, int N> mat<T,M,N> operator+(mat<T,M,N> a, mat<T,M,N> b) { return mat<T,M,N>::not_implemented(); }
template <typename T, int M, int N> mat<T,M,N> operator-(mat<T,M,N> a, mat<T,M,N> b) { return mat<T,M,N>::not_implemented(); }
template <typename T, int M, int N> mat<T,M,N> operator*(mat<T,M,N> a, T b) { return mat<T,M,N>::not_implemented(); }
template <typename T, int M, int N> mat<T,M,N> operator*(T b, mat<T,M,N> a) { return a*b; }
template <typename T, int M, int N> mat<T,M,N> operator/(mat<T,M,N> a, T b) { return mat<T,M,N>::not_implemented(); }

template <typename T, int L, int M, int N> mat<T,L,N> dot(mat<T,L,M> a, mat<T,M,N> b) { return mat<T,L,N>::not_implemented(); }


#define MAT_PART_SPEC_TYPE(T,M,N) \
typedef mat<T,M,N> T##M##N; \
template <> struct mat<T,M,N> : public mat_(T,M,N) { \
    typedef mat_(T,M,N) base; \
    mat(base m) : base(m) {} \
    mat(std::initializer_list<T> l) { \
        assert(int(l.size()) == M*N); \
        std::copy(l.begin(), l.end(), s); \
    } \
    T &operator[](int i) { \
        return s[i]; \
    } \
    const T &operator[](int i) const { \
        return s[i]; \
    } \
}; \

#define MAT_PART_SPEC_DEF(T,N,M) \
template <> \
mat<T,N,M> transpose(mat<T,M,N> m) { \
    return matfn_(T,M,N,transpose)(m); \
} \
template <> mat<T,M,N> operator-(mat<T,M,N> m) { return matfn_(T,M,N,neg)(m); } \
template <> mat<T,M,N> operator+(mat<T,M,N> a, mat<T,M,N> b) { return matfn_(T,M,N,add)(a, b); } \
template <> mat<T,M,N> operator-(mat<T,M,N> a, mat<T,M,N> b) { return matfn_(T,M,N,sub)(a, b); } \
template <> mat<T,M,N> operator*(mat<T,M,N> a, T b) { return matfn_(T,M,N,mul)(a, b); } \
template <> mat<T,M,N> operator/(mat<T,M,N> a, T b) { return matfn_(T,M,N,div)(a, b); } \

#define MAT_PART_SPEC_DOT(T,L,M,N) \
template <> mat<T,L,N> dot(mat<T,L,M> a, mat<T,M,N> b) { return T##L##x##M##x##N##_dot(a, b); }


#define MAT_PART_SPEC_234(T) \
MAT_PART_SPEC_TYPE(T,2,2) \
MAT_PART_SPEC_TYPE(T,2,3) \
MAT_PART_SPEC_TYPE(T,2,4) \
MAT_PART_SPEC_TYPE(T,3,2) \
MAT_PART_SPEC_TYPE(T,3,3) \
MAT_PART_SPEC_TYPE(T,3,4) \
MAT_PART_SPEC_TYPE(T,4,2) \
MAT_PART_SPEC_TYPE(T,4,3) \
MAT_PART_SPEC_TYPE(T,4,4) \
\
MAT_PART_SPEC_DEF(T,2,2) \
MAT_PART_SPEC_DEF(T,2,3) \
MAT_PART_SPEC_DEF(T,2,4) \
MAT_PART_SPEC_DEF(T,3,2) \
MAT_PART_SPEC_DEF(T,3,3) \
MAT_PART_SPEC_DEF(T,3,4) \
MAT_PART_SPEC_DEF(T,4,2) \
MAT_PART_SPEC_DEF(T,4,3) \
MAT_PART_SPEC_DEF(T,4,4) \
\
MAT_PART_SPEC_DOT(T,2,2,2) \
MAT_PART_SPEC_DOT(T,2,2,3) \
MAT_PART_SPEC_DOT(T,2,2,4) \
MAT_PART_SPEC_DOT(T,2,3,2) \
MAT_PART_SPEC_DOT(T,2,3,3) \
MAT_PART_SPEC_DOT(T,2,3,4) \
MAT_PART_SPEC_DOT(T,2,4,2) \
MAT_PART_SPEC_DOT(T,2,4,3) \
MAT_PART_SPEC_DOT(T,2,4,4) \
MAT_PART_SPEC_DOT(T,3,2,2) \
MAT_PART_SPEC_DOT(T,3,2,3) \
MAT_PART_SPEC_DOT(T,3,2,4) \
MAT_PART_SPEC_DOT(T,3,3,2) \
MAT_PART_SPEC_DOT(T,3,3,3) \
MAT_PART_SPEC_DOT(T,3,3,4) \
MAT_PART_SPEC_DOT(T,3,4,2) \
MAT_PART_SPEC_DOT(T,3,4,3) \
MAT_PART_SPEC_DOT(T,3,4,4) \
MAT_PART_SPEC_DOT(T,4,2,2) \
MAT_PART_SPEC_DOT(T,4,2,3) \
MAT_PART_SPEC_DOT(T,4,2,4) \
MAT_PART_SPEC_DOT(T,4,3,2) \
MAT_PART_SPEC_DOT(T,4,3,3) \
MAT_PART_SPEC_DOT(T,4,3,4) \
MAT_PART_SPEC_DOT(T,4,4,2) \
MAT_PART_SPEC_DOT(T,4,4,3) \
MAT_PART_SPEC_DOT(T,4,4,4) \


MAT_PART_SPEC_234(int)


template <typename T, int M, int N>
bool operator==(mat<T,M,N> a, mat<T,M,N> b) {
    bool eq = true;
    for (int j = 0; j < M; ++j) {
        for (int i = 0; i < N; ++i) {
            eq = eq && (a.s[j*N + i] == b.s[j*N + i]);
        }
    }
    return eq;
}

template <typename T, int M, int N>
std::ostream &operator<<(std::ostream &o, mat<T,M,N> a) {
    o << std::endl;
    for (int j = 0; j < M; ++j) {
        o << "| ";
        for (int i = 0; i < N; ++i) {
            o << a.s[j*N + i];
            if (i < N-1) {
                o << ",\t";
            }
        }
        o << "\t|" << std::endl;
    }
    return o;
}
