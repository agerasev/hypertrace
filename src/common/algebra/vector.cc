#include "vector.hh"


#ifdef TEST_UNIT

#include <rtest.hpp>

rtest_module_(vector) {
    rtest_(approx) {
        assert_eq_(
            MAKE(real2)(R1 + EPS/2, R0 - EPS/2),
            approx(MAKE(real2)(R1, R0))
        );
    }
    rtest_(contruction) {
        auto a = MAKE(int3)(-1);
        for (int i = 0; i < a.SIZE; ++i) {
            assert_eq_(a[i], -1);
        }
        auto b = MAKE(int4)(0, 1, 2, 3);
        for (int i = 0; i < b.SIZE; ++i) {
            assert_eq_(b[i], i);
        }
        auto c = int4(int3(0, int2(1, 2)), 3);
        for (int i = 0; i < c.SIZE; ++i) {
            assert_eq_(c[i], i);
        }
    }
    rtest_(field_alignment) {
        vec<int, 2> a2;
        a2[0] = 123456;
        assert_eq_(a2.x, 123456);
        a2.y = 654321;
        assert_eq_(a2[1], 654321);

        vec<int, 4> a4;
        a4.yz = vec<int, 2>(1, 2);
        assert_eq_(a4[1], 1);
        assert_eq_(a4[2], 2);

        vec<int, 8> a8(0,1,2,3,4,5,6,7);
        assert_eq_(a8.s0123, (vec<int, 4>(0,1,2,3)));
        assert_eq_(a8.s4567, (vec<int, 4>(4,5,6,7)));

        vec<int, 16> a16(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15);
        assert_eq_(a16.s01234567, (vec<int, 8>(0,1,2,3,4,5,6,7)));
        assert_eq_(a16.s89abcdef, (vec<int, 8>(8,9,10,11,12,13,14,15)));
        assert_eq_(a16.s0123, (vec<int, 4>(0,1,2,3)));
        assert_eq_(a16.s4567, (vec<int, 4>(4,5,6,7)));
        assert_eq_(a16.s89ab, (vec<int, 4>(8,9,10,11)));
        assert_eq_(a16.scdef, (vec<int, 4>(12,13,14,15)));
    }
    rtest_(compare) {
        assert_eq_(real4(1, 2, 3, 4), approx(real4(1, 2, 3, 4)));
    }
    rtest_(norm) {
        assert_eq_(length(real2(3, -4)), approx(5));
    }
    rtest_(abs) {
        assert_eq_(fabs(real4(1, -2, 3, -4)), approx(real4(1, 2, 3, 4)));
    }
    rtest_(fract) {
        real4 i;
        real4 p = fract(real4(1.1, -1.8, 3.3, -3.6), &i);
        assert_eq_(p, approx(real4(0.1, 0.2, 0.3, 0.4)));
        assert_eq_(i, approx(real4(1, -2, 3, -4)));
    }
}

#endif // TEST_UNIT


#ifdef TEST_DEV

#include <rtest.hpp>

#include <vector>

#include <test/devtest.hpp>

extern_lazy_static_(devtest::Selector, devtest_selector);

rtest_module_(vector) {
    rtest_(square) {
        for (devtest::Target target : *devtest_selector) {
            auto queue = target.make_queue();
            auto kernel = devtest::KernelBuilder(target.device_id(), queue)
            .source("vector.cl", std::string(
                "#include <common/algebra/vector.hh>\n"
                "__kernel void square(__global const real4 *ibuf, __global real4 *obuf) {\n"
                "    int i = get_global_id(0);\n"
                "    real4 x = ibuf[i];\n"
                "    obuf[i] = x*x;\n"
                "}\n"
            ))
            .build("square").unwrap();

            const int n = 256;
            std::vector<real4> ibuf(n), obuf(n);
            for (size_t i = 0; i < ibuf.size(); ++i) {
                ibuf[i] = real4(i, 2*i, 2*i + 1, 3*i);
            }

            devtest::KernelRunner(queue, std::move(kernel))
            .run(n, ibuf, obuf).expect("Kernel run error");

            for(size_t i = 0; i < obuf.size(); ++i) {
                ulong4 x = ulong4(i, 2*i, 2*i + 1, 3*i);
                assert_eq_(x*x, obuf[i].convert<ulong>());
            }
        }
    }
}

#endif // TEST_DEV
