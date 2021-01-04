#pragma once

// Concatenate `A` and `B` without macro expansion.
#define _CONCAT_2(A, B) A##B

// Expand `A` and `B` and concatenate the results.
#define CONCAT_2(A, B) _CONCAT_2(A, B)

#define CONCAT_1(A) A

#define EVAL(...) __VA_ARGS__
#define _VA_COUNT(_8,_7,_6,_5,_4,_3,_2,_1,_N,...) _N
#define VA_COUNT(...) _VA_COUNT(__VA_ARGS__,8,7,6,5,4,3,2,1)

#define CONCAT_3(X, ...) CONCAT_2(X, CONCAT_2(__VA_ARGS__))
#define CONCAT_4(X, ...) CONCAT_2(X, CONCAT_3(__VA_ARGS__))
#define CONCAT_5(X, ...) CONCAT_2(X, CONCAT_4(__VA_ARGS__))
#define CONCAT_6(X, ...) CONCAT_2(X, CONCAT_5(__VA_ARGS__))
#define CONCAT_7(X, ...) CONCAT_2(X, CONCAT_6(__VA_ARGS__))
#define CONCAT_8(X, ...) CONCAT_2(X, CONCAT_7(__VA_ARGS__))

#define CONCAT(...) CONCAT_2(CONCAT_, VA_COUNT(__VA_ARGS__))(__VA_ARGS__)

#define $(...) CONCAT(__VA_ARGS__)

#ifndef HOST
#include "macros.cc"
#endif // HOST
