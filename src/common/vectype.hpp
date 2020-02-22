#pragma once

#include <type_traits>
#include <cmath> 
#include <iostream>


// Structure definition

#define _packed_ //__attribute__((packed))
#define _self (*this)

template <typename T, int N>
struct _vecbase;

template <typename T, int N>
class vectype;

template <typename T, int N>
struct _packed_ _vecbase {
    T s[N];
    static constexpr int size = N;
};
template <typename T>
struct _vecbase<T, 1>;
template <typename T>
struct _packed_ _vecbase<T, 2> {
    union _packed_ {
        T s[2];
        struct { T x, y; };
    };
    _vecbase() {};
    _vecbase(T x, T y) : x(x), y(y) {}
    _vecbase(T c) : _vecbase(c, c) {}
    _vecbase(vectype<T, 2> v) : _vecbase(v.x, v.y) {}
};
template <typename T>
struct _packed_ _vecbase<T, 3> {
    union _packed_ {
        T s[3];
        struct _packed_ { T x, y, z; };
        struct _packed_ { vectype<T, 2> xy; T __0_z; };
        struct _packed_ { T __1_x; vectype<T, 2> yz; };
    };
    _vecbase() {};
    _vecbase(T x, T y, T z) : x(x), y(y), z(z) {}
    _vecbase(T c) : _vecbase(c, c, c) {}
    _vecbase(vectype<T, 2> xy, T z) : _vecbase(xy.x, xy.y, z) {}
    _vecbase(T x, vectype<T, 2> yz) : _vecbase(x, yz.x, yz.y) {}
    _vecbase(vectype<T, 3> v) : _vecbase(v.x, v.y, v.z) {}
};
template <typename T>
struct _packed_ _vecbase<T, 4> {
    union _packed_ {
        T s[4];
        struct _packed_ { T x, y, z, w; };
        struct _packed_ { vectype<T, 2> xy; vectype<T, 2> zw; };
        struct _packed_ { T __0_x; vectype<T, 2> yz; T __0_w; };
        struct _packed_ { vectype<T, 3> xyz; T __1_w; };
        struct _packed_ { T __2_x; vectype<T, 3> yzw; };
    };
    _vecbase() {};
    _vecbase(T x, T y, T z, T w) : x(x), y(y), z(z), w(w) {}
    _vecbase(T c) : _vecbase(c, c, c, c) {}
    _vecbase(vectype<T, 2> xy, T z, T w) : _vecbase(xy.x, xy.y, z, w) {}
    _vecbase(T x, vectype<T, 2> yz, T w) : _vecbase(x, yz.x, yz.y, w) {}
    _vecbase(T x, T y, vectype<T, 2> zw) : _vecbase(x, y, zw.x, zw.y) {}
    _vecbase(vectype<T, 2> xy, vectype<T, 2> zw) : _vecbase(xy.x, xy.y, zw.x, zw.y) {}
    _vecbase(vectype<T, 3> xyz, T w) : _vecbase(xyz.x, xyz.y, xyz.z, w) {}
    _vecbase(T x, vectype<T, 3> yzw) : _vecbase(x, yzw.x, yzw.y, yzw.z) {}
    _vecbase(vectype<T, 4> v) : _vecbase(v.x, v.y, v.z, v.w) {}
};


template <typename T, int N>
class _packed_ vectype : public _vecbase<T, N> {
    public:
    template <typename ... Args>
    vectype(Args ...args) : _vecbase<T, N>(args...) {}

    T &operator[](int i) {
        return this->s[i];
    }
    const T &operator[](int i) const {
        return this->s[i];
    }

    static vectype load(const T *data) {
        vectype v;
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
    vectype<S, N> cast() const {
        vectype<S, N> r;
        for (int i = 0; i < N; ++i) {
            r[i] = (*this)[i];
        }
        return r;
    }

    template <typename F>
    vectype<T, N> map(F f) const {
        return vmap(f, *this);
    }

    friend vectype<T, N> operator+(vectype<T, N> a) {
        return a;
    }
    friend vectype<T, N> operator-(vectype<T, N> a) {
        return vmap([](T x) { return -x; }, a);
    }

    friend vectype<T, N> operator+(vectype<T, N> a, vectype<T, N> b) {
        return vmap([](T x, T y) { return x + y; }, a, b);
    }
    friend vectype<T, N> operator+(vectype<T, N> a, T b) {
        return vmap([b](T x) { return x + b; }, a);
    }
    friend vectype<T, N> operator+(T a, vectype<T, N> b) {
        return vmap([a](T y) { return a + y; }, b);
    }
    friend vectype<T, N> operator-(vectype<T, N> a, vectype<T, N> b) {
        return vmap([](T x, T y) { return x - y; }, a, b);
    }
    friend vectype<T, N> operator-(vectype<T, N> a, T b) {
        return vmap([b](T x) { return x - b; }, a);
    }
    friend vectype<T, N> operator-(T a, vectype<T, N> b) {
        return vmap([a](T y) { return a - y; }, b);
    }

    friend vectype<T, N> operator*(vectype<T, N> a, vectype<T, N> b) {
        return vmap([](T x, T y) { return x * y; }, a, b);
    }
    friend vectype<T, N> operator*(vectype<T, N> a, T b) {
        return vmap([b](T x) { return x * b; }, a);
    }
    friend vectype<T, N> operator*(T a, vectype<T, N> b) {
        return vmap([a](T y) { return a * y; }, b);
    }
    friend vectype<T, N> operator/(vectype<T, N> a, vectype<T, N> b) {
        return vmap([](T x, T y) { return x / y; }, a, b);
    }
    friend vectype<T, N> operator/(vectype<T, N> a, T b) {
        return vmap([b](T x) { return x / b; }, a);
    }
    friend vectype<T, N> operator/(T a, vectype<T, N> b) {
        return vmap([a](T y) { return a / y; }, b);
    }

    vectype &operator+=(vectype v) {
        return *this = *this + v;
    }
    vectype &operator+=(T s) {
        return *this = *this + s;
    }
    vectype &operator-=(vectype v) {
        return *this = *this - v;
    }
    vectype &operator-=(T s) {
        return *this = *this - s;
    }
    vectype &operator*=(vectype v) {
        return *this = *this * v;
    }
    vectype &operator*=(T s) {
        return *this = *this * s;
    }
    vectype &operator/=(vectype v) {
        return *this = *this / v;
    }
    vectype &operator/=(T s) {
        return *this = *this / s;
    }

    friend std::ostream &operator<<(std::ostream &s, vectype<T, N> v) {
        s << "(";
        for (int i = 0; i < N - 1; ++i) {
            s << v[i] << ", ";
        }
        s << v[N - 1] << ")";
        return s;
    }
};

template <typename T, int N, typename F>
vectype<T, N> vmap(F f, vectype<T, N> a) {
    vectype<T, N> r;
    for (int i = 0; i < N; ++i) {
        r[i] = f(a[i]);
    }
    return r;
}
template <typename T, int N, typename F>
vectype<T, N> vmap(F f, vectype<T, N> a, vectype<T, N> b) {
    vectype<T, N> r;
    for (int i = 0; i < N; ++i) {
        r[i] = f(a[i], b[i]);
    }
    return r;
}


template <typename T, int N>
T dot(vectype<T, N> a, vectype<T, N> b) {
    T c = (T)0;
    for (int i = 0; i < N; ++i) {
        c += a[i]*b[i];
    }
    return c;
}
template <typename T>
vectype<T, 3> cross(vectype<T, 3> a, vectype<T, 3> b) {
    return vectype<T, 3>(
        a.y*b.z - a.z*b.y,
        a.z*b.x - a.x*b.z,
        a.x*b.y - a.y*b.x
    );
}
template <typename T>
vectype<T, 4> cross(vectype<T, 4> a, vectype<T, 4> b) {
    return vectype<T, 4>(cross(a.xyz, b.xyz), (T)0);
}

template <typename T, int N>
T length(vectype<T, N> a) {
    return sqrt(dot(a, a));
}

template <typename T, int N>
vectype<T, N> normalize(vectype<T, N> a) {
    return a/length(a);
}

double fract(double x, double *intpart);

template <typename T, int N>
vectype<T, N> fract(vectype<T, N> x, vectype<T, N> *intpart) {
    vectype<T, N> r;
    for (int i = 0; i < N; ++i) {
        r[i] = fract(x[i], intpart->s + i);
    }
    return r;
}

template <typename T, int N>
vectype<T, N> floor(vectype<T, N> v) {
    return v.map([](T x){ return floor(x); });
}

template <typename T, int N>
vectype<T, N> ceil(vectype<T, N> v) {
    return v.map([](T x){ return ceil(x); });
}


#ifdef UNIT_TEST
#include <catch.hpp>

template <typename T, int N>
class VecApprox {
    public:
    typedef vectype<T, N> vtype;
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
VecApprox<T, N> ApproxV(vectype<T, N> v) {
    return VecApprox<T, N>(v);
}
#endif // UNIT_TEST
