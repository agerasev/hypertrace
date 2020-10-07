#include "quaternion.hh"


quat q_conj(quat a) {
    return q_new(a.x, -a.yzw);
}
real q_abs2(quat a) {
    return dot(a, a);
}
real q_abs(quat a) {
    return length(a);
}


quat q_mul(quat a, quat b) {
    return q_new(
        a.x*b.x - a.y*b.y - a.z*b.z - a.w*b.w,
        a.x*b.y + a.y*b.x + a.z*b.w - a.w*b.z,
        a.x*b.z + a.z*b.x - a.y*b.w + a.w*b.y,
        a.x*b.w + a.w*b.x + a.y*b.z - a.z*b.y
    );
}

quat qc_mul(quat a, comp b) {
    return q_new(
        a.x*b.x - a.y*b.y,
        a.x*b.y + a.y*b.x,
        a.z*b.x + a.w*b.y,
        a.w*b.x - a.z*b.y
    );
}

quat cq_mul(comp a, quat b) {
    return q_new(
        a.x*b.x - a.y*b.y,
        a.x*b.y + a.y*b.x,
        a.x*b.z - a.y*b.w,
        a.x*b.w + a.y*b.z
    );
}

quat q_inverse(quat a) {
    return q_conj(a)/q_abs2(a);
}

quat q_div(quat a, quat b) {
    return q_mul(a, q_inverse(b));
}

quat qc_div(quat a, comp b) {
    return qc_mul(a, c_inverse(b));
}

quat cq_div(comp a, quat b) {
    return cq_mul(a, q_inverse(b));
}


#ifdef TEST_UNIT

#include <rtest.hpp>

#include <vector>
#include <utility>
#include <functional>


rtest_module_(quaternion) {
    static_thread_local_(TestRng<quat>, qrng) {
        return TestRng<quat>(0xfeed);
    }

    rtest_(imaginary_units) {
        assert_eq_(q_mul(QI, QI), approx(-Q1));
        assert_eq_(q_mul(QJ, QJ), approx(-Q1));
        assert_eq_(q_mul(QK, QK), approx(-Q1));
        assert_eq_(q_mul(q_mul(QI, QJ), QK), approx(-Q1));

        assert_eq_(q_mul(QI, QJ), approx(QK));
        assert_eq_(q_mul(QJ, QK), approx(QI));
        assert_eq_(q_mul(QK, QI), approx(QJ));
        
        assert_eq_(q_mul(QJ, QI), approx(-QK));
        assert_eq_(q_mul(QK, QJ), approx(-QI));
        assert_eq_(q_mul(QI, QK), approx(-QJ));
    }
    rtest_(inversion) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat a = qrng->nonzero();
            assert_eq_(q_div(a, a), approx(Q1));
        }
    }
    rtest_(law_of_cosines) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat a = qrng->normal(), b = qrng->normal();
            assert_eq_(q_abs2(a) + q_abs2(b) + 2*dot(a, b), Approx(q_abs2(a + b)));
        }
    }
    rtest_(conjugation) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat a = qrng->normal();
            assert_eq_(q_mul(a, q_conj(a)), approx(q_abs2(a)*Q1));
            assert_eq_(q_mul(q_conj(a), a), approx(q_abs2(a)*Q1));
        }
    }
    rtest_(derivation) {
        std::vector<std::pair<
            std::function<quat(quat)>,
            std::function<quat(quat, quat)>
        >> cases = {
            std::make_pair(
                [](quat p) { return p; },
                [](quat, quat v) { return v; }
            ),
            std::make_pair(
                [](quat p) { return q_mul(p, p); },
                [](quat p, quat v) { return q_mul(p, v) + q_mul(v, p); }
            ),
            std::make_pair(
                [](quat p) { return q_inverse(p); },
                [](quat p, quat v) {
                    real p2 = q_abs2(p);
                    return (q_conj(v) - (2*dot(p, v)/p2)*q_conj(p))/p2;
                }
            )
        };
        
        for (auto p : cases) {
            auto f = p.first;
            auto dfdv = p.second;
            for (int i = 0; i < TEST_ATTEMPTS; ++i) {
                quat p = qrng->normal();
                quat v = qrng->unit();
                quat deriv = dfdv(p, v);
                real dabs = q_abs(deriv);
                assert_eq_(
                    approx(deriv).epsilon(1e3*EPS*dabs),
                    (f(p + EPS*v) - f(p))/EPS
                );
            }
        }
    }
};

#endif // TEST_UNIT
