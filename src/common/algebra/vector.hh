#pragma once

#define _packed_ __attribute__((packed))


template <typename T, int N>
struct _packed_ _typeN {
    T s[N];

    template <typename F>
    static _typeN map(F f, _typeN a);
    template <typename F>
    static _typeN map(F f, _typeN a, _typeN b);
};

template <typename T>
struct _packed_ _typeN<T, 2> {
    union _packed_ {
        T s[2];
        struct { T x, y; };
    };

    template <typename F>
    static _typeN map(F f, _typeN a);
    template <typename F>
    static _typeN map(F f, _typeN a, _typeN b);
};

template <typename T>
struct _packed_ _typeN<T, 3> {
    union _packed_ {
        T s[3];
        struct _packed_ { T x, y, z; };
        struct _packed_ { _typeN<T, 2> xy; T __0_z; };
        struct _packed_ { T __1_x; _typeN<T, 2> yz; };
    };

    template <typename F>
    static _typeN map(F f, _typeN a);
    template <typename F>
    static _typeN map(F f, _typeN a, _typeN b);
};

template <typename T>
struct _packed_ _typeN<T, 4> {
    union _packed_ {
        T s[4];
        struct _packed_ { T x, y, z, w; };
        struct _packed_ { _typeN<T, 2> xy; _typeN<T, 2> zw; };
        struct _packed_ { T __0_x; _typeN<T, 2> yz; T __0_w; };
        struct _packed_ { _typeN<T, 3> xyz; T __1_w; };
        struct _packed_ { T __2_x; _typeN<T, 3> yzw; };
    };
    
    template <typename F>
    static _typeN map(F f, _typeN a);
    template <typename F>
    static _typeN map(F f, _typeN a, _typeN b);
};


template <typename T, int N>
template <typename F>
_typeN<T, N> _typeN<T, N>::map(F f, _typeN<T, N> a) {
    _typeN<T, N> r;
    for (int i = 0; i < N; ++i) {
        r.s[i] = f(a.s[i]);
    }
    return r;
}
template <typename T, int N>
template <typename F>
_typeN<T, N> _typeN<T, N>::map(F f, _typeN<T, N> a, _typeN<T, N> b) {
    _typeN r;
    for (int i = 0; i < N; ++i) {
        r.s[i] = f(a.s[i], b.s[i]);
    }
    return r;
}

#ifdef UNIT_TEST
#include <catch.hpp>

TEST_CASE("Vector types", "[vector]") {
    SECTION("Field alignment") {
        _typeN<int, 2> a2;
        a2.s[0] = 123456;
        REQUIRE(a2.x == 123456);
        a2.y = 654321;
        REQUIRE(a2.s[1] == 654321);

        _typeN<int, 4> a4;
        a4.yz = {1, 2};
        REQUIRE(a4.s[1] == 1);
        REQUIRE(a4.s[2] == 2);
    }
};

#endif // UNIT_TEST
