#pragma once

#define _CONCAT_1(A)               A
#define _CONCAT_2(A,B)             A##B
#define _CONCAT_3(A,B,C)           A##B##C
#define _CONCAT_4(A,B,C,D)         A##B##C##D
#define _CONCAT_5(A,B,C,D,E)       A##B##C##D##E
#define _CONCAT_6(A,B,C,D,E,F)     A##B##C##D##E##F
#define _CONCAT_7(A,B,C,D,E,F,G)   A##B##C##D##E##F##G
#define _CONCAT_8(A,B,C,D,E,F,G,H) A##B##C##D##E##F##G##H

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
