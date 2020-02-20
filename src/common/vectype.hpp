#pragma once

#include <type_traits> 
#include <iostream>


// Structure definition

#define _VECTYPE_COMMON_DEFINITIONS \
    vectype() = default; \
    \
    static vectype load(const T *data) { return vload(data); } \
    void store(T *data) const { vstore(data, *this); } \
    \
    template <typename F> \
    static vectype map(F f, vectype a) { return vmap(f, a); } \
    template <typename F> \
    static vectype map(F f, vectype a, vectype b) { return vmap(f, a, b); } \
    \
    T &operator[](size_t i) { return s[i]; } \
    const T &operator[](size_t i) const { return s[i]; } \
    \
    vectype &operator+=(vectype v) { return *this = *this + v; } \
    vectype &operator+=(T s) { return *this = *this + s; } \
    vectype &operator-=(vectype v) { return *this = *this - v; } \
    vectype &operator-=(T s) { return *this = *this - s; } \
    vectype &operator*=(vectype v) { return *this = *this * v; } \
    vectype &operator*=(T s) { return *this = *this * s; } \
    vectype &operator/=(vectype v) { return *this = *this / v; } \
    vectype &operator/=(T s) { return *this = *this / s; } \

#define _packed_ __attribute__((packed))

template <typename T, int N>
struct _packed_ vectype {
    T s[N];
    static constexpr int size = N;

    _VECTYPE_COMMON_DEFINITIONS
};
template <typename T>
struct _packed_ vectype<T, 2> {
    union _packed_ {
        T s[2];
        struct { T x, y; };
    };

    _VECTYPE_COMMON_DEFINITIONS
};
template <typename T>
struct _packed_ vectype<T, 3> {
    union _packed_ {
        T s[3];
        struct _packed_ { T x, y, z; };
        struct _packed_ { vectype<T, 2> xy; T __0_z; };
        struct _packed_ { T __1_x; vectype<T, 2> yz; };
    };

    _VECTYPE_COMMON_DEFINITIONS
};
template <typename T>
struct _packed_ vectype<T, 4> {
    union _packed_ {
        T s[4];
        struct _packed_ { T x, y, z, w; };
        struct _packed_ { vectype<T, 2> xy; vectype<T, 2> zw; };
        struct _packed_ { T __0_x; vectype<T, 2> yz; T __0_w; };
        struct _packed_ { vectype<T, 3> xyz; T __1_w; };
        struct _packed_ { T __2_x; vectype<T, 3> yzw; };
    };
    
    _VECTYPE_COMMON_DEFINITIONS
};


// Load from and store to the memory

template <typename T, int N>
vectype<T, N> vload(const T *data) {
    vectype<T, N> v;
    for (int i = 0; i < N; ++i) {
        v.s[i] = data[i];
    }
    return v;
}
template <typename T, int N>
void vstore(T *data, vectype<T, N> v) {
    for (int i = 0; i < N; ++i) {
        data[i] = v.s[i];
    }
}


// Construction of the new vector

template <typename ... Args>
struct _VecArgs {};
template <typename T, typename ... Args>
struct _VecArgs<T, Args ...> {
    typedef _VecArgs<Args ...> next;
    static constexpr int count = next::count + 1;
    typedef typename std::enable_if<std::is_same<T, typename next::type>::value, T>::type type;
    static void store(T *data, T s, Args ...args) {
        *data = s;
        next::store(data + 1, args...);
    }
};
template <typename T>
struct _VecArgs<T> {
    static constexpr int count = 1;
    typedef T type;
    static void store(T *data, T s) {
        *data = s;
    }
};
template <typename T, int N, typename ... Args>
struct _VecArgs<vectype<T, N>, Args ...> {
    typedef _VecArgs<Args ...> next;
    static constexpr int count = next::count + N;
    typedef typename std::enable_if<std::is_same<T, typename next::type>::value, T>::type type;
    static void store(T *data, vectype<T, N> v, Args ...args) {
        v.store(data);
        next::store(data + N, args...);
    }
};
template <typename T, int N>
struct _VecArgs<vectype<T, N>> {
    static constexpr int count = N;
    typedef T type;
    static void store(T *data, vectype<T, N> v) {
        v.store(data);
    }
};

template <typename ... Args>
vectype<typename _VecArgs<Args...>::type, _VecArgs<Args...>::count> vnew(Args ... args) {
    vectype<typename _VecArgs<Args...>::type, _VecArgs<Args...>::count> v;
    _VecArgs<Args...>::store(v.s, args...);
    return v;
}


// Operators

template <typename T, int N, typename F>
vectype<T, N> vmap(F f, vectype<T, N> a) {
    vectype<T, N> r;
    for (int i = 0; i < N; ++i) {
        r.s[i] = f(a.s[i]);
    }
    return r;
}
template <typename T, int N, typename F>
vectype<T, N> vmap(F f, vectype<T, N> a, vectype<T, N> b) {
    vectype<T, N> r;
    for (int i = 0; i < N; ++i) {
        r.s[i] = f(a.s[i], b.s[i]);
    }
    return r;
}

template <typename T, int N>
vectype<T, N> operator+(vectype<T, N> a) {
    return a;
}
template <typename T, int N>
vectype<T, N> operator-(vectype<T, N> a) {
    return vmap([](T x) { return -x; }, a);
}

template <typename T, int N>
vectype<T, N> operator+(vectype<T, N> a, vectype<T, N> b) {
    return vmap([](T x, T y) { return x + y; }, a, b);
}
template <typename T, int N>
vectype<T, N> operator+(vectype<T, N> a, T b) {
    return vmap([b](T x) { return x + b; }, a);
}
template <typename T, int N>
vectype<T, N> operator+(T a, vectype<T, N> b) {
    return vmap([a](T y) { return a + y; }, b);
}
template <typename T, int N>
vectype<T, N> operator-(vectype<T, N> a, vectype<T, N> b) {
    return vmap([](T x, T y) { return x - y; }, a, b);
}
template <typename T, int N>
vectype<T, N> operator-(vectype<T, N> a, T b) {
    return vmap([b](T x) { return x - b; }, a);
}
template <typename T, int N>
vectype<T, N> operator-(T a, vectype<T, N> b) {
    return vmap([a](T y) { return a - y; }, b);
}

template <typename T, int N>
vectype<T, N> operator*(vectype<T, N> a, vectype<T, N> b) {
    return vmap([](T x, T y) { return x * y; }, a, b);
}
template <typename T, int N>
vectype<T, N> operator*(vectype<T, N> a, T b) {
    return vmap([b](T x) { return x * b; }, a);
}
template <typename T, int N>
vectype<T, N> operator*(T a, vectype<T, N> b) {
    return vmap([a](T y) { return a * y; }, b);
}
template <typename T, int N>
vectype<T, N> operator/(vectype<T, N> a, vectype<T, N> b) {
    return vmap([](T x, T y) { return x / y; }, a, b);
}
template <typename T, int N>
vectype<T, N> operator/(vectype<T, N> a, T b) {
    return vmap([b](T x) { return x / b; }, a);
}
template <typename T, int N>
vectype<T, N> operator/(T a, vectype<T, N> b) {
    return vmap([a](T y) { return a / y; }, b);
}

template <typename T, int N>
std::ostream &operator<<(std::ostream &s, vectype<T, N> v) {
    s << "(";
    for (int i = 0; i < N - 1; ++i) {
        s << v[i] << ", ";
    }
    s << v[N - 1] << ")";
}


#ifdef UNIT_TEST
#include <catch.hpp>

TEST_CASE("Vector types", "[vectypes.hh]") {
    SECTION("Field alignment") {
        vectype<int, 2> a2;
        a2[0] = 123456;
        REQUIRE(a2.x == 123456);
        a2.y = 654321;
        REQUIRE(a2[1] == 654321);

        vectype<int, 4> a4;
        a4.yz = vnew(1, 2);
        REQUIRE(a4[1] == 1);
        REQUIRE(a4[2] == 2);
    }

    SECTION("Contruction") {
        auto v = vnew(vnew(0, 1), 2, vnew(3, 4, 5), vnew(6, vnew(7, 8), 9));
        for (int i = 0; i < 10; ++i) {
            REQUIRE(v[i] == i);
        }
    }
};

#endif // UNIT_TEST
