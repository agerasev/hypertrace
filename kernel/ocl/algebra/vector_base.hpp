#pragma once


template <typename T, int N>
struct _vec;

template <typename T, int N>
class vec;

template <typename T, int N>
struct _vec {
    T s[N];
};
template <typename T>
struct _vec<T, 1>;
template <typename T>
struct _vec<T, 2> {
    union {
        T s[2];
        struct { T x, y; };
    };
    _vec() = default;
    _vec(T x, T y) : s{x, y} {}
    explicit _vec(T c) : _vec(c, c) {}
    _vec(vec<T, 2> v) : _vec(v.x, v.y) {}
};
template <typename T>
struct _vec<T, 3> {
    union {
        T s[3];
        struct { T x, y, z; };
        struct { vec<T, 2> xy; T __0_z; };
        struct { T __1_x; vec<T, 2> yz; };
    };
    _vec() = default;
    _vec(T x, T y, T z) : s{x, y, z} {}
    explicit _vec(T c) : _vec(c, c, c) {}
    _vec(vec<T, 2> xy, T z) : _vec(xy.x, xy.y, z) {}
    _vec(T x, vec<T, 2> yz) : _vec(x, yz.x, yz.y) {}
    _vec(vec<T, 3> v) : _vec(v.x, v.y, v.z) {}
};
template <typename T>
struct _vec<T, 4> {
    union {
        T s[4];
        struct { T x, y, z, w; };
        struct { vec<T, 2> xy; vec<T, 2> zw; };
        struct { T __0_x; vec<T, 2> yz; T __0_w; };
        struct { vec<T, 3> xyz; T __1_w; };
        struct { T __2_x; vec<T, 3> yzw; };
    };
    _vec() = default;
    _vec(T x, T y, T z, T w) : s{x, y, z, w} {}
    explicit _vec(T c) : _vec(c, c, c, c) {}
    _vec(vec<T, 2> xy, T z, T w) : _vec(xy.x, xy.y, z, w) {}
    _vec(T x, vec<T, 2> yz, T w) : _vec(x, yz.x, yz.y, w) {}
    _vec(T x, T y, vec<T, 2> zw) : _vec(x, y, zw.x, zw.y) {}
    _vec(vec<T, 2> xy, vec<T, 2> zw) : _vec(xy.x, xy.y, zw.x, zw.y) {}
    _vec(vec<T, 3> xyz, T w) : _vec(xyz.x, xyz.y, xyz.z, w) {}
    _vec(T x, vec<T, 3> yzw) : _vec(x, yzw.x, yzw.y, yzw.z) {}
    _vec(vec<T, 4> v) : _vec(v.x, v.y, v.z, v.w) {}
};
template <typename T>
struct _vec<T, 8> {
    union {
        T s[8];
        struct { T s0, s1, s2, s3, s4, s5, s6, s7; };
        struct { vec<T, 2> s01, s23, s45, s67; };
        struct { vec<T, 4> s0123, s4567; };
    };
    _vec() = default;
    _vec(T s0, T s1, T s2, T s3, T s4, T s5, T s6, T s7) :
        s{s0, s1, s2, s3, s4, s5, s6, s7} {}
    explicit _vec(T c) : _vec(c, c, c, c, c, c, c, c) {}
    _vec(vec<T, 2> s01, vec<T, 2> s23, vec<T, 2> s45, vec<T, 2> s67) :
        _vec(s01.x, s01.y, s23.x, s23.y, s45.x, s45.y, s67.x, s67.y) {}
    _vec(vec<T, 4> s0123, vec<T, 4> s4567) :
        _vec(s0123.xy, s0123.zw, s4567.xy, s4567.zw) {}
};
template <typename T>
struct _vec<T, 16> {
    union {
        T s[16];
        struct { T s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, sa, sb, sc, sd, se, sf; };
        struct { vec<T, 2> s01, s23, s45, s67, s89, sab, scd, sef; };
        struct { vec<T, 4> s0123, s4567, s89ab, scdef; };
        struct { vec<T, 8> s01234567, s89abcdef; };
    };
    _vec() = default;
    _vec(T s0, T s1, T s2, T s3, T s4, T s5, T s6, T s7, T s8, T s9, T sa, T sb, T sc, T sd, T se, T sf) :
        s{s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, sa, sb, sc, sd, se, sf} {}
    explicit _vec(T c) : _vec(c, c, c, c, c, c, c, c, c, c, c, c, c, c, c, c) {}
    _vec(vec<T, 2> s01, vec<T, 2> s23, vec<T, 2> s45, vec<T, 2> s67, vec<T, 2> s89, vec<T, 2> sab, vec<T, 2> scd, vec<T, 2> sef) :
        _vec(s01.x, s01.y, s23.x, s23.y, s45.x, s45.y, s67.x, s67.y, s89.x, s89.y, sab.x, sab.y, scd.x, scd.y, sef.x, sef.y) {}
    _vec(vec<T, 4> s0123, vec<T, 4> s4567, vec<T, 4> s89ab, vec<T, 4> scdef) :
        _vec(s0123.xy, s0123.zw, s4567.xy, s4567.zw, s89ab.xy, s89ab.zw, scdef.xy, scdef.zw) {}
    _vec(vec<T, 8> s01234567, vec<T, 8> s89abcdef) :
        _vec(s01234567.s0123, s01234567.s4567, s89abcdef.s0123, s89abcdef.s4567) {}
};
