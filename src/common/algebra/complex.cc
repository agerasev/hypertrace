#include "complex.hh"


comp c_conj(comp a) {
    return c_new(a.x, -a.y);
}
real c_abs2(comp a) {
    return dot(a, a);
}
real c_abs(comp a) {
    return length(a);
}
real c_norm_l1(comp a) {
    return fabs(a.x) + fabs(a.y);
}

comp c_mul(comp a, comp b) {
    return c_new(
        a.x*b.x - a.y*b.y,
        a.x*b.y + a.y*b.x
    );
}

comp c_inverse(comp a) {
    return c_conj(a)/c_abs2(a);
}

comp c_div(comp a, comp b) {
    return c_mul(a, c_inverse(b));
}

comp c_exp(comp p) {
    return exp(p.x)*c_new(cos(p.y), sin(p.y));
}
comp c_pow_r(comp a, real p) {
    real r = pow(c_abs2(a), p/2);
    real phi = p*atan2(a.y, a.x);
    return c_new(r*cos(phi), r*sin(phi));
}
comp c_sqrt(comp a) {
    real r = sqrt(c_abs(a));
    real phi = 0.5f*atan2(a.y, a.x);
    return c_new(r*cos(phi), r*sin(phi));
}


#ifdef TEST_UNIT

#include <rtest.hpp>
#include <rstd/prelude.hpp>
#include <lazy_static.hpp>

rtest_module_(complex) {
    lazy_static_(rstd::Mutex<TestRng<real>>, rng) {
        return rstd::Mutex(TestRng<real>(0xbeef));
    }
    lazy_static_(rstd::Mutex<TestRng<comp>>, crng) {
        return rstd::Mutex(TestRng<comp>(0xbeef));
    }

    rtest_(constructor) {
        comp a = c_new(0.0, 1.0);
        assert_eq_(a.x, Approx(0.0));
        assert_eq_(a.y, Approx(1.0));
    }
    rtest_(inversion) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp a = crng->lock()->nonzero();
            assert_eq_(c_div(a, a), approx(C1));
        }
    }
    rtest_(square_root) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp a = crng->lock()->normal();
            comp b = c_sqrt(a);
            assert_eq_(c_mul(b, b), approx(a));
        }
    }
    rtest_(power) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp a = crng->lock()->normal();
            int n = int(floor(2 + 10*rng->lock()->uniform()));
            comp b = c_pow_r(a, 1.0/n);
            comp c = C1;
            for (int i = 0; i < n; ++i) {
                c = c_mul(c, b);
            }
            assert_eq_(c, approx(a));
        }
    }
    rtest_(norm) {
        assert_eq_(c_norm_l1(c_new(-1, 2)), approx(3));
        assert_eq_(length(c_new(3, -4)), approx(5));
    }
};

#endif // TEST_UNIT
