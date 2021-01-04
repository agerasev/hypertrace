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
    return (real)rng_next(rng)/(real)((ulong)1 << 32);
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
    real zeta = 2*random_uniform(rng);
    real cos_theta = 1 - zeta;
    real sin_theta = sqrt((2 - zeta)*zeta);
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
    real zeta = (R1 - cos_alpha)*random_uniform(rng);
    real cos_theta = 1 - zeta;
    real sin_theta = sqrt((2 - zeta)*zeta);
    return _sphere_point(cos_theta, sin_theta, phi);
}


#ifdef UNITTEST

#include <gtest/gtest.h>

// Monte Carlo confidence interval
static const real CONF = 3.0/sqrt(real(TEST_ATTEMPTS));

std::vector<real> SphereGrid::sizes() const {
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
size_t SphereGrid::index(real3 x) const {
    real phi = atan2(x.y, x.x);
    if (phi < R0) {
        phi += 2*PI;
    }
    real theta = atan2(length(x.xy), x.z);

    size_t p = P*phi/(2*PI);
    size_t t = T*theta/max_theta;

    return p + t*P;
}

std::vector<real> DiskGrid::sizes() const {
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
size_t DiskGrid::index(real2 x) const {
    real phi = atan2(x.y, x.x);
    if (phi < R0) {
        phi += 2*PI;
    }
    real rad = length(x);

    size_t p = P*phi/(2*PI);
    size_t r = R*rad;

    return p + r*P;
}

TEST(RandomTest, uniform) {
    Rng rng = rng_init(0xdeadbeef);
    real mean = 0.0;
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        mean += sin(2*PI*random_uniform(&rng));
    }
    mean /= TEST_ATTEMPTS;
    EXPECT_EQ(mean, approx(0).epsilon(2*CONF));
}
TEST(RandomTest, uniform2) {
    Rng rng = rng_init(0xdeadbeef);
    real mean = 0.0;
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        real2 v = 2*PI*random_uniform2(&rng);
        mean += sin(v.x)*sin(v.y);
    }
    mean /= TEST_ATTEMPTS;
    EXPECT_EQ(mean, approx(0).epsilon(2*CONF));
}
TEST(RandomTest, uniform3) {
    Rng rng = rng_init(0xdeadbeef);
    real mean = 0.0;
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        real3 v = 2*PI*random_uniform3(&rng);
        mean += sin(v.x)*sin(v.y)*sin(v.z);
    }
    mean /= TEST_ATTEMPTS;
    EXPECT_EQ(mean, approx(0).epsilon(2*CONF));
}
TEST(RandomTest, uniform4) {
    Rng rng = rng_init(0xdeadbeef);
    real mean = 0.0;
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        real4 v = 2*PI*random_uniform4(&rng);
        mean += sin(v.x)*sin(v.y)*sin(v.z)*sin(v.w);
    }
    mean /= TEST_ATTEMPTS;
    EXPECT_EQ(mean, approx(0).epsilon(2*CONF));
}

TEST(RandomTest, sphere) {
    Rng rng = rng_init(0xdeadbeef);

    real3 sum = real3(R0);
    size_t P = 16, T = 32;
    SphereGrid grid(P, T);

    const size_t N = P*T*TEST_ATTEMPTS;
    for (size_t i = 0; i < N; ++i) {
        real3 v = random_sphere(&rng);
        EXPECT_EQ(length(v), approx(1));
        sum += v;
        grid[v] += 1.0;
    }

    EXPECT_EQ(sum/N, approx(real3(R0)).epsilon(2*CONF));
    ASSERT_TRUE(grid.check_all(N, CONF));
}
TEST(RandomTest, hemisphere) {
    Rng rng = rng_init(0xdeadbeef);

    real3 sum = real3(R0);
    size_t P = 16, T = 16;
    SphereGrid grid(P, T, PI/2);

    const size_t N = P*T*TEST_ATTEMPTS;
    for (size_t i = 0; i < N; ++i) {
        real3 v = random_hemisphere(&rng);
        EXPECT_EQ(length(v), approx(1));
        ASSERT_TRUE(v.z > -EPS);
        sum += v;
        grid[v] += 1.0;
    }

    EXPECT_EQ(sum.xy/N, approx(real2(R0)).epsilon(2*CONF));
    ASSERT_TRUE(grid.check_all(N, CONF));
}
TEST(RandomTest, hemisphere_cosine) {
    Rng rng = rng_init(0xdeadbeef);

    real3 sum = real3(R0);
    size_t P = 16, R = 16;
    DiskGrid grid(P, R);

    const size_t N = P*R*TEST_ATTEMPTS;
    for (size_t i = 0; i < N; ++i) {
        real3 v = random_hemisphere_cosine(&rng);
        EXPECT_EQ(length(v), approx(1));
        ASSERT_TRUE(v.z > -EPS);
        sum += v;
        grid[v.xy] += 1.0;
    }

    EXPECT_EQ(sum.xy/N, approx(real2(R0)).epsilon(2*CONF));
    ASSERT_TRUE(grid.check_all(N, CONF));
}
TEST(RandomTest, sphere_cap) {
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
            EXPECT_EQ(length(v), approx(1));
            ASSERT_TRUE(v.z > ct - EPS);
            sum += v;
            grid[v] += 1.0;
        }

        EXPECT_EQ(sum.xy/N, approx(real2(R0)).epsilon(2*CONF));
        ASSERT_TRUE(grid.check_all(N, CONF));
    }
}

#endif // UNITTEST
