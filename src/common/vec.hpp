#pragma once

#include <type_traits>
#include <cmath> 
#include <iostream>
#include <algorithm>


// Structure definition

#define _packed_ //__attribute__((packed))
#define _self (*this)

template <typename T, int N>
struct _vec;

template <typename T, int N>
class vec;

template <typename T, int N>
struct _packed_ _vec {
    T s[N];
    static constexpr int size = N;
};
template <typename T>
struct _vec<T, 1>;
template <typename T>
struct _packed_ _vec<T, 2> {
    union _packed_ {
        T s[2];
        struct { T x, y; };
    };
    _vec() = default;
    _vec(T x, T y) : x(x), y(y) {}
    _vec(T c) : _vec(c, c) {}
    _vec(vec<T, 2> v) : _vec(v.x, v.y) {}
};
template <typename T>
struct _packed_ _vec<T, 3> {
    union _packed_ {
        T s[3];
        struct _packed_ { T x, y, z; };
        struct _packed_ { vec<T, 2> xy; T __0_z; };
        struct _packed_ { T __1_x; vec<T, 2> yz; };
    };
    _vec() = default;
    _vec(T x, T y, T z) : x(x), y(y), z(z) {}
    _vec(T c) : _vec(c, c, c) {}
    _vec(vec<T, 2> xy, T z) : _vec(xy.x, xy.y, z) {}
    _vec(T x, vec<T, 2> yz) : _vec(x, yz.x, yz.y) {}
    _vec(vec<T, 3> v) : _vec(v.x, v.y, v.z) {}
};
template <typename T>
struct _packed_ _vec<T, 4> {
    union _packed_ {
        T s[4];
        struct _packed_ { T x, y, z, w; };
        struct _packed_ { vec<T, 2> xy; vec<T, 2> zw; };
        struct _packed_ { T __0_x; vec<T, 2> yz; T __0_w; };
        struct _packed_ { vec<T, 3> xyz; T __1_w; };
        struct _packed_ { T __2_x; vec<T, 3> yzw; };
    };
    _vec() = default;
    _vec(T x, T y, T z, T w) : x(x), y(y), z(z), w(w) {}
    _vec(T c) : _vec(c, c, c, c) {}
    _vec(vec<T, 2> xy, T z, T w) : _vec(xy.x, xy.y, z, w) {}
    _vec(T x, vec<T, 2> yz, T w) : _vec(x, yz.x, yz.y, w) {}
    _vec(T x, T y, vec<T, 2> zw) : _vec(x, y, zw.x, zw.y) {}
    _vec(vec<T, 2> xy, vec<T, 2> zw) : _vec(xy.x, xy.y, zw.x, zw.y) {}
    _vec(vec<T, 3> xyz, T w) : _vec(xyz.x, xyz.y, xyz.z, w) {}
    _vec(T x, vec<T, 3> yzw) : _vec(x, yzw.x, yzw.y, yzw.z) {}
    _vec(vec<T, 4> v) : _vec(v.x, v.y, v.z, v.w) {}
};


template <typename T, int N>
class _packed_ vec : public _vec<T, N> {
    public:
    vec() = default;
    template <typename ... Args>
    vec(Args ...args) : _vec<T, N>(args...) {}

    T &operator[](int i) {
        return this->s[i];
    }
    const T &operator[](int i) const {
        return this->s[i];
    }

    static vec load(const T *data) {
        vec v;
        for (int i = 0; i < N; ++i) {
            v[i] = data[i];
        }
        return v;
    }
    void store(T *data) const {
        for (int i = 0; i < N; ++i) {
            data[i] = (*this)[i];
        }
    }

    template <typename S>
    vec<S, N> cast() const {
        vec<S, N> r;
        for (int i = 0; i < N; ++i) {
            r[i] = (*this)[i];
        }
        return r;
    }

    template <typename F>
    vec<T, N> map(F f) const {
        return vmap(f, *this);
    }

    friend vec<T, N> operator+(vec<T, N> a) {
        return a;
    }
    friend vec<T, N> operator-(vec<T, N> a) {
        return vmap([](T x) { return -x; }, a);
    }

    friend vec<T, N> operator+(vec<T, N> a, vec<T, N> b) {
        return vmap([](T x, T y) { return x + y; }, a, b);
    }
    friend vec<T, N> operator+(vec<T, N> a, T b) {
        return vmap([b](T x) { return x + b; }, a);
    }
    friend vec<T, N> operator+(T a, vec<T, N> b) {
        return vmap([a](T y) { return a + y; }, b);
    }
    friend vec<T, N> operator-(vec<T, N> a, vec<T, N> b) {
        return vmap([](T x, T y) { return x - y; }, a, b);
    }
    friend vec<T, N> operator-(vec<T, N> a, T b) {
        return vmap([b](T x) { return x - b; }, a);
    }
    friend vec<T, N> operator-(T a, vec<T, N> b) {
        return vmap([a](T y) { return a - y; }, b);
    }

    friend vec<T, N> operator*(vec<T, N> a, vec<T, N> b) {
        return vmap([](T x, T y) { return x * y; }, a, b);
    }
    friend vec<T, N> operator*(vec<T, N> a, T b) {
        return vmap([b](T x) { return x * b; }, a);
    }
    friend vec<T, N> operator*(T a, vec<T, N> b) {
        return vmap([a](T y) { return a * y; }, b);
    }
    friend vec<T, N> operator/(vec<T, N> a, vec<T, N> b) {
        return vmap([](T x, T y) { return x / y; }, a, b);
    }
    friend vec<T, N> operator/(vec<T, N> a, T b) {
        return vmap([b](T x) { return x / b; }, a);
    }
    friend vec<T, N> operator/(T a, vec<T, N> b) {
        return vmap([a](T y) { return a / y; }, b);
    }

    vec &operator+=(vec v) {
        return *this = *this + v;
    }
    vec &operator+=(T s) {
        return *this = *this + s;
    }
    vec &operator-=(vec v) {
        return *this = *this - v;
    }
    vec &operator-=(T s) {
        return *this = *this - s;
    }
    vec &operator*=(vec v) {
        return *this = *this * v;
    }
    vec &operator*=(T s) {
        return *this = *this * s;
    }
    vec &operator/=(vec v) {
        return *this = *this / v;
    }
    vec &operator/=(T s) {
        return *this = *this / s;
    }

    friend std::ostream &operator<<(std::ostream &s, vec<T, N> v) {
        s << "(";
        for (int i = 0; i < N - 1; ++i) {
            s << v[i] << ", ";
        }
        s << v[N - 1] << ")";
        return s;
    }
};

template <typename T, int N, typename F>
vec<T, N> vmap(F f, vec<T, N> a) {
    vec<T, N> r;
    for (int i = 0; i < N; ++i) {
        r[i] = f(a[i]);
    }
    return r;
}
template <typename T, int N, typename F>
vec<T, N> vmap(F f, vec<T, N> a, vec<T, N> b) {
    vec<T, N> r;
    for (int i = 0; i < N; ++i) {
        r[i] = f(a[i], b[i]);
    }
    return r;
}


template <typename T, int N>
T dot(vec<T, N> a, vec<T, N> b) {
    T c = (T)0;
    for (int i = 0; i < N; ++i) {
        c += a[i]*b[i];
    }
    return c;
}
template <typename T>
vec<T, 3> cross(vec<T, 3> a, vec<T, 3> b) {
    return vec<T, 3>(
        a.y*b.z - a.z*b.y,
        a.z*b.x - a.x*b.z,
        a.x*b.y - a.y*b.x
    );
}
template <typename T>
vec<T, 4> cross(vec<T, 4> a, vec<T, 4> b) {
    return vec<T, 4>(cross(a.xyz, b.xyz), (T)0);
}

template <typename T, int N>
T length(vec<T, N> a) {
    return sqrt(dot(a, a));
}

template <typename T, int N>
vec<T, N> normalize(vec<T, N> a) {
    return a/length(a);
}

inline double fract(double x, double *intpart) {
    return modf(x, intpart);
}
template <typename T, int N>
vec<T, N> fract(vec<T, N> x, vec<T, N> *intpart) {
    vec<T, N> r;
    for (int i = 0; i < N; ++i) {
        r[i] = fract(x[i], intpart->s + i);
    }
    return r;
}

template <typename T, int N>
vec<T, N> floor(vec<T, N> v) {
    return v.map([](T x){ return floor(x); });
}
template <typename T, int N>
vec<T, N> ceil(vec<T, N> v) {
    return v.map([](T x){ return ceil(x); });
}

template <typename T, int N>
vec<T, N> fabs(vec<T, N> v) {
    return v.map([](T x){ return fabs(x); });
}

template <typename T>
T min(T x, T y) {
    return std::min(x, y);
}
template <typename T, int N>
T min(vec<T, N> x, T y) {
    return vmap([y](T a) { return min(a, y); }, x);
}
template <typename T, int N>
T min(vec<T, N> x, vec<T, N> y) {
    return vmap([](T a, T b) { return min(a, b); }, x, y);
}

template <typename T>
T max(T x, T y) {
    return std::max(x, y);
}
template <typename T, int N>
T max(vec<T, N> x, T y) {
    return vmap([y](T a) { return max(a, y); }, x);
}
template <typename T, int N>
T max(vec<T, N> x, vec<T, N> y) {
    return vmap([](T a, T b) { return max(a, b); }, x, y);
}

template <typename T>
T clamp(T x, T y, T z) {
    return std::min(std::max(x, y), z);
}
template <typename T, int N>
T max(vec<T, N> x, T y, T z) {
    return min(max(x, y), z);
}
template <typename T, int N>
T max(vec<T, N> x, vec<T, N> y, vec<T, N> z) {
    return min(max(x, y), z);
}


#ifdef UNIT_TEST
#include <catch.hpp>

template <typename T, int N>
class VecApprox {
    public:
    typedef vec<T, N> vtype;
    vtype v;
    VecApprox(vtype c) : v(c) {}
    friend bool operator==(vtype a, VecApprox b) {
        for (int i = 0; i < N; ++i) {
            if (a[i] != Approx(b.v[i])) {
                return false;
            }
        }
        return true;
    }
    friend bool operator==(VecApprox a, vtype b){
        return b == a;
    }
    friend std::ostream &operator<<(std::ostream &s, VecApprox a) {
        return s << a.v;
    }
};
template <typename T, int N>
VecApprox<T, N> ApproxV(vec<T, N> v) {
    return VecApprox<T, N>(v);
}
#endif // UNIT_TEST
