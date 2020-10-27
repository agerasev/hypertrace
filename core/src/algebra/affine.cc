#include "affine.hh"


Affine3 aff3_identity() {
    return r44_one();
}
Affine3 aff3_from_ls(Linear3 m, real3 v) {
    m.s3 = v.x;
    m.s7 = v.y;
    m.sb = v.z;
    return m;
}

Linear3 aff3_get_linear(Affine3 m) {
    return r44_clip_to_r33(m);
}
real3 aff3_get_shift(Affine3 m) {
    return r3_new(m.s3, m.s7, m.sb);
}

real4 aff3_apply4(Affine3 m, real4 v) {
    return r44_dot_mv(m, v);
}
real3 aff3_apply_pos(Affine3 m, real3 p) {
    return aff3_apply4(m, r4_new(p, R1)).xyz;
}
real3 aff3_apply_dir(Affine3 m, real3 d) {
    return aff3_apply4(m, r4_new(d, R0)).xyz;
}

Affine3 aff3_chain(Affine3 a, Affine3 b) {
    return r44_dot(a, b);
}
Affine3 aff3_inverse(Affine3 m) {
    Linear3 l = lin3_inverse(m); // aff3_get_linear(m)
    real3 s = -lin3_apply(l, aff3_get_shift(m));
    return aff3_from_ls(l, s);
}

Affine3 aff3_interpolate(Affine3 a, Affine3 b, real t) {
    // FIXME: Use matrix power operation
    return (R1 - t)*a + t*b;
}


#ifdef TEST

Affine3 TestRngAffine3::normal() {
    return aff3_from_ls(lrng.normal(), srng.normal());
}
Affine3 TestRngAffine3::uniform() {
    return aff3_from_ls(lrng.uniform(), srng.uniform());
}
Affine3 TestRngAffine3::invertible() {
    return aff3_from_ls(lrng.invertible(), srng.normal());
}

#include <rtest.hpp>

#ifdef TEST_UNIT

rtest_module_(affine) {
    static_thread_local_(TestRngAffine3, arng) {
        return TestRngAffine3(0xAFFE);
    }
    static_thread_local_(TestRng<real3>, vrng) {
        return TestRng<real3>(0xAFFE);
    }

    rtest_(chaining) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Affine3 a = arng->normal();
            Affine3 b = arng->normal();
            real3 c = vrng->normal();

            assert_eq_(aff3_apply_pos(aff3_chain(a, b), c), approx(aff3_apply_pos(a, aff3_apply_pos(b, c))));
            assert_eq_(aff3_apply_dir(aff3_chain(a, b), c), approx(aff3_apply_dir(a, aff3_apply_dir(b, c))));
        }
    }
    rtest_(inversion) {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Affine3 a = arng->invertible();
            real3 x = vrng->normal();
            
            assert_eq_(aff3_apply_pos(aff3_inverse(a), aff3_apply_pos(a, x)), approx(x));
            assert_eq_(aff3_apply_pos(a, aff3_apply_pos(aff3_inverse(a), x)), approx(x));
            assert_eq_(aff3_apply_dir(aff3_inverse(a), aff3_apply_dir(a, x)), approx(x));
            assert_eq_(aff3_apply_dir(a, aff3_apply_dir(aff3_inverse(a), x)), approx(x));
        }
    }
};

#endif // TEST_UNIT

#ifdef TEST_DEV

#include <rtest.hpp>

#include <vector>

#include <test/devtest.hpp>

extern devtest::Target devtest_make_target();

rtest_module_(affine) {
    rtest_(chain) {
        TestRngAffine3 arng(0xcafe);

        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("moebius.cl", std::string(
            "#include <algebra/affine.hh>\n"
            "__kernel void chain(__global const Affine3 *x, __global const Affine3 *y, __global Affine3 *z) {\n"
            "    int i = get_global_id(0);\n"
            "    z[i] = aff3_chain(x[i], y[i]);\n"
            "}\n"
        ))
        .build("chain").expect("Kernel build error");

        const int n = TEST_ATTEMPTS;
        std::vector<Affine3> xbuf(n), ybuf(n), zbuf(n);
        for (size_t i = 0; i < n; ++i) {
            xbuf[i] = arng.normal();
            ybuf[i] = arng.normal();
        }

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, xbuf, ybuf, zbuf).expect("Kernel run error");

        for(size_t i = 0; i < n; ++i) {
            Affine3 z = aff3_chain(xbuf[i], ybuf[i]);
            assert_eq_(dev_approx(z), zbuf[i]);
        }
    }
}

#endif // TEST_DEV

#endif // TEST
