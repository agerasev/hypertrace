#pragma once

#include <types.h>
#include <real.h>
#include <math.hpp>
#include "traits.hpp"


template <typename T, int N, int L, int A>
class vector_base;
template <typename T, int N>
class vector;

template <typename T, int N, int L, int A>
class __attribute__((aligned(A))) vector_base {
    static_assert(L >= N, "`L` must be greater or equal than `N`");
    
public:
    static const int LEN = L;
    template <int P=0, typename ...Args>
    static constexpr void check_args(vector<T, N>, Args ...args) {
        check_args<P + 1>(args...);
    }
    template <int P=0>
    static constexpr void check_args() {}
    
    template <int P, int ...Ps>
    static constexpr void check_indices() {
        static_assert(P >= 0 && P < N, "Index is out of bounds");
        check_indices<Ps...>();
    }
    static constexpr void check_indices() {}

protected:
    T s[L];

private:
    vector<T, N> &as_vec() {
        return *reinterpret_cast<vector<T, N>*>(this);
    }
    const vector<T, N> &as_vec() const {
        return *reinterpret_cast<const vector<T, N>*>(this);
    }

    template <int P=0, typename ...Args>
    void unwind(T x, Args ...args) {
        static_assert(P < N, "Too many elements in the constructor");
        s[P] = x;
        unwind<P + 1>(args...);
    }
    template <int P=0, int M, typename ...Args>
    void unwind(vector<T, M> x, Args ...args) {
        static_assert(P + M <= N, "Too many elements in the constructor");
        for (int i = 0; i < M; ++i) {
            s[P + i] = x[i];
        }
        unwind<P + M>(args...);
    }
    template <int P=0>
    void unwind() {
        static_assert(P == N, "Too few elements in the constructor");
    }
public:
    vector_base() = default;
    explicit vector_base(T x) {
        for (int i = 0; i < N; ++i) {
            s[i] = x;
        }
    }
    template <typename ...Args, enable_if<(sizeof...(Args) > 1)>* = nullptr>
    explicit vector_base(Args ...args) {
        unwind(args...);
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
    static constexpr int size() {
        return N;
    }
    template <int P>
    T &elem() {
        static_assert(P >= 0 && P < N, "Index is out of bounds");
        return s[P];
    }
    template <int P>
    const T &elem() const {
        static_assert(P >= 0 && P < N, "Index is out of bounds");
        return s[P];
    }
    template <int P, int S>
    vector<T, S> &slice() {
        static const int SA = vector<T, S>::LEN;
        static_assert(P >= 0 && S > 0 && P + SA <= N, "Indices is out of bounds");
        static_assert((LEN % SA) == 0 && (P % SA) == 0, "Slicing breaks alignment");
        return *reinterpret_cast<vector<T, S>*>(s + P);
    }
    template <int P, int S>
    const vector<T, S> &slice() const {
        static const int SA = vector<T, S>::LEN;
        static_assert(P >= 0 && S > 0 && P + SA <= N, "Indices is out of bounds");
        static_assert((LEN % SA) == 0 && (P % SA) == 0, "Slicing breaks alignment");
        return *reinterpret_cast<const vector<T, S>*>(s + P);
    }
    template <typename ...Args>
    vector<T, sizeof...(Args)> shuffle(Args ...args) const {
        return vector<T, sizeof...(Args)>(s[args]...);
    }
    template <int ...Ps>
    vector<T, sizeof...(Ps)> shuffle() const {
        return vector<T, sizeof...(Ps)>(s[Ps]...);
    }

    static vector<T, N> load(const T *data, int stride=1) {
        vector<T, N> v;
        for (int i = 0; i < N; ++i) {
            v[i] = data[i*stride];
        }
        return v;
    }
    void store(T *data, int stride=1) const {
        for (int i = 0; i < N; ++i) {
            data[i*stride] = (*this)[i];
        }
    }

    static vector<T, N> vload(const T *p, size_t i=0) {
        load(p + i*N);
    }
    void vstore(T *p, size_t i=0) const {
        store(p + i*N);
    }

    template <typename S=T, typename F, typename ...Args>
    static vector<S, N> map(F f, Args ...args) {
        check_args(args...);
        vector<S, N> r;
        for (int i = 0; i < N; ++i) {
            r[i] = f((args[i])...);
        }
        return r;
    }
    template <typename S>
    vector<S, N> convert() const {
        return ::convert<vector<S, N>>(as_vec());
    }
};

template <typename T, int N>
class vector : public vector_base<T, N, N, alignof(T)> {
private:
    typedef vector_base<T, N, N, alignof(T)> base;
public:
    template <typename ...Args>
    explicit vector(Args ...args) : base(args...) {}

    template <typename F, typename ...Args>
    inline static vector<T, N> map(F f, Args ...args) {
        return base::map(f, args...);
    }

    friend vector operator+(vector a) {
        return a;
    }
    friend vector operator-(vector a) {
        return map([](T x) { return -x; }, a);
    }

    friend vector operator+(vector a, vector b) {
        return map([](T x, T y) { return x + y; }, a, b);
    }
    friend vector operator+(vector a, T b) {
        return map([b](T x) { return x + b; }, a);
    }
    friend vector operator+(T a, vector b) {
        return map([a](T y) { return a + y; }, b);
    }
    friend vector operator-(vector a, vector b) {
        return map([](T x, T y) { return x - y; }, a, b);
    }
    friend vector operator-(vector a, T b) {
        return map([b](T x) { return x - b; }, a);
    }
    friend vector operator-(T a, vector b) {
        return map([a](T y) { return a - y; }, b);
    }

    friend vector operator*(vector a, vector b) {
        return map([](T x, T y) { return x * y; }, a, b);
    }
    friend vector operator*(vector a, T b) {
        return map([b](T x) { return x * b; }, a);
    }
    friend vector operator*(T a, vector b) {
        return map([a](T y) { return a * y; }, b);
    }
    friend vector operator/(vector a, vector b) {
        return map([](T x, T y) { return x / y; }, a, b);
    }
    friend vector operator/(vector a, T b) {
        return map([b](T x) { return x / b; }, a);
    }
    friend vector operator/(T a, vector b) {
        return map([a](T y) { return a / y; }, b);
    }

    vector &operator+=(vector a) {
        return *this = *this + a;
    }
    vector &operator+=(T a) {
        return *this = *this + a;
    }
    vector &operator-=(vector a) {
        return *this = *this - a;
    }
    vector &operator-=(T a) {
        return *this = *this - a;
    }
    vector &operator*=(vector a) {
        return *this = *this * a;
    }
    vector &operator*=(T a) {
        return *this = *this * a;
    }
    vector &operator/=(vector a) {
        return *this = *this / a;
    }
    vector &operator/=(T a) {
        return *this = *this / a;
    }
};

// Remove vector of size 1, scalar is used instead
template <typename T>
class vector<T, 1> {};


template <typename T, int N>
struct Zero<vector<T, N>> {
    static vector<T, N> zero() {
        return vector<T, N>(::zero<T>());
    } 
};

template <typename T, int N>
struct Dim<vector<T, N>> {
    static const int value = N;
};
template <typename T, int N>
struct BaseType<vector<T, N>> {
    typedef base_type<T> type;
};
template <typename T, int N>
struct ElementType<vector<T, N>> {
    typedef T type;
};

template <typename S, typename T, int N>
struct Convert<vector<S, N>, vector<T, N>> {
    static vector<S, N> convert(vector<T, N> v) {
        return vector<T, N>::template map<S>([](T x) {
            return ::convert<S, T>(x);
        }, v);
    }
};

template <typename T, int N>
T dot(vector<T, N> a, vector<T, N> b) {
    T c = zero<T>();
    for (int i = 0; i < N; ++i) {
        c += a[i]*b[i];
    }
    return c;
}

template <typename T>
vector<T, 3> cross(vector<T, 3> a, vector<T, 3> b) {
    return vector<T, 3>(
        a[1]*b[2] - a[2]*b[1],
        a[2]*b[0] - a[0]*b[2],
        a[0]*b[1] - a[1]*b[0]
    );
}
template <typename T>
vector<T, 4> cross(vector<T, 4> a, vector<T, 4> b) {
    return vector<T, 4>(
        a[1]*b[2] - a[2]*b[1],
        a[2]*b[0] - a[0]*b[2],
        a[0]*b[1] - a[1]*b[0],
        zero<T>()
    );
}

template <typename T, int N>
T length2(vector<T, N> a) {
    return dot(a, a);
}

template <typename T, int N>
T length(vector<T, N> a) {
    return math::sqrt(length2(a));
}

template <typename T, int N>
vector<T, N> normalize(vector<T, N> a) {
    return a/length(a);
}

template <typename S, int N>
struct Sequence<vector<S, N>> {
private:
    typedef base_type<S> T;
    typedef vector<S, N> V;
public:
    template <typename F>
    static V map(F f, V a) {
        return V::map([f](S x) {
            return Sequence<S>::map(f, x);
        }, a);
    }
    template <typename F>
    static T reduce(F f, T t, V a) {
        for (int i = 0; i < N; ++i) {
            t = Sequence<S>::reduce(f, t, a[i]);
        }
        return t;
    }
};
