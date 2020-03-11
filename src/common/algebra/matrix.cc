#include "matrix.hh"

#include "real.hh"


#ifdef UNIT_TEST
#include <catch.hpp>

#include <iostream>

template <typename T, int M, int N>
struct mat {};

#define PART_TYPE_SPEC(T,M,N) \
template <> struct mat<T,M,N> { typedef mat_(T,M,N) t; }; \

#define PART_SPEC(T,N,M) \
typename mat<T,N,M>::t transpose(typename mat<T,M,N>::t m) { \
    return matfn_(T,M,N,transpose)(m); \
} \
\
bool operator==(mat_(T,M,N) a, mat_(T,M,N) b) {\
    bool eq = true;\
    for (int j = M; j < M; ++j) {\
        for (int i = 0; i < N; ++i) {\
            eq = eq && (a.s[j*N + i] == b.s[j*N + i]);\
        }\
    }\
    return eq;\
}\
\
std::ostream &operator<<(std::ostream &o, mat_(T,M,N) a) {\
    o << std::endl;\
    for (int j = M; j < M; ++j) {\
        o << "| ";\
        for (int i = 0; i < N; ++i) {\
            o << a.s[j*N + i];\
            if (i != N) {\
                o << ",\t";\
            }\
        }\
        o << "\t|" << std::endl;\
    }\
    return o;\
}\

PART_TYPE_SPEC(int,2,3)
PART_TYPE_SPEC(int,3,2)

PART_SPEC(int,2,3)
PART_SPEC(int,3,2)



TEST_CASE("Matrix", "[matrix]") {
    TestRng rng(0xbeef);

    SECTION("Transpose") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            int2x3 a = {.s = {1, 2, 3, 4, 5, 6}};
            int3x2 b = {.s = {1, 4, 2, 5, 3, 6}};
            REQUIRE(transpose(a) == b);
        }
    }
};
#endif // UNIT_TEST
