#pragma once

#ifdef HOST
#include <iostream>
#endif

#include "types.hpp"
#include "traits.hpp"
#include "vector.hpp"
#include "complex.hpp"


template <typename T, int M, int N>
class matrix {
private:
    T s[M*N];

    template <int P=0, int Q, int R, typename ...Args>
    static void check_all_dims(matrix<T, Q, R>, Args ...args) {
        static_assert(M == Q && N == R, "Wrong matrix size");
        check_all_dims<P + 1>(args...);
    }
    template <int P=0>
    static void check_all_dims() {}

    template <int P, int Q, typename ...Args>
    void unwind(T x, Args ...args) {
        static_assert(Q < N, "Argument is out of row bounds (unreachable)");
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
        static_assert(P == M && Q == 0, "Too few elements in the constructor");
    }
public:
    matrix() = default;
    explicit matrix(T x) {
        for (int i = 0; i < M*N; ++i) {
            s[i] = x;
        }
    }
    template <typename ...Args>
    explicit matrix(Args ...args) {
        unwind<0, 0>(args...);
    }

    T *data() {
        return s;
    }
    const T *data() const {
        return s;
    }
    T &operator[](int i) {
        return s[i];
    }
    const T &operator[](int i) const {
        return s[i];
    }
    T &operator()(int i, int j) {
        return s[i*N + j];
    }
    const T &operator()(int i, int j) const {
        return s[i*N + j];
    }
    vector<T, N> row(int y) const {
        return vector<T, N>::load(s + y*N);
    }
    vector<T, M> col(int x) const {
        return vector<T, M>::load(s + x, N);
    }
    static constexpr int size() {
        return M*N;
    }
    template <int D>
    static constexpr int shape() {
        return D == 0 ? M : N;
    }
    static constexpr pair<int, int> shapes() {
        return make_pair(M, N);
    }

    static matrix<T, M, N> load(const T *data, int pitch=N, int stride=1) {
        matrix<T, M, N> r;
        for (int i = 0; i < M; ++i) {
            for (int j = 0; j < N; ++j) {
                r(i, j) = data[i*pitch + j*stride];
            }
        }
        return r;
    }
    void store(T *data, int pitch=N, int stride=1) const {
        for (int i = 0; i < M; ++i) {
            for (int j = 0; j < N; ++j) {
                data[i*pitch + j*stride] = (*this)(i, j);
            }
        }
    }

    template <typename F, typename ...Args>
    static matrix<T, M, N> map(F f, Args ...args) {
        check_all_dims(args...);
        matrix r;
        for (int i = 0; i < N*M; ++i) {
            r[i] = f((args[i])...);
        }
        return r;
    }

    friend matrix operator+(matrix a) {
        return a;
    }
    friend matrix operator-(matrix a) {
        return map([](T x) { return -x; }, a);
    }

    friend matrix operator+(matrix a, matrix b) {
        return map([](T x, T y) { return x + y; }, a, b);
    }
    friend matrix operator+(matrix a, T b) {
        return map([b](T x) { return x + b; }, a);
    }
    friend matrix operator+(T a, matrix b) {
        return map([a](T y) { return a + y; }, b);
    }
    friend matrix operator-(matrix a, matrix b) {
        return map([](T x, T y) { return x - y; }, a, b);
    }
    friend matrix operator-(matrix a, T b) {
        return map([b](T x) { return x - b; }, a);
    }
    friend matrix operator-(T a, matrix b) {
        return map([a](T y) { return a - y; }, b);
    }

    friend matrix operator*(matrix a, matrix b) {
        return map([](T x, T y) { return x * y; }, a, b);
    }
    friend matrix operator*(matrix a, T b) {
        return map([b](T x) { return x * b; }, a);
    }
    friend matrix operator*(T a, matrix b) {
        return map([a](T y) { return a * y; }, b);
    }
    friend matrix operator/(matrix a, matrix b) {
        return map([](T x, T y) { return x / y; }, a, b);
    }
    friend matrix operator/(matrix a, T b) {
        return map([b](T x) { return x / b; }, a);
    }
    friend matrix operator/(T a, matrix b) {
        return map([a](T y) { return a / y; }, b);
    }

    matrix &operator+=(matrix a) {
        return *this = *this + a;
    }
    matrix &operator+=(T a) {
        return *this = *this + a;
    }
    matrix &operator-=(matrix a) {
        return *this = *this - a;
    }
    matrix &operator-=(T a) {
        return *this = *this - a;
    }
    matrix &operator*=(matrix a) {
        return *this = *this * a;
    }
    matrix &operator*=(T a) {
        return *this = *this * a;
    }
    matrix &operator/=(matrix a) {
        return *this = *this / a;
    }
    matrix &operator/=(T a) {
        return *this = *this / a;
    }
};

// Remove matrix with at least one shape of 1
// Vectors and scalars are used instead
template <typename T, int M>
class matrix<T, M, 1> {};
template <typename T, int N>
class matrix<T, 1, N> {};

template <typename T, int L, int M, int N>
matrix<T, L, N> dot(matrix<T, L, M> a, matrix<T, M, N> b) {
    matrix<T, L, N> r;
    for (int i = 0; i < L; i++) {
        for (int j = 0; j < N; ++j) {
            r(i, j) = dot(a.row(i), b.col(j));
        }
    }
    return r;
}
template <typename T, int M, int N>
vector<T, M> dot(matrix<T, M, N> a, vector<T, N> b) {
    vector<T, M> r;
    for (int i = 0; i < M; i++) {
        r[i] = dot(a.row(i), b);
    }
    return r;
}
template <typename T, int M, int N>
vector<T, N> dot(vector<T, M> a, matrix<T, M, N> b) {
    vector<T, M> r;
    for (int j = 0; j < N; j++) {
        r[j] = dot(a, b.col(j));
    }
    return r;
}
template <typename T, int M, int N>
matrix<T, M, N> outer(vector<T, M> a, vector<T, N> b) {
    matrix<T, M, N> r;
    for (int i = 0; i < M; i++) {
        for (int j = 0; j < N; j++) {
            r(i, j) = a[i]*b[j];
        }
    }
    return r;
}

template <typename T, int M, int N>
struct BaseType<matrix<T, M, N>> {
    typedef T type;
};

template <typename T, int M, int N>
struct Zero<matrix<T, M, N>> {
    static matrix<T, M, N> zero() {
        return matrix<T, M, N>(::zero<T>());
    }
};

template <typename T, int N>
struct One<matrix<T, N, N>> {
    static matrix<T, N, N> one() {
        matrix<T, N, N> r(::zero<T>());
        for (int i = 0; i < N; ++i) {
            r(i, i) = ::one<T>();
        }
        return r;
    }
};

template <typename T, int M, int N>
matrix<T, N, M> transpose(matrix<T, M, N> m) {
    return matrix<T, N, M>::load(m.data(), 1, N);
}

template <typename T, int M, int N>
matrix<T, M - 1, N - 1> submatrix(matrix<T, M, N> m, int y, int x) {
    static_assert(N >= 3 && M >= 3, "One of the matrix shape is less than 3");
    matrix<T, M - 1, N - 1> r;
    for (int i = 0; i < M - 1; ++i) {
        for (int j = 0; j < N - 1; ++j) {
            r(i, j) = m(i + (i >= y), j + (j >= x));
        }
    }
    return r;
}

template <typename T, int N>
T cofactor(matrix<T, N, N> m, int y, int x) {
    return (((x + y) % 2) == 0 ? one<T>() : -one<T>())*det(submatrix(m, y, x));
}

template <typename T>
T cofactor(matrix<T, 2, 2> m, int y, int x) {
    return (((x + y) % 2) == 0 ? one<T>() : -one<T>())*m(1 - y, 1 - x);
}

// Determinant
template <typename T, int N>
T det(matrix<T, N, N> m) {
    T t = zero<T>();
    int j = 0;
    for (int i = 0; i < N; ++i) {
        t += m(i, j)*cofactor(m, i, j);
    }
    return t;
}

template <typename T>
T det(matrix<T, 2, 2> m) {
    return m(0,0)*m(1,1) - m(0,1)*m(1,0);
}

// Adjugate matrix
template <typename T, int N>
matrix<T, N, N> adj(matrix<T, N, N> m) {
    matrix<T, N, N> r;
    for (int i = 0; i < N; ++i) {
        for (int j = 0; j < N; ++j) {
            r(i, j) = cofactor(m, j, i);
        }
    }
    return r;
}
template <typename T>
matrix<T, 2, 2> adj(matrix<T, 2, 2> m) {
    return matrix<T, 2, 2>(
         m[3], -m[1],
        -m[2],  m[0]
    );
}

template <typename T, int N>
matrix<T, N, N> inverse(matrix<T, N, N> m) {
    return adj(m)/det(m);
}

template <typename T, int N>
matrix<T, N, N> operator!(matrix<T, N, N> m) {
    return inverse(m);
}


template <typename S, int M, int N>
struct Sequence<matrix<S, M, N>> {
private:
    typedef base_type<S> T;
    typedef matrix<S, M, N> V;
public:
    template <typename F>
    static V map(F f, V a) {
        return V::map([f](S x) {
            return Sequence<S>::map(f, x);
        }, a);
    }
    template <typename F>
    static T reduce(F f, T t, V a) {
        for (int i = 0; i < M*N; ++i) {
            t = Sequence<S>::reduce(f, t, a[i]);
        }
        return t;
    }
};

#define DEFINE_MATRIX_MATH_FUNCTION_A(F) \
template <typename T, int M, int N> \
matrix<T, M, N> F(matrix<T, M, N> a) { \
    return matrix<T, M, N>::map([](T x) { return F(x); }, a); \
} \

#define DEFINE_MATRIX_MATH_FUNCTION_AB(F) \
template <typename T, int M, int N> \
matrix<T, M, N> F(matrix<T, M, N> a, matrix<T, M, N> b) { \
    return matrix<T, M, N>::map([](T x, T y) { return F(x, y); }, a, b); \
} \

namespace math {

DEFINE_MATRIX_MATH_FUNCTION_A(abs)
DEFINE_MATRIX_MATH_FUNCTION_AB(max)
DEFINE_MATRIX_MATH_FUNCTION_AB(min)
DEFINE_MATRIX_MATH_FUNCTION_A(acos)
DEFINE_MATRIX_MATH_FUNCTION_A(acosh)
DEFINE_MATRIX_MATH_FUNCTION_A(asin)
DEFINE_MATRIX_MATH_FUNCTION_A(asinh)
DEFINE_MATRIX_MATH_FUNCTION_A(atan)
DEFINE_MATRIX_MATH_FUNCTION_AB(atan2)
DEFINE_MATRIX_MATH_FUNCTION_A(atanh)
DEFINE_MATRIX_MATH_FUNCTION_A(ceil)
DEFINE_MATRIX_MATH_FUNCTION_A(cos)
DEFINE_MATRIX_MATH_FUNCTION_A(cosh)
DEFINE_MATRIX_MATH_FUNCTION_A(erfc)
DEFINE_MATRIX_MATH_FUNCTION_A(erf)
DEFINE_MATRIX_MATH_FUNCTION_A(exp)
DEFINE_MATRIX_MATH_FUNCTION_A(floor)
DEFINE_MATRIX_MATH_FUNCTION_AB(mod)
DEFINE_MATRIX_MATH_FUNCTION_A(log)
DEFINE_MATRIX_MATH_FUNCTION_AB(pow)
DEFINE_MATRIX_MATH_FUNCTION_A(round)
DEFINE_MATRIX_MATH_FUNCTION_A(sqrt)
DEFINE_MATRIX_MATH_FUNCTION_A(sin)
DEFINE_MATRIX_MATH_FUNCTION_A(sinh)
DEFINE_MATRIX_MATH_FUNCTION_A(tan)
DEFINE_MATRIX_MATH_FUNCTION_A(tanh)
DEFINE_MATRIX_MATH_FUNCTION_A(tgamma)
template <typename T, int M, int N>
matrix<T, M, N> clamp(matrix<T, M, N> a, matrix<T, M, N> b, matrix<T, M, N> c) {
    return matrix<T, M, N>::map(
        [](T x, T y, T z) { return clamp(x, y, z); },
        a, b, c
    );
}
template <typename T, int M, int N>
pair<matrix<T, M, N>, matrix<T, M, N>> fract(matrix<T, M, N> a) {
    pair<matrix<T, M, N>, matrix<T, M, N>> p;
    for (int i = 0; i < M*N; ++i) {
        pair<T, T> x = fract(a[i]);
        p.first[i] = x.first;
        p.second[i] = x.second;
    }
    return p;
}

} // namespace math

using namespace math;

template <typename T, int N>
matrix<T, N, N> normalize(matrix<T, N, N> m) {
    return m/pow(det(m), 1_r/N);
}
template <typename T>
matrix<T, 2, 2> normalize(matrix<T, 2, 2> m) {
    return m/sqrt(det(m));
}

template <typename T>
void eigen(
    matrix<T, 2, 2> m,
    matrix<T, 2, 2> &l,
    matrix<T, 2, 2> &v,
    bool normalized=false
) {
    const T T0 = zero<T>(), T1 = one<T>();
    const T D = normalized ? T1 : det(m);
    if (norm_l1(m[1]) < EPS && norm_l1(m[2]) < EPS) {
        l = matrix<T, 2, 2>(m[0], T0, T0, m[3]);
        v = matrix<T, 2, 2>(T1, T0, T0, T1);
    } else {
        T m0p3 = m[0] + m[3];
        T ad = m0p3/2;
        T dis = sqrt(ad*ad - D);
        if (norm_l1(dis) > EPS) {
            l = matrix<T, 2, 2>(ad + dis, T0, T0, ad - D);
            if (norm_l1(m[1]) > EPS) {
                v = matrix<T, 2, 2>(m[1], m[1], l[0] - m[0], l[3] - m[0]);
            } else {
                v = matrix<T, 2, 2>(l[0] - m[3], l[3] - m[3], m[2], m[2]);
            }
        } else {
            T m0m3 = m[0] - m[3];
            l = matrix<T, 2, 2>(ad, T1, T0, ad);
            if (norm_l1(m[1]) > EPS) {
                T g = 4*m[1]*m[1] + m0m3*m0m3;
                v = matrix<T, 2, 2>(
                    m[1],
                    2*m[1]*m0m3/g,
                    -m0m3/2,
                    4*m[1]*m[1]/g
                );
            } else {
                T g = 4*m[2]*m[2] + m0m3*m0m3;
                v = matrix<T, 2, 2>(
                    m0m3/2,
                    4*m[2]*m[2]/g,
                    m[2],
                    -2*m[2]*m0m3/g
                );
            }
        }
        v = normalize(v);
    }
}
/*
complex2x2 _complex2x2_pow_common(
    complex2x2 m,
    real p,
    complex2x2 j,
    complex2x2 v
) {
    complex2x2 x = complex2x2_sub(m, complex2x2_one());
    real y = complex2x2_fabs(x);
    // FIXME: Why 1e4?
    if (y*y > 1e4*EPS) {
        complex2x2 k;
        if (c_fabs(j.s[1]) < EPS) {
            k = complex2x2_new(c_powr(j.s[0], p), C0, C0, c_powr(j.s[3], p));
        } else {
            // Assume j.s[0] == j.s[3]
            k = complex2x2_new(
                c_powr(j.s[0], p),
                p*c_mul(c_powr(j.s[0], p - 1), j.s[1]),
                C0,
                c_powr(j.s[3], p)
            );
        }
        return complex2x2x2_dot(
            complex2x2x2_dot(v, k),
            complex2x2_inverse(v)
        );
    } else {
        return complex2x2_add(
            complex2x2_one(),
            complex2x2_mul(x, c_new(p, 0))
        );
    }
}

complex2x2 complex2x2_pow(complex2x2 m, real p) {
    complex2x2 j, v;
    complex2x2_eigen(m, &j, &v);
    return _complex2x2_pow_common(m, p, j, v);
}

complex2x2 complex2x2_pow_normalized(complex2x2 m, real p) {
    complex2x2 j, v;
    complex2x2_eigen_normalized(m, &j, &v);
    complex2x2 r = _complex2x2_pow_common(m, p, j, v);
    //r = complex2x2_normalize(r);
    return r;
}
*/

typedef matrix<real, 2, 2> real2x2;
typedef matrix<real, 2, 3> real2x3;
typedef matrix<real, 2, 4> real2x4;
typedef matrix<real, 3, 2> real3x2;
typedef matrix<real, 3, 3> real3x3;
typedef matrix<real, 3, 4> real3x4;
typedef matrix<real, 4, 2> real4x2;
typedef matrix<real, 4, 3> real4x3;
typedef matrix<real, 4, 4> real4x4;

#ifdef HOST
template <typename T, int M, int N>
std::ostream &operator<<(std::ostream &s, matrix<T, M, N> v) {
    s << std::endl;
    for (int i = 0; i < M; ++i) {
        s << "|\t";
        for (int j = 0; j < N; ++j) {
            s << v(i, j) << "\t";
        }
        s << "|" << std::endl;
    }
    return s;
}
#endif


#ifdef UNIT_TEST
#include <catch.hpp>
#include "test.hpp"

class MatTestRng : public VecTestRng {
public:
    inline MatTestRng(uint32_t seed) : VecTestRng(seed) {}
    template <int M, int N>
    matrix<real, M, N> mat_normal() {
        return matrix<real, M, N>::map([this]() { return normal(); });
    }
    template <int M, int N>
    matrix<real, M, N> mat_uniform() {
        return matrix<real, M, N>::map([this]() { return uniform(); });
    }
    template <int N>
    matrix<real, N, N> mat_invertible() {
        matrix<real, N, N> r;
        do {
            r = mat_normal<N, N>();
        } while(det(r) > EPS);
        return r;
    }
};

template <typename T, int M, int N>
class MatApprox {
private:
    matrix<T, M, N> v;

public:
    MatApprox(matrix<T, M, N> c) : v(c) {}

    bool operator==(matrix<T, M, N> a) {
        for (int i = 0; i < M; ++i) {
            for (int j = 0; j < N; ++j) {
                if (a(i, j) != approx(v(i, j))) {
                    return false;
                }
            }
        }
        return true;
    }

    friend std::ostream &operator<<(std::ostream &s, MatApprox a) {
        return s << a.v;
    }
};

TEST_DEFINE_CMP_OPS(
    template <typename T COMMA int M COMMA int N>,
    MatApprox<T COMMA M COMMA N>, matrix<T COMMA M COMMA N>
)

template <typename T, int M, int N>
MatApprox<T, M, N> approx(matrix<T, M, N> v) {
    return MatApprox<T, M, N>(v);
}

#endif
