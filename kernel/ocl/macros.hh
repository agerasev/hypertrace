#pragma once

#define _CONCAT_1(A)               A
#define _CONCAT_2(A,B)             A##B
#define _CONCAT_3(A,B,C)           A##B##C
#define _CONCAT_4(A,B,C,D)         A##B##C##D
#define _CONCAT_5(A,B,C,D,E)       A##B##C##D##E
#define _CONCAT_6(A,B,C,D,E,F)     A##B##C##D##E##F
#define _CONCAT_7(A,B,C,D,E,F,G)   A##B##C##D##E##F##G
#define _CONCAT_8(A,B,C,D,E,F,G,H) A##B##C##D##E##F##G##H

#define CONCAT_1(A)               _CONCAT_1(A)
#define CONCAT_2(A,B)             _CONCAT_2(A,B)
#define CONCAT_3(A,B,C)           _CONCAT_3(A,B,C)
#define CONCAT_4(A,B,C,D)         _CONCAT_4(A,B,C,D)
#define CONCAT_5(A,B,C,D,E)       _CONCAT_5(A,B,C,D,E)
#define CONCAT_6(A,B,C,D,E,F)     _CONCAT_6(A,B,C,D,E,F)
#define CONCAT_7(A,B,C,D,E,F,G)   _CONCAT_7(A,B,C,D,E,F,G)
#define CONCAT_8(A,B,C,D,E,F,G,H) _CONCAT_8(A,B,C,D,E,F,G,H)

#define $1(A)               CONCAT_1(A)
#define $2(A,B)             CONCAT_2(A,B)
#define $3(A,B,C)           CONCAT_3(A,B,C)
#define $4(A,B,C,D)         CONCAT_4(A,B,C,D)
#define $5(A,B,C,D,E)       CONCAT_5(A,B,C,D,E)
#define $6(A,B,C,D,E,F)     CONCAT_6(A,B,C,D,E,F)
#define $7(A,B,C,D,E,F,G)   CONCAT_7(A,B,C,D,E,F,G)
#define $8(A,B,C,D,E,F,G,H) CONCAT_8(A,B,C,D,E,F,G,H)

#ifdef VARIADIC_MACROS

#define _CONCAT(_1,_2,_3,_4,_5,_6,_7,_8,_N,...) _N
#define CONCAT(...) _CONCAT(__VA_ARGS__,\
        _CONCAT_8,\
        _CONCAT_7,\
        _CONCAT_6,\
        _CONCAT_5,\
        _CONCAT_4,\
        _CONCAT_3,\
        _CONCAT_2,\
        _CONCAT_1,\
    )(__VA_ARGS__)

#define $(...) CONCAT(__VA_ARGS__)

#endif // VARIADIC_MACROS
