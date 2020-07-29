#pragma once

#include <type_traits>
#include <iostream>


template <typename T, int N>
struct _vector;

template <typename T, int N>
class vector;

template <typename T, int N>
struct _vector {
    T s[N];
};
template <typename T>
struct _vector<T, 1>;
template <typename T>
struct _vector<T, 2> {
    union {
        T s[2];
        struct { T x, y; };
    };
    _vector() = default;
    _vector(T x, T y) : s{x, y} {}
    explicit _vector(T c) : _vector(c, c) {}
    _vector(vector<T, 2> v) : _vector(v.x, v.y) {}
};
template <typename T>
struct _vector<T, 3> {
    union {
        T s[3];
        struct { T x, y, z; };
        struct { vector<T, 2> xy; T __0_z; };
        struct { T __1_x; vector<T, 2> yz; };
    };
    _vector() = default;
    _vector(T x, T y, T z) : s{x, y, z} {}
    explicit _vector(T c) : _vector(c, c, c) {}
    _vector(vector<T, 2> xy, T z) : _vector(xy.x, xy.y, z) {}
    _vector(T x, vector<T, 2> yz) : _vector(x, yz.x, yz.y) {}
    _vector(vector<T, 3> v) : _vector(v.x, v.y, v.z) {}
};
template <typename T>
struct _vector<T, 4> {
    union {
        T s[4];
        struct { T x, y, z, w; };
        struct { vector<T, 2> xy; vector<T, 2> zw; };
        struct { T __0_x; vector<T, 2> yz; T __0_w; };
        struct { vector<T, 3> xyz; T __1_w; };
        struct { T __2_x; vector<T, 3> yzw; };
    };
    _vector() = default;
    _vector(T x, T y, T z, T w) : s{x, y, z, w} {}
    explicit _vector(T c) : _vector(c, c, c, c) {}
    _vector(vector<T, 2> xy, T z, T w) : _vector(xy.x, xy.y, z, w) {}
    _vector(T x, vector<T, 2> yz, T w) : _vector(x, yz.x, yz.y, w) {}
    _vector(T x, T y, vector<T, 2> zw) : _vector(x, y, zw.x, zw.y) {}
    _vector(vector<T, 2> xy, vector<T, 2> zw) : _vector(xy.x, xy.y, zw.x, zw.y) {}
    _vector(vector<T, 3> xyz, T w) : _vector(xyz.x, xyz.y, xyz.z, w) {}
    _vector(T x, vector<T, 3> yzw) : _vector(x, yzw.x, yzw.y, yzw.z) {}
    _vector(vector<T, 4> v) : _vector(v.x, v.y, v.z, v.w) {}
};
template <typename T>
struct _vector<T, 8> {
    union {
        T s[8];
        struct { T s0, s1, s2, s3, s4, s5, s6, s7; };
        struct { vector<T, 2> s01, s23, s45, s67; };
        struct { vector<T, 4> s0123, s4567; };
    };
    _vector() = default;
    _vector(T s0, T s1, T s2, T s3, T s4, T s5, T s6, T s7) :
        s{s0, s1, s2, s3, s4, s5, s6, s7} {}
    explicit _vector(T c) : _vector(c, c, c, c, c, c, c, c) {}
    _vector(vector<T, 2> s01, vector<T, 2> s23, vector<T, 2> s45, vector<T, 2> s67) :
        _vector(s01.x, s01.y, s23.x, s23.y, s45.x, s45.y, s67.x, s67.y) {}
    _vector(vector<T, 4> s0123, vector<T, 4> s4567) :
        _vector(s0123.xy, s0123.zw, s4567.xy, s4567.zw) {}
};
template <typename T>
struct _vector<T, 16> {
    union {
        T s[16];
        struct { T s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, sa, sb, sc, sd, se, sf; };
        struct { vector<T, 2> s01, s23, s45, s67, s89, sab, scd, sef; };
        struct { vector<T, 4> s0123, s4567, s89ab, scdef; };
        struct { vector<T, 8> s01234567, s89abscdef; };
    };
    _vector() = default;
    _vector(T s0, T s1, T s2, T s3, T s4, T s5, T s6, T s7, T s8, T s9, T sa, T sb, T sc, T sd, T se, T sf) :
        s{s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, sa, sb, sc, sd, se, sf} {}
    explicit _vector(T c) : _vector(c, c, c, c, c, c, c, c, c, c, c, c, c, c, c, c) {}
    _vector(vector<T, 2> s01, vector<T, 2> s23, vector<T, 2> s45, vector<T, 2> s67, vector<T, 2> s89, vector<T, 2> sab, vector<T, 2> scd, vector<T, 2> sef) :
        _vector(s01.x, s01.y, s23.x, s23.y, s45.x, s45.y, s67.x, s67.y, s89.x, s89.y, sab.x, sab.y, scd.x, scd.y, sef.x, sef.y) {}
    _vector(vector<T, 4> s0123, vector<T, 4> s4567, vector<T, 4> s89ab, vector<T, 4> scdef) :
        _vector(s0123.xy, s0123.zw, s4567.xy, s4567.zw, s89ab.xy, s89ab.zw, scdef.xy, scdef.zw) {}
    _vector(vector<T, 8> s01234567, vector<T, 8> s89abcdef) :
        _vector(s01234567.s0123, s01234567.s4567, s89abcdef.s0123, s89abcdef.s4567) {}
};

template <typename T, int N>
class vector : public _vector<T, N> {
private:
    // check vector alignment is the same as the element type
    static_assert(std::alignment_of_v<T> == std::alignment_of_v<_vector<T, N>>);

public:
    static const int SIZE = N; 

    vector() = default;
    template <typename ... Args>
    vector(Args ...args) : _vector<T, N>(args...) {}

    T &operator[](int i) {
        return this->s[i];
    }
    const T &operator[](int i) const {
        return this->s[i];
    }

    static vector load(const T *data) {
        vector v;
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
    vector<S, N> cast() const {
        vector<S, N> r;
        for (int i = 0; i < N; ++i) {
            r[i] = (*this)[i];
        }
        return r;
    }

    template <typename F>
    vector<T, N> map(F f) const {
        return vmap(f, *this);
    }

    friend vector<T, N> operator+(vector<T, N> a) {
        return a;
    }
    friend vector<T, N> operator-(vector<T, N> a) {
        return vmap([](T x) { return -x; }, a);
    }

    friend vector<T, N> operator+(vector<T, N> a, vector<T, N> b) {
        return vmap([](T x, T y) { return x + y; }, a, b);
    }
    friend vector<T, N> operator+(vector<T, N> a, T b) {
        return vmap([b](T x) { return x + b; }, a);
    }
    friend vector<T, N> operator+(T a, vector<T, N> b) {
        return vmap([a](T y) { return a + y; }, b);
    }
    friend vector<T, N> operator-(vector<T, N> a, vector<T, N> b) {
        return vmap([](T x, T y) { return x - y; }, a, b);
    }
    friend vector<T, N> operator-(vector<T, N> a, T b) {
        return vmap([b](T x) { return x - b; }, a);
    }
    friend vector<T, N> operator-(T a, vector<T, N> b) {
        return vmap([a](T y) { return a - y; }, b);
    }

    friend vector<T, N> operator*(vector<T, N> a, vector<T, N> b) {
        return vmap([](T x, T y) { return x * y; }, a, b);
    }
    friend vector<T, N> operator*(vector<T, N> a, T b) {
        return vmap([b](T x) { return x * b; }, a);
    }
    friend vector<T, N> operator*(T a, vector<T, N> b) {
        return vmap([a](T y) { return a * y; }, b);
    }
    friend vector<T, N> operator/(vector<T, N> a, vector<T, N> b) {
        return vmap([](T x, T y) { return x / y; }, a, b);
    }
    friend vector<T, N> operator/(vector<T, N> a, T b) {
        return vmap([b](T x) { return x / b; }, a);
    }
    friend vector<T, N> operator/(T a, vector<T, N> b) {
        return vmap([a](T y) { return a / y; }, b);
    }

    vector &operator+=(vector v) {
        return *this = *this + v;
    }
    vector &operator+=(T s) {
        return *this = *this + s;
    }
    vector &operator-=(vector v) {
        return *this = *this - v;
    }
    vector &operator-=(T s) {
        return *this = *this - s;
    }
    vector &operator*=(vector v) {
        return *this = *this * v;
    }
    vector &operator*=(T s) {
        return *this = *this * s;
    }
    vector &operator/=(vector v) {
        return *this = *this / v;
    }
    vector &operator/=(T s) {
        return *this = *this / s;
    }

    friend std::ostream &operator<<(std::ostream &s, vector<T, N> v) {
        s << "(";
        for (int i = 0; i < N - 1; ++i) {
            s << v[i] << ", ";
        }
        s << v[N - 1] << ")";
        return s;
    }
};

template <typename T, int N, typename F>
vector<T, N> vmap(F f, vector<T, N> a) {
    vector<T, N> r;
    for (int i = 0; i < N; ++i) {
        r[i] = f(a[i]);
    }
    return r;
}
template <typename T, int N, typename F>
vector<T, N> vmap(F f, vector<T, N> a, vector<T, N> b) {
    vector<T, N> r;
    for (int i = 0; i < N; ++i) {
        r[i] = f(a[i], b[i]);
    }
    return r;
}
