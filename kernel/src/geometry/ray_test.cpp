#ifdef UNITTEST

#include <gtest/gtest.h>

#define $Geo Eu
#define $geo eu
#include "ray.inl"
#undef $Geo
#undef $geo

TEST(RayEuTest, advance) {
    RayEu ray;
    ray.start = eu_origin();
    ray.direction = MAKE(real3)(R0, R0, R1);

    RayEu new_ray = ray_eu_advance(ray, (real)2);
    ASSERT_EQ(new_ray.start, approx(MAKE(real3)(R0, R0, (real)2)));
}

#define $Geo Hy
#define $geo hy
#include "ray.inl"
#undef $Geo
#undef $geo

TEST(RayHyTest, advance) {
    RayHy ray;
    ray.start = hy_origin();
    ray.direction = MAKE(quat)(R0, R0, R1, R0);

    RayHy new_ray = ray_hy_advance(ray, (real)2);
    ASSERT_EQ(new_ray.start, approx(MAKE(quat)(R0, R0, exp((real)2), R0)));
}

#endif // UNITEST
