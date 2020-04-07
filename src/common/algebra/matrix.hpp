#pragma once

#include "traits.hpp"
#include "vector.hpp"


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

    template <typename F>
    friend matrix<T, M, N> map(F f, matrix<T, M, N> a) {
        matrix<T, M, N> r;
        for (int i = 0; i < M*N; ++i) {
            r.s[i] = f(a.s[i]);
        }
        return r;
    }
    template <typename F>
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
