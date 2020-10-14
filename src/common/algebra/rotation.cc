#include "rotation.hh"


Rotation2 rot2_identity() {
    return C1;
}
Rotation2 rot2_from_angle(real angle) {
    return c_new(cos(angle), sin(angle));
}
Rotation2 rot2_look_at(real2 pos) {
    return normalize(c_new(pos.x, pos.y));
}

real2 rot2_apply(Rotation2 m, real2 pos) {
    return c_mul(m, pos);
}
Rotation2 rot2_chain(Rotation2 a, Rotation2 b) {
    return c_mul(a, b);
}
Rotation2 rot2_inverse(Rotation2 m) {
    return c_inverse(m);
}
Linear2 rot2_to_linear(Rotation2 m) {
    return r22_transpose(r22_from_comp(m));
}


Rotation3 rot3_identity() {
    return Q1;
}
Rotation3 rot3_from_axis(real3 axis, real angle) {
    return q_new(cos(angle/2), sin(angle/2)*axis);
}

real3 rot3_apply(Rotation3 m, real3 p) {
    return q_mul(q_mul(m, q_new(R0, p)), q_conj(m)).yzw;
}
Rotation3 rot3_chain(Rotation3 a, Rotation3 b) {
    return q_mul(a, b);
}
Rotation3 rot3_inverse(Rotation3 m) {
    return q_conj(m);
}
Linear3 rot3_to_linear(Rotation3 m) {
    return r33_new(
        1 - 2*m.z*m.z - 2*m.w*m.w,
        2*m.y*m.z - 2*m.w*m.x,
        2*m.y*m.w + 2*m.z*m.x,
        R0,

        2*m.y*m.z + 2*m.w*m.x,
        1 - 2*m.y*m.y - 2*m.w*m.w,
        2*m.z*m.w - 2*m.y*m.x,
        R0,

        2*m.y*m.w - 2*m.z*m.x,
        2*m.z*m.w + 2*m.y*m.x,
        1 - 2*m.y*m.y - 2*m.z*m.z,
        R0,

        R0, R0, R0, R1
    );
}


#ifdef TEST

Rotation2 TestRngRotation2::uniform() {
    return rot2_from_angle(2*PI*rng.uniform());
}

Rotation3 TestRngRotation3::uniform() {
    return rot3_from_axis(
        vrng.unit(),
        2*PI*rng.uniform()
    );
}

#include <rtest.hpp>

#ifdef TEST_UNIT

rtest_module_(rotation_2d) {
    static_thread_local_(TestRng<real>, rng) {
        return TestRng<real>(0x807A);
    }
    static_thread_local_(TestRng<real2>, r2rng) {
        return TestRng<real2>(0x807A);
    }
    static_thread_local_(TestRngRotation2, rot2rng) {
        return TestRngRotation2(0x807A);
    }

    rtest_(mapping) {
        for (int k = 0; k < TEST_ATTEMPTS; ++k) {
            real angle = 2*PI*rng->uniform();
            Rotation2 r = rot2_from_angle(angle);
            real2 a = r2rng->normal();
            real2 b = rot2_apply(r, a);
            assert_eq_(length(a), approx(length(b)));
            assert_eq_(dot(a, b)/length2(a), approx(cos(angle)));
        }
    }
    rtest_(chaining) {
        for (int k = 0; k < TEST_ATTEMPTS; ++k) {
            Rotation2 a = rot2rng->uniform();
            Rotation2 b = rot2rng->uniform();
            real2 v = r2rng->normal();
            assert_eq_(
                rot2_apply(rot2_chain(a, b), v), 
                approx(rot2_apply(a, rot2_apply(b, v)))
            );
        }
    }
    rtest_(inversion) {
        for (int k = 0; k < TEST_ATTEMPTS; ++k) {
            Rotation2 r = rot2rng->uniform();
            assert_eq_(rot2_chain(r, rot2_inverse(r)), approx(rot2_identity()));
        }
    }
    rtest_(to_linear) {
        for (int k = 0; k < TEST_ATTEMPTS; ++k) {
            Rotation2 a = rot2rng->uniform();
            Rotation2 b = rot2rng->uniform();
            real2 x = r2rng->normal();
            assert_eq_(lin2_apply(rot2_to_linear(a), x), approx(rot2_apply(a, x)));
            assert_eq_(
                lin2_chain(rot2_to_linear(a), rot2_to_linear(b)),
                approx(rot2_to_linear(rot2_chain(a, b)))
            );
        }
    }
}

rtest_module_(rotation_3d) {
    static_thread_local_(TestRng<real>, rng) {
        return TestRng<real>(0x807A);
    }
    static_thread_local_(TestRng<real3>, r3rng) {
        return TestRng<real3>(0x807A);
    }
    static_thread_local_(TestRngRotation3, rot3rng) {
        return TestRngRotation3(0x807A);
    }

    rtest_(mapping) {
        for (int k = 0; k < TEST_ATTEMPTS; ++k) {
            real3 axis = r3rng->unit();
            real angle = 2*PI*rng->uniform();
            Rotation3 r = rot3_from_axis(axis, angle);
            real3 a = r3rng->normal();
            real3 b = rot3_apply(r, a);
            assert_eq_(length(a), approx(length(b)));
            a -= dot(a, axis)*axis;
            b -= dot(b, axis)*axis;
            real aa = length2(a);
            assert_eq_(dot(a, b)/aa, approx(cos(angle)));
            assert_eq_(cross(a, b)/aa, approx(axis*sin(angle)));
        }
    }
    rtest_(chaining) {
        for (int k = 0; k < TEST_ATTEMPTS; ++k) {
            Rotation3 a = rot3rng->uniform();
            Rotation3 b = rot3rng->uniform();
            real3 v = r3rng->normal();
            assert_eq_(
                rot3_apply(rot3_chain(a, b), v),
                approx(rot3_apply(a, rot3_apply(b, v)))
            );
        }
    }
    rtest_(inversion) {
        for (int k = 0; k < TEST_ATTEMPTS; ++k) {
            Rotation3 r = rot3rng->uniform();
            assert_eq_(rot3_chain(r, rot3_inverse(r)), approx(rot3_identity()));
        }
    }
    rtest_(to_linear) {
        for (int k = 0; k < TEST_ATTEMPTS; ++k) {
            Rotation3 a = rot3rng->uniform();
            Rotation3 b = rot3rng->uniform();
            real3 x = r3rng->normal();
            assert_eq_(lin3_apply(rot3_to_linear(a), x), approx(rot3_apply(a, x)));
            assert_eq_(
                lin3_chain(rot3_to_linear(a), rot3_to_linear(b)),
                approx(rot3_to_linear(rot3_chain(a, b)))
            );
        }
    }
}

#endif // TEST_UNIT

#ifdef TEST_DEV

#include <rtest.hpp>

#include <vector>

#include <test/devtest.hpp>

extern_lazy_static_(devtest::Selector, devtest_selector);

rtest_module_(rotation) {
    rtest_(chain) {
        TestRngRotation3 rrng(0xcafe);

        for (devtest::Target target : *devtest_selector) {
            auto queue = target.make_queue();
            auto kernel = devtest::KernelBuilder(target.device_id(), queue)
            .source("rotation.cl", std::string(
                "#include <common/algebra/rotation.hh>\n"
                "__kernel void chain(__global const Rotation3 *x, __global const Rotation3 *y, __global Rotation3 *z) {\n"
                "    int i = get_global_id(0);\n"
                "    z[i] = rot3_chain(x[i], y[i]);\n"
                "}\n"
            ))
            .build("chain").unwrap();

            const int n = TEST_ATTEMPTS;
            std::vector<Rotation3> xbuf(n), ybuf(n), zbuf(n);
            for (size_t i = 0; i < n; ++i) {
                xbuf[i] = rrng.uniform();
                ybuf[i] = rrng.uniform();
            }

            devtest::KernelRunner(queue, std::move(kernel))
            .run(n, xbuf, ybuf, zbuf).expect("Kernel run error");

            for(size_t i = 0; i < n; ++i) {
                Rotation3 z = rot3_chain(xbuf[i], ybuf[i]);
                assert_eq_(dev_approx(z), zbuf[i]);
            }
        }
    }
}

#endif // TEST_DEV

#endif // TEST
