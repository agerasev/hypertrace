#include <mat.hpp>

#ifdef UNIT_TEST
#include <catch.hpp>

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

    SECTION("Determinant") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            mat<real,2,2> a = rand_mat_map<real,2,2>(
                rng, [](TestRng &rng) { return rng.normal(); }
            );
            REQUIRE(det(a) == Approx(a[0]*a[3] - a[1]*a[2]));
        }
    }

    SECTION("Inverse") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            mat<real,2,2> a = rand_mat_map<real,2,2>(
                rng, [](TestRng &rng) { return rng.normal(); }
            );
            mat<real,2,2> b = inverse(a);
            MatApprox<real,2,2,Approx> o = one<real,2>();
            REQUIRE(dot(a, b) == o);
        }
    }
};

#endif // UNIT_TEST
