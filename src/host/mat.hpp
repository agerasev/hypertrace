#pragma once

#include <iostream>
#include <cassert>

#include <algebra/matrix.hh>


template <typename T, int M, int N> struct mat {};

template <typename T, int M, int N> mat<T,M,N> zero() { return mat<T,M,N>::not_implemented(); }
template <typename T, int N> mat<T,N,N> one() { return mat<T,N,N>::not_implemented(); }

template <typename T, int M, int N> mat<T,N,M> transpose(mat<T,M,N> m) { return mat<T,N,M>::not_implemented(); }

template <typename T, int M, int N> mat<T,M,N> operator+(mat<T,M,N> m) { return m; }
template <typename T, int M, int N> mat<T,M,N> operator-(mat<T,M,N> m) { return mat<T,M,N>::not_implemented(); }
template <typename T, int M, int N> mat<T,M,N> operator+(mat<T,M,N> a, mat<T,M,N> b) { return mat<T,M,N>::not_implemented(); }
template <typename T, int M, int N> mat<T,M,N> operator-(mat<T,M,N> a, mat<T,M,N> b) { return mat<T,M,N>::not_implemented(); }
template <typename T, int M, int N> mat<T,M,N> operator*(mat<T,M,N> a, T b) { return mat<T,M,N>::not_implemented(); }
template <typename T, int M, int N> mat<T,M,N> operator*(T b, mat<T,M,N> a) { return a*b; }
template <typename T, int M, int N> mat<T,M,N> operator/(mat<T,M,N> a, T b) { return mat<T,M,N>::not_implemented(); }

template <typename T, int L, int M, int N> mat<T,L,N> dot(mat<T,L,M> a, mat<T,M,N> b) { return mat<T,L,N>::not_implemented(); }

template <typename T, int N> T det(mat<T,N,N> m) { return T::not_implemented(); }
template <typename T, int N> mat<T,N,N> inverse(mat<T,N,N> m) { return mat<T,N,N>::not_implemented(); }


#define MXX_TYPE(T,M,N) \
typedef mat<T,M,N> T##M##N; \
template <> struct mat<T,M,N> : public MAT_(T,M,N) { \
    typedef MAT_(T,M,N) base; \
    mat() = default; \
    mat(base m) : base(m) {} \
    mat(std::initializer_list<T> l) { \
        assert(int(l.size()) == M*N); \
        std::copy(l.begin(), l.end(), s); \
    } \
    T &operator[](int i) { return s[i]; } \
    const T &operator[](int i) const { return s[i]; } \
}; \

#define MXX_COMMON(T,M,N) \
template <> inline mat<T,M,N> zero<T,M,N>() { return MATF_(T,M,N,zero)(); } \
template <> inline mat<T,N,M> transpose(mat<T,M,N> m) { return MATF_(T,M,N,transpose)(m); } \
template <> inline mat<T,M,N> operator-(mat<T,M,N> m) { return MATF_(T,M,N,neg)(m); } \
template <> inline mat<T,M,N> operator+(mat<T,M,N> a, mat<T,M,N> b) { return MATF_(T,M,N,add)(a, b); } \
template <> inline mat<T,M,N> operator-(mat<T,M,N> a, mat<T,M,N> b) { return MATF_(T,M,N,sub)(a, b); } \
template <> inline mat<T,M,N> operator*(mat<T,M,N> a, T b) { return MATF_(T,M,N,mul)(a, b); } \
template <> inline mat<T,M,N> operator/(mat<T,M,N> a, T b) { return MATF_(T,M,N,div)(a, b); } \

#define MXX_DOT(T,L,M,N) \
template <> inline mat<T,L,N> dot(mat<T,L,M> a, mat<T,M,N> b) { return T##L##x##M##x##N##_dot(a, b); }

#define MXX_SQUARE(T,N) \
template <> inline mat<T,N,N> one<T,N>() { return MATF_(T,N,N,one)(); } \

#define MXX_SQUARE2(T) \
MXX_SQUARE(T,2) \
\
template <> inline T det(mat<T,2,2> m) { return MATF_(T,2,2,det)(m); } \
template <> inline mat<T,2,2> inverse(mat<T,2,2> m) { return MATF_(T,2,2,inverse)(m); } \

#define MXX_SQUARE3(T) \
MXX_SQUARE(T,3) \

#define MXX_SQUARE4(T) \
MXX_SQUARE(T,4) \

#define MXX_2(T) \
\
MXX_TYPE(T,1,1) \
MXX_TYPE(T,1,2) \
MXX_TYPE(T,2,1) \
MXX_TYPE(T,2,2) \
\
MXX_COMMON(T,1,1) \
MXX_COMMON(T,1,2) \
MXX_COMMON(T,2,1) \
MXX_COMMON(T,2,2) \
\
MXX_DOT(T,1,1,1) \
MXX_DOT(T,1,1,2) \
MXX_DOT(T,1,2,1) \
MXX_DOT(T,1,2,2) \
MXX_DOT(T,2,1,1) \
MXX_DOT(T,2,1,2) \
MXX_DOT(T,2,2,1) \
MXX_DOT(T,2,2,2) \
\
MXX_SQUARE2(T) \

#define MXX_3(T) \
MXX_2(T) \
\
MXX_TYPE(T,1,3) \
MXX_TYPE(T,2,3) \
MXX_TYPE(T,3,1) \
MXX_TYPE(T,3,2) \
MXX_TYPE(T,3,3) \
\
MXX_COMMON(T,1,3) \
MXX_COMMON(T,2,3) \
MXX_COMMON(T,3,1) \
MXX_COMMON(T,3,2) \
MXX_COMMON(T,3,3) \
\
MXX_DOT(T,1,1,3) \
MXX_DOT(T,1,2,3) \
MXX_DOT(T,1,3,1) \
MXX_DOT(T,1,3,2) \
MXX_DOT(T,1,3,3) \
MXX_DOT(T,2,1,3) \
MXX_DOT(T,2,2,3) \
MXX_DOT(T,2,3,1) \
MXX_DOT(T,2,3,2) \
MXX_DOT(T,2,3,3) \
MXX_DOT(T,3,1,1) \
MXX_DOT(T,3,1,2) \
MXX_DOT(T,3,1,3) \
MXX_DOT(T,3,2,1) \
MXX_DOT(T,3,2,2) \
MXX_DOT(T,3,2,3) \
MXX_DOT(T,3,3,1) \
MXX_DOT(T,3,3,2) \
MXX_DOT(T,3,3,3) \
\
MXX_SQUARE3(T) \

#define MXX_4(T) \
MXX_3(T) \
\
MXX_TYPE(T,1,4) \
MXX_TYPE(T,2,4) \
MXX_TYPE(T,3,4) \
MXX_TYPE(T,4,1) \
MXX_TYPE(T,4,2) \
MXX_TYPE(T,4,3) \
MXX_TYPE(T,4,4) \
\
MXX_COMMON(T,1,4) \
MXX_COMMON(T,2,4) \
MXX_COMMON(T,3,4) \
MXX_COMMON(T,4,1) \
MXX_COMMON(T,4,2) \
MXX_COMMON(T,4,3) \
MXX_COMMON(T,4,4) \
\
MXX_DOT(T,1,1,4) \
MXX_DOT(T,1,2,4) \
MXX_DOT(T,1,3,4) \
MXX_DOT(T,1,4,1) \
MXX_DOT(T,1,4,2) \
MXX_DOT(T,1,4,3) \
MXX_DOT(T,1,4,4) \
MXX_DOT(T,2,1,4) \
MXX_DOT(T,2,2,4) \
MXX_DOT(T,2,3,4) \
MXX_DOT(T,2,4,1) \
MXX_DOT(T,2,4,2) \
MXX_DOT(T,2,4,3) \
MXX_DOT(T,2,4,4) \
MXX_DOT(T,3,1,4) \
MXX_DOT(T,3,2,4) \
MXX_DOT(T,3,3,4) \
MXX_DOT(T,3,4,1) \
MXX_DOT(T,3,4,2) \
MXX_DOT(T,3,4,3) \
MXX_DOT(T,3,4,4) \
MXX_DOT(T,4,1,1) \
MXX_DOT(T,4,1,2) \
MXX_DOT(T,4,1,3) \
MXX_DOT(T,4,1,4) \
MXX_DOT(T,4,2,1) \
MXX_DOT(T,4,2,2) \
MXX_DOT(T,4,2,3) \
MXX_DOT(T,4,2,4) \
MXX_DOT(T,4,3,1) \
MXX_DOT(T,4,3,2) \
MXX_DOT(T,4,3,3) \
MXX_DOT(T,4,3,4) \
MXX_DOT(T,4,4,1) \
MXX_DOT(T,4,4,2) \
MXX_DOT(T,4,4,3) \
MXX_DOT(T,4,4,4) \
\
MXX_SQUARE4(T) \


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


// Instantiation

#include <algebra/real.hh>
#include <algebra/complex.hh>

MXX_4(real)
MXX_2(complex)

inline std::ostream &operator<<(std::ostream &s, complex2x2 a) {
    return s << mat<complex,2,2>(a);
}

// Testing

#ifdef UNIT_TEST
#include <iostream>
#include <functional>

MXX_4(int)

template <typename T, int M, int N>
mat<T,M,N> rand_mat_map(TestRng &rng, std::function<T(TestRng&)> f) {
    mat<T,N,M> r;
    for (int i = 0; i < M*N; ++i) {
        r[i] = f(rng);
    }
    return r;
}

template <typename T, int M, int N, typename ApproxT>
class MatApprox {
    public:
    mat<T,M,N> m;
    MatApprox(mat<T,M,N> c) : m(c) {}
    friend bool operator==(mat<T,M,N> a, MatApprox b) {
        for (int i = 0; i < M*N; ++i) {
            if (a[i] != ApproxT(b.m[i])) {
                return false;
            }
        }
        return true;
    }
    friend bool operator==(MatApprox a, mat<T,M,N> b){
        return b == a;
    }
    friend bool operator!=(mat<T,M,N> a, MatApprox b){
        return !(a == b);
    }
    friend bool operator!=(MatApprox a, mat<T,M,N> b){
        return a != b;
    }
    friend std::ostream &operator<<(std::ostream &s, MatApprox a) {
        return s << a.m;
    }
};

#endif // UNIT_TEST
