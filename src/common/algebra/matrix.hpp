#pragma once

#include "traits.hpp"
#include "vector.hpp"
#include "complex.hpp"


template <typename T, int M, int N>
class matrix;

template <typename T, int M, int N, typename F, typename ...Args>
matrix<T, M, N> map(F f, Args ...args);

template <typename T, int M, int N>
class matrix {
private:
    T s[M*N];

public:
    template <int P=0, int Q, int R, typename ...Args>
    static void check_all_dims(matrix<T, Q, R>, Args ...args) {
        static_assert(M == Q && N == R, "Wrong matrix size");
        check_all_dims<P + 1>(args...);
    }
    template <int P=0>
    static void check_all_dims() {}

private:
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

    template <typename F, typename ...Args>
    static matrix map(F f, Args ...args) {
        return ::map<T, N>(f, args...);
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

template <typename T, int M, int N, typename F, typename ...Args>
matrix<T, M, N> map(F f, Args ...args) {
    matrix<T, M, N>::template check_all_dims(args...);
    matrix<T, M, N> r;
    for (int i = 0; i < N*M; ++i) {
        r[i] = f((args[i])...);
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
        for (int j = 0; j < N - 1; ++i) {
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

template <typename T, int N>
matrix<T, N, N> inverse(matrix<T, N, N> m) {
    return adj(m)/det(m);
}

template <typename T, int N>
matrix<T, N, N> operator!(matrix<T, N, N> m) {
    return inverse(m);
}

typedef matrix<real, 2, 2> real2x2;
typedef matrix<real, 3, 3> real3x3;
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
        return map<real, M, N>([this]() { return normal(); });
    }
    template <int M, int N>
    matrix<real, M, N> mat_uniform() {
        return map<real, M, N>([this]() { return uniform(); });
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
