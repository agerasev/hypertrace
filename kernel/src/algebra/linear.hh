#pragma once

#include "matrix.hh"


typedef real2x2 Linear2;
typedef real3x3 Linear3;


#define lin2_new r22_new
Linear2 lin2_identity();

real2 lin2_apply(Linear2 m, real2 v);
Linear2 lin2_chain(Linear2 a, Linear2 b);
Linear2 lin2_inverse(Linear2 m);


#define lin3_new r33_new
Linear3 lin3_identity();

Linear3 lin3_look_to(real3 dir);
Linear3 lin3_look_to_up(real3 dir, real3 up);

real3 lin3_apply(Linear3 m, real3 v);
Linear3 lin3_chain(Linear3 a, Linear3 b);
Linear3 lin3_inverse(Linear3 m);


#ifndef HOST
#include "linear.cc"
#endif // !HOST
