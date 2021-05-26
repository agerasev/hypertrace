#pragma once

#include "vector.hh"
#include "complex.hh"


typedef real4 quat;
#define q_new make(quat)

#define Q0 q_new(R0, R0, R0, R0)
#define Q1 q_new(R1, R0, R0, R0)
#define QI q_new(R0, R1, R0, R0)
#define QJ q_new(R0, R0, R1, R0)
#define QK q_new(R0, R0, R0, R1)


quat q_conj(quat a);
real q_abs2(quat a);
real q_abs(quat a);


quat q_mul(quat a, quat b);
quat qc_mul(quat a, comp b);
quat cq_mul(comp a, quat b);

quat q_inverse(quat a);
quat q_div(quat a, quat b);
quat qc_div(quat a, comp b);
quat cq_div(comp a, quat b);


#ifndef HOST
#include "quaternion.cc"
#endif // !HOST
