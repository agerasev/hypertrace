#include "vector.hh"


#ifdef TEST_UNIT

#include <catch.hpp>


TEST_CASE("Vector types", "[vec]") {
    SECTION("Contruction") {
        auto a = MAKE(int3)(-1);
        for (int i = 0; i < a.SIZE; ++i) {
            REQUIRE(a[i] == -1);
        }
        auto b = MAKE(int4)(0, 1, 2, 3);
        for (int i = 0; i < b.SIZE; ++i) {
            REQUIRE(b[i] == i);
        }
        auto c = int4(int3(0, int2(1, 2)), 3);
        for (int i = 0; i < c.SIZE; ++i) {
            REQUIRE(c[i] == i);
        }
    }
    SECTION("Field alignment") {
        vec<int, 2> a2;
        a2[0] = 123456;
        REQUIRE(a2.x == 123456);
        a2.y = 654321;
        REQUIRE(a2[1] == 654321);

        vec<int, 4> a4;
        a4.yz = vec<int, 2>(1, 2);
        REQUIRE(a4[1] == 1);
        REQUIRE(a4[2] == 2);

        vec<int, 8> a8(0,1,2,3,4,5,6,7);
        REQUIRE(a8.s0123 == (vec<int, 4>(0,1,2,3)));
        REQUIRE(a8.s4567 == (vec<int, 4>(4,5,6,7)));

        vec<int, 16> a16(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15);
        REQUIRE(a16.s01234567 == (vec<int, 8>(0,1,2,3,4,5,6,7)));
        REQUIRE(a16.s89abcdef == (vec<int, 8>(8,9,10,11,12,13,14,15)));
        REQUIRE(a16.s0123 == (vec<int, 4>(0,1,2,3)));
        REQUIRE(a16.s4567 == (vec<int, 4>(4,5,6,7)));
        REQUIRE(a16.s89ab == (vec<int, 4>(8,9,10,11)));
        REQUIRE(a16.scdef == (vec<int, 4>(12,13,14,15)));
    }
    SECTION("Compare") {
        REQUIRE(real4(1, 2, 3, 4) == approx(real4(1, 2, 3, 4)));
    }
    SECTION("Norm") {
        REQUIRE(length(real2(3, -4)) == approx(5));
    }
    SECTION("Abs") {
        REQUIRE(fabs(real4(1, -2, 3, -4)) == approx(real4(1, 2, 3, 4)));
    }
    SECTION("Fract") {
        real4 i;
        real4 p = fract(real4(1.1, -1.8, 3.3, -3.6), &i);
        REQUIRE(p == approx(real4(0.1, 0.2, 0.3, 0.4)));
        REQUIRE(i == approx(real4(1, -2, 3, -4)));
    }
};

#endif // TEST_UNIT


#ifdef TEST_DEV

#include <vector>

#include <host/opencl/search.hpp>
#include <host/opencl/opencl.hpp>


REG_TEST_DEV("Vector on device", "[vec_dev]") {
    std::vector<cl_platform_id> platforms = cl::get_platforms();
    std::vector<std::vector<cl_device_id>> platform_devices(platforms.size());
    for (size_t i = 0; i < platforms.size(); ++i) {
        cl_platform_id platform = platforms[i];
        std::vector<cl_device_id> devices = cl::get_devices(platform);

        for (size_t j = 0; j < devices.size(); ++j) {
            cl_device_id device = devices[j];

            cl::Context context(device);
            cl::Queue queue(context, device);

            cl::Program program(context, device, " \
                __kernel void square(__global const int *ibuf, __global int *obuf) { \
                    int i = get_global_id(0); \
                    int x = ibuf[i]; \
                    obuf[i] = x*x; \
                } \
            ");
            cl::Kernel kernel(program, "square");

            const int n = 256;
            cl::Buffer ibuf(queue, n*sizeof(int)), obuf(queue, n*sizeof(int), true);
            std::vector<int> host_buf(n);
            for (int k = 0; k < host_buf.size(); ++n) {
                host_buf[i] = k;
            }
            ibuf.store(queue, host_buf.data());
            kernel.set_arg(0, ibuf);
            kernel.set_arg(1, obuf);
            kernel.run(queue, n);
            obuf.load(queue, host_buf.data());
            for(int k = 0; k < host_buf.size(); ++n) {
                REQUIRE(k*k == host_buf[i]);
            }
        }

        platform_devices[i] = std::move(devices);
    }
};

#endif // TEST_DEV