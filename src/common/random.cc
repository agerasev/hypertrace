#include "random.hh"


Rng rng_init(uint seed) {
    Rng rng;
    rng.state = seed;
    return rng;
}
uint rng_next(Rng *rng) {
    return (rng->state = 1103515245*rng->state + 12345);
}

real random_uniform(Rng *rng) {
    return (real)rng_next(rng)/(real)0x100000000l;
}
real2 random_uniform2(Rng *rng) {
    return MAKE(real2)(
        random_uniform(rng),
        random_uniform(rng)
    );
}
real3 random_uniform3(Rng *rng) {
    return MAKE(real3)(
        random_uniform(rng),
        random_uniform(rng),
        random_uniform(rng)
    );
}
real4 random_uniform4(Rng *rng) {
    return MAKE(real4)(
        random_uniform(rng),
        random_uniform(rng),
        random_uniform(rng),
        random_uniform(rng)
    );
}

static real3 _sphere_point(real cos_theta, real sin_theta, real phi) {
    return MAKE(real3)(cos(phi)*sin_theta, sin(phi)*sin_theta, cos_theta);
}
real3 random_sphere(Rng *rng) {
    real phi = 2*PI*random_uniform(rng);
    real cos_theta = R1 - 2*random_uniform(rng);
    real sin_theta = sqrt(1 - cos_theta*cos_theta);
    return _sphere_point(cos_theta, sin_theta, phi);
}
real3 random_hemisphere(Rng *rng) {
    real phi = 2*PI*random_uniform(rng);
    real cos_theta = random_uniform(rng);
    real sin_theta = sqrt(1 - cos_theta*cos_theta);
    return _sphere_point(cos_theta, sin_theta, phi);
}
real3 random_hemisphere_cosine(Rng *rng) {
    real phi = 2*PI*random_uniform(rng);
    real sqr_cos_theta = random_uniform(rng);
    real cos_theta = sqrt(sqr_cos_theta);
    real sin_theta = sqrt(1 - sqr_cos_theta);
    return _sphere_point(cos_theta, sin_theta, phi);
}
real3 random_sphere_cap(Rng *rng, real cos_alpha) {
    real phi = 2*PI*random_uniform(rng);
    real cos_theta = 1 - (1 - cos_alpha)*random_uniform(rng);
    real sin_theta = sqrt(1 - cos_theta*cos_theta);
    return _sphere_point(cos_theta, sin_theta, phi);
}


#ifdef TEST

#include <rtest.hpp>

#ifdef TEST_UNIT

rtest_module_(random) {
    // Monte Carlo confidence interval
    static const real CONF = 3.0/sqrt(real(TEST_ATTEMPTS));

    rtest_(uniform) {
        Rng rng = rng_init(0xdeadbeef);
        real mean = 0.0;
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            mean += sin(2*PI*random_uniform(&rng));
        }
        mean /= TEST_ATTEMPTS;
        assert_eq_(mean, approx(0).epsilon(2*CONF));
    }
    rtest_(uniform2) {
        Rng rng = rng_init(0xdeadbeef);
        real mean = 0.0;
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real2 v = 2*PI*random_uniform2(&rng);
            mean += sin(v.x)*sin(v.y);
        }
        mean /= TEST_ATTEMPTS;
        assert_eq_(mean, approx(0).epsilon(2*CONF));
    }
    rtest_(uniform3) {
        Rng rng = rng_init(0xdeadbeef);
        real mean = 0.0;
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 v = 2*PI*random_uniform3(&rng);
            mean += sin(v.x)*sin(v.y)*sin(v.z);
        }
        mean /= TEST_ATTEMPTS;
        assert_eq_(mean, approx(0).epsilon(2*CONF));
    }
    rtest_(uniform4) {
        Rng rng = rng_init(0xdeadbeef);
        real mean = 0.0;
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real4 v = 2*PI*random_uniform4(&rng);
            mean += sin(v.x)*sin(v.y)*sin(v.z)*sin(v.w);
        }
        mean /= TEST_ATTEMPTS;
        assert_eq_(mean, approx(0).epsilon(2*CONF));
    }

    rtest_(sphere) {
        Rng rng = rng_init(0xdeadbeef);
        real3 mean = real3(R0);
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 v = random_sphere(&rng);
            assert_eq_(length(v), approx(1));
            mean += v;
        }
        mean /= TEST_ATTEMPTS;
        assert_eq_(mean, approx(real3(R0)).epsilon(2*CONF));
    }
    rtest_(hemisphere) {
        Rng rng = rng_init(0xdeadbeef);
        real3 mean = real3(R0);
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 v = random_hemisphere(&rng);
            assert_eq_(length(v), approx(1));
            assert_(v.z > -EPS);
            mean += v;
        }
        mean /= TEST_ATTEMPTS;
        assert_eq_(mean.xy, approx(real2(R0)).epsilon(2*CONF));
    }
}

#endif // TEST_UNIT
/*
#ifdef TEST_DEV

#include <vector>

#include <test/devtest.hpp>

extern devtest::Target devtest_make_target();

rtest_module_(random) {
    rtest_(square) {
        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("real.cl", std::string(
            "#include <common/algebra/real.hh>\n"
            "__kernel void square(__global const real *ibuf, __global real *obuf) {\n"
            "    int i = get_global_id(0);\n"
            "    real x = ibuf[i];\n"
            "    obuf[i] = x*x;\n"
            "}\n"
        ))
        .build("square").expect("Kernel build error");

        const int n = TEST_ATTEMPTS;
        std::vector<real> ibuf(n), obuf(n);
        for (size_t i = 0; i < ibuf.size(); ++i) {
            ibuf[i] = real(i);
        }

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, ibuf, obuf).expect("Kernel run error");

        for(size_t i = 0; i < obuf.size(); ++i) {
            assert_eq_(i*i, size_t(obuf[i]));
        }
    }
}

#endif // TEST_DEV
*/
#endif // TEST
