#pragma once

#include "matrix.hh"


typedef real3x3 Linear3;

#define lin3_new r33_new
Linear3 lin3_identity();

Linear3 lin3_look_to(real3 dir);
Linear3 lin3_look_to_up(real3 dir, real3 up);

real3 lin3_apply(Linear3 m, real3 v);
Linear3 lin3_chain(Linear3 a, Linear3 b);
Linear3 lin3_inverse(Linear3 m);

Linear3 interpolate(Linear3 a, Linear3 b, real t);
