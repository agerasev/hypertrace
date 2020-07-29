#pragma once

#include "vector.hh"
#include "complex.hh"
#include "quaternion.hh"


typedef real8 comp2x2;


comp2x2 comp2x2_zero();
comp2x2 comp2x2_one();

comp2x2 comp2x2_transpose(comp2x2 m);
comp2x2 comp2x2_mul(comp2x2 m, comp c);
comp2x2 comp2x2_div(comp2x2 m, comp c);
real comp2x2_norm_l1(comp2x2 m);

comp2x2 comp2x2_dot(comp2x2 a, comp2x2 b);

comp comp2x2_det(comp2x2 m);
comp2x2 comp2x2_normalize(comp2x2 m);

comp2x2 comp2x2_inverse(comp2x2 m);
comp2x2 comp2x2_inverse_n(comp2x2 m);

void comp2x2_eigen(comp2x2 m, comp2x2 *l, comp2x2 *v);
void comp2x2_eigen_n(comp2x2 m, comp2x2 *l, comp2x2 *v);

comp2x2 comp2x2_pow(comp2x2 m, real p);
comp2x2 comp2x2_pow_n(comp2x2 m, real p);


#ifdef UNIT_TEST

class TestRngComp2x2 {
private:
    TestRng<comp2x2> rng;

public:
    inline TestRngComp2x2() = default;
    inline explicit TestRngComp2x2(uint32_t seed) : rng(seed) {}

    comp2x2 normal() {
        return rng.normal();
    }
    comp2x2 uniform() {
        return rng.normal();
    }
    comp2x2 invertible() {
        comp2x2 r;
        do {
            r = normal();
        } while(c_norm_l1(comp2x2_det(r)) < EPS);
        return r;
    }
    comp2x2 normalized() {
        return normalize(invertible());
    }
};

#endif // UNIT_TEST
