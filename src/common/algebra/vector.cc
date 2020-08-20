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


#ifdef _TEST_DEV

#include <vector>
#include <cassert>

#include <host/opencl/search.hpp>
#include <host/opencl/opencl.hpp>


void test_dev_vector(
    cl_device_id device,
    cl_context context,
    cl_command_queue queue
) {
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
    for (int i = 0; i < int(host_buf.size()); ++i) {
        host_buf[i] = i;
    }
    ibuf.store(queue, host_buf.data());
    kernel.set_arg(0, ibuf);
    kernel.set_arg(1, obuf);
    kernel.run(queue, n);
    obuf.load(queue, host_buf.data());
    for(int i = 0; i < int(host_buf.size()); ++i) {
        assert(i*i == host_buf[i]);
    }
}

#endif // TEST_DEV
