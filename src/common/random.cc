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

// Monte Carlo confidence interval
static const real CONF = 3.0/sqrt(real(TEST_ATTEMPTS));

#ifdef TEST_UNIT

#include <functional>
#include <iterator>
#include <algorithm>

template <typename T>
class Grid {
private:
    std::vector<real> cells;

public:
    virtual std::vector<real> sizes() const = 0;
    virtual size_t index(T x) const = 0;

public:
    Grid(size_t N) : cells(N, 0.0) {}

    real &operator[](T x) {
        size_t i = index(x);
        assert_(i < cells.size());
        return cells[i];
    }
    std::vector<real> total(real norm) const {
        std::vector<real> total(cells.size());
        std::vector<real> sizes = this->sizes();
        for (size_t i = 0; i < cells.size(); ++i) {
            total[i] = cells[i]/sizes[i]/norm;
        }
        return total;
    }
    void assert_all(real norm, real eps) const {
        std::vector<real> total = this->total(norm);
        std::vector<real> sizes = this->sizes();
        for (size_t i = 0; i < total.size(); ++i) {
            assert_eq_(total[i], approx(1).epsilon(eps/sizes[i]));
        }
    }
};
class SphereGrid : public Grid<real3> {
private:
    size_t P, T;
    real max_theta;

public:
    std::vector<real> sizes() const override {
        std::vector<real> s(P*T);
        real ar = 2*PI*(1 - cos(max_theta));
        real dp = 2*PI/P;
        real dt = max_theta/T;
        for (size_t t = 0; t < T; ++t) {
            for (size_t p = 0; p < P; ++p) {
                s[p + t*P] = dp*(cos(t*dt) - cos((t + 1)*dt))/ar;
            }
        }
        return s;
    }
    size_t index(real3 x) const override {
        real phi = atan2(x.y, x.x);
        if (phi < 0.0) {
            phi += 2*PI;
        }
        real theta = atan2(length(x.xy), x.z);

        size_t p = P*phi/(2*PI);
        size_t t = T*theta/max_theta;

        return p + t*P;
    }

public:
    SphereGrid(
        size_t P, size_t T,
        real mt=PI
    ) :
        Grid<real3>(P*T),
        P(P), T(T),
        max_theta(mt)
    {}
};
class DiskGrid : public Grid<real2> {
private:
    size_t P, R;

public:
    std::vector<real> sizes() const override {
        std::vector<real> s(P*R);
        real ar = PI;
        real dp = 2*PI/P;
        real dr = 1.0/R;
        for (size_t r = 0; r < R; ++r) {
            for (size_t p = 0; p < P; ++p) {
                real r1 = r*dr, r2 = (r + 1)*dr;
                s[p + r*P] = dp*(r2*r2 - r1*r1)/ar;
            }
        }
        return s;
    }
    size_t index(real2 x) const override {
        real phi = atan2(x.y, x.x);
        if (phi < 0.0) {
            phi += 2*PI;
        }
        real rad = length(x);

        size_t p = P*phi/(2*PI);
        size_t r = R*rad;

        return p + r*P;
    }

public:
    DiskGrid(
        size_t P, size_t R
    ) :
        Grid<real2>(P*R),
        P(P), R(R)
    {}
};

rtest_module_(random) {
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

        real3 sum = real3(R0);
        size_t P = 16, T = 32;
        SphereGrid grid(P, T);

        const size_t N = P*T*TEST_ATTEMPTS;
        for (size_t i = 0; i < N; ++i) {
            real3 v = random_sphere(&rng);
            assert_eq_(length(v), approx(1));
            sum += v;
            grid[v] += 1.0;
        }

        assert_eq_(sum/N, approx(real3(R0)).epsilon(2*CONF));
        grid.assert_all(N, CONF);
    }
    rtest_(hemisphere) {
        Rng rng = rng_init(0xdeadbeef);

        real3 sum = real3(R0);
        size_t P = 16, T = 16;
        SphereGrid grid(P, T, PI/2);

        const size_t N = P*T*TEST_ATTEMPTS;
        for (size_t i = 0; i < N; ++i) {
            real3 v = random_hemisphere(&rng);
            assert_eq_(length(v), approx(1));
            assert_(v.z > -EPS);
            sum += v;
            grid[v] += 1.0;
        }

        assert_eq_(sum.xy/N, approx(real2(R0)).epsilon(2*CONF));
        grid.assert_all(N, CONF);
    }
    rtest_(hemisphere_cosine) {
        Rng rng = rng_init(0xdeadbeef);

        real3 sum = real3(R0);
        size_t P = 16, R = 16;
        DiskGrid grid(P, R);

        const size_t N = P*R*TEST_ATTEMPTS;
        for (size_t i = 0; i < N; ++i) {
            real3 v = random_hemisphere_cosine(&rng);
            assert_eq_(length(v), approx(1));
            assert_(v.z > -EPS);
            sum += v;
            grid[v.xy] += 1.0;
        }

        assert_eq_(sum.xy/N, approx(real2(R0)).epsilon(2*CONF));
        grid.assert_all(N, CONF);
    }
    rtest_(sphere_cap) {
        Rng rng = rng_init(0xdeadbeef);
        size_t C = 16;

        for (size_t j = 0; j < C; ++j) {
            real t = PI*random_uniform(&rng);
            real ct = cos(t);

            real3 sum = real3(R0);
            size_t P = 16, T = 16;
            SphereGrid grid(P, T, t);

            const size_t N = P*T*TEST_ATTEMPTS/C;
            for (size_t i = 0; i < N; ++i) {
                real3 v = random_sphere_cap(&rng, ct);
                assert_eq_(length(v), approx(1));
                assert_(v.z > ct - EPS);
                sum += v;
                grid[v] += 1.0;
            }

            assert_eq_(sum.xy/N, approx(real2(R0)).epsilon(2*CONF));
            grid.assert_all(N, CONF);
        }
    }
}

#endif // TEST_UNIT

#ifdef TEST_DEV

#include <vector>
#include <random>

#include <test/devtest.hpp>

extern devtest::Target devtest_make_target();

rtest_module_(random) {
    rtest_(sphere) {
        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("real.cl", std::string(
            "#include <common/random.hh>\n"
            "__kernel void sphere(__global const uint *seed, __global real3 *point) {\n"
            "    int i = get_global_id(0);\n"
            "    Rng rng = rng_init(seed[i]);\n"
            "    point[i] = random_sphere(&rng);\n"
            "}\n"
        ))
        .build("sphere").expect("Kernel build error");

        const int n = TEST_ATTEMPTS;
        std::vector<uint> seed(n);
        std::vector<real3> point(n);
        std::mt19937 rng(0xdeadbeef);
        for (size_t i = 0; i < n; ++i) {
            seed[i] = rng();
        }

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, seed, point).expect("Kernel run error");

        real3 sum = real3(R0);
        const size_t N = TEST_ATTEMPTS;
        for (size_t i = 0; i < N; ++i) {
            real3 v = point[i];
            assert_eq_(length(v), dev_approx(1));
            sum += v;
        }
        assert_eq_(sum/N, dev_approx(real3(R0)).epsilon(2*CONF));
    }
}

#endif // TEST_DEV

#endif // TEST
