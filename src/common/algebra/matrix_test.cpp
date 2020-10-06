#ifdef TEST

#include "matrix.hh"


comp2x2 TestRngComp2x2::normal() {
    return rng.normal();
}
comp2x2 TestRngComp2x2::uniform() {
    return rng.uniform();
}
comp2x2 TestRngComp2x2::invertible() {
    comp2x2 r;
    do {
        r = normal();
    } while(c_norm_l1(c22_det(r)) < EPS);
    return r;
}
comp2x2 TestRngComp2x2::normalized() {
    return normalize(invertible());
}

real3x3 TestRngReal3x3::normal() {
    return r44_clip_to_r33(rng.normal());
}
real3x3 TestRngReal3x3::uniform() {
    return r44_clip_to_r33(rng.uniform());
}
real3x3 TestRngReal3x3::invertible() {
    real3x3 r;
    do {
        r = normal();
    } while(abs(r33_det(r)) < EPS);
    return r;
}

real4x4 TestRngReal4x4::normal() {
    return rng.normal();
}
real4x4 TestRngReal4x4::uniform() {
    return rng.normal();
}

#ifdef TEST_UNIT

#include <rtest.hpp>

#include <lazy_static.hpp>

namespace test_matrix {

lazy_static_(rstd::Mutex<TestRng<real>>, rng) {
    return rstd::Mutex(TestRng<real>(0xAA));
}
lazy_static_(rstd::Mutex<TestRng<comp>>, crng) {
    return rstd::Mutex(TestRng<comp>(0xBB));
}

} // namespace matrix


rtest_module_(matrix_complex) {
    using namespace test_matrix;

    lazy_static_(rstd::Mutex<TestRngComp2x2>, mrng) {
        return rstd::Mutex(TestRngComp2x2(0xCAFE));
    }
    
    rtest_(one) {
        assert_eq_((c22_new(C1, C0, C0, C1)), approx(c22_one()));
    }
    rtest_(determinant) {
        assert_eq_(c22_det(c22_new(C1, 2*C1, 3*C1, 4*C1)), approx(-2*C1));
    }
    rtest_(inversion) {
        for (int k = 0; k < TEST_ATTEMPTS; ++k) {
            comp2x2 m = mrng->lock()->invertible();
            assert_eq_(c22_dot(m, c22_inverse(m)), approx(c22_one()));
            assert_eq_(c22_dot(c22_inverse(m), m), approx(c22_one()));
        }
    }
    rtest_(norm) {
        assert_eq_(c22_norm_l1(c22_new(-1*C1, 2*C1, -3*C1, 4*C1)), approx(10));
        assert_eq_(length(c22_new(-1*C1, 1*C1, -1*C1, 1*C1)), approx(2));
    }
    rtest_(abs) {
        assert_eq_(
            fabs(c22_new(1*C1, -2*C1, 3*C1, -4*C1)),
            approx(c22_new(1*C1, 2*C1, 3*C1, 4*C1))
        );
    }
    rtest_(eigenvalues_and_eigenvectors) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp2x2 m = mrng->lock()->normal();
            comp2x2 l, v;
            c22_eigen(m, &l, &v);

            comp2x2 x = c22_dot(m, v);
            comp2x2 y = c22_dot(v, c22_new(l.s01, C0, C0, l.s67));
            assert_eq_(x, approx(y));

            comp2x2 o = c22_dot(c22_dot(v, l), c22_inverse(v));
            assert_eq_(o, approx(m));
        }
    }
    rtest_(non_diagonalizable_matrix) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp2x2 m(C1, C0, C0, C1);
            if (rng->lock()->uniform() > 0.5) {
                m.s23 = crng->lock()->normal();
            } else {
                m.s45 = crng->lock()->normal();
            }
            comp2x2 l, v;
            c22_eigen(m, &l, &v);

            comp2x2 o = c22_dot(c22_dot(v, l), c22_inverse(v));
            assert_eq_(o, approx(m));
        }
    }
    rtest_(power) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp2x2 m = mrng->lock()->normalized();
            int n = int(floor(8*rng->lock()->uniform())) + 2;

            comp2x2 p = c22_pow(m, R1/n);
            comp2x2 o = c22_one();

            for(int i = 0; i < n; ++i) {
                o = c22_dot(o, p);
            }

            assert_eq_(o, approx(m));
        }
    }
    rtest_(non_diagonalizable_power) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp2x2 m(C1, C0, C0, C1);
            if (rng->lock()->uniform() > 0.5) {
                m.s23 = crng->lock()->normal();
            } else {
                m.s45 = crng->lock()->normal();
            }
            int p = int(floor(8*rng->lock()->uniform())) + 2;
            int q = int(floor(8*rng->lock()->uniform())) + 2;

            comp2x2 n = c22_one();
            for (int i = 0; i < p; ++i) {
                n = c22_dot(n, m);
            }
            comp2x2 l = c22_pow(m, (real)p/q);
            comp2x2 o = c22_one();
            for (int i = 0; i < q; ++i) {
                o = c22_dot(o, l);
            }

            assert_eq_(o, approx(n));
        }
    }
    rtest_(power_of_almost_identity_transformation) {
        comp2x2 m(
            c_new(1, 8.2331e-07),
            c_new(4.75378e-06, -7.47005e-07),
            c_new(9.3315e-08, -2.74772e-08),
            c_new(1, 9.82438e-07)
        );

        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            int n = int(floor(8*rng->lock()->uniform())) + 2;

            comp2x2 p = c22_pow(m, R1/n);
            comp2x2 o = c22_one();
            for(int i = 0; i < n; ++i) {
                o = c22_dot(o, p);
            }

            assert_eq_(o, approx(m));
        }
    }
}

rtest_module_(matrix_real) {
    using namespace test_matrix;

    lazy_static_(rstd::Mutex<TestRngReal3x3>, r3rng) {
        return rstd::Mutex(TestRngReal3x3(0xABC3));
    }
    lazy_static_(rstd::Mutex<TestRngReal4x4>, r4rng) {
        return rstd::Mutex(TestRngReal4x4(0xABC4));
    }
    lazy_static_(rstd::Mutex<TestRng<quat>>, qrng) {
        return rstd::Mutex(TestRng<quat>(0xABC5));
    }

    rtest_(transpose) {
        for (int k = 0; k < TEST_ATTEMPTS; ++k) {
            real4x4 m = r4rng->lock()->normal();
            assert_eq_(r44_transpose(r44_transpose(m)), approx(m));
        }
    }
    rtest_(dot_product) {
        assert_eq_(r44_dot(r44_one(), r44_one()), approx(r44_one()));

        assert_eq_(r44_dot(
            r44_new(1, -2, 0, 0, 3, 4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1),
            r44_new(0.4, 0.2, 0, 0, -0.3, 0.1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
        ), approx(r44_one()));

        for (int k = 0; k < TEST_ATTEMPTS; ++k) {
            real4x4 a = r4rng->lock()->normal(), b = r4rng->lock()->normal();

            assert_eq_(r44_dot(a, r44_one()), approx(a));
            assert_eq_(r44_dot(r44_one(), b), approx(b));

            assert_eq_(
                r44_transpose(r44_dot(r44_transpose(b), r44_transpose(a))),
                approx(r44_dot(a, b))
            );
        }
    }
    rtest_(determinant) {
        real4x4 r2 = r44_one();
        r2.s01 = r2_new(1, 2);
        r2.s45 = r2_new(3, 4);
        assert_eq_(r33_det(r2), approx(-2));

        for (int k = 0; k < TEST_ATTEMPTS; ++k) {
            real3x3 a = r3rng->lock()->normal(), b = r3rng->lock()->normal();
            assert_eq_(r33_det(a)*r33_det(b), approx(r33_det(r33_dot(a, b))));
        }
    }
    rtest_(inversion) {
        assert_eq_(r33_inverse(r33_one()), approx(r33_one()));

        assert_eq_(
            r33_inverse(r33_new(1, -2, 0, 0, 3, 4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)),
            approx(r33_new(0.4, 0.2, 0, 0, -0.3, 0.1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1))
        );
        assert_eq_(
            r33_inverse(r33_new(1, 0, 0, 0, 0, 1, -2, 0, 0, 3, 4, 0, 0, 0, 0, 1)),
            approx(r33_new(1, 0, 0, 0, 0, 0.4, 0.2, 0, 0, -0.3, 0.1, 0, 0, 0, 0, 1))
        );

        for (int k = 0; k < TEST_ATTEMPTS; ++k) {
            real4x4 m = r3rng->lock()->invertible();

            assert_eq_(r33_dot(m, r33_inverse(m)), approx(r33_one()));
            assert_eq_(r33_dot(r33_inverse(m), m), approx(r33_one()));
        }
    }
    rtest_(outer_product) {
        real4 a = r4_new(0, 1, 2, 4);
        real4 b = r4_new(0, 1, 2, 3);
        real4x4 c = r44_new(
            0, 0, 0, 0,
            0, 1, 2, 3,
            0, 2, 4, 6,
            0, 4, 8, 12
        );
        assert_eq_(r44_outer(a, b), approx(c));
    }
    rtest_(complex_representation) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp a = crng->lock()->normal();
            comp b = crng->lock()->normal();
            assert_eq_(r22_from_comp(a) + r22_from_comp(b), approx(r22_from_comp(a + b)));
            assert_eq_(r22_dot(r22_from_comp(a), r22_from_comp(b)), approx(r22_from_comp(c_mul(a, b))));
            assert_eq_(r22_transpose(r22_from_comp(a)), approx(r22_from_comp(c_conj(a))));
            assert_eq_(r22_inverse(r22_from_comp(a)), approx(r22_from_comp(c_inverse(a))));
        }
    }
    rtest_(quaternion_representation) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat a = qrng->lock()->normal();
            quat b = qrng->lock()->normal();
            assert_eq_(r44_from_quat(a) + r44_from_quat(b), approx(r44_from_quat(a + b)));
            assert_eq_(r44_dot(r44_from_quat(a), r44_from_quat(b)), approx(r44_from_quat(q_mul(a, b))));
            assert_eq_(r44_transpose(r44_from_quat(a)), approx(r44_from_quat(q_conj(a))));
            //assert_eq_(r44_inverse(r44_from_quat(a)), approx(r44_from_quat(q_inverse(a))));
        }
    }
}

#endif // TEST_UNIT

#endif // TEST
