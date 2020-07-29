#pragma once

#include "vector.hh"


typedef real2 comp;
#define c_new MAKE(comp)

#define C0 c_new(R0, R0)
#define C1 c_new(R1, R0)
#define CI c_new(R0, R1)


comp c_conj(comp a);
real c_abs2(comp a);
real c_abs(comp a);
real c_norm_l1(comp a);

comp c_mul(comp a, comp b);
comp c_inverse(comp a);
comp c_div(comp a, comp b);

comp c_exp(comp p);
comp c_powr(comp a, real p);
comp c_sqrt(comp a);
