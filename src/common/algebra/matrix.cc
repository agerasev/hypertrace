#include "matrix.hh"

#include "real.hh"


#ifdef UNIT_TEST
#include <catch.hpp>

#include <iostream>

#include <mat.hpp>

template <typename T, int M, int N, typename F>
mat<T,M,N> rand_mat_map(TestRng &rng, F f) {
    mat<T,N,M> r;
    for (int i = 0; i < M*N; ++i) {
        r[i] = f(rng);
    }
    return r;
}

TEST_CASE("Matrix", "[matrix]") {
    TestRng rng(0xbeef);

    SECTION("Transpose") {
        mat<int,2,3> a {1, 2, 3, 4, 5, 6};
        mat<int,3,2> b {1, 4, 2, 5, 3, 6};
        REQUIRE(transpose(a) == b);
    }

    SECTION("Add") {
        mat<int,2,3> a {1, 2, 3, 4, 5, 6}, b {10,20,30,40,50,60};
        mat<int,2,3> c {11,22,33,44,55,66};
        REQUIRE((a + b) == c);
    }

    SECTION("Dot") {
        mat<int,2,3> a {
            1, 2, 3,
            4, 5, 6
        };
        mat<int,3,4> b {
            1, 2, 3, 4,
            5, 6, 7, 8,
            9,10,11,12
        };
        mat<int,2,4> c {
            38, 44, 50, 56,
            83, 98, 113, 128
        };

        REQUIRE(dot(a, b) == c);
    }
};
#endif // UNIT_TEST
