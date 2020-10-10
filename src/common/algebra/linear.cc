#include "linear.hh"


Linear2 lin2_identity() {
    return r22_one();
}

real2 lin2_apply(Linear2 m, real2 v) {
    return r22_dot_mv(m, v);
}

Linear2 lin2_chain(Linear2 a, Linear2 b) {
    return r22_dot(a, b);
}

Linear2 lin2_inverse(Linear2 m) {
    return r22_inverse(m);
}


Linear3 lin3_identity() {
    return r44_one();
}

Linear3 lin3_look_to(real3 dir) {
    if (2*fabs(dir.z) < 1) {
        return lin3_look_to_up(dir, r3_new(0,0,1));
    } else {
        return lin3_look_to_up(dir, r3_new(0,1,0));
    }
}

Linear3 lin3_look_to_up(real3 dir, real3 up) {
    real3 right = normalize(cross(dir, up));
    real3 down = cross(dir, right);
    return MAKE(Linear3)(
        r4_new(right, R0),
        r4_new(down, R0),
        r4_new(dir, R0),
        r4_new(R0)
    );
}

real3 lin3_apply(Linear3 m, real3 v) {
    return r33_dot_mv(m, v);
}

Linear3 lin3_chain(Linear3 a, Linear3 b) {
    return r33_dot(a, b);
}

Linear3 lin3_inverse(Linear3 m) {
    return r33_inverse(m);
}


#ifdef TEST

#include <rtest.hpp>

#ifdef TEST_UNIT

rtest_module_(linear) {
    static_thread_local_(TestRng<real>, rng) {
        return TestRng<real>(0xABBA);
    }
    static_thread_local_(TestRng<real3>, vrng) {
        return TestRng<real3>(0xBAAB);
    }
    static_thread_local_(TestRngReal3x3, mrng) {
        return TestRngReal3x3(0xBEEB);
    }

    rtest_(linearity) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Linear3 m = mrng->normal();
            real a = rng->normal();
            real3 x = vrng->normal();

            assert_eq_(lin3_apply(a*m, x), approx(lin3_apply(m, a*x)));
            assert_eq_(lin3_apply(a*m, x), approx(a*lin3_apply(m, x)));
        }
    }
    rtest_(chaining) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Linear3 a = mrng->normal();
            Linear3 b = mrng->normal();
            real3 c = vrng->normal();

            assert_eq_(lin3_chain(a, lin3_identity()), approx(a));
            assert_eq_(lin3_chain(lin3_identity(), b), approx(b));
            assert_eq_(lin3_apply(lin3_chain(a, b), c), approx(lin3_apply(a, lin3_apply(b, c))));
        }
    }
    rtest_(inversion) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Linear3 a = mrng->invertible();
            real3 x = vrng->normal();

            assert_eq_(lin3_chain(a, lin3_inverse(a)), approx(lin3_identity()));
            assert_eq_(lin3_chain(lin3_inverse(a), a), approx(lin3_identity()));
            assert_eq_(lin3_apply(lin3_inverse(a), lin3_apply(a, x)), approx(x));
            assert_eq_(lin3_apply(a, lin3_apply(lin3_inverse(a), x)), approx(x));
        }
    }
    rtest_(look_to_the_direction) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 d = vrng->unit();
            Linear3 m = lin3_look_to(d);

            assert_eq_(lin3_apply(m, d), approx(r3_new(R0, R0, R1)));
        }
    }
};

#endif // TEST_UNIT

#ifdef TEST_DEV

#include <rtest.hpp>

#include <vector>

#include <test/devtest.hpp>

extern_lazy_static_(devtest::Selector, devtest_selector);

rtest_module_(linear) {
    rtest_(chain) {
        TestRngReal3x3 mrng(0xcafe);

        for (devtest::Target target : *devtest_selector) {
            auto queue = target.make_queue();
            auto kernel = devtest::KernelBuilder(target.device_id(), queue)
            .source("moebius.cl", std::string(
                "#include <common/algebra/linear.hh>\n"
                "__kernel void chain(__global const Linear3 *x, __global const Linear3 *y, __global Linear3 *z) {\n"
                "    int i = get_global_id(0);\n"
                "    z[i] = lin3_chain(x[i], y[i]);\n"
                "}\n"
                "#include <device/source.cl>\n"
            ))
            .build("chain").unwrap();

            const int n = TEST_ATTEMPTS;
            std::vector<Linear3> xbuf(n), ybuf(n), zbuf(n);
            for (size_t i = 0; i < n; ++i) {
                xbuf[i] = mrng.normal();
                ybuf[i] = mrng.normal();
            }

            devtest::KernelRunner(queue, std::move(kernel))
            .run(n, xbuf, ybuf, zbuf).expect("Kernel run error");

            for(size_t i = 0; i < n; ++i) {
                Linear3 z = lin3_chain(xbuf[i], ybuf[i]);
                assert_eq_(approx(z).epsilon(1e-4), zbuf[i]);
            }
        }
    }
}

#endif // TEST_DEV

#endif // TEST
